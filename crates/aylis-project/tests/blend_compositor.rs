//! O `blend_compositor` do `.aylisproj`: vazio é o efeito do Direct2D (e não é gravado);
//! `direct3d` liga o shader do Aylis; um nome desconhecido fica no padrão.

use aylis_core::gpu::BlendCompositor;
use aylis_project::Project;

fn project(extra: &str) -> Project {
    serde_json::from_str(&format!(r#"{{"aylis_project": 1, "name": "Teste", "main_form": "frmPrincipal", "forms": []{extra}}}"#)).expect("o projeto lê")
}

#[test]
fn the_project_chooses_the_blend_compositor() {
    assert_eq!(project("").blend_compositor(), BlendCompositor::Direct2d);
    assert_eq!(project(r#", "blend_compositor": "direct3d""#).blend_compositor(), BlendCompositor::Direct3d);
    assert_eq!(project(r#", "blend_compositor": "Direct3D ""#).blend_compositor(), BlendCompositor::Direct3d);
    assert_eq!(project(r#", "blend_compositor": "vulkan""#).blend_compositor(), BlendCompositor::Direct2d);
    let saved = serde_json::to_string(&project("")).unwrap();
    assert!(!saved.contains("blend_compositor"), "o padrão não é gravado: {saved}");
}
