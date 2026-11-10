// Acumular as amostras do traçado de raios com a cena parada: um triângulo que cobre a tela
// copia a textura de origem pixel a pixel. Desenhado na textura da média com a mistura
// constante (1/n da amostra nova), e depois da média para a saída.

@group(0) @binding(0) var source: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    return textureLoad(source, vec2<i32>(pos.xy), 0);
}
