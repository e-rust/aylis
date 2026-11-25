// A mistura das camadas com `blend_mode` (o compositor em Direct3D do aylis-d2d): o fundo
// copiado do alvo e a camada desenhada pelo Direct2D, pixel a pixel, com as fórmulas do
// "Compositing and Blending" do W3C, as mesmas do Skia e do navegador. Cores
// pré-multiplicadas, como o Direct2D guarda.

cbuffer Params : register(b0) {
    // O pedaço em pixels do alvo: esquerda, topo, direita, base.
    float4 area;
    // O tamanho do alvo em pixels.
    float2 target_size;
    // O modo, na ordem do `aylis_core::BlendMode` (0 normal ... 15 luminosity).
    uint mode;
    // A opacidade da camada.
    float opacity;
};

Texture2D<float4> layer : register(t0);
Texture2D<float4> backdrop : register(t1);

// Um retângulo sobre o pedaço com quatro vértices em tira (sem buffer de vértices).
float4 vs_main(uint id : SV_VertexID) : SV_Position {
    float x = (id & 1) ? area.z : area.x;
    float y = (id & 2) ? area.w : area.y;
    return float4(x / target_size.x * 2.0 - 1.0, 1.0 - y / target_size.y * 2.0, 0.0, 1.0);
}

float lum(float3 c) { return dot(c, float3(0.3, 0.59, 0.11)); }

float3 clip_color(float3 c) {
    float l = lum(c);
    float n = min(c.r, min(c.g, c.b));
    float x = max(c.r, max(c.g, c.b));
    if (n < 0.0) c = l + (c - l) * l / (l - n);
    if (x > 1.0) c = l + (c - l) * (1.0 - l) / (x - l);
    return c;
}

float3 set_lum(float3 c, float l) { return clip_color(c + (l - lum(c))); }

float sat(float3 c) { return max(c.r, max(c.g, c.b)) - min(c.r, min(c.g, c.b)); }

float3 set_sat(float3 c, float s) {
    float n = min(c.r, min(c.g, c.b));
    float x = max(c.r, max(c.g, c.b));
    return x > n ? (c - n) * s / (x - n) : float3(0.0, 0.0, 0.0);
}

float hard_light(float b, float s) {
    return s <= 0.5 ? b * 2.0 * s : b + (2.0 * s - 1.0) - b * (2.0 * s - 1.0);
}

float color_dodge(float b, float s) {
    if (b == 0.0) return 0.0;
    if (s >= 1.0) return 1.0;
    return min(1.0, b / (1.0 - s));
}

float color_burn(float b, float s) {
    if (b == 1.0) return 1.0;
    if (s <= 0.0) return 0.0;
    return 1.0 - min(1.0, (1.0 - b) / s);
}

float soft_light(float b, float s) {
    if (s <= 0.5) return b - (1.0 - 2.0 * s) * b * (1.0 - b);
    float d = b <= 0.25 ? ((16.0 * b - 12.0) * b + 4.0) * b : sqrt(b);
    return b + (2.0 * s - 1.0) * (d - b);
}

// A cor misturada B(cb, cs), sem o alfa.
float3 blend(float3 b, float3 s) {
    switch (mode) {
        case 1: return b * s;
        case 2: return b + s - b * s;
        case 3: return float3(hard_light(s.r, b.r), hard_light(s.g, b.g), hard_light(s.b, b.b));
        case 4: return min(b, s);
        case 5: return max(b, s);
        case 6: return float3(color_dodge(b.r, s.r), color_dodge(b.g, s.g), color_dodge(b.b, s.b));
        case 7: return float3(color_burn(b.r, s.r), color_burn(b.g, s.g), color_burn(b.b, s.b));
        case 8: return float3(hard_light(b.r, s.r), hard_light(b.g, s.g), hard_light(b.b, s.b));
        case 9: return float3(soft_light(b.r, s.r), soft_light(b.g, s.g), soft_light(b.b, s.b));
        case 10: return abs(b - s);
        case 11: return b + s - 2.0 * b * s;
        case 12: return set_lum(set_sat(s, sat(b)), lum(b));
        case 13: return set_lum(set_sat(b, sat(s)), lum(b));
        case 14: return set_lum(s, lum(b));
        case 15: return set_lum(b, lum(s));
        default: return s;
    }
}

// A camada sozinha (com a opacidade), para as misturas que a placa faz sem ler o fundo: a
// `exclusion` vira a mistura fixa `INV_DEST_COLOR`/`INV_SRC_COLOR` do Direct3D. Com as cores
// pré-multiplicadas, a fórmula de `ps_main` no `case 11` dá, com qualquer alfa do fundo,
// rgb = s + b - 2·s·b = s·(1 - b) + b·(1 - s), e o alfa s.a + b.a·(1 - s.a).
float4 ps_layer(float4 position : SV_Position) : SV_Target {
    int2 p = int2(position.xy) - int2(area.xy);
    return layer.Load(int3(p, 0)) * opacity;
}

float4 ps_main(float4 position : SV_Position) : SV_Target {
    int2 p = int2(position.xy) - int2(area.xy);
    float4 s = layer.Load(int3(p, 0)) * opacity;
    float4 b = backdrop.Load(int3(p, 0));
    float3 cs = s.a > 0.0 ? s.rgb / s.a : float3(0.0, 0.0, 0.0);
    float3 cb = b.a > 0.0 ? b.rgb / b.a : float3(0.0, 0.0, 0.0);
    float3 mixed = saturate(blend(cb, cs));
    float3 rgb = s.rgb * (1.0 - b.a) + b.rgb * (1.0 - s.a) + s.a * b.a * mixed;
    return float4(rgb, s.a + b.a - s.a * b.a);
}
