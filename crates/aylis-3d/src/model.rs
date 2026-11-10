//! O modelo lido do glTF/GLB (malhas, materiais, nós, esqueletos e clipes) e a pose:
//! amostrar um clipe num instante, misturar dois clipes e calcular as matrizes do mundo e
//! dos ossos. Nada aqui usa a GPU.

use glam::{Mat4, Quat, Vec3};

/// Um vértice como vai para a GPU.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub joints: [u32; 4],
    pub weights: [f32; 4],
    /// Cor do vértice (`COLOR_0`), multiplicada pela do material. Branca sem ela.
    pub color: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex { position: [0.0; 3], normal: [0.0; 3], uv: [0.0; 2], joints: [0; 4], weights: [0.0; 4], color: [1.0; 4] }
    }
}

/// Diferença de posição e normal de um morph target (por vértice).
#[derive(Clone, Debug, Default)]
pub struct MorphTarget {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
}

#[derive(Clone, Debug, Default)]
pub struct Primitive {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: usize,
    pub targets: Vec<MorphTarget>,
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub name: String,
    pub primitives: Vec<Primitive>,
    /// Pesos dos morph targets quando o nó não diz outros.
    pub weights: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub base_color: [f32; 4],
    /// Textura da cor (sRGB), multiplicando `base_color`.
    pub texture: Option<usize>,
    pub metallic: f32,
    pub roughness: f32,
    /// Metal no canal azul e aspereza no verde (como no glTF), multiplicando os fatores.
    pub metallic_roughness_texture: Option<usize>,
    /// Mapa de normais (espaço tangente, +Y para cima) e a força dele.
    pub normal_texture: Option<usize>,
    pub normal_scale: f32,
    /// Oclusão de ambiente (canal vermelho) e a força dela.
    pub occlusion_texture: Option<usize>,
    pub occlusion_strength: f32,
    pub emissive: [f32; 3],
    /// Textura da emissão (sRGB), multiplicando `emissive`.
    pub emissive_texture: Option<usize>,
    pub double_sided: bool,
    /// Transparente (`alphaMode: BLEND`): desenhado depois dos opacos.
    pub blend: bool,
    /// Verniz (`KHR_materials_clearcoat`): uma camada brilhante por cima, com a força e a
    /// aspereza dela (0 = sem verniz).
    pub clearcoat: f32,
    pub clearcoat_roughness: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: [1.0; 4],
            texture: None,
            metallic: 0.0,
            roughness: 0.6,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            emissive: [0.0; 3],
            emissive_texture: None,
            double_sided: false,
            blend: false,
            clearcoat: 0.0,
            clearcoat_roughness: 0.0,
        }
    }
}

/// Tipo de uma luz do arquivo (`KHR_lights_punctual`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightKind {
    /// Como o sol: só a direção conta (o -Z do nó).
    Directional,
    /// Em todas as direções, a partir da posição do nó.
    Point,
    /// Cone no -Z do nó; ângulos em radianos.
    Spot { inner: f32, outer: f32 },
}

/// Uma luz presa a um nó (anda com a animação dele).
#[derive(Clone, Debug, PartialEq)]
pub struct Light {
    pub name: String,
    pub node: usize,
    pub kind: LightKind,
    /// Cor linear.
    pub color: [f32; 3],
    /// Lux na direcional; candela (lm/sr) na pontual e no cone.
    pub intensity: f32,
    /// Distância em que a luz some (sem = infinita).
    pub range: Option<f32>,
}

/// Projeção de uma câmera do arquivo.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Projection {
    /// `yfov` em radianos; sem `aspect`, a do controle.
    Perspective { yfov: f32, aspect: Option<f32>, znear: f32, zfar: Option<f32> },
    /// Meia largura e meia altura da vista.
    Orthographic { xmag: f32, ymag: f32, znear: f32, zfar: f32 },
}

/// Uma câmera presa a um nó (olha para o -Z dele).
#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub name: String,
    pub node: usize,
    pub projection: Projection,
}

/// Imagem RGBA (8 bits por canal, sRGB).
#[derive(Clone, Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub mesh: Option<usize>,
    pub skin: Option<usize>,
    pub weights: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct Skin {
    pub joints: Vec<usize>,
    pub inverse_bind: Vec<Mat4>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    Linear,
    Step,
    /// Spline cúbica: cada chave guarda tangente de entrada, valor e tangente de saída.
    Cubic,
}

#[derive(Clone, Debug)]
pub enum Values {
    Translation(Vec<Vec3>),
    Rotation(Vec<Quat>),
    Scale(Vec<Vec3>),
    /// Pesos dos morph targets, `n` por chave.
    Weights(Vec<f32>, usize),
}

#[derive(Clone, Debug)]
pub struct Track {
    pub node: usize,
    pub times: Vec<f32>,
    pub interpolation: Interpolation,
    pub values: Values,
}

/// Um clipe de animação ("andar", "correr"...).
#[derive(Clone, Debug)]
pub struct Clip {
    pub name: String,
    pub duration: f32,
    pub tracks: Vec<Track>,
}

/// Um modelo 3D pronto para desenhar e animar.
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub nodes: Vec<Node>,
    /// Nós sem pai da cena, e todos os nós com pai antes dos filhos.
    pub roots: Vec<usize>,
    pub order: Vec<usize>,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub textures: Vec<Texture>,
    pub skins: Vec<Skin>,
    pub clips: Vec<Clip>,
    /// Luzes do arquivo (`KHR_lights_punctual`).
    pub lights: Vec<Light>,
    /// Câmeras do arquivo.
    pub cameras: Vec<Camera>,
    /// Caixa da pose de repouso (mínimo, máximo).
    pub bounds: (Vec3, Vec3),
}

/// Pose dos nós: transformação local e pesos dos morph targets.
#[derive(Clone, Debug, PartialEq)]
pub struct Pose {
    pub local: Vec<(Vec3, Quat, Vec3)>,
    pub weights: Vec<Vec<f32>>,
}

/// Lê o glTF validando tudo, menos as extensões obrigatórias que o crate gltf não conhece
/// (o verniz do `KHR_materials_clearcoat`, que o aylis lê à parte, ou a dispersão do
/// `KHR_materials_dispersion`): o modelo abre sem o que faltar.
fn import(bytes: &[u8]) -> Result<(gltf::Document, Vec<gltf::buffer::Data>, Vec<gltf::image::Data>), String> {
    use gltf::json::validation::{Error, Validate};
    let gltf = gltf::Gltf::from_slice_without_validation(bytes).map_err(|e| format!("glTF inválido: {e}"))?;
    let root = gltf.document.as_json();
    let mut errors = Vec::new();
    root.validate(root, gltf::json::Path::new, &mut |path, error| {
        let path = path();
        if !(matches!(error, Error::Unsupported) && path.as_str().starts_with("extensionsRequired")) {
            errors.push(format!("{path} = {error}"));
        }
    });
    if !errors.is_empty() {
        return Err(format!("glTF inválido: {}", errors.join("; ")));
    }
    let buffers = gltf::import_buffers(&gltf.document, None, gltf.blob).map_err(|e| format!("glTF inválido: {e}"))?;
    // O gltf só decodifica imagem embutida (`data:`) com uma pasta base: passa uma quando
    // todas são embutidas (assim nenhum arquivo do disco é lido).
    let embedded = gltf.document.images().all(|i| !matches!(i.source(), gltf::image::Source::Uri { uri, .. } if !uri.starts_with("data:")));
    let base = embedded.then_some(std::path::Path::new("."));
    let images = gltf::import_images(&gltf.document, base, &buffers).map_err(|e| format!("glTF inválido: {e}"))?;
    Ok((gltf.document, buffers, images))
}

impl Model {
    /// Lê um `.glb` (ou `.gltf` com tudo embutido).
    pub fn from_glb(bytes: &[u8]) -> Result<Model, String> {
        let (doc, buffers, images) = import(bytes)?;
        let get = |b: gltf::Buffer| buffers.get(b.index()).map(|d| d.0.as_slice());

        let textures = images
            .iter()
            .map(|img| Texture { width: img.width, height: img.height, rgba: to_rgba(img) })
            .collect::<Vec<_>>();
        // Só o primeiro conjunto de UV (TEXCOORD_0) é lido: texturas de outro conjunto ficam
        // de fora.
        let uv0 = |t: gltf::texture::Info| (t.tex_coord() == 0).then(|| t.texture().source().index());
        let materials: Vec<Material> = doc
            .materials()
            .map(|m| {
                let pbr = m.pbr_metallic_roughness();
                // O verniz entra só pelos fatores (as texturas dele ficam de fora).
                let coat = |key: &str| m.extension_value("KHR_materials_clearcoat").and_then(|c| c.get(key)).and_then(serde_json::Value::as_f64).unwrap_or(0.0) as f32;
                Material {
                    base_color: pbr.base_color_factor(),
                    texture: pbr.base_color_texture().and_then(uv0),
                    metallic: pbr.metallic_factor(),
                    roughness: pbr.roughness_factor(),
                    metallic_roughness_texture: pbr.metallic_roughness_texture().and_then(uv0),
                    normal_texture: m.normal_texture().filter(|t| t.tex_coord() == 0).map(|t| t.texture().source().index()),
                    normal_scale: m.normal_texture().map_or(1.0, |t| t.scale()),
                    occlusion_texture: m.occlusion_texture().filter(|t| t.tex_coord() == 0).map(|t| t.texture().source().index()),
                    occlusion_strength: m.occlusion_texture().map_or(1.0, |t| t.strength()),
                    emissive: m.emissive_factor(),
                    emissive_texture: m.emissive_texture().and_then(uv0),
                    double_sided: m.double_sided(),
                    blend: m.alpha_mode() == gltf::material::AlphaMode::Blend,
                    clearcoat: coat("clearcoatFactor"),
                    clearcoat_roughness: coat("clearcoatRoughnessFactor"),
                }
            })
            .collect();
        let default_material = materials.len();

        let mut meshes = Vec::new();
        for mesh in doc.meshes() {
            let mut primitives = Vec::new();
            for prim in mesh.primitives() {
                if prim.mode() != gltf::mesh::Mode::Triangles {
                    continue;
                }
                let r = prim.reader(get);
                let Some(positions) = r.read_positions() else { continue };
                let positions: Vec<[f32; 3]> = positions.collect();
                let n = positions.len();
                let normals: Vec<[f32; 3]> = r.read_normals().map(|i| i.collect()).unwrap_or_default();
                let mut uvs: Vec<[f32; 2]> = r.read_tex_coords(0).map(|i| i.into_f32().collect()).unwrap_or_default();
                // KHR_texture_transform: o da textura de cor vale para o conjunto de UV (as
                // texturas do material costumam ter a mesma transformação).
                if let Some(t) = prim.material().pbr_metallic_roughness().base_color_texture().and_then(|t| t.texture_transform()) {
                    let ([ox, oy], [sx, sy], (sin, cos)) = (t.offset(), t.scale(), t.rotation().sin_cos());
                    for [u, v] in &mut uvs {
                        (*u, *v) = (cos * sx * *u + sin * sy * *v + ox, -sin * sx * *u + cos * sy * *v + oy);
                    }
                }
                let colors: Vec<[f32; 4]> = r.read_colors(0).map(|i| i.into_rgba_f32().collect()).unwrap_or_default();
                let joints: Vec<[u16; 4]> = r.read_joints(0).map(|i| i.into_u16().collect()).unwrap_or_default();
                let weights: Vec<[f32; 4]> = r.read_weights(0).map(|i| i.into_f32().collect()).unwrap_or_default();
                let indices: Vec<u32> = r.read_indices().map(|i| i.into_u32().collect()).unwrap_or_else(|| (0..n as u32).collect());
                let mut vertices: Vec<Vertex> = (0..n)
                    .map(|i| Vertex {
                        position: positions[i],
                        normal: normals.get(i).copied().unwrap_or([0.0, 1.0, 0.0]),
                        uv: uvs.get(i).copied().unwrap_or_default(),
                        joints: joints.get(i).map_or([0; 4], |j| j.map(u32::from)),
                        weights: weights.get(i).copied().unwrap_or_default(),
                        color: colors.get(i).copied().unwrap_or([1.0; 4]),
                    })
                    .collect();
                if normals.is_empty() {
                    compute_normals(&mut vertices, &indices);
                }
                let targets = r
                    .read_morph_targets()
                    .map(|(p, nrm, _)| MorphTarget {
                        positions: p.map(|i| i.collect()).unwrap_or_else(|| vec![[0.0; 3]; n]),
                        normals: nrm.map(|i| i.collect()).unwrap_or_else(|| vec![[0.0; 3]; n]),
                    })
                    .collect();
                primitives.push(Primitive { vertices, indices, material: prim.material().index().unwrap_or(default_material), targets });
            }
            meshes.push(Mesh { name: mesh.name().unwrap_or_default().to_string(), primitives, weights: mesh.weights().map(<[f32]>::to_vec).unwrap_or_default() });
        }
        let mut materials = materials;
        materials.push(Material::default());

        let mut nodes: Vec<Node> = doc
            .nodes()
            .map(|n| {
                let (t, r, s) = n.transform().decomposed();
                Node {
                    name: n.name().unwrap_or_default().to_string(),
                    parent: None,
                    children: n.children().map(|c| c.index()).collect(),
                    translation: Vec3::from(t),
                    rotation: Quat::from_array(r).normalize(),
                    scale: Vec3::from(s),
                    mesh: n.mesh().map(|m| m.index()),
                    skin: n.skin().map(|s| s.index()),
                    weights: n.weights().map(<[f32]>::to_vec).unwrap_or_default(),
                }
            })
            .collect();
        for i in 0..nodes.len() {
            for c in nodes[i].children.clone() {
                if let Some(child) = nodes.get_mut(c) {
                    child.parent = Some(i);
                }
            }
        }
        let roots: Vec<usize> = match doc.default_scene().or_else(|| doc.scenes().next()) {
            Some(scene) => scene.nodes().map(|n| n.index()).collect(),
            None => (0..nodes.len()).filter(|&i| nodes[i].parent.is_none()).collect(),
        };
        let mut order = Vec::with_capacity(nodes.len());
        let mut stack: Vec<usize> = roots.iter().rev().copied().collect();
        while let Some(i) = stack.pop() {
            if order.contains(&i) {
                continue;
            }
            order.push(i);
            stack.extend(nodes[i].children.iter().rev().copied());
        }

        let skins = doc
            .skins()
            .map(|s| {
                let joints: Vec<usize> = s.joints().map(|j| j.index()).collect();
                let inverse_bind: Vec<Mat4> = s
                    .reader(get)
                    .read_inverse_bind_matrices()
                    .map(|i| i.map(|m| Mat4::from_cols_array_2d(&m)).collect())
                    .unwrap_or_else(|| vec![Mat4::IDENTITY; joints.len()]);
                Skin { joints, inverse_bind }
            })
            .collect();

        let mut clips = Vec::new();
        for (ai, anim) in doc.animations().enumerate() {
            let mut tracks = Vec::new();
            let mut duration = 0.0f32;
            for ch in anim.channels() {
                let r = ch.reader(get);
                let (Some(inputs), Some(outputs)) = (r.read_inputs(), r.read_outputs()) else { continue };
                let times: Vec<f32> = inputs.collect();
                duration = duration.max(times.last().copied().unwrap_or(0.0));
                let interpolation = match ch.sampler().interpolation() {
                    gltf::animation::Interpolation::Linear => Interpolation::Linear,
                    gltf::animation::Interpolation::Step => Interpolation::Step,
                    gltf::animation::Interpolation::CubicSpline => Interpolation::Cubic,
                };
                use gltf::animation::util::ReadOutputs;
                let values = match outputs {
                    ReadOutputs::Translations(i) => Values::Translation(i.map(Vec3::from).collect()),
                    ReadOutputs::Rotations(i) => Values::Rotation(i.into_f32().map(Quat::from_array).collect()),
                    ReadOutputs::Scales(i) => Values::Scale(i.map(Vec3::from).collect()),
                    ReadOutputs::MorphTargetWeights(i) => {
                        let w: Vec<f32> = i.into_f32().collect();
                        let keys = times.len().max(1) * if interpolation == Interpolation::Cubic { 3 } else { 1 };
                        let n = w.len() / keys;
                        Values::Weights(w, n)
                    }
                };
                tracks.push(Track { node: ch.target().node().index(), times, interpolation, values });
            }
            let name = anim.name().map(str::to_string).unwrap_or_else(|| format!("Clipe {}", ai + 1));
            clips.push(Clip { name, duration, tracks });
        }

        let mut lights = Vec::new();
        let mut cameras = Vec::new();
        for node in doc.nodes() {
            let name = |own: Option<&str>| own.filter(|n| !n.is_empty()).or(node.name()).unwrap_or_default().to_string();
            if let Some(light) = node.light() {
                use gltf::khr_lights_punctual::Kind;
                let kind = match light.kind() {
                    Kind::Directional => LightKind::Directional,
                    Kind::Point => LightKind::Point,
                    Kind::Spot { inner_cone_angle, outer_cone_angle } => LightKind::Spot { inner: inner_cone_angle, outer: outer_cone_angle },
                };
                lights.push(Light { name: name(light.name()), node: node.index(), kind, color: light.color(), intensity: light.intensity(), range: light.range() });
            }
            if let Some(camera) = node.camera() {
                let projection = match camera.projection() {
                    gltf::camera::Projection::Perspective(p) => Projection::Perspective { yfov: p.yfov(), aspect: p.aspect_ratio(), znear: p.znear(), zfar: p.zfar() },
                    gltf::camera::Projection::Orthographic(o) => Projection::Orthographic { xmag: o.xmag(), ymag: o.ymag(), znear: o.znear(), zfar: o.zfar() },
                };
                cameras.push(Camera { name: name(camera.name()), node: node.index(), projection });
            }
        }

        let mut model = Model { nodes, roots, order, meshes, materials, textures, skins, clips, lights, cameras, bounds: (Vec3::ZERO, Vec3::ZERO) };
        model.bounds = model.rest_bounds();
        Ok(model)
    }

    /// A câmera pelo nome.
    pub fn camera(&self, name: &str) -> Option<&Camera> {
        self.cameras.iter().find(|c| c.name == name)
    }

    pub fn clip(&self, name: &str) -> Option<usize> {
        self.clips.iter().position(|c| c.name == name)
    }

    /// A pose de repouso (a do arquivo).
    pub fn rest_pose(&self) -> Pose {
        Pose {
            local: self.nodes.iter().map(|n| (n.translation, n.rotation, n.scale)).collect(),
            weights: self
                .nodes
                .iter()
                .map(|n| if !n.weights.is_empty() { n.weights.clone() } else { n.mesh.map(|m| self.meshes[m].weights.clone()).unwrap_or_default() })
                .collect(),
        }
    }

    /// Aplica o clipe `clip` no instante `t` (segundos) sobre `pose`.
    pub fn sample(&self, clip: usize, t: f32, pose: &mut Pose) {
        let Some(clip) = self.clips.get(clip) else { return };
        for track in &clip.tracks {
            let Some(local) = pose.local.get_mut(track.node) else { continue };
            let (i, f) = key(&track.times, t);
            let cubic = track.interpolation == Interpolation::Cubic;
            let span = if i + 1 < track.times.len() { track.times[i + 1] - track.times[i] } else { 0.0 };
            match &track.values {
                Values::Translation(v) => local.0 = vec3_at(v, i, f, track.interpolation, span),
                Values::Scale(v) => local.2 = vec3_at(v, i, f, track.interpolation, span),
                Values::Rotation(v) => {
                    local.1 = if cubic {
                        let at = |k: usize, o: usize| v.get(k * 3 + o).copied().unwrap_or(Quat::IDENTITY);
                        if i + 1 < track.times.len() {
                            let q = hermite4(at(i, 1).into(), (at(i, 2) * span).into(), at(i + 1, 1).into(), (at(i + 1, 0) * span).into(), f);
                            Quat::from_vec4(q).normalize()
                        } else {
                            at(i, 1)
                        }
                    } else {
                        let a = v.get(i).copied().unwrap_or(Quat::IDENTITY);
                        match (track.interpolation, v.get(i + 1)) {
                            (Interpolation::Linear, Some(&b)) => a.slerp(b, f),
                            _ => a,
                        }
                    }
                }
                Values::Weights(w, n) => {
                    let n = *n;
                    let stride = if cubic { 3 } else { 1 };
                    let at = |k: usize, j: usize| w.get((k * stride + usize::from(cubic)) * n + j).copied().unwrap_or(0.0);
                    let out = &mut pose.weights[track.node];
                    out.resize(n, 0.0);
                    for (j, o) in out.iter_mut().enumerate() {
                        *o = if track.interpolation == Interpolation::Step || i + 1 >= track.times.len() {
                            at(i, j)
                        } else {
                            at(i, j) + (at(i + 1, j) - at(i, j)) * f
                        };
                    }
                }
            }
        }
    }

    /// Matrizes do mundo de cada nó.
    pub fn world(&self, pose: &Pose, root: Mat4) -> Vec<Mat4> {
        let mut world = vec![Mat4::IDENTITY; self.nodes.len()];
        for &i in &self.order {
            let (t, r, s) = pose.local[i];
            let local = Mat4::from_scale_rotation_translation(s, r, t);
            world[i] = match self.nodes[i].parent {
                Some(p) => world[p] * local,
                None => root * local,
            };
        }
        world
    }

    /// Matrizes dos ossos de um esqueleto (o que vai para o shader).
    pub fn joint_matrices(&self, skin: usize, world: &[Mat4]) -> Vec<Mat4> {
        let s = &self.skins[skin];
        s.joints.iter().zip(&s.inverse_bind).map(|(&j, ib)| world[j] * *ib).collect()
    }

    pub(crate) fn rest_bounds(&self) -> (Vec3, Vec3) {
        let pose = self.rest_pose();
        let world = self.world(&pose, Mat4::IDENTITY);
        let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
        for (i, node) in self.nodes.iter().enumerate() {
            let Some(m) = node.mesh else { continue };
            // Malha com esqueleto: os vértices já estão no espaço do modelo (o nó é ignorado).
            let joints = node.skin.map(|s| self.joint_matrices(s, &world));
            for p in &self.meshes[m].primitives {
                for v in &p.vertices {
                    let pos = Vec3::from(v.position);
                    let at = match &joints {
                        Some(j) => {
                            let w: f32 = v.weights.iter().sum();
                            if w > 0.0 {
                                (0..4).map(|k| j.get(v.joints[k] as usize).copied().unwrap_or(Mat4::IDENTITY).transform_point3(pos) * v.weights[k]).sum::<Vec3>() / w
                            } else {
                                pos
                            }
                        }
                        None => world[i].transform_point3(pos),
                    };
                    lo = lo.min(at);
                    hi = hi.max(at);
                }
            }
        }
        if lo.x > hi.x { (Vec3::splat(-1.0), Vec3::splat(1.0)) } else { (lo, hi) }
    }
}

impl Pose {
    /// Mistura com `other` (0 = esta, 1 = a outra): transição entre clipes.
    pub fn blend(&mut self, other: &Pose, f: f32) {
        for (a, b) in self.local.iter_mut().zip(&other.local) {
            a.0 = a.0.lerp(b.0, f);
            a.1 = a.1.slerp(b.1, f);
            a.2 = a.2.lerp(b.2, f);
        }
        for (a, b) in self.weights.iter_mut().zip(&other.weights) {
            for (x, y) in a.iter_mut().zip(b) {
                *x += (y - *x) * f;
            }
        }
    }
}

/// A chave anterior a `t` e a fração até a próxima.
fn key(times: &[f32], t: f32) -> (usize, f32) {
    if times.len() < 2 || t <= times[0] {
        return (0, 0.0);
    }
    let i = times.partition_point(|&k| k <= t).saturating_sub(1).min(times.len() - 1);
    if i + 1 >= times.len() {
        return (i, 0.0);
    }
    let span = times[i + 1] - times[i];
    (i, if span > 0.0 { ((t - times[i]) / span).clamp(0.0, 1.0) } else { 0.0 })
}

fn hermite(p0: Vec3, m0: Vec3, p1: Vec3, m1: Vec3, t: f32) -> Vec3 {
    let (t2, t3) = (t * t, t * t * t);
    p0 * (2.0 * t3 - 3.0 * t2 + 1.0) + m0 * (t3 - 2.0 * t2 + t) + p1 * (-2.0 * t3 + 3.0 * t2) + m1 * (t3 - t2)
}

fn hermite4(p0: glam::Vec4, m0: glam::Vec4, p1: glam::Vec4, m1: glam::Vec4, t: f32) -> glam::Vec4 {
    let (t2, t3) = (t * t, t * t * t);
    p0 * (2.0 * t3 - 3.0 * t2 + 1.0) + m0 * (t3 - 2.0 * t2 + t) + p1 * (-2.0 * t3 + 3.0 * t2) + m1 * (t3 - t2)
}

fn vec3_at(v: &[Vec3], i: usize, f: f32, interp: Interpolation, span: f32) -> Vec3 {
    match interp {
        Interpolation::Cubic => {
            let at = |k: usize, o: usize| v.get(k * 3 + o).copied().unwrap_or(Vec3::ZERO);
            if v.len() >= (i + 2) * 3 { hermite(at(i, 1), at(i, 2) * span, at(i + 1, 1), at(i + 1, 0) * span, f) } else { at(i, 1) }
        }
        Interpolation::Step => v.get(i).copied().unwrap_or(Vec3::ZERO),
        Interpolation::Linear => match (v.get(i), v.get(i + 1)) {
            (Some(a), Some(b)) => a.lerp(*b, f),
            (Some(a), None) => *a,
            _ => Vec3::ZERO,
        },
    }
}

fn compute_normals(vertices: &mut [Vertex], indices: &[u32]) {
    let mut acc = vec![Vec3::ZERO; vertices.len()];
    for t in indices.chunks_exact(3) {
        let [a, b, c] = [t[0] as usize, t[1] as usize, t[2] as usize];
        if a.max(b).max(c) >= vertices.len() {
            continue;
        }
        let (pa, pb, pc) = (Vec3::from(vertices[a].position), Vec3::from(vertices[b].position), Vec3::from(vertices[c].position));
        let n = (pb - pa).cross(pc - pa);
        acc[a] += n;
        acc[b] += n;
        acc[c] += n;
    }
    for (v, n) in vertices.iter_mut().zip(acc) {
        v.normal = n.normalize_or(Vec3::Y).into();
    }
}

fn to_rgba(img: &gltf::image::Data) -> Vec<u8> {
    use gltf::image::Format;
    let n = (img.width * img.height) as usize;
    let p = &img.pixels;
    match img.format {
        Format::R8G8B8A8 => p.clone(),
        Format::R8G8B8 => p.chunks_exact(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        Format::R8G8 => p.chunks_exact(2).flat_map(|c| [c[0], c[0], c[0], c[1]]).collect(),
        Format::R8 => p.iter().flat_map(|&c| [c, c, c, 255]).collect(),
        // 16 bits por canal: fica o byte mais significativo.
        Format::R16G16B16A16 => p.chunks_exact(8).flat_map(|c| [c[1], c[3], c[5], c[7]]).collect(),
        Format::R16G16B16 => p.chunks_exact(6).flat_map(|c| [c[1], c[3], c[5], 255]).collect(),
        _ => vec![255; n * 4],
    }
}
