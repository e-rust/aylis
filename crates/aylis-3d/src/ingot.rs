//! O lingote do logo do aylis em 3D (o da abertura da IDE): paredes de metal escovado (os
//! riscos num mapa de normais), a brasa do topo como emissão, as quinas chanfradas e as luzes
//! paradas, para o reflexo andar pelas faces enquanto ele gira. [`model`] monta o modelo e
//! [`apply`] põe no `Model3D` a câmera, o giro e o brilho da abertura: assim a IDE, os
//! exemplos e os testes de celular mostram exatamente o mesmo lingote.

use glam::{Quat, Vec3};

use crate::model::{Material, Mesh, Node, Primitive, Texture, Vertex};
use crate::{Camera, Light, LightKind, Model, Model3D, Projection};

/// Giro sozinho, em graus por segundo.
pub const SPIN: f32 = 52.0;
/// A inclinação da câmera do logo (o "telhado" das arestas de cima sobe 119 pontos em 186
/// de largura: o seno dela).
const PITCH: f32 = 0.692;
/// A câmera do lingote, a meia altura da vista dela no centro do lingote (no mundo) e a
/// abertura: uma lente longa, com perspectiva leve. Vista de baixo, a base fica um pouco mais
/// larga que o topo, como num objeto de verdade (sem perspectiva, as paredes paralelas
/// parecem afinar para baixo).
pub const CAMERA: &str = "Câmera";
const VIEW_HALF: f32 = 1.69;
pub const FOV: f32 = 22.0;
/// A força do brilho da brasa.
pub const GLOW: f32 = 1.8;
/// A emissão das pontas (as paredes emitem 1,1): bem mais quentes que as arestas.
const CORNER_HEAT: f32 = 4.5;
/// Meia altura do lingote (a base tem lado 1).
const HALF_HEIGHT: f32 = 1.1;
/// Onde as paredes começam a sumir no fundo e o pé, onde já sumiram (como o logo, que não tem
/// base): a transparência é do modelo, então a abertura da IDE, o celular e a página mostram o
/// mesmo lingote curto, com qualquer câmera e qualquer fundo. Na câmera do logo, na abertura,
/// vão de 42% a 72% da altura da janela.
const FADE_TOP: f32 = 0.13;
const FOOT: f32 = -0.76;

/// Liga o lingote num `Model3D` como na abertura: a câmera do logo, de quina para a frente,
/// girando sozinho debaixo das luzes, sem sombra, com o brilho da brasa.
pub fn apply(view: &mut Model3D) {
    view.set_model(model());
    view.camera = CAMERA.into();
    view.model_rotation = 45.0;
    view.auto_rotate = SPIN;
    view.orbit = false;
    // Sem órbita, o arrasto do dedo rola a tela por baixo (como um enfeite).
    view.base.set_pan_on_touch(true);
    view.shadows = false;
    view.turntable = true;
    // O halo da brasa: a luz que ela emite espalhada em volta (bloom), como no logo.
    view.glow = GLOW;
}

/// A face escura do logo, endireitada num retângulo (sem o filete da aresta): os riscos e a
/// brasa das paredes vêm dela.
const WALL: &[u8] = include_bytes!("../assets/ingot-wall.png");
/// O chanfro das quinas verticais: pega a luz quando a quina está de frente (o filete branco
/// do logo) e fica apagado quando ela está de lado.
const CHAMFER: f32 = 0.012;

/// O lingote: quatro paredes com o metal e a brasa do logo, as quinas chanfradas, o topo em
/// brasa, o pé sumindo no fundo e as luzes paradas (a forte à esquerda, que clareia a face da
/// esquerda como no logo; a da frente, que faz o reflexo andar e acende o chanfro da quina
/// da frente; e a laranja logo acima do topo).
pub fn model() -> Model {
    let h = HALF_HEIGHT;
    // Em volta, de fora: cada parede vai de uma quina à seguinte, da esquerda para a direita.
    let corners = [Vec3::new(-0.5, 0.0, 0.5), Vec3::new(0.5, 0.0, 0.5), Vec3::new(0.5, 0.0, -0.5), Vec3::new(-0.5, 0.0, -0.5)];
    let vertex = |p: Vec3, y: f32, n: Vec3, uv: [f32; 2], alpha: f32| Vertex { position: [p.x, y, p.z], normal: n.into(), uv, color: [1.0, 1.0, 1.0, alpha], ..Vertex::default() };
    // A face do logo vai do topo (`h`) ao pé da aresta dele (`-h`).
    let v = |y: f32| (h - y) / (2.0 * h) * 0.99;
    // `u`: as colunas da textura. O chanfro é uma tira fina: com a textura inteira nela, a placa
    // usaria o mipmap mais reduzido (a média do degradê, com a brasa) e o pé ficava em brasa.
    let quad = |prim: &mut Primitive, a: Vec3, b: Vec3, n: Vec3, (u0, u1): (f32, f32)| {
        // Opaca até `FADE_TOP`; dali ao pé, sumindo no fundo.
        for (top, bottom, a0, a1) in [(h, FADE_TOP, 1.0, 1.0), (FADE_TOP, FOOT, 1.0, 0.0)] {
            let base = prim.vertices.len() as u32;
            prim.vertices.extend([
                vertex(a, top, n, [u0, v(top)], a0),
                vertex(b, top, n, [u1, v(top)], a0),
                vertex(b, bottom, n, [u1, v(bottom)], a1),
                vertex(a, bottom, n, [u0, v(bottom)], a1),
            ]);
            prim.indices.extend([base, base + 3, base + 2, base, base + 2, base + 1]);
        }
    };

    let (mut walls, mut bevels) = (Primitive { material: 0, ..Primitive::default() }, Primitive { material: 2, ..Primitive::default() });
    for k in 0..4 {
        let (a, b) = (corners[k], corners[(k + 1) % 4]);
        let along = (b - a).normalize();
        quad(&mut walls, a + along * CHAMFER, b - along * CHAMFER, (a + b).normalize(), (0.0, 1.0));
        // O chanfro da quina `b`, até o começo da parede seguinte.
        let next = (corners[(k + 2) % 4] - b).normalize();
        quad(&mut bevels, b - along * CHAMFER, b + next * CHAMFER, b.normalize(), (0.5, 0.5));
    }
    let cap = |y: f32, material: usize| {
        let n = Vec3::new(0.0, y.signum(), 0.0);
        Primitive {
            vertices: corners.iter().map(|&p| vertex(p, y, n, [p.x + 0.5, p.z + 0.5], 1.0)).collect(),
            indices: vec![0, 1, 2, 0, 2, 3],
            material,
            ..Primitive::default()
        }
    };

    let (wall_color, wall_ember) = wall_textures();
    let materials = vec![
        // O metal do logo, com a brasa dele (emissão) no alto. Transparente (o pé some no
        // fundo) e só de um lado: por dentro, as paredes de trás apareceriam pelo pé.
        Material {
            base_color: [1.0, 1.0, 1.0, 1.0],
            texture: Some(0),
            metallic: 0.9,
            roughness: 0.3,
            emissive: [1.1, 1.1, 1.1],
            emissive_texture: Some(1),
            blend: true,
            ..Material::default()
        },
        // O topo em brasa, só por cima: por baixo, o brilho dele vazava pelas paredes, que são
        // transparentes.
        Material { base_color: [0.2, 0.1, 0.05, 1.0], emissive: [4.0, 1.7, 0.35], ..Material::default() },
        // O chanfro: metal polido; no alto, as pontas, a parte mais quente do lingote (o brilho
        // espalha mais luz em volta delas que das arestas).
        Material {
            base_color: [0.95, 0.95, 0.97, 1.0],
            metallic: 1.0,
            roughness: 0.12,
            emissive: [CORNER_HEAT, CORNER_HEAT, CORNER_HEAT],
            emissive_texture: Some(2),
            blend: true,
            ..Material::default()
        },
    ];

    let node = |name: &str, translation: Vec3, mesh: Option<usize>| Node {
        name: name.into(),
        parent: None,
        children: Vec::new(),
        translation,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
        mesh,
        skin: None,
        weights: Vec::new(),
    };
    // A câmera do logo: um pouco abaixo, olhando para o centro com a inclinação dele, longe o
    // bastante para a vista ter `VIEW_HALF` de meia altura ali.
    let (sp, cp) = PITCH.sin_cos();
    let mut camera = node("Câmera", Vec3::new(0.0, -sp, cp) * camera_distance(), None);
    camera.rotation = Quat::from_rotation_x(PITCH);
    let nodes = vec![
        node("Lingote", Vec3::ZERO, Some(0)),
        // Onde o reflexo da face da esquerda cai junto da quina da frente, como no logo (o
        // espelho da câmera pela face, passando pela quina um pouco acima do meio).
        node("Luz", Vec3::new(-2.2, 3.4, 2.6), None),
        node("Frente", Vec3::new(0.3, -0.4, 3.2), None),
        node("Brasa", Vec3::new(0.0, h + 0.35, 0.25), None),
        camera,
    ];
    let point = |name: &str, node: usize, color: [f32; 3], intensity: f32, range: Option<f32>| Light { name: name.into(), node, kind: LightKind::Point, color, intensity, range };
    let lights = vec![
        point("Luz", 1, [1.0, 0.97, 0.94], 45.0, None),
        point("Frente", 2, [1.0, 0.98, 0.96], 0.6, None),
        point("Brasa", 3, [1.0, 0.45, 0.12], 2.5, Some(2.5)),
    ];

    Model {
        nodes,
        roots: vec![0, 1, 2, 3, 4],
        order: vec![0, 1, 2, 3, 4],
        cameras: vec![Camera { name: CAMERA.into(), node: 4, projection: Projection::Perspective { yfov: FOV.to_radians(), aspect: None, znear: 0.5, zfar: Some(40.0) } }],
        meshes: vec![Mesh { name: "Lingote".into(), primitives: vec![walls, cap(h, 1), bevels], weights: Vec::new() }],
        materials,
        textures: vec![wall_color, wall_ember, corner_ember()],
        lights,
        bounds: (Vec3::new(-0.5, FOOT, -0.5), Vec3::new(0.5, h, 0.5)),
        ..Model::default()
    }
}

/// A distância da câmera ao centro do lingote para a vista ter `VIEW_HALF` de meia altura
/// ali, com a abertura `FOV`.
fn camera_distance() -> f32 {
    VIEW_HALF / (FOV.to_radians() / 2.0).tan()
}

/// As texturas das paredes, a partir da face do logo: a cor do metal (só o relevo dos riscos,
/// sobre um metal neutro, porque a luz do logo já vem pintada nela e aqui quem ilumina é a
/// cena; escura onde está em brasa, para o reflexo não lavar o laranja) e a brasa (uma faixa
/// igual em toda a largura, como no logo, com o mesmo relevo).
fn wall_textures() -> (Texture, Texture) {
    let Ok(img) = image::load_from_memory(WALL) else {
        return (vertical(&[(0.0, [0.6, 0.6, 0.6]), (1.0, [0.6, 0.6, 0.6])]), ember());
    };
    let img = img.to_rgba8();
    let (w, h) = img.dimensions();
    let lum = |x: u32, y: u32| {
        let p = img.get_pixel(x, y).0;
        0.3 * f32::from(p[0]) + 0.59 * f32::from(p[1]) + 0.11 * f32::from(p[2])
    };
    let ember = ember();
    let mut color = Vec::with_capacity((w * h * 4) as usize);
    let mut glow = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        let v = y as f32 / (h - 1) as f32;
        let mean = (0..w).map(|x| lum(x, y)).sum::<f32>() / w as f32;
        // Onde está em brasa o metal escurece.
        let heat = 0.25 + 0.75 * ((v - 0.04) / 0.17).clamp(0.0, 1.0);
        let row = ((v * (ember.height - 1) as f32).round() as usize) * 8;
        let hot = &ember.rgba[row..row + 3];
        for x in 0..w {
            let detail = (lum(x, y) / mean.max(1.0)).clamp(0.6, 1.4);
            let g = (0.38 * detail * heat * 255.0).round() as u8;
            color.extend([g, g, g, 255]);
            // Mais quente perto das quinas que no meio da aresta.
            let u = x as f32 / (w - 1) as f32;
            let corner = (-(u.min(1.0 - u) / 0.14).powi(2)).exp();
            let d = (0.75 + 0.25 * detail) * (0.72 + 0.28 * corner);
            glow.extend(hot.iter().map(|&c| (f32::from(c) * d).min(255.0).round() as u8));
            glow.push(255);
        }
    }
    (Texture { width: w, height: h, rgba: color }, Texture { width: w, height: h, rgba: glow })
}

/// A brasa (das paredes e do chanfro): do branco quente na aresta de cima ao laranja do logo e
/// ao vermelho, até apagar (em sRGB; o material multiplica pela emissão).
fn ember() -> Texture {
    vertical(&[
        (0.0, [1.0, 0.62, 0.30]),
        (0.008, [1.0, 0.36, 0.03]),
        (0.03, [0.95, 0.27, 0.01]),
        (0.08, [0.78, 0.18, 0.01]),
        (0.15, [0.42, 0.09, 0.01]),
        (0.22, [0.13, 0.04, 0.01]),
        (0.28, [0.0, 0.0, 0.0]),
    ])
}

/// A brasa das pontas (no alto dos chanfros): curta e quente.
fn corner_ember() -> Texture {
    vertical(&[(0.0, [1.0, 0.80, 0.50]), (0.01, [1.0, 0.55, 0.15]), (0.04, [1.0, 0.35, 0.03]), (0.09, [0.50, 0.10, 0.0]), (0.14, [0.0, 0.0, 0.0])])
}

/// Uma textura de 2 colunas com o degradê de cima (0) para baixo (1).
fn vertical(stops: &[(f32, [f32; 3])]) -> Texture {
    let h = 256u32;
    let mut rgba = Vec::with_capacity((h * 2 * 4) as usize);
    for y in 0..h {
        let t = y as f32 / (h - 1) as f32;
        let i = stops.iter().position(|&(at, _)| at >= t).unwrap_or(stops.len() - 1).max(1);
        let ((a, ca), (b, cb)) = (stops[i - 1], stops[i]);
        let f = ((t - a) / (b - a)).clamp(0.0, 1.0);
        let c = [0, 1, 2].map(|k| ((ca[k] + (cb[k] - ca[k]) * f) * 255.0).round() as u8);
        for _ in 0..2 {
            rgba.extend(c);
            rgba.push(255);
        }
    }
    Texture { width: 2, height: h, rgba }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// O lingote é um modelo válido: as paredes, o topo e os chanfros, com as normais para
    /// fora e as texturas que os materiais citam.
    #[test]
    fn ingot_model_is_complete() {
        let m = model();
        let prims = &m.meshes[0].primitives;
        assert_eq!(prims.len(), 3);
        assert_eq!(prims[0].vertices.len(), 32);
        for v in &prims[0].vertices {
            let (p, n) = (Vec3::from(v.position), Vec3::from(v.normal));
            assert!(Vec3::new(p.x, 0.0, p.z).dot(n) > 0.0, "normal para dentro em {p:?}");
        }
        for mat in &m.materials {
            for t in [mat.texture, mat.normal_texture, mat.emissive_texture].into_iter().flatten() {
                let tex = &m.textures[t];
                assert_eq!(tex.rgba.len(), (tex.width * tex.height * 4) as usize);
            }
        }
        assert_eq!(m.lights.len(), 3);
        assert!(m.order.iter().all(|&i| i < m.nodes.len()));
    }}
