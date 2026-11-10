//! Leitura do glTF e animação (sem GPU).

use glam::{Mat4, Quat, Vec3};
use aylis_3d::{Model, sample};

#[test]
fn reads_the_sample_glb() {
    let m = Model::from_glb(&sample::tube_glb()).unwrap();
    assert_eq!(m.nodes.len(), 7);
    assert_eq!(m.meshes.len(), 2);
    assert_eq!(m.skins[0].joints.len(), 5);
    let names: Vec<&str> = m.clips.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["Balançar", "Respirar", "Girar"]);
    assert!((m.clips[0].duration - 2.0).abs() < 1e-5);
    assert_eq!(m.meshes[0].primitives[0].targets.len(), 1, "o morph target");
    let (lo, hi) = m.bounds;
    assert!(lo.y.abs() < 1e-3 && (hi.y - 2.0).abs() < 1e-3, "{lo:?} {hi:?}");
    assert!((hi.x - 1.4).abs() < 1e-3, "o chão vai a 1,4");
}

#[test]
fn samples_and_blends_clips() {
    let m = Model::from_glb(&sample::tube_glb()).unwrap();
    let rest = m.rest_pose();
    // Girar: em 0,75 s (um quarto de 3 s) o osso0 virou 90° em Y.
    let mut p = rest.clone();
    m.sample(m.clip("Girar").unwrap(), 0.75, &mut p);
    let expected = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    assert!(p.local[1].1.angle_between(expected) < 1e-3);
    // Respirar: o peso do morph sobe até 1 no meio.
    let mut b = rest.clone();
    m.sample(m.clip("Respirar").unwrap(), 0.375, &mut b);
    assert!((b.weights[0][0] - 0.5).abs() < 1e-5);
    // A mistura no meio do caminho.
    let mut mix = rest.clone();
    mix.blend(&p, 0.5);
    assert!(mix.local[1].1.angle_between(Quat::from_rotation_y(std::f32::consts::FRAC_PI_4)) < 1e-3);
}

#[test]
fn joint_matrices_follow_the_bones() {
    let m = Model::from_glb(&sample::tube_glb()).unwrap();
    let rest = m.rest_pose();
    let world = m.world(&rest, Mat4::IDENTITY);
    // Na pose de repouso, osso × inversa do bind = identidade.
    for j in m.joint_matrices(0, &world) {
        assert!(j.abs_diff_eq(Mat4::IDENTITY, 1e-5));
    }
    assert!(world[5].transform_point3(Vec3::ZERO).abs_diff_eq(Vec3::new(0.0, 2.0, 0.0), 1e-5), "a ponta em y = 2");
}

#[test]
fn glb_export_round_trips_skin_morph_and_clips() {
    let m = Model::from_glb(&sample::tube_glb()).unwrap();
    let back = Model::from_glb(&aylis_3d::glb::to_glb(&m).unwrap()).unwrap();
    assert_eq!(back.nodes.len(), m.nodes.len());
    assert_eq!(back.clips.iter().map(|c| c.name.clone()).collect::<Vec<_>>(), ["Balançar", "Respirar", "Girar"]);
    assert_eq!(back.meshes[0].primitives[0].targets.len(), 1);
    for (clip, t) in [(0, 0.7), (1, 0.4), (2, 1.3)] {
        let (mut a, mut b) = (m.rest_pose(), back.rest_pose());
        m.sample(clip, t, &mut a);
        back.sample(clip, t, &mut b);
        let (wa, wb) = (m.world(&a, Mat4::IDENTITY), back.world(&b, Mat4::IDENTITY));
        for (x, y) in wa.iter().zip(&wb) {
            assert!(x.abs_diff_eq(*y, 1e-5));
        }
        assert_eq!(a.weights, b.weights);
    }
}

#[test]
fn the_demo_scene_moves_the_cube_and_bounces_the_ball() {
    let m = Model::from_glb(&sample::scene_glb()).unwrap();
    let names: Vec<&str> = m.nodes.iter().map(|n| n.name.as_str()).collect();
    assert_eq!(names, ["Cubo", "Bola", "Chão"]);
    let clip = m.clip("Mover").unwrap();
    assert!((m.clips[clip].duration - 4.0).abs() < 1e-5);
    let mut p = m.rest_pose();
    // A bola: no chão no começo de cada quique, no alto no meio (0,5 s).
    m.sample(clip, 0.0, &mut p);
    let ground = p.local[1].0.y;
    m.sample(clip, 0.5, &mut p);
    let top = p.local[1].0.y;
    assert!((ground - 0.3).abs() < 1e-3 && top > 1.2, "{ground} {top}");
    // O cubo: em 1 s (um quarto do clipe) virou 90° em Y e foi para a direita.
    m.sample(clip, 1.0, &mut p);
    assert!(p.local[0].1.angle_between(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)) < 1e-2);
    assert!(p.local[0].0.x > -0.4);
}

/// Um triângulo em glTF (JSON com o buffer embutido): posições, UV e cores dos vértices.
/// `material` e `extensions` entram como estão no JSON.
fn triangle_gltf(material: &str, extensions: &str) -> Vec<u8> {
    fn base64(bytes: &[u8]) -> String {
        const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut s = String::new();
        for c in bytes.chunks(3) {
            let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
            for i in 0..4 {
                s.push(if i <= c.len() { T[(n >> (18 - 6 * i) & 63) as usize] as char } else { '=' });
            }
        }
        s
    }
    let floats: Vec<f32> = [
        [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], // POSITION
    ]
    .concat()
    .into_iter()
    .chain([0.0, 0.0, 1.0, 0.0, 0.0, 1.0]) // TEXCOORD_0
    .chain([1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0]) // COLOR_0
    .collect();
    let bytes: Vec<u8> = floats.iter().flat_map(|f| f.to_le_bytes()).collect();
    format!(
        r#"{{"asset":{{"version":"2.0"}},{extensions}
        "buffers":[{{"byteLength":{len},"uri":"data:application/octet-stream;base64,{data}"}}],
        "bufferViews":[{{"buffer":0,"byteOffset":0,"byteLength":36}},{{"buffer":0,"byteOffset":36,"byteLength":24}},{{"buffer":0,"byteOffset":60,"byteLength":48}}],
        "accessors":[{{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3","min":[0,0,0],"max":[1,1,0]}},{{"bufferView":1,"componentType":5126,"count":3,"type":"VEC2"}},{{"bufferView":2,"componentType":5126,"count":3,"type":"VEC4"}}],
        "materials":[{material}],
        "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0,"TEXCOORD_0":1,"COLOR_0":2}},"material":0}}]}}],
        "nodes":[{{"mesh":0}}],"scenes":[{{"nodes":[0]}}],"scene":0}}"#,
        len = bytes.len(),
        data = base64(&bytes)
    )
    .into_bytes()
}

#[test]
fn a_required_extension_aylis_does_not_know_opens_without_the_effect() {
    // O verniz (KHR_materials_clearcoat) é obrigatório no arquivo e o crate gltf não o conhece.
    let m = Model::from_glb(&triangle_gltf(
        r#"{"extensions":{"KHR_materials_clearcoat":{"clearcoatFactor":1.0}}}"#,
        r#""extensionsUsed":["KHR_materials_clearcoat"],"extensionsRequired":["KHR_materials_clearcoat"],"#,
    ))
    .unwrap();
    assert_eq!(m.meshes[0].primitives[0].vertices.len(), 3);
    assert_eq!((m.materials[0].clearcoat, m.materials[0].clearcoat_roughness), (1.0, 0.0), "o verniz pelos fatores");
    // Os outros erros continuam valendo (um acessor que não existe).
    let broken = String::from_utf8(triangle_gltf("{}", "")).unwrap().replace(r#""COLOR_0":2"#, r#""COLOR_0":9"#);
    assert!(Model::from_glb(broken.as_bytes()).is_err());
}

#[test]
fn vertex_colors_and_the_texture_transform_are_read() {
    let m = Model::from_glb(&triangle_gltf("{}", "")).unwrap();
    let v = &m.meshes[0].primitives[0].vertices;
    assert_eq!(v[0].color, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(v[2].color, [0.0, 0.0, 1.0, 1.0]);
    // Sem a cor no arquivo, o vértice fica branco.
    assert_eq!(aylis_3d::model::Vertex::default().color, [1.0; 4]);

    // KHR_texture_transform: escala 2 e deslocamento 0,5 no UV.
    let m = Model::from_glb(&triangle_gltf(
        r#"{"pbrMetallicRoughness":{"baseColorTexture":{"index":0,"extensions":{"KHR_texture_transform":{"offset":[0.5,0.5],"scale":[2,2]}}}}}"#,
        r#""extensionsUsed":["KHR_texture_transform"],"textures":[{"source":0}],"images":[{"uri":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="}],"#,
    ))
    .unwrap();
    let uv: Vec<[f32; 2]> = m.meshes[0].primitives[0].vertices.iter().map(|v| v.uv).collect();
    assert_eq!(uv, [[0.5, 0.5], [2.5, 0.5], [0.5, 2.5]]);
}
