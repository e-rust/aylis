//! Modelos de exemplo gerados em código (sem arquivo):
//!
//! - [`scene_glb`]: um cubo girando e uma bola quicando sobre um chão, o modelo de
//!   demonstração (os exemplos e a IDE);
//! - [`tube_glb`]: um tubo com esqueleto de 5 ossos, um morph target ("Inchar"), um chão e
//!   três clipes ("Balançar", "Respirar", "Girar"); serve aos testes e cobre o que um `.glb`
//!   de verdade usa.

use std::f32::consts::{PI, TAU};

/// Um cubo que gira e anda de um lado para o outro e uma bola que quica, sobre um chão; um
/// clipe ("Mover", 4 s, repete sem emenda).
pub fn scene_glb() -> Vec<u8> {
    let mut b = Builder::default();

    // ---- cubo de lado 0,6: 6 faces com normais próprias (cantos vivos) ----
    let (mut cpos, mut cnrm, mut cidx) = (vec![], vec![], vec![]);
    let faces: [([f32; 3], [f32; 3], [f32; 3]); 6] = [
        ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        ([-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        ([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ([0.0, 0.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
    ];
    const HALF: f32 = 0.3;
    for (n, u, v) in faces {
        let base = cpos.len() as u32;
        for (su, sv) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            cpos.push([0, 1, 2].map(|i| HALF * (n[i] + su * u[i] + sv * v[i])));
            cnrm.push(n);
        }
        cidx.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    let cube_pos = b.vec3(&cpos, true);
    let cube_nrm = b.vec3(&cnrm, false);
    let cube_idx = b.indices(&cidx);

    // ---- bola de raio 0,3: esfera UV ----
    const RADIUS: f32 = 0.3;
    const LAT: usize = 24;
    const LON: usize = 40;
    let (mut spos, mut snrm, mut sidx) = (vec![], vec![], vec![]);
    for i in 0..=LAT {
        let theta = PI * i as f32 / LAT as f32;
        for j in 0..=LON {
            let phi = TAU * j as f32 / LON as f32;
            let n = [theta.sin() * phi.cos(), theta.cos(), theta.sin() * phi.sin()];
            spos.push(n.map(|c| c * RADIUS));
            snrm.push(n);
        }
    }
    for i in 0..LAT as u32 {
        for j in 0..LON as u32 {
            let a = i * (LON as u32 + 1) + j;
            let c = a + LON as u32 + 1;
            sidx.extend([a, a + 1, c, a + 1, c + 1, c]);
        }
    }
    let ball_pos = b.vec3(&spos, true);
    let ball_nrm = b.vec3(&snrm, false);
    let ball_idx = b.indices(&sidx);

    // ---- chão: quadrado de 4 x 4 ----
    let gpos = vec![[-2.0, 0.0, -2.0], [2.0, 0.0, -2.0], [2.0, 0.0, 2.0], [-2.0, 0.0, 2.0]];
    let ground_pos = b.vec3(&gpos, true);
    let ground_nrm = b.vec3(&vec![[0.0, 1.0, 0.0]; 4], false);
    let ground_idx = b.indices(&[0, 2, 1, 0, 3, 2]);

    // ---- clipe "Mover" (4 s) ----
    const SECONDS: f32 = 4.0;
    let times: Vec<f32> = (0..=80).map(|i| SECONDS * i as f32 / 80.0).collect();
    let t_acc = b.scalars(&times, true);
    // Cubo: vai e volta em X e dá uma volta em Y.
    let cube_t: Vec<[f32; 3]> = times.iter().map(|t| [-0.7 + 0.35 * (TAU * t / SECONDS).sin(), HALF, -0.3]).collect();
    let cube_r: Vec<[f32; 4]> = times
        .iter()
        .map(|t| {
            let a = TAU * t / SECONDS;
            [0.0, (a / 2.0).sin(), 0.0, (a / 2.0).cos()]
        })
        .collect();
    // Bola: quatro quiques (parábolas) indo e voltando em Z.
    let ball_t: Vec<[f32; 3]> = times
        .iter()
        .map(|t| {
            let phase = (t / SECONDS * 4.0).fract();
            let height = 4.0 * phase * (1.0 - phase);
            [0.7, RADIUS + 1.1 * height, 0.4 * (TAU * t / SECONDS).cos()]
        })
        .collect();
    let cube_t_acc = b.vec3(&cube_t, false);
    let cube_r_acc = b.f32x4(&cube_r);
    let ball_t_acc = b.vec3(&ball_t, false);

    let json = format!(
        r#"{{
"asset":{{"version":"2.0","generator":"aylis-3d"}},
"scene":0,
"scenes":[{{"nodes":[0,1,2]}}],
"nodes":[
 {{"name":"Cubo","mesh":0,"translation":[-0.7,{HALF},-0.3]}},
 {{"name":"Bola","mesh":1,"translation":[0.7,{RADIUS},0.4]}},
 {{"name":"Chão","mesh":2}}
],
"materials":[
 {{"name":"Azul","pbrMetallicRoughness":{{"baseColorFactor":[0.1,0.35,0.9,1],"metallicFactor":0.2,"roughnessFactor":0.35}}}},
 {{"name":"Laranja","pbrMetallicRoughness":{{"baseColorFactor":[0.95,0.4,0.05,1],"metallicFactor":0.0,"roughnessFactor":0.25}}}},
 {{"name":"Chão","pbrMetallicRoughness":{{"baseColorFactor":[0.55,0.58,0.62,1],"metallicFactor":0.0,"roughnessFactor":0.9}},"doubleSided":true}}
],
"meshes":[
 {{"name":"Cubo","primitives":[{{"attributes":{{"POSITION":{cube_pos},"NORMAL":{cube_nrm}}},"indices":{cube_idx},"material":0}}]}},
 {{"name":"Bola","primitives":[{{"attributes":{{"POSITION":{ball_pos},"NORMAL":{ball_nrm}}},"indices":{ball_idx},"material":1}}]}},
 {{"name":"Chão","primitives":[{{"attributes":{{"POSITION":{ground_pos},"NORMAL":{ground_nrm}}},"indices":{ground_idx},"material":2}}]}}
],
"animations":[
 {{"name":"Mover","samplers":[
   {{"input":{t_acc},"output":{cube_t_acc},"interpolation":"LINEAR"}},
   {{"input":{t_acc},"output":{cube_r_acc},"interpolation":"LINEAR"}},
   {{"input":{t_acc},"output":{ball_t_acc},"interpolation":"LINEAR"}}
 ],"channels":[
   {{"sampler":0,"target":{{"node":0,"path":"translation"}}}},
   {{"sampler":1,"target":{{"node":0,"path":"rotation"}}}},
   {{"sampler":2,"target":{{"node":1,"path":"translation"}}}}
 ]}}
],
"buffers":[{{"byteLength":{len}}}],
"bufferViews":[{views}],
"accessors":[{accs}]
}}"#,
        len = b.bin.len(),
        views = b.views.join(","),
        accs = b.accessors.join(","),
    );
    glb(&json, &b.bin)
}

/// Monta o `.glb` do tubo.
pub fn tube_glb() -> Vec<u8> {
    let mut b = Builder::default();

    // ---- tubo: anéis ao longo de +Y, de 0 a 2 ----
    const RINGS: usize = 33;
    const SIDES: usize = 24;
    const HEIGHT: f32 = 2.0;
    const RADIUS: f32 = 0.22;
    let (mut pos, mut nrm, mut joints, mut weights, mut bulge) = (vec![], vec![], vec![], vec![], vec![]);
    for r in 0..RINGS {
        let y = HEIGHT * r as f32 / (RINGS - 1) as f32;
        // Afina para a ponta.
        let radius = RADIUS * (1.0 - 0.45 * y / HEIGHT);
        // Pesos suaves: cada anel segue os ossos mais perto (até 4), sem dobras nas juntas.
        let mut near: Vec<(f32, u16)> = (0..5).map(|k| ((-((y - 0.5 * k as f32) / 0.32).powi(2)).exp(), k as u16)).collect();
        near.sort_by(|a, b| b.0.total_cmp(&a.0));
        near.truncate(4);
        let total: f32 = near.iter().map(|n| n.0).sum();
        for s in 0..SIDES {
            let a = TAU * s as f32 / SIDES as f32;
            let (sx, cz) = (a.cos(), a.sin());
            pos.push([sx * radius, y, cz * radius]);
            nrm.push([sx, 0.0, cz]);
            joints.push([near[0].1, near[1].1, near[2].1, near[3].1]);
            weights.push([near[0].0 / total, near[1].0 / total, near[2].0 / total, near[3].0 / total]);
            let k = 0.12 * (PI * y / HEIGHT).sin();
            bulge.push([sx * k, 0.0, cz * k]);
        }
    }
    let mut idx: Vec<u32> = vec![];
    for r in 0..RINGS - 1 {
        for s in 0..SIDES {
            let a = (r * SIDES + s) as u32;
            let b2 = (r * SIDES + (s + 1) % SIDES) as u32;
            let c = a + SIDES as u32;
            let d = b2 + SIDES as u32;
            idx.extend([a, c, b2, b2, c, d]);
        }
    }
    let tube_pos = b.vec3(&pos, true);
    let tube_nrm = b.vec3(&nrm, false);
    let tube_joints = b.u16x4(&joints);
    let tube_weights = b.f32x4(&weights);
    let tube_idx = b.indices(&idx);
    let tube_bulge = b.vec3(&bulge, true);
    let zero_n = b.vec3(&vec![[0.0; 3]; pos.len()], false);

    // ---- chão: disco ----
    let (mut gpos, mut gnrm, mut gidx) = (vec![[0.0, 0.0, 0.0]], vec![[0.0, 1.0, 0.0]], vec![]);
    for s in 0..48 {
        let a = TAU * s as f32 / 48.0;
        gpos.push([a.cos() * 1.4, 0.0, a.sin() * 1.4]);
        gnrm.push([0.0, 1.0, 0.0]);
        gidx.extend([0u32, 1 + (s + 1) % 48, 1 + s]);
    }
    let ground_pos = b.vec3(&gpos, true);
    let ground_nrm = b.vec3(&gnrm, false);
    let ground_idx = b.indices(&gidx);

    // ---- esqueleto ----
    let ibm: Vec<[f32; 16]> = (0..5)
        .map(|k| [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5 * k as f32, 0.0, 1.0])
        .collect();
    let ibm_acc = b.mat4(&ibm);

    // ---- clipes ----
    let wave_times: Vec<f32> = (0..=20).map(|i| i as f32 * 0.1).collect();
    let wave_t = b.scalars(&wave_times, true);
    let mut wave_channels = vec![];
    for k in 1..5 {
        let rot: Vec<[f32; 4]> = wave_times
            .iter()
            .map(|t| {
                let a = 0.35 * (TAU * t / 2.0 + k as f32 * 0.6).sin();
                [0.0, 0.0, (a / 2.0).sin(), (a / 2.0).cos()]
            })
            .collect();
        wave_channels.push((k + 1, b.f32x4(&rot)));
    }
    let breath_t = b.scalars(&[0.0, 0.75, 1.5], true);
    let breath_w = b.scalars(&[0.0, 1.0, 0.0], false);
    let spin_times: Vec<f32> = (0..=12).map(|i| i as f32 * 0.25).collect();
    let spin_t = b.scalars(&spin_times, true);
    let spin: Vec<[f32; 4]> = spin_times
        .iter()
        .map(|t| {
            let a = TAU * t / 3.0;
            [0.0, (a / 2.0).sin(), 0.0, (a / 2.0).cos()]
        })
        .collect();
    let spin_r = b.f32x4(&spin);

    let mut samplers_wave = vec![];
    let mut channels_wave = vec![];
    for (i, (node, out)) in wave_channels.iter().enumerate() {
        samplers_wave.push(format!(r#"{{"input":{wave_t},"output":{out},"interpolation":"LINEAR"}}"#));
        channels_wave.push(format!(r#"{{"sampler":{i},"target":{{"node":{node},"path":"rotation"}}}}"#));
    }

    let json = format!(
        r#"{{
"asset":{{"version":"2.0","generator":"aylis-3d"}},
"scene":0,
"scenes":[{{"nodes":[0,1,6]}}],
"nodes":[
 {{"name":"Tubo","mesh":0,"skin":0}},
 {{"name":"osso0","children":[2]}},
 {{"name":"osso1","translation":[0,0.5,0],"children":[3]}},
 {{"name":"osso2","translation":[0,0.5,0],"children":[4]}},
 {{"name":"osso3","translation":[0,0.5,0],"children":[5]}},
 {{"name":"osso4","translation":[0,0.5,0]}},
 {{"name":"Chão","mesh":1}}
],
"skins":[{{"joints":[1,2,3,4,5],"inverseBindMatrices":{ibm_acc}}}],
"materials":[
 {{"name":"Laranja","pbrMetallicRoughness":{{"baseColorFactor":[0.9,0.22,0.04,1],"metallicFactor":0.1,"roughnessFactor":0.4}}}},
 {{"name":"Chão","pbrMetallicRoughness":{{"baseColorFactor":[0.55,0.58,0.62,1],"metallicFactor":0.0,"roughnessFactor":0.9}},"doubleSided":true}}
],
"meshes":[
 {{"name":"Tubo","weights":[0],"extras":{{"targetNames":["Inchar"]}},"primitives":[{{"attributes":{{"POSITION":{tube_pos},"NORMAL":{tube_nrm},"JOINTS_0":{tube_joints},"WEIGHTS_0":{tube_weights}}},"indices":{tube_idx},"material":0,"targets":[{{"POSITION":{tube_bulge},"NORMAL":{zero_n}}}]}}]}},
 {{"name":"Chão","primitives":[{{"attributes":{{"POSITION":{ground_pos},"NORMAL":{ground_nrm}}},"indices":{ground_idx},"material":1}}]}}
],
"animations":[
 {{"name":"Balançar","samplers":[{sw}],"channels":[{cw}]}},
 {{"name":"Respirar","samplers":[{{"input":{breath_t},"output":{breath_w},"interpolation":"LINEAR"}}],"channels":[{{"sampler":0,"target":{{"node":0,"path":"weights"}}}}]}},
 {{"name":"Girar","samplers":[{{"input":{spin_t},"output":{spin_r},"interpolation":"LINEAR"}}],"channels":[{{"sampler":0,"target":{{"node":1,"path":"rotation"}}}}]}}
],
"buffers":[{{"byteLength":{len}}}],
"bufferViews":[{views}],
"accessors":[{accs}]
}}"#,
        sw = samplers_wave.join(","),
        cw = channels_wave.join(","),
        len = b.bin.len(),
        views = b.views.join(","),
        accs = b.accessors.join(","),
    );
    glb(&json, &b.bin)
}

#[derive(Default)]
struct Builder {
    bin: Vec<u8>,
    views: Vec<String>,
    accessors: Vec<String>,
}

impl Builder {
    fn push(&mut self, bytes: &[u8], target: Option<u32>, accessor: String) -> usize {
        while self.bin.len() % 4 != 0 {
            self.bin.push(0);
        }
        let offset = self.bin.len();
        self.bin.extend_from_slice(bytes);
        let target = target.map(|t| format!(r#","target":{t}"#)).unwrap_or_default();
        self.views.push(format!(r#"{{"buffer":0,"byteOffset":{offset},"byteLength":{}{target}}}"#, bytes.len()));
        let view = self.views.len() - 1;
        self.accessors.push(accessor.replace("VIEW", &view.to_string()));
        self.accessors.len() - 1
    }

    fn vec3(&mut self, v: &[[f32; 3]], bounds: bool) -> usize {
        let bytes: Vec<u8> = v.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
        let mm = if bounds {
            let lo = (0..3).map(|i| v.iter().map(|p| p[i]).fold(f32::MAX, f32::min).to_string()).collect::<Vec<_>>().join(",");
            let hi = (0..3).map(|i| v.iter().map(|p| p[i]).fold(f32::MIN, f32::max).to_string()).collect::<Vec<_>>().join(",");
            format!(r#","min":[{lo}],"max":[{hi}]"#)
        } else {
            String::new()
        };
        self.push(&bytes, Some(34962), format!(r#"{{"bufferView":VIEW,"componentType":5126,"count":{},"type":"VEC3"{mm}}}"#, v.len()))
    }

    fn f32x4(&mut self, v: &[[f32; 4]]) -> usize {
        let bytes: Vec<u8> = v.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
        self.push(&bytes, None, format!(r#"{{"bufferView":VIEW,"componentType":5126,"count":{},"type":"VEC4"}}"#, v.len()))
    }

    fn u16x4(&mut self, v: &[[u16; 4]]) -> usize {
        let bytes: Vec<u8> = v.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
        self.push(&bytes, Some(34962), format!(r#"{{"bufferView":VIEW,"componentType":5123,"count":{},"type":"VEC4"}}"#, v.len()))
    }

    fn indices(&mut self, v: &[u32]) -> usize {
        let bytes: Vec<u8> = v.iter().flat_map(|f| f.to_le_bytes()).collect();
        self.push(&bytes, Some(34963), format!(r#"{{"bufferView":VIEW,"componentType":5125,"count":{},"type":"SCALAR"}}"#, v.len()))
    }

    fn scalars(&mut self, v: &[f32], bounds: bool) -> usize {
        let bytes: Vec<u8> = v.iter().flat_map(|f| f.to_le_bytes()).collect();
        let mm = if bounds {
            format!(r#","min":[{}],"max":[{}]"#, v.iter().copied().fold(f32::MAX, f32::min), v.iter().copied().fold(f32::MIN, f32::max))
        } else {
            String::new()
        };
        self.push(&bytes, None, format!(r#"{{"bufferView":VIEW,"componentType":5126,"count":{},"type":"SCALAR"{mm}}}"#, v.len()))
    }

    fn mat4(&mut self, v: &[[f32; 16]]) -> usize {
        let bytes: Vec<u8> = v.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
        self.push(&bytes, None, format!(r#"{{"bufferView":VIEW,"componentType":5126,"count":{},"type":"MAT4"}}"#, v.len()))
    }
}

/// O contêiner GLB: cabeçalho, pedaço JSON e pedaço binário.
fn glb(json: &str, bin: &[u8]) -> Vec<u8> {
    let mut j = json.as_bytes().to_vec();
    while j.len() % 4 != 0 {
        j.push(b' ');
    }
    let mut b = bin.to_vec();
    while b.len() % 4 != 0 {
        b.push(0);
    }
    let total = 12 + 8 + j.len() + 8 + b.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(j.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&j);
    out.extend_from_slice(&(b.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\0");
    out.extend_from_slice(&b);
    out
}
