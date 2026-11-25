//! O compositor da mistura em Direct3D 11 (`blend_compositor = direct3d`): a camada com
//! `blend_mode`, desenhada pelo Direct2D, se junta ao fundo por um shader nosso
//! (`blend.hlsl`, as fórmulas do W3C) em vez do efeito Blend do Direct2D. O resultado é o
//! mesmo; o que muda é quem faz a conta, que passa a ser código do Aylis: a base para efeitos
//! que o Direct2D não tem.
//!
//! Só vale quando o alvo é uma textura do Direct3D (a janela e a captura na placa) e os
//! recortes abertos são retângulos alinhados (viram o *scissor*); senão, o efeito do Direct2D
//! (veja `D2dCanvas::compose_blend`).

use std::sync::OnceLock;

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D::Fxc::{D3DCOMPILE_OPTIMIZATION_LEVEL3, D3DCompile};
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows_core::{Interface, Result, s};

/// O código dos dois shaders, compilado uma vez no processo (o `D3DCompile` do
/// `d3dcompiler_47.dll`, que vem no Windows). `None`: não compilou.
fn bytecode() -> Option<&'static (Vec<u8>, Vec<u8>)> {
    static CODE: OnceLock<Option<(Vec<u8>, Vec<u8>)>> = OnceLock::new();
    CODE.get_or_init(|| Some((compile(s!("vs_main"), s!("vs_4_0"))?, compile(s!("ps_main"), s!("ps_4_0"))?))).as_ref()
}

fn compile(entry: windows_core::PCSTR, target: windows_core::PCSTR) -> Option<Vec<u8>> {
    const SOURCE: &str = include_str!("blend.hlsl");
    let mut code = None;
    let mut errors = None;
    // SAFETY: o texto e os ponteiros de saída vivem até a volta; o blob devolvido é lido pelo
    // tamanho que ele mesmo informa.
    unsafe {
        let done = D3DCompile(
            SOURCE.as_ptr().cast(),
            SOURCE.len(),
            s!("blend.hlsl"),
            None,
            None,
            entry,
            target,
            D3DCOMPILE_OPTIMIZATION_LEVEL3,
            0,
            &mut code,
            Some(&mut errors),
        );
        if done.is_err() {
            if let Some(e) = errors {
                let text = std::slice::from_raw_parts(e.GetBufferPointer() as *const u8, e.GetBufferSize());
                eprintln!("aylis-d2d: o shader da mistura não compilou: {}", String::from_utf8_lossy(text));
            }
            return None;
        }
        let code: ID3DBlob = code?;
        Some(std::slice::from_raw_parts(code.GetBufferPointer() as *const u8, code.GetBufferSize()).to_vec())
    }
}

/// Os parâmetros do `cbuffer Params` (32 bytes, alinhados a 16).
#[repr(C)]
#[derive(Clone, Copy)]
struct Params {
    area: [f32; 4],
    target_size: [f32; 2],
    mode: u32,
    opacity: f32,
}

/// As peças do Direct3D para compor num dispositivo: os shaders, os parâmetros, e o que é
/// reaproveitado de uma camada para a outra (o fundo por tamanho, as vistas por textura).
pub(crate) struct Compositor {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    vs: ID3D11VertexShader,
    ps: ID3D11PixelShader,
    params: ID3D11Buffer,
    rasterizer: ID3D11RasterizerState,
    /// As cópias do fundo, por tamanho e formato.
    backdrops: Vec<(u32, u32, DXGI_FORMAT, ID3D11Texture2D, ID3D11ShaderResourceView)>,
    /// As vistas de leitura das camadas e de escrita dos alvos, pela textura (guardada junto:
    /// outra textura no mesmo endereço não pega a vista errada).
    reads: Vec<(ID3D11Texture2D, ID3D11ShaderResourceView)>,
    writes: Vec<(ID3D11Texture2D, ID3D11RenderTargetView)>,
}

/// Quantas peças de cada tipo ficam guardadas.
const KEEP: usize = 16;

impl Compositor {
    /// As peças no dispositivo de `texture`. `None`: o shader não compilou ou a placa recusou.
    pub(crate) fn new(texture: &ID3D11Texture2D) -> Option<Self> {
        let (vs_code, ps_code) = bytecode()?;
        // SAFETY: chamadas COM do Direct3D com descrições locais; os ponteiros de saída são
        // locais.
        unsafe {
            let device = texture.GetDevice().ok()?;
            let context = device.GetImmediateContext().ok()?;
            let mut vs = None;
            device.CreateVertexShader(vs_code, None, Some(&mut vs)).ok()?;
            let mut ps = None;
            device.CreatePixelShader(ps_code, None, Some(&mut ps)).ok()?;
            let desc = D3D11_BUFFER_DESC {
                ByteWidth: std::mem::size_of::<Params>() as u32,
                Usage: D3D11_USAGE_DEFAULT,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                ..Default::default()
            };
            let mut params = None;
            device.CreateBuffer(&desc, None, Some(&mut params)).ok()?;
            let raster = D3D11_RASTERIZER_DESC {
                FillMode: D3D11_FILL_SOLID,
                CullMode: D3D11_CULL_NONE,
                ScissorEnable: true.into(),
                DepthClipEnable: true.into(),
                ..Default::default()
            };
            let mut rasterizer = None;
            device.CreateRasterizerState(&raster, Some(&mut rasterizer)).ok()?;
            Some(Self {
                device,
                context,
                vs: vs?,
                ps: ps?,
                params: params?,
                rasterizer: rasterizer?,
                backdrops: Vec::new(),
                reads: Vec::new(),
                writes: Vec::new(),
            })
        }
    }

    /// Se as peças são do dispositivo de `texture` (depois de trocar de placa, não são).
    pub(crate) fn serves(&self, texture: &ID3D11Texture2D) -> bool {
        // SAFETY: só pergunta o dispositivo da textura.
        unsafe { texture.GetDevice() }.is_ok_and(|d| d.as_raw() == self.device.as_raw())
    }

    fn read_view(&mut self, texture: &ID3D11Texture2D) -> Result<ID3D11ShaderResourceView> {
        if let Some((_, v)) = self.reads.iter().find(|(t, _)| t.as_raw() == texture.as_raw()) {
            return Ok(v.clone());
        }
        let mut view = None;
        // SAFETY: chamada COM do Direct3D; a vista padrão da textura inteira.
        unsafe { self.device.CreateShaderResourceView(texture, None, Some(&mut view))? };
        let view = view.ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_FAIL))?;
        if self.reads.len() >= KEEP {
            self.reads.remove(0);
        }
        self.reads.push((texture.clone(), view.clone()));
        Ok(view)
    }

    fn write_view(&mut self, texture: &ID3D11Texture2D) -> Result<ID3D11RenderTargetView> {
        if let Some((_, v)) = self.writes.iter().find(|(t, _)| t.as_raw() == texture.as_raw()) {
            return Ok(v.clone());
        }
        let mut view = None;
        // SAFETY: chamada COM do Direct3D; a vista padrão da textura inteira.
        unsafe { self.device.CreateRenderTargetView(texture, None, Some(&mut view))? };
        let view = view.ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_FAIL))?;
        if self.writes.len() >= KEEP {
            self.writes.remove(0);
        }
        self.writes.push((texture.clone(), view.clone()));
        Ok(view)
    }

    /// A cópia do fundo de `w × h` no formato `format` (reaproveitada: a placa ordena a cópia
    /// nova depois da leitura da anterior, no mesmo contexto).
    fn backdrop(&mut self, w: u32, h: u32, format: DXGI_FORMAT) -> Result<(ID3D11Texture2D, ID3D11ShaderResourceView)> {
        if let Some((.., t, v)) = self.backdrops.iter().find(|(bw, bh, f, ..)| (*bw, *bh, *f) == (w, h, format)) {
            return Ok((t.clone(), v.clone()));
        }
        let desc = D3D11_TEXTURE2D_DESC {
            Width: w,
            Height: h,
            MipLevels: 1,
            ArraySize: 1,
            Format: format,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
            ..Default::default()
        };
        let mut texture = None;
        let mut view = None;
        // SAFETY: chamadas COM do Direct3D com a descrição local.
        unsafe {
            self.device.CreateTexture2D(&desc, None, Some(&mut texture))?;
            let texture = texture.as_ref().ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_FAIL))?;
            self.device.CreateShaderResourceView(texture, None, Some(&mut view))?;
        }
        let (texture, view) = (texture.unwrap(), view.ok_or_else(|| windows_core::Error::from(windows::Win32::Foundation::E_FAIL))?);
        if self.backdrops.len() >= KEEP {
            self.backdrops.remove(0);
        }
        self.backdrops.push((w, h, format, texture.clone(), view.clone()));
        Ok((texture, view))
    }

    /// Junta `layer` (do tamanho do pedaço `area`, em pixels de `target`) ao fundo de `target`
    /// no modo `mode` (a ordem do `aylis_core::BlendMode`) com a opacidade `opacity`, só dentro
    /// de `scissor`. Quem chama já mandou à placa o que o Direct2D tinha na fila.
    pub(crate) fn compose(&mut self, target: &ID3D11Texture2D, layer: &ID3D11Texture2D, area: RECT, scissor: RECT, mode: u32, opacity: f32) -> Result<()> {
        let (w, h) = ((area.right - area.left) as u32, (area.bottom - area.top) as u32);
        // SAFETY: chamadas COM do Direct3D no contexto imediato, com descrições e vistas
        // locais; as vistas soltas no fim para o Direct2D voltar a usar as texturas.
        unsafe {
            let mut desc = D3D11_TEXTURE2D_DESC::default();
            target.GetDesc(&mut desc);
            let (copy, copy_view) = self.backdrop(w, h, desc.Format)?;
            let source = D3D11_BOX { left: area.left as u32, top: area.top as u32, front: 0, right: area.right as u32, bottom: area.bottom as u32, back: 1 };
            self.context.CopySubresourceRegion(&copy, 0, 0, 0, 0, target, 0, Some(&source));
            let layer_view = self.read_view(layer)?;
            let out = self.write_view(target)?;
            let params = Params {
                area: [area.left as f32, area.top as f32, area.right as f32, area.bottom as f32],
                target_size: [desc.Width as f32, desc.Height as f32],
                mode,
                opacity,
            };
            self.context.UpdateSubresource(&self.params, 0, None, &params as *const Params as *const _, 0, 0);
            let c = &self.context;
            c.OMSetRenderTargets(Some(&[Some(out)]), None);
            c.OMSetBlendState(None, None, 0xffff_ffff);
            c.OMSetDepthStencilState(None, 0);
            c.RSSetState(&self.rasterizer);
            c.RSSetViewports(Some(&[D3D11_VIEWPORT { TopLeftX: 0.0, TopLeftY: 0.0, Width: desc.Width as f32, Height: desc.Height as f32, MinDepth: 0.0, MaxDepth: 1.0 }]));
            c.RSSetScissorRects(Some(&[scissor]));
            c.IASetInputLayout(None);
            c.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            c.VSSetShader(&self.vs, None);
            c.VSSetConstantBuffers(0, Some(&[Some(self.params.clone())]));
            c.PSSetShader(&self.ps, None);
            c.PSSetConstantBuffers(0, Some(&[Some(self.params.clone())]));
            c.PSSetShaderResources(0, Some(&[Some(layer_view), Some(copy_view)]));
            c.Draw(4, 0);
            // Solta as texturas: o Direct2D volta a escrever nelas.
            c.PSSetShaderResources(0, Some(&[None, None]));
            c.OMSetRenderTargets(None, None);
        }
        Ok(())
    }
}
