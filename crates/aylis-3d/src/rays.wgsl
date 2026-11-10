// O traçado de raios do shader do aylis-3d (ray query): entra no lugar do bloco
// `@rays-begin`…`@rays-end` do `shader.wgsl` quando a placa traça raios. A cena (as
// instâncias de cada desenho) e a cor de cada uma vêm no grupo 4.

@group(4) @binding(0) var scene: acceleration_structure;
// Duas por desenho: (cor, metal) e (emissão, aspereza).
@group(4) @binding(1) var<storage, read> hit_colors: array<vec4<f32>>;

fn ray_hit(origin: vec3<f32>, dir: vec3<f32>, t_max: f32, first: bool) -> RayIntersection {
    var rq: ray_query;
    var flags = RAY_FLAG_FORCE_OPAQUE;
    if (first) {
        flags = flags | RAY_FLAG_TERMINATE_ON_FIRST_HIT;
    }
    rayQueryInitialize(&rq, scene, RayDesc(flags, 0xFFu, 0.0, t_max, origin, dir));
    rayQueryProceed(&rq);
    return rayQueryGetCommittedIntersection(&rq);
}

// 1 se nada está entre o ponto e a luz (até `dist`), 0 se algo está.
fn ray_visible(origin: vec3<f32>, dir: vec3<f32>, dist: f32) -> f32 {
    let hit = ray_hit(origin, dir, dist, true);
    return select(0.0, 1.0, hit.kind == RAY_QUERY_INTERSECTION_NONE);
}

// A cor do que o reflexo encontra (alfa 0: nada, o céu). Sem a normal do ponto atingido, a
// luz é a média do céu e do chão mais um tanto da luz principal.
fn ray_reflect(origin: vec3<f32>, dir: vec3<f32>) -> vec4<f32> {
    let hit = ray_hit(origin, dir, 1e30, false);
    if (hit.kind == RAY_QUERY_INTERSECTION_NONE) {
        return vec4<f32>(0.0);
    }
    let base = hit_colors[hit.instance_custom_data * 2u];
    let glow = hit_colors[hit.instance_custom_data * 2u + 1u];
    let light = (camera.sky.rgb + camera.ground.rgb) * 0.5 + camera.lights[0].color.rgb * 0.35;
    return vec4<f32>(base.rgb * light * (1.0 - base.a * 0.5) + glow.rgb, 1.0);
}

// Quanto do céu o ponto vê (0 a 1): raios no hemisfério da normal, até `camera.rays.z`,
// girados por um sorteio do pixel (sem desenho repetido). Sem acumular, 6 raios na mesma
// altura e o sorteio fixo (a imagem não tremula); acumulando, 4 por amostra, altura também
// sorteada (distribuição do cosseno: mais raios perto da normal).
fn ray_ao(origin: vec3<f32>, n: vec3<f32>, pixel: vec2<f32>) -> f32 {
    let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.9);
    let t = normalize(cross(up, n));
    let b = cross(n, t);
    let accumulating = camera.rays.y > 0.5;
    let count = select(6u, 4u, accumulating);
    let turn = random(pixel, 30u) * 6.2831853;
    let lift = random(pixel, 31u);
    var open = 0.0;
    for (var k = 0u; k < count; k++) {
        let a = turn + f32(k) * 6.2831853 / f32(count);
        var u = 0.5;
        if (accumulating) {
            u = fract(lift + f32(k) / f32(count));
        }
        let s = sqrt(u * 0.9);
        let d = normalize(n * sqrt(1.0 - s * s) + (t * cos(a) + b * sin(a)) * s);
        let hit = ray_hit(origin, d, camera.rays.z, true);
        open += select(0.0, 1.0, hit.kind == RAY_QUERY_INTERSECTION_NONE);
    }
    return 0.25 + 0.75 * (open / f32(count));
}
