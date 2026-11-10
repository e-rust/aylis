//! Importar FBX: o cubo com dois ossos escrito à mão (`data/cube_bones.fbx`).
#![cfg(feature = "fbx")]

use glam::{Mat4, Vec3};
use aylis_3d::{Model, fbx, glb};

fn load() -> Model {
    fbx::from_fbx(include_bytes!("data/cube_bones.fbx"), None).unwrap()
}

/// Onde vai o vértice `v` da primeira primitiva com a pose do clipe 0 no instante `t`.
fn skinned(m: &Model, t: f32, v: usize) -> Vec3 {
    let mut pose = m.rest_pose();
    m.sample(0, t, &mut pose);
    let world = m.world(&pose, Mat4::IDENTITY);
    let joints = m.joint_matrices(0, &world);
    let vert = m.meshes[0].primitives[0].vertices[v];
    (0..4).map(|k| joints[vert.joints[k] as usize].transform_point3(Vec3::from(vert.position)) * vert.weights[k]).sum()
}

#[test]
fn converts_mesh_skin_material_and_clip() {
    let m = load();
    assert_eq!(m.meshes.len(), 1);
    let prim = &m.meshes[0].primitives[0];
    assert_eq!(prim.indices.len(), 36, "6 quadrados viram 12 triângulos");
    assert_eq!(m.skins.len(), 1);
    assert_eq!(m.skins[0].joints.len(), 2);
    let mat = &m.materials[prim.material];
    assert!((mat.base_color[0] - 0.8).abs() < 0.01 && mat.base_color[1] < 0.2, "{:?}", mat.base_color);
    assert_eq!(m.clips.len(), 1);
    assert_eq!(m.clips[0].name, "Dobrar");
    assert!((m.clips[0].duration - 1.0).abs() < 0.05, "{}", m.clips[0].duration);
    let (lo, hi) = m.bounds;
    assert!((hi.y - lo.y - 1.0).abs() < 1e-3, "um metro de altura: {lo:?} {hi:?}");
}

#[test]
fn bones_bend_the_top_of_the_cube() {
    let m = load();
    let prim = &m.meshes[0].primitives[0];
    // Um vértice de cima (y = 1) com x = 0,5: o osso de cima gira 90° em Z no fim do clipe.
    let v = prim.vertices.iter().position(|v| v.position[1] > 0.99 && v.position[0] > 0.49).unwrap();
    let rest = skinned(&m, 0.0, v);
    assert!(rest.abs_diff_eq(Vec3::from(prim.vertices[v].position), 1e-4), "{rest:?}");
    let end = skinned(&m, 1.0, v);
    // (0,5; 1; z) em volta de (0; 1; 0) por 90° em Z vai para (0; 1,5; z).
    assert!((end.x - 0.0).abs() < 0.02 && (end.y - 1.5).abs() < 0.02, "{end:?}");
    // Os de baixo ficam onde estão.
    let b = prim.vertices.iter().position(|v| v.position[1] < 0.01).unwrap();
    assert!(skinned(&m, 1.0, b).abs_diff_eq(Vec3::from(prim.vertices[b].position), 1e-4));
}

#[test]
fn round_trips_through_glb() {
    let m = load();
    let bytes = glb::to_glb(&m).unwrap();
    let back = Model::from_glb(&bytes).unwrap();
    assert_eq!(back.meshes[0].primitives[0].indices.len(), 36);
    assert_eq!(back.skins[0].joints.len(), 2);
    assert_eq!(back.clips[0].name, "Dobrar");
    let prim = &m.meshes[0].primitives[0];
    let v = prim.vertices.iter().position(|v| v.position[1] > 0.99 && v.position[0] > 0.49).unwrap();
    for t in [0.0, 0.5, 1.0] {
        assert!(skinned(&m, t, v).abs_diff_eq(skinned(&back, t, v), 1e-4), "em {t}");
    }
    assert!((back.materials[back.meshes[0].primitives[0].material].base_color[0] - 0.8).abs() < 0.01);
}

#[test]
fn rejects_garbage() {
    assert!(fbx::from_fbx(b"isto nao e um fbx", None).is_err());
}


/// Um FBX de verdade, exportado do Blender (`assimp_huesitos.fbx`, dos modelos de teste do
/// Assimp, licença BSD em `data/LICENSE-assimp.txt`): malha com pele, esqueleto e animação.
#[test]
fn a_real_blender_fbx_with_bones_and_animation() {
    let m = fbx::from_fbx(include_bytes!("data/assimp_huesitos.fbx"), None).unwrap();
    assert_eq!(m.meshes.len(), 1);
    assert_eq!(m.skins.len(), 1);
    assert_eq!(m.clips.len(), 1);
    assert!((m.clips[0].duration - 1.56).abs() < 0.05, "{}", m.clips[0].duration);
    // A animação mexe na malha: a pose no meio do clipe é outra que a de repouso.
    let rest = m.world(&m.rest_pose(), Mat4::IDENTITY);
    let mut pose = m.rest_pose();
    m.sample(0, m.clips[0].duration / 2.0, &mut pose);
    let moved = m.world(&pose, Mat4::IDENTITY);
    assert!(rest.iter().zip(&moved).any(|(a, b)| !a.abs_diff_eq(*b, 1e-3)), "a animação não mexeu nos ossos");
    // E sai em GLB e volta igual.
    let back = aylis_3d::Model::from_glb(&glb::to_glb(&m).unwrap()).unwrap();
    assert_eq!(back.clips.len(), 1);
}
