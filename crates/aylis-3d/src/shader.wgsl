// O shader do aylis-3d: esqueleto na GPU e PBR (metal/aspereza) com até 4 luzes (direcional,
// pontual, cone), luz ambiente de céu e chão, mapas de normal, oclusão, metal/aspereza e
// emissão, e sombra da primeira luz direcional. Sai com alfa pré-multiplicado, pronto para
// compor.

// Variantes do pipeline (constantes fixadas ao criar o pipeline): o que está desligado nem
// entra no shader compilado, então um material simples não paga pelos mapas, pela sombra
// nem pelo laço das luzes.
override MAPS: bool = true;
override SHADOW: bool = true;
override LIGHTS: bool = true;

struct Light {
    // xyz: posição; w: tipo (0 direcional, 1 pontual, 2 cone)
    position: vec4<f32>,
    // xyz: para onde a luz vai; w: alcance (0 = infinito)
    direction: vec4<f32>,
    // rgb: cor × intensidade
    color: vec4<f32>,
    // cone: escala e deslocamento do cosseno (x, y)
    cone: vec4<f32>,
};

struct Camera {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    // x: luzes; y: sombra ligada; z: um texel da sombra (em UV); w: o mesmo no mundo
    counts: vec4<f32>,
    // da luz que faz sombra (mundo → recorte)
    shadow_view_proj: mat4x4<f32>,
    lights: array<Light, 4>,
    // traçado de raios: x ligado; y amostra acumulada (0 = sem acumular); z alcance da
    // oclusão de ambiente; w afastamento da superfície (no mundo)
    rays: vec4<f32>,
    // brilho (bloom): x a força com que a emissão vai para a textura do brilho
    glow: vec4<f32>,
};
@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var shadow_map: texture_depth_2d;
@group(0) @binding(2) var shadow_sampler: sampler_comparison;

struct Draw {
    world: mat4x4<f32>,
    normal: mat4x4<f32>,
    base_color: vec4<f32>,
    // metal, aspereza, tem esqueleto, tem textura de cor
    params: vec4<f32>,
    // emissão; w: transparente
    emissive: vec4<f32>,
    // força do mapa de normais (0 = sem), força da oclusão (0 = sem), tem metal/aspereza,
    // tem textura de emissão
    maps: vec4<f32>,
    // verniz (KHR_materials_clearcoat): força, aspereza
    coat: vec4<f32>,
};
@group(1) @binding(0) var<uniform> draw: Draw;

@group(2) @binding(0) var base_tex: texture_2d<f32>;
@group(2) @binding(1) var base_sampler: sampler;
@group(2) @binding(2) var normal_tex: texture_2d<f32>;
@group(2) @binding(3) var mr_tex: texture_2d<f32>;
@group(2) @binding(4) var occlusion_tex: texture_2d<f32>;
@group(2) @binding(5) var emissive_tex: texture_2d<f32>;

struct Joints {
    m: array<mat4x4<f32>, 128>,
};
@group(3) @binding(0) var<uniform> joints: Joints;

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) joints: vec4<u32>,
    @location(4) weights: vec4<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) color: vec4<f32>,
};

fn joint(i: u32) -> mat4x4<f32> {
    return joints.m[min(i, 127u)];
}

@vertex
fn vs_main(v: VertexIn) -> VertexOut {
    var out: VertexOut;
    var n: vec3<f32>;
    var p: vec4<f32>;
    if (draw.params.z > 0.5) {
        let w = v.weights;
        let skin = joint(v.joints.x) * w.x + joint(v.joints.y) * w.y + joint(v.joints.z) * w.z + joint(v.joints.w) * w.w;
        p = skin * vec4<f32>(v.position, 1.0);
        n = (skin * vec4<f32>(v.normal, 0.0)).xyz;
    } else {
        p = draw.world * vec4<f32>(v.position, 1.0);
        n = (draw.normal * vec4<f32>(v.normal, 0.0)).xyz;
    }
    out.clip = camera.view_proj * p;
    out.world = p.xyz;
    out.normal = n;
    out.uv = v.uv;
    out.color = v.color;
    return out;
}

const PI: f32 = 3.14159265;

fn aces(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

// Normal do mapa (espaço tangente) sem tangentes nos vértices: a base sai das derivadas da
// posição e do UV na tela. O sinal do determinante acerta a orientação qualquer que seja o
// sentido do Y da tela. T e B ficam na direção em que U e V crescem; o V do glTF cresce para
// baixo na imagem e o verde do mapa aponta para cima, por isso o B vai invertido.
fn perturb(n: vec3<f32>, p: vec3<f32>, uv: vec2<f32>, m: vec3<f32>) -> vec3<f32> {
    let dp1 = dpdx(p);
    let dp2 = dpdy(p);
    let duv1 = dpdx(uv);
    let duv2 = dpdy(uv);
    let dp2perp = cross(dp2, n);
    let dp1perp = cross(n, dp1);
    let side = select(-1.0, 1.0, dot(dp1, dp2perp) >= 0.0);
    let t = (dp2perp * duv1.x + dp1perp * duv2.x) * side;
    let b = (dp2perp * duv1.y + dp1perp * duv2.y) * side;
    let len = max(dot(t, t), dot(b, b));
    if (len < 1e-20) {
        return n; // sem UV (ou degenerado): fica a normal da malha
    }
    let s = inverseSqrt(len);
    return normalize(t * s * m.x - b * s * m.y + n * m.z);
}

// Luz direta de uma fonte (Cook-Torrance), já com a chegada `radiance`.
fn shade(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, radiance: vec3<f32>, base: vec3<f32>, f0: vec3<f32>, metallic: f32, rough: f32) -> vec3<f32> {
    let h = normalize(v + l);
    let ndl = max(dot(n, l), 0.0);
    let ndv = max(dot(n, v), 0.0001);
    let ndh = max(dot(n, h), 0.0);
    let vdh = max(dot(v, h), 0.0);
    let a = rough * rough;
    let a2 = a * a;
    let dd = ndh * ndh * (a2 - 1.0) + 1.0;
    let d = a2 / (PI * dd * dd);
    let k = (rough + 1.0) * (rough + 1.0) / 8.0;
    let g = (ndv / (ndv * (1.0 - k) + k)) * (ndl / (ndl * (1.0 - k) + k));
    let f = f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - vdh, 5.0);
    let spec = d * g * f / max(4.0 * ndv * ndl, 0.0001);
    let kd = (vec3<f32>(1.0) - f) * (1.0 - metallic);
    return (kd * base / PI + spec) * radiance * ndl;
}

// Número pseudoaleatório de 0 a 1 (PCG) para o pixel, a amostra e o canal `k`.
fn random(pixel: vec2<f32>, k: u32) -> f32 {
    var s = u32(pixel.x) * 1973u + u32(pixel.y) * 9277u + u32(camera.rays.y) * 26699u + k * 7919u;
    s = s * 747796405u + 2891336453u;
    let w = ((s >> ((s >> 28u) + 4u)) ^ s) * 277803737u;
    return f32((w >> 22u) ^ w) / 4294967295.0;
}

// Uma direção perto de `d`, espalhada até `spread` (radianos), sorteada para o pixel. Sem
// acumular (amostra 0), a própria `d`.
fn jitter(d: vec3<f32>, spread: f32, pixel: vec2<f32>, k: u32) -> vec3<f32> {
    if (camera.rays.y < 0.5 || spread <= 0.0) {
        return d;
    }
    let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(d.y) > 0.9);
    let t = normalize(cross(up, d));
    let b = cross(d, t);
    let a = random(pixel, k) * 6.2831853;
    let r = sqrt(random(pixel, k + 1u)) * spread;
    return normalize(d + (t * cos(a) + b * sin(a)) * r);
}

// @rays-begin: sem traçado de raios (trocado pelo `rays.wgsl` quando a placa traça).
fn ray_visible(origin: vec3<f32>, dir: vec3<f32>, dist: f32) -> f32 {
    return 1.0;
}
fn ray_reflect(origin: vec3<f32>, dir: vec3<f32>) -> vec4<f32> {
    return vec4<f32>(0.0);
}
fn ray_ao(origin: vec3<f32>, n: vec3<f32>, pixel: vec2<f32>) -> f32 {
    return 1.0;
}
// @rays-end

// O verniz: só o reflexo (GGX com F0 = 0,04), na normal da superfície (sem o mapa de
// normais, que fica por baixo dele).
fn coat_spec(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, radiance: vec3<f32>, rough: f32) -> vec3<f32> {
    let h = normalize(v + l);
    let ndl = max(dot(n, l), 0.0);
    let ndv = max(dot(n, v), 0.0001);
    let ndh = max(dot(n, h), 0.0);
    let a = rough * rough;
    let a2 = a * a;
    let dd = ndh * ndh * (a2 - 1.0) + 1.0;
    let d = a2 / (PI * dd * dd);
    let k = (rough + 1.0) * (rough + 1.0) / 8.0;
    let g = (ndv / (ndv * (1.0 - k) + k)) * (ndl / (ndl * (1.0 - k) + k));
    let f = 0.04 + 0.96 * pow(1.0 - max(dot(v, h), 0.0), 5.0);
    return vec3<f32>(d * g * f / max(4.0 * ndv * ndl, 0.0001)) * radiance * ndl;
}

// Quanto do ponto a luz da sombra alcança (0 a 1), com 3×3 amostras (PCF).
fn shadow_at(world: vec3<f32>, n: vec3<f32>) -> f32 {
    let texel = camera.counts.z;
    // Afasta o ponto na direção da normal para não sombrear a si mesmo ("acne").
    let p = camera.shadow_view_proj * vec4<f32>(world + n * camera.counts.w * 1.5, 1.0);
    let c = p.xyz / p.w;
    let uv = vec2<f32>(c.x * 0.5 + 0.5, 0.5 - c.y * 0.5);
    let depth = c.z - 0.0015;
    var lit = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            lit += textureSampleCompare(shadow_map, shadow_sampler, uv + vec2<f32>(f32(x), f32(y)) * texel, depth);
        }
    }
    lit = lit / 9.0;
    // Fora do mapa, tudo iluminado.
    let outside = any(uv < vec2<f32>(0.0)) || any(uv > vec2<f32>(1.0)) || c.z > 1.0;
    return select(lit, 1.0, outside);
}

// Luz direta de `light` (a `k`-ésima; a 0 recebe a sombra do mapa). Com o traçado de
// raios, toda luz faz sombra: um raio até ela (espalhado, ao acumular: sombra suave).
fn light_contribution(light: Light, k: i32, world: vec3<f32>, n: vec3<f32>, geo: vec3<f32>, v: vec3<f32>, shadow: f32, base: vec3<f32>, f0: vec3<f32>, metallic: f32, rough: f32, pixel: vec2<f32>) -> vec3<f32> {
    var l: vec3<f32>;
    var radiance = light.color.rgb;
    var dist = 1e30;
    if (light.position.w < 0.5) {
        l = normalize(-light.direction.xyz);
        // A sombra é da primeira luz (a direcional que a faz).
        if (k == 0) {
            radiance = radiance * shadow;
        }
    } else {
        let to_light = light.position.xyz - world;
        let d2 = max(dot(to_light, to_light), 1e-4);
        dist = sqrt(d2);
        l = to_light * inverseSqrt(d2);
        var att = 1.0 / d2;
        let range = light.direction.w;
        if (range > 0.0) {
            let r = sqrt(d2) / range;
            let w = clamp(1.0 - r * r * r * r, 0.0, 1.0);
            att = att * w * w;
        }
        if (light.position.w > 1.5) {
            let cd = dot(normalize(light.direction.xyz), -l);
            let s = clamp(cd * light.cone.x + light.cone.y, 0.0, 1.0);
            att = att * s * s;
        }
        radiance = radiance * att;
    }
    if (camera.rays.x > 0.5 && dot(geo, l) > 0.0) {
        let to = jitter(l, 0.035, pixel, 10u + u32(k) * 2u);
        radiance = radiance * ray_visible(world + geo * camera.rays.w, to, dist);
    }
    var c = shade(n, v, l, radiance, base, f0, metallic, rough);
    if (draw.coat.x > 0.0) {
        let fc = 0.04 + 0.96 * pow(1.0 - max(dot(geo, v), 0.0), 5.0);
        c = c * (1.0 - draw.coat.x * fc) + draw.coat.x * coat_spec(geo, v, l, radiance, clamp(draw.coat.y, 0.04, 1.0));
    }
    return c;
}

// O pixel: a cor final (tom ajustado, alfa pré-multiplicado) e a luz que o material emite
// (linear, antes do ajuste de tom), que o brilho espalha em volta.
struct Lit {
    color: vec4<f32>,
    emitted: vec3<f32>,
};

@fragment
fn fs_main(i: VertexOut) -> @location(0) vec4<f32> {
    return lit(i).color;
}

struct GlowOut {
    @location(0) color: vec4<f32>,
    @location(1) glow: vec4<f32>,
};

// Com o brilho: a emissão vai também para a segunda saída, de onde o bloom a espalha.
@fragment
fn fs_glow(i: VertexOut) -> GlowOut {
    let l = lit(i);
    return GlowOut(l.color, vec4<f32>(l.emitted * camera.glow.x, l.color.a));
}

fn lit(i: VertexOut) -> Lit {
    var base = draw.base_color * i.color;
    if (draw.params.w > 0.5) {
        base = base * textureSample(base_tex, base_sampler, i.uv);
    }
    let v = normalize(camera.eye.xyz - i.world);
    // A face vista por trás (material de dupla face) usa a normal do lado da câmera. Não
    // usa front_facing: no OpenGL ES (sem glClipControl) o wgpu inverte o Y e ele vem trocado.
    var geo = normalize(i.normal);
    if (dot(geo, v) < 0.0) {
        geo = -geo;
    }
    var n = geo;
    if (MAPS && draw.maps.x != 0.0) {
        let m = textureSample(normal_tex, base_sampler, i.uv).xyz * 2.0 - 1.0;
        n = perturb(geo, i.world, i.uv, normalize(vec3<f32>(m.xy * draw.maps.x, m.z)));
    }
    var metallic = draw.params.x;
    var rough = draw.params.y;
    if (MAPS && draw.maps.z > 0.5) {
        let mr = textureSample(mr_tex, base_sampler, i.uv);
        metallic = metallic * mr.b;
        rough = rough * mr.g;
    }
    metallic = clamp(metallic, 0.0, 1.0);
    rough = clamp(rough, 0.04, 1.0);
    var ao = 1.0;
    if (MAPS && draw.maps.y != 0.0) {
        ao = 1.0 + draw.maps.y * (textureSample(occlusion_tex, base_sampler, i.uv).r - 1.0);
    }
    var emissive = draw.emissive.rgb;
    if (MAPS && draw.maps.w > 0.5) {
        emissive = emissive * textureSample(emissive_tex, base_sampler, i.uv).rgb;
    }
    // Sem sombra, nem amostra o mapa (a condição vem do uniform: o fluxo continua
    // uniforme, como a comparação de profundidade exige).
    var shadow = 1.0;
    if (SHADOW && camera.counts.y > 0.5) {
        shadow = shadow_at(i.world, geo);
    }

    let f0 = mix(vec3<f32>(0.04), base.rgb, metallic);
    var direct = vec3<f32>(0.0);
    if (LIGHTS) {
        let count = i32(camera.counts.x);
        for (var k = 0; k < 4; k++) {
            if (k >= count) {
                break;
            }
            direct += light_contribution(camera.lights[k], k, i.world, n, geo, v, shadow, base.rgb, f0, metallic, rough, i.clip.xy);
        }
    } else {
        direct = light_contribution(camera.lights[0], 0, i.world, n, geo, v, shadow, base.rgb, f0, metallic, rough, i.clip.xy);
    }

    let hemi = mix(camera.ground.rgb, camera.sky.rgb, n.y * 0.5 + 0.5);
    var env_spec = hemi * f0 * (1.0 - rough * 0.7);
    if (camera.rays.x > 0.5) {
        // Oclusão de ambiente por raios: os cantos e o chão debaixo dos objetos escurecem.
        ao = ao * ray_ao(i.world + geo * camera.rays.w, geo, i.clip.xy);
        // Reflexo de verdade nos materiais lisos (espalhado pela aspereza, ao acumular).
        if (rough < 0.6) {
            let r = jitter(reflect(-v, n), rough * 0.5, i.clip.xy, 20u);
            let hit = ray_reflect(i.world + geo * camera.rays.w, r);
            let sky = mix(camera.ground.rgb, camera.sky.rgb, r.y * 0.5 + 0.5);
            let seen = select(sky, hit.rgb, hit.a > 0.5);
            let fr = f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - max(dot(n, v), 0.0), 5.0);
            env_spec = seen * fr * (1.0 - rough);
        }
    }
    var ambient = (hemi * base.rgb * (1.0 - metallic) + env_spec) * ao;
    if (draw.coat.x > 0.0) {
        // O verniz reflete o céu e o chão na direção do reflexo e tira um pouco da luz de baixo.
        let fc = 0.04 + 0.96 * pow(1.0 - max(dot(geo, v), 0.0), 5.0);
        let r = reflect(-v, geo);
        let coat_env = mix(camera.ground.rgb, camera.sky.rgb, r.y * 0.5 + 0.5) * fc * (1.0 - draw.coat.y * 0.7);
        ambient = ambient * (1.0 - draw.coat.x * fc) + draw.coat.x * coat_env * ao;
    }

    var c = direct + ambient + emissive;
    c = pow(aces(c), vec3<f32>(1.0 / 2.2));
    var alpha = 1.0;
    if (draw.emissive.w > 0.5) {
        alpha = base.a;
    }
    return Lit(vec4<f32>(c * alpha, alpha), emissive * alpha);
}
