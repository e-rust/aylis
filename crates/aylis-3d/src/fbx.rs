//! Lê FBX (com o ufbx) e converte para o [`Model`] do aylis: malhas por material, normais,
//! UV, esqueleto (pesos dos ossos e matrizes de bind), blend shapes (morph targets),
//! materiais e texturas embutidas, e um clipe por "take" (pilha de animação), já assado em
//! chaves. A IDE grava o resultado como `.glb` ([`crate::glb::to_glb`]): o app nunca lê FBX.

use std::collections::HashMap;
use std::path::Path;

use glam::{Mat4, Quat, Vec3, Vec4};

use crate::model::{Clip, Interpolation, Material, Mesh, MorphTarget, Node, Primitive, Skin, Texture, Track, Values, Vertex};
use crate::Model;

/// Lê um FBX (binário ou texto). `base_dir`: a pasta do arquivo, para achar texturas que
/// não vêm embutidas.
pub fn from_fbx(bytes: &[u8], base_dir: Option<&Path>) -> Result<Model, String> {
    let opts = ufbx::LoadOpts {
        target_axes: ufbx::CoordinateAxes::right_handed_y_up(),
        target_unit_meters: 1.0,
        generate_missing_normals: true,
        ..Default::default()
    };
    let scene = ufbx::load_memory(bytes, opts).map_err(|e| format!("FBX inválido: {}", e.description))?;

    // ----- materiais e texturas -----
    let mut textures: Vec<Texture> = Vec::new();
    let mut texture_index: HashMap<u32, Option<usize>> = HashMap::new();
    let mut materials: Vec<Material> = Vec::new();
    for m in &scene.materials {
        let pbr = &m.pbr;
        let base = pbr.base_color.value_vec4;
        let factor = if pbr.base_factor.has_value { pbr.base_factor.value_vec4.x } else { 1.0 };
        let opacity = if pbr.opacity.has_value { pbr.opacity.value_vec4.x } else { 1.0 };
        let emission = pbr.emission_color.value_vec4;
        let emission_factor = if pbr.emission_factor.has_value { pbr.emission_factor.value_vec4.x } else { 1.0 };
        let mut texture_of = |map: &ufbx::MaterialMap| {
            map.texture.as_ref().and_then(|t| {
                *texture_index.entry(t.element.typed_id).or_insert_with(|| {
                    let rgba = load_texture(t, base_dir)?;
                    textures.push(rgba);
                    Some(textures.len() - 1)
                })
            })
        };
        let texture = texture_of(&pbr.base_color);
        // Mapa de normais e oclusão; metal e aspereza no FBX vêm em mapas separados (não
        // no par do glTF) e ficam de fora.
        let normal_texture = texture_of(&pbr.normal_map);
        let occlusion_texture = texture_of(&pbr.ambient_occlusion);
        let emissive_texture = texture_of(&pbr.emission_color);
        materials.push(Material {
            base_color: [
                (base.x * factor) as f32,
                (base.y * factor) as f32,
                (base.z * factor) as f32,
                (opacity.clamp(0.0, 1.0)) as f32,
            ],
            texture,
            metallic: if pbr.metalness.has_value { pbr.metalness.value_vec4.x as f32 } else { 0.0 },
            roughness: if pbr.roughness.has_value { pbr.roughness.value_vec4.x as f32 } else { 0.6 },
            emissive: [(emission.x * emission_factor) as f32, (emission.y * emission_factor) as f32, (emission.z * emission_factor) as f32],
            normal_texture,
            occlusion_texture,
            emissive_texture,
            double_sided: false,
            blend: opacity < 0.999,
            ..Material::default()
        });
    }
    let default_material = materials.len();

    // ----- nós -----
    let mut nodes: Vec<Node> = scene
        .nodes
        .iter()
        .map(|n| {
            let t = &n.local_transform;
            Node {
                name: n.element.name.to_string(),
                parent: n.parent.as_ref().map(|p| p.element.typed_id as usize),
                children: n.children.iter().map(|c| c.element.typed_id as usize).collect(),
                translation: vec3(t.translation),
                rotation: quat(t.rotation),
                scale: vec3(t.scale),
                mesh: None,
                skin: None,
                weights: Vec::new(),
            }
        })
        .collect();

    // ----- malhas, esqueletos e blend shapes -----
    let mut meshes: Vec<Mesh> = Vec::new();
    let mut mesh_index: HashMap<u32, usize> = HashMap::new();
    let mut skins: Vec<Skin> = Vec::new();
    // Por malha: os canais de blend shape (para os pesos animados).
    let mut channels_of: HashMap<usize, Vec<u32>> = HashMap::new();
    for node in &scene.nodes {
        let Some(mesh) = node.mesh.as_ref() else { continue };
        let skin = mesh.skin_deformers.first();
        let mi = match mesh_index.get(&mesh.element.typed_id) {
            Some(&mi) => mi,
            None => {
                // Sem esqueleto, a transformação geométrica do nó vai para os vértices.
                let geometry = if skin.is_none() && node.has_geometry_transform { matrix(&node.geometry_to_node) } else { Mat4::IDENTITY };
                let (converted, channels) = convert_mesh(mesh, skin.map(|s| &**s), geometry, &scene, default_material);
                meshes.push(converted);
                mesh_index.insert(mesh.element.typed_id, meshes.len() - 1);
                channels_of.insert(meshes.len() - 1, channels);
                meshes.len() - 1
            }
        };
        let ni = node.element.typed_id as usize;
        nodes[ni].mesh = Some(mi);
        if let Some(s) = skin {
            let root = scene.root_node.element.typed_id as usize;
            skins.push(Skin {
                joints: s.clusters.iter().map(|c| c.bone_node.as_ref().map_or(root, |b| b.element.typed_id as usize)).collect(),
                inverse_bind: s.clusters.iter().map(|c| matrix(&c.geometry_to_bone)).collect(),
            });
            nodes[ni].skin = Some(skins.len() - 1);
        }
    }
    materials.push(Material::default());

    // ----- clipes: um por pilha de animação, assado em chaves lineares -----
    let mut clips = Vec::new();
    for stack in &scene.anim_stacks {
        let bake = ufbx::BakeOpts { trim_start_time: true, key_reduction_enabled: true, ..Default::default() };
        let Ok(baked) = ufbx::bake_anim(&scene, &stack.anim, bake) else { continue };
        let mut tracks = Vec::new();
        for bn in &baked.nodes {
            let node = bn.typed_id as usize;
            if !bn.constant_translation && !bn.translation_keys.is_empty() {
                tracks.push(Track {
                    node,
                    times: bn.translation_keys.iter().map(|k| k.time as f32).collect(),
                    interpolation: Interpolation::Linear,
                    values: Values::Translation(bn.translation_keys.iter().map(|k| vec3(k.value)).collect()),
                });
            }
            if !bn.constant_rotation && !bn.rotation_keys.is_empty() {
                tracks.push(Track {
                    node,
                    times: bn.rotation_keys.iter().map(|k| k.time as f32).collect(),
                    interpolation: Interpolation::Linear,
                    values: Values::Rotation(bn.rotation_keys.iter().map(|k| quat(k.value)).collect()),
                });
            }
            if !bn.constant_scale && !bn.scale_keys.is_empty() {
                tracks.push(Track {
                    node,
                    times: bn.scale_keys.iter().map(|k| k.time as f32).collect(),
                    interpolation: Interpolation::Linear,
                    values: Values::Scale(bn.scale_keys.iter().map(|k| vec3(k.value)).collect()),
                });
            }
        }
        // Pesos dos blend shapes ("DeformPercent" de cada canal), juntos por nó de malha.
        let percent: HashMap<u32, Vec<(f64, f64)>> = baked
            .elements
            .iter()
            .filter_map(|e| {
                let p = e.props.iter().find(|p| &*p.name == "DeformPercent")?;
                Some((e.element_id, p.keys.iter().map(|k| (k.time, k.value.x / 100.0)).collect()))
            })
            .collect();
        for (ni, node) in nodes.iter().enumerate() {
            let Some(channels) = node.mesh.and_then(|m| channels_of.get(&m)) else { continue };
            if channels.is_empty() || !channels.iter().any(|c| percent.contains_key(c)) {
                continue;
            }
            let mut times: Vec<f64> = channels.iter().flat_map(|c| percent.get(c).into_iter().flatten().map(|k| k.0)).collect();
            times.sort_by(f64::total_cmp);
            times.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
            let mut w = Vec::with_capacity(times.len() * channels.len());
            for &t in &times {
                for c in channels {
                    w.push(percent.get(c).map_or(0.0, |keys| sample(keys, t)) as f32);
                }
            }
            tracks.push(Track { node: ni, times: times.iter().map(|&t| t as f32).collect(), interpolation: Interpolation::Linear, values: Values::Weights(w, channels.len()) });
        }
        if tracks.is_empty() {
            continue;
        }
        let duration = tracks.iter().flat_map(|t| t.times.last().copied()).fold(0.0f32, f32::max).max(baked.playback_duration as f32);
        let name = if stack.element.name.is_empty() { format!("Clipe {}", clips.len() + 1) } else { stack.element.name.to_string() };
        clips.push(Clip { name, duration, tracks });
    }

    let root = scene.root_node.element.typed_id as usize;
    let mut order = Vec::with_capacity(nodes.len());
    let mut stack = vec![root];
    while let Some(i) = stack.pop() {
        if order.contains(&i) {
            continue;
        }
        order.push(i);
        stack.extend(nodes[i].children.iter().rev().copied());
    }
    // Luzes e câmeras do FBX não entram (as unidades de luz de cada programa variam).
    let mut model = Model { nodes, roots: vec![root], order, meshes, materials, textures, skins, clips, lights: Vec::new(), cameras: Vec::new(), bounds: (Vec3::ZERO, Vec3::ZERO) };
    model.bounds = model.rest_bounds();
    if model.meshes.is_empty() {
        return Err("o FBX não tem malhas".into());
    }
    Ok(model)
}

/// Uma malha do FBX: triângulos agrupados por material, vértices repetidos só quando
/// mudam normal ou UV. Devolve também os canais de blend shape (na ordem dos targets).
fn convert_mesh(mesh: &ufbx::Mesh, skin: Option<&ufbx::SkinDeformer>, geometry: Mat4, scene: &ufbx::Scene, default_material: usize) -> (Mesh, Vec<u32>) {
    let normal_matrix = geometry.inverse().transpose();
    // Os canais de blend shape com forma.
    let channels: Vec<&ufbx::BlendChannel> = mesh.blend_deformers.iter().flat_map(|d| d.channels.iter()).filter(|c| c.target_shape.is_some()).map(|c| &**c).collect();

    struct Part {
        vertices: Vec<Vertex>,
        sources: Vec<u32>,
        indices: Vec<u32>,
        lookup: HashMap<(u32, [u32; 5]), u32>,
    }
    let mut parts: HashMap<usize, Part> = HashMap::new();
    let mut tri = vec![0u32; mesh.max_face_triangles * 3];
    for (fi, face) in mesh.faces.iter().enumerate() {
        let n = mesh.triangulate_face(&mut tri, *face) as usize;
        let material = mesh
            .face_material
            .get(fi)
            .and_then(|&m| mesh.materials.get(m as usize))
            .map_or(default_material, |m| m.element.typed_id as usize);
        let part = parts.entry(material).or_insert_with(|| Part { vertices: Vec::new(), sources: Vec::new(), indices: Vec::new(), lookup: HashMap::new() });
        for &ix in &tri[..n * 3] {
            let ix = ix as usize;
            let source = mesh.vertex_indices[ix];
            let p = geometry.transform_point3(vec3(mesh.vertex_position[ix]));
            let nrm = if mesh.vertex_normal.exists { normal_matrix.transform_vector3(vec3(mesh.vertex_normal[ix])).normalize_or(Vec3::Y) } else { Vec3::Y };
            let uv = if mesh.vertex_uv.exists {
                let t = mesh.vertex_uv[ix];
                // O V do FBX cresce para cima; o do glTF, para baixo.
                [t.x as f32, 1.0 - t.y as f32]
            } else {
                [0.0, 0.0]
            };
            let key = (source, [nrm.x.to_bits(), nrm.y.to_bits(), nrm.z.to_bits(), uv[0].to_bits(), uv[1].to_bits()]);
            let index = match part.lookup.get(&key) {
                Some(&i) => i,
                None => {
                    let (joints, weights) = skin.map_or(([0; 4], [0.0; 4]), |s| skin_of(s, source as usize));
                    part.vertices.push(Vertex { position: p.into(), normal: nrm.into(), uv, joints, weights, ..Default::default() });
                    part.sources.push(source);
                    let i = part.vertices.len() as u32 - 1;
                    part.lookup.insert(key, i);
                    i
                }
            };
            part.indices.push(index);
        }
    }

    let mut keys: Vec<usize> = parts.keys().copied().collect();
    keys.sort();
    let mut primitives = Vec::new();
    for material in keys {
        let part = parts.remove(&material).unwrap();
        // Morph targets: o deslocamento de cada vértice de origem, levado a cada vértice.
        let targets = channels
            .iter()
            .map(|c| {
                let shape = c.target_shape.as_ref().unwrap();
                let mut pos: HashMap<u32, Vec3> = HashMap::new();
                let mut nrm: HashMap<u32, Vec3> = HashMap::new();
                for (k, &v) in shape.offset_vertices.iter().enumerate() {
                    pos.insert(v, geometry.transform_vector3(vec3(shape.position_offsets[k])));
                    if let Some(n) = shape.normal_offsets.get(k) {
                        nrm.insert(v, normal_matrix.transform_vector3(vec3(*n)));
                    }
                }
                MorphTarget {
                    positions: part.sources.iter().map(|s| pos.get(s).copied().unwrap_or(Vec3::ZERO).into()).collect(),
                    normals: part.sources.iter().map(|s| nrm.get(s).copied().unwrap_or(Vec3::ZERO).into()).collect(),
                }
            })
            .collect();
        primitives.push(Primitive { vertices: part.vertices, indices: part.indices, material, targets });
    }
    let _ = scene;
    let weights = channels.iter().map(|c| c.weight as f32).collect();
    let ids = channels.iter().map(|c| c.element.element_id).collect();
    (Mesh { name: mesh.element.name.to_string(), primitives, weights }, ids)
}

/// Os 4 ossos de maior peso do vértice, com os pesos somando 1.
fn skin_of(skin: &ufbx::SkinDeformer, vertex: usize) -> ([u32; 4], [f32; 4]) {
    let Some(v) = skin.vertices.get(vertex) else { return ([0; 4], [0.0; 4]) };
    let begin = v.weight_begin as usize;
    let mut list: Vec<(u32, f32)> = (begin..begin + v.num_weights as usize)
        .filter_map(|i| skin.weights.get(i))
        .map(|w| (w.cluster_index, w.weight as f32))
        .collect();
    list.sort_by(|a, b| b.1.total_cmp(&a.1));
    list.truncate(4);
    let total: f32 = list.iter().map(|w| w.1).sum();
    let (mut joints, mut weights) = ([0u32; 4], [0.0f32; 4]);
    for (k, (j, w)) in list.into_iter().enumerate() {
        joints[k] = j;
        weights[k] = if total > 0.0 { w / total } else { 0.0 };
    }
    (joints, weights)
}

/// A textura do material: embutida no FBX ou o arquivo ao lado dele.
fn load_texture(t: &ufbx::Texture, base_dir: Option<&Path>) -> Option<Texture> {
    let bytes: Vec<u8> = if !t.content.is_empty() {
        t.content.to_vec()
    } else {
        let candidates = [t.absolute_filename.to_string(), t.relative_filename.to_string(), t.filename.to_string()];
        candidates.iter().filter(|c| !c.is_empty()).find_map(|c| {
            let path = Path::new(c);
            std::fs::read(path).ok().or_else(|| {
                let dir = base_dir?;
                std::fs::read(dir.join(path)).ok().or_else(|| std::fs::read(dir.join(path.file_name()?)).ok())
            })
        })?
    };
    let image = image::load_from_memory(&bytes).ok()?.into_rgba8();
    Some(Texture { width: image.width(), height: image.height(), rgba: image.into_raw() })
}

fn sample(keys: &[(f64, f64)], t: f64) -> f64 {
    match keys.iter().position(|k| k.0 > t) {
        Some(0) => keys[0].1,
        Some(i) => {
            let (a, b) = (keys[i - 1], keys[i]);
            let f = if b.0 > a.0 { (t - a.0) / (b.0 - a.0) } else { 0.0 };
            a.1 + (b.1 - a.1) * f
        }
        None => keys.last().map_or(0.0, |k| k.1),
    }
}

fn vec3(v: ufbx::Vec3) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}

fn quat(q: ufbx::Quat) -> Quat {
    Quat::from_xyzw(q.x as f32, q.y as f32, q.z as f32, q.w as f32).normalize()
}

fn matrix(m: &ufbx::Matrix) -> Mat4 {
    Mat4::from_cols(
        Vec4::new(m.m00 as f32, m.m10 as f32, m.m20 as f32, 0.0),
        Vec4::new(m.m01 as f32, m.m11 as f32, m.m21 as f32, 0.0),
        Vec4::new(m.m02 as f32, m.m12 as f32, m.m22 as f32, 0.0),
        Vec4::new(m.m03 as f32, m.m13 as f32, m.m23 as f32, 1.0),
    )
}
