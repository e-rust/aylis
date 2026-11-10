//! Grava um [`Model`] como `.glb` (glTF 2.0 binário): malhas, materiais e texturas (PNG),
//! nós, esqueletos, morph targets, clipes, luzes (`KHR_lights_punctual`) e câmeras. É o
//! formato que o app carrega; a IDE usa para guardar os modelos importados de FBX.

use serde_json::{Value, json};

use crate::model::{Interpolation, LightKind, Model, Projection, Values};

/// O `.glb` do modelo.
pub fn to_glb(model: &Model) -> Result<Vec<u8>, String> {
    let mut b = Bin::default();

    let images: Vec<Value> = model
        .textures
        .iter()
        .map(|t| -> Result<Value, String> {
            let png = encode_png(t.width, t.height, &t.rgba)?;
            let view = b.view(&png, None);
            Ok(json!({ "bufferView": view, "mimeType": "image/png" }))
        })
        .collect::<Result<_, _>>()?;
    let textures: Vec<Value> = (0..images.len()).map(|i| json!({ "source": i, "sampler": 0 })).collect();

    // O último material do Model é o padrão (para primitivas sem material): fica de fora.
    let used = model.materials.len().saturating_sub(1).max(1).min(model.materials.len());
    let materials: Vec<Value> = model.materials[..used]
        .iter()
        .map(|m| {
            let valid = |t: Option<usize>| t.filter(|&t| t < textures.len());
            let mut pbr = json!({ "baseColorFactor": m.base_color, "metallicFactor": m.metallic, "roughnessFactor": m.roughness });
            if let Some(t) = valid(m.texture) {
                pbr["baseColorTexture"] = json!({ "index": t });
            }
            if let Some(t) = valid(m.metallic_roughness_texture) {
                pbr["metallicRoughnessTexture"] = json!({ "index": t });
            }
            let mut v = json!({
                "pbrMetallicRoughness": pbr,
                "emissiveFactor": m.emissive,
                "doubleSided": m.double_sided,
                "alphaMode": if m.blend { "BLEND" } else { "OPAQUE" },
            });
            if let Some(t) = valid(m.normal_texture) {
                v["normalTexture"] = json!({ "index": t, "scale": m.normal_scale });
            }
            if let Some(t) = valid(m.occlusion_texture) {
                v["occlusionTexture"] = json!({ "index": t, "strength": m.occlusion_strength });
            }
            if let Some(t) = valid(m.emissive_texture) {
                v["emissiveTexture"] = json!({ "index": t });
            }
            v
        })
        .collect();

    // Malhas com esqueleto levam JOINTS/WEIGHTS.
    let skinned: Vec<bool> = (0..model.meshes.len()).map(|m| model.nodes.iter().any(|n| n.mesh == Some(m) && n.skin.is_some())).collect();
    let mut meshes = Vec::new();
    for (mi, mesh) in model.meshes.iter().enumerate() {
        let mut prims = Vec::new();
        for p in &mesh.primitives {
            let positions: Vec<[f32; 3]> = p.vertices.iter().map(|v| v.position).collect();
            let mut attrs = json!({
                "POSITION": b.vec3(&positions, true),
                "NORMAL": b.vec3(&p.vertices.iter().map(|v| v.normal).collect::<Vec<_>>(), false),
            });
            if p.vertices.iter().any(|v| v.uv != [0.0, 0.0]) {
                attrs["TEXCOORD_0"] = json!(b.floats(&p.vertices.iter().flat_map(|v| v.uv).collect::<Vec<_>>(), "VEC2", Some(34962)));
            }
            if p.vertices.iter().any(|v| v.color != [1.0; 4]) {
                attrs["COLOR_0"] = json!(b.floats(&p.vertices.iter().flat_map(|v| v.color).collect::<Vec<_>>(), "VEC4", Some(34962)));
            }
            if skinned[mi] {
                let joints: Vec<u16> = p.vertices.iter().flat_map(|v| v.joints.map(|j| j.min(u16::MAX as u32) as u16)).collect();
                attrs["JOINTS_0"] = json!(b.accessor(&bytes_u16(&joints), 5123, p.vertices.len(), "VEC4", Some(34962), None));
                attrs["WEIGHTS_0"] = json!(b.floats(&p.vertices.iter().flat_map(|v| v.weights).collect::<Vec<_>>(), "VEC4", Some(34962)));
            }
            let indices = b.accessor(&bytes_u32(&p.indices), 5125, p.indices.len(), "SCALAR", Some(34963), None);
            let mut prim = json!({ "attributes": attrs, "indices": indices });
            if p.material < materials.len() {
                prim["material"] = json!(p.material);
            }
            if !p.targets.is_empty() {
                let targets: Vec<Value> = p
                    .targets
                    .iter()
                    .map(|t| json!({ "POSITION": b.vec3(&t.positions, true), "NORMAL": b.vec3(&t.normals, false) }))
                    .collect();
                prim["targets"] = json!(targets);
            }
            prims.push(prim);
        }
        let mut m = json!({ "name": mesh.name, "primitives": prims });
        if !mesh.weights.is_empty() {
            m["weights"] = json!(mesh.weights);
        }
        meshes.push(m);
    }

    let nodes: Vec<Value> = model
        .nodes
        .iter()
        .map(|n| {
            let mut v = json!({
                "name": n.name,
                "translation": n.translation.to_array(),
                "rotation": n.rotation.to_array(),
                "scale": n.scale.to_array(),
            });
            if !n.children.is_empty() {
                v["children"] = json!(n.children);
            }
            if let Some(m) = n.mesh {
                v["mesh"] = json!(m);
            }
            if let Some(s) = n.skin {
                v["skin"] = json!(s);
            }
            if !n.weights.is_empty() {
                v["weights"] = json!(n.weights);
            }
            v
        })
        .collect();

    // Luzes e câmeras: uma de cada por nó no glTF; as que sobram vão num nó filho.
    let mut nodes = nodes;
    let attach = |nodes: &mut Vec<Value>, node: usize, key: &str, value: Value| -> usize {
        let taken = |n: &Value| if key == "camera" { n.get("camera").is_some() } else { n.pointer("/extensions/KHR_lights_punctual").is_some() };
        let target = if nodes.get(node).is_some_and(|n| !taken(n)) {
            node
        } else {
            nodes.push(json!({ "name": format!("{key} {}", nodes.len()) }));
            let child = nodes.len() - 1;
            if let Some(children) = nodes.get_mut(node).map(|n| n.as_object_mut().unwrap().entry("children").or_insert(json!([]))) {
                children.as_array_mut().unwrap().push(json!(child));
            }
            child
        };
        if key == "camera" {
            nodes[target]["camera"] = value;
        } else {
            nodes[target]["extensions"] = json!({ "KHR_lights_punctual": { "light": value } });
        }
        target
    };
    let lights: Vec<Value> = model
        .lights
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mut v = json!({ "name": l.name, "color": l.color, "intensity": l.intensity });
            match l.kind {
                LightKind::Directional => v["type"] = json!("directional"),
                LightKind::Point => v["type"] = json!("point"),
                LightKind::Spot { inner, outer } => {
                    v["type"] = json!("spot");
                    v["spot"] = json!({ "innerConeAngle": inner, "outerConeAngle": outer });
                }
            }
            if let Some(r) = l.range {
                v["range"] = json!(r);
            }
            attach(&mut nodes, l.node, "light", json!(i));
            v
        })
        .collect();
    let cameras: Vec<Value> = model
        .cameras
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let v = match c.projection {
                Projection::Perspective { yfov, aspect, znear, zfar } => {
                    let mut p = json!({ "yfov": yfov, "znear": znear });
                    if let Some(a) = aspect {
                        p["aspectRatio"] = json!(a);
                    }
                    if let Some(z) = zfar {
                        p["zfar"] = json!(z);
                    }
                    json!({ "name": c.name, "type": "perspective", "perspective": p })
                }
                Projection::Orthographic { xmag, ymag, znear, zfar } => {
                    json!({ "name": c.name, "type": "orthographic", "orthographic": { "xmag": xmag, "ymag": ymag, "znear": znear, "zfar": zfar } })
                }
            };
            attach(&mut nodes, c.node, "camera", json!(i));
            v
        })
        .collect();

    let skins: Vec<Value> = model
        .skins
        .iter()
        .map(|s| {
            let ibm: Vec<f32> = s.inverse_bind.iter().flat_map(|m| m.to_cols_array()).collect();
            json!({ "joints": s.joints, "inverseBindMatrices": b.floats(&ibm, "MAT4", None) })
        })
        .collect();

    let mut animations = Vec::new();
    for clip in &model.clips {
        let (mut samplers, mut channels) = (Vec::new(), Vec::new());
        for track in &clip.tracks {
            if track.times.is_empty() {
                continue;
            }
            let lo = track.times.iter().copied().fold(f32::MAX, f32::min);
            let hi = track.times.iter().copied().fold(f32::MIN, f32::max);
            let input = b.accessor(&bytes_f32(&track.times), 5126, track.times.len(), "SCALAR", None, Some((vec![lo], vec![hi])));
            let (output, path) = match &track.values {
                Values::Translation(v) => (b.floats(&v.iter().flat_map(|x| x.to_array()).collect::<Vec<_>>(), "VEC3", None), "translation"),
                Values::Scale(v) => (b.floats(&v.iter().flat_map(|x| x.to_array()).collect::<Vec<_>>(), "VEC3", None), "scale"),
                Values::Rotation(v) => (b.floats(&v.iter().flat_map(|x| x.to_array()).collect::<Vec<_>>(), "VEC4", None), "rotation"),
                Values::Weights(w, _) => (b.floats(w, "SCALAR", None), "weights"),
            };
            let interpolation = match track.interpolation {
                Interpolation::Linear => "LINEAR",
                Interpolation::Step => "STEP",
                Interpolation::Cubic => "CUBICSPLINE",
            };
            channels.push(json!({ "sampler": samplers.len(), "target": { "node": track.node, "path": path } }));
            samplers.push(json!({ "input": input, "output": output, "interpolation": interpolation }));
        }
        if !channels.is_empty() {
            animations.push(json!({ "name": clip.name, "samplers": samplers, "channels": channels }));
        }
    }

    let mut doc = json!({
        "asset": { "version": "2.0", "generator": "aylis-3d" },
        "scene": 0,
        "scenes": [{ "nodes": model.roots }],
        "nodes": nodes,
        "meshes": meshes,
        "materials": materials,
        "buffers": [{ "byteLength": b.data.len() }],
        "bufferViews": b.views,
        "accessors": b.accessors,
    });
    if !skins.is_empty() {
        doc["skins"] = json!(skins);
    }
    if !animations.is_empty() {
        doc["animations"] = json!(animations);
    }
    if !lights.is_empty() {
        doc["extensions"] = json!({ "KHR_lights_punctual": { "lights": lights } });
        doc["extensionsUsed"] = json!(["KHR_lights_punctual"]);
    }
    if !cameras.is_empty() {
        doc["cameras"] = json!(cameras);
    }
    if !images.is_empty() {
        doc["images"] = json!(images);
        doc["textures"] = json!(textures);
        doc["samplers"] = json!([{ "magFilter": 9729, "minFilter": 9987, "wrapS": 10497, "wrapT": 10497 }]);
    }
    let text = serde_json::to_string(&doc).map_err(|e| e.to_string())?;
    Ok(container(text.as_bytes(), &b.data))
}

#[derive(Default)]
struct Bin {
    data: Vec<u8>,
    views: Vec<Value>,
    accessors: Vec<Value>,
}

impl Bin {
    fn view(&mut self, bytes: &[u8], target: Option<u32>) -> usize {
        while self.data.len() % 4 != 0 {
            self.data.push(0);
        }
        let mut v = json!({ "buffer": 0, "byteOffset": self.data.len(), "byteLength": bytes.len() });
        if let Some(t) = target {
            v["target"] = json!(t);
        }
        self.data.extend_from_slice(bytes);
        self.views.push(v);
        self.views.len() - 1
    }

    fn accessor(&mut self, bytes: &[u8], component: u32, count: usize, kind: &str, target: Option<u32>, bounds: Option<(Vec<f32>, Vec<f32>)>) -> usize {
        let view = self.view(bytes, target);
        let mut a = json!({ "bufferView": view, "componentType": component, "count": count, "type": kind });
        if let Some((lo, hi)) = bounds {
            a["min"] = json!(lo);
            a["max"] = json!(hi);
        }
        self.accessors.push(a);
        self.accessors.len() - 1
    }

    fn floats(&mut self, values: &[f32], kind: &str, target: Option<u32>) -> usize {
        let per = match kind {
            "SCALAR" => 1,
            "VEC2" => 2,
            "VEC3" => 3,
            "VEC4" => 4,
            _ => 16,
        };
        self.accessor(&bytes_f32(values), 5126, values.len() / per, kind, target, None)
    }

    fn vec3(&mut self, v: &[[f32; 3]], bounds: bool) -> usize {
        let flat: Vec<f32> = v.iter().flatten().copied().collect();
        let mm = (bounds && !v.is_empty()).then(|| {
            let lo = (0..3).map(|i| v.iter().map(|p| p[i]).fold(f32::MAX, f32::min)).collect();
            let hi = (0..3).map(|i| v.iter().map(|p| p[i]).fold(f32::MIN, f32::max)).collect();
            (lo, hi)
        });
        self.accessor(&bytes_f32(&flat), 5126, v.len(), "VEC3", Some(34962), mm)
    }
}

fn bytes_f32(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

fn bytes_u32(v: &[u32]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

fn bytes_u16(v: &[u16]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = std::io::Cursor::new(Vec::new());
    image::write_buffer_with_format(&mut out, rgba, width, height, image::ExtendedColorType::Rgba8, image::ImageFormat::Png).map_err(|e| format!("PNG: {e}"))?;
    Ok(out.into_inner())
}

/// O contêiner GLB: cabeçalho, pedaço JSON e pedaço binário.
pub(crate) fn container(json: &[u8], bin: &[u8]) -> Vec<u8> {
    let mut j = json.to_vec();
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
