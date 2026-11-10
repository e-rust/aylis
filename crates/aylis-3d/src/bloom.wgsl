// O brilho (bloom): a luz que os materiais emitem, reduzida em níveis (cada um com metade do
// tamanho do anterior) e somada de volta subindo, espalha em volta do que brilha: forte junto
// da fonte e caindo rápido, como a luz forte vista por uma lente. No fim, entra por cima do
// quadro como luz somada (alfa pré-multiplicado, como o resto do aylis-3d).

// Quanto a textura do brilho guarda a menos (com 8 bits ela guarda 1/RANGE da luz, para não
// estourar em 1); a composição multiplica de volta.
override RANGE: f32 = 1.0;

@group(0) @binding(0) var source: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;

struct Vary {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Um triângulo que cobre a tela.
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> Vary {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u));
    return Vary(vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0), vec2<f32>(p.x, 1.0 - p.y));
}

// Descer um nível: o centro e as quatro diagonais (meio texel da origem).
@fragment
fn fs_down(v: Vary) -> @location(0) vec4<f32> {
    let t = 1.0 / vec2<f32>(textureDimensions(source));
    var c = textureSample(source, smp, v.uv) * 4.0;
    c += textureSample(source, smp, v.uv + vec2<f32>(-t.x, -t.y));
    c += textureSample(source, smp, v.uv + vec2<f32>(t.x, -t.y));
    c += textureSample(source, smp, v.uv + vec2<f32>(-t.x, t.y));
    c += textureSample(source, smp, v.uv + vec2<f32>(t.x, t.y));
    return c / 8.0;
}

// Subir um nível: uma tenda em volta (somada por cima do nível de cima pela mistura).
@fragment
fn fs_up(v: Vary) -> @location(0) vec4<f32> {
    let t = 1.0 / vec2<f32>(textureDimensions(source));
    var c = textureSample(source, smp, v.uv + vec2<f32>(-2.0 * t.x, 0.0));
    c += textureSample(source, smp, v.uv + vec2<f32>(2.0 * t.x, 0.0));
    c += textureSample(source, smp, v.uv + vec2<f32>(0.0, -2.0 * t.y));
    c += textureSample(source, smp, v.uv + vec2<f32>(0.0, 2.0 * t.y));
    c += textureSample(source, smp, v.uv + vec2<f32>(-t.x, -t.y)) * 2.0;
    c += textureSample(source, smp, v.uv + vec2<f32>(t.x, -t.y)) * 2.0;
    c += textureSample(source, smp, v.uv + vec2<f32>(-t.x, t.y)) * 2.0;
    c += textureSample(source, smp, v.uv + vec2<f32>(t.x, t.y)) * 2.0;
    return c / 12.0;
}

// O resto fraquinho da luz espalhada, que fica de fora: a curva de saída o deixaria visível
// longe da fonte, e o brilho deve cair rápido.
const TAIL: f32 = 0.012;

// Entra no quadro: a luz espalhada com um ajuste de tom suave (nunca passa de 1) e na mesma
// curva de saída do shader; o alfa é o quanto ela cobre o que está atrás.
@fragment
fn fs_composite(v: Vary) -> @location(0) vec4<f32> {
    let g = max(textureSample(source, smp, v.uv).rgb * RANGE - vec3<f32>(TAIL), vec3<f32>(0.0));
    let c = pow(vec3<f32>(1.0) - exp(-g), vec3<f32>(1.0 / 2.2));
    return vec4<f32>(c, max(c.r, max(c.g, c.b)));
}
