//! A versão que a publicação grava: a do projeto ou a da última tag do git
//! (`publish.version_from_git_tag`), com a mensagem clara quando não há tag: a publicação para
//! antes de compilar, e todas as plataformas saem com a mesma versão.

use aylis_project::{Language, Project, create};

fn git(dir: &std::path::Path, args: &[&str]) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.name=Aylis", "-c", "user.email=teste@aylis.invalid", "-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success())
}

#[test]
fn the_published_version_comes_from_the_project_or_from_the_git_tag() {
    let base = std::env::temp_dir().join(format!("aylis_published_version_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let mut project = create(&base, "Loja", Language::Rust).unwrap();
    project.publish.version = "1.2.0".into();
    project.publish.description = "Ponto de venda".into();
    project.publish.copyright = "© 2026 Empresa".into();
    project.save().unwrap();
    // Os campos novos vão e voltam pelo `.aylisproj`; a opção desligada nem aparece.
    let text = std::fs::read_to_string(&project.path).unwrap();
    assert!(text.contains("\"description\": \"Ponto de venda\"") && text.contains("\"copyright\": \"© 2026 Empresa\""), "{text}");
    assert!(!text.contains("version_from_git_tag"), "{text}");
    let again = Project::open(&project.path).unwrap();
    assert_eq!((again.publish.description.as_str(), again.publish.copyright.as_str()), ("Ponto de venda", "© 2026 Empresa"));
    // Sem a opção: a do projeto, sem git nenhum.
    assert_eq!(project.published_version().unwrap(), "1.2.0");

    if std::process::Command::new("git").arg("--version").output().is_err() {
        eprintln!("sem o git: o resto pulado");
        return;
    }
    project.publish.version_from_git_tag = true;
    project.save().unwrap();
    assert!(Project::open(&project.path).unwrap().publish.version_from_git_tag);
    let dir = project.dir();
    assert!(git(&dir, &["init", "-q"]));
    assert!(git(&dir, &["add", "-A"]));
    assert!(git(&dir, &["commit", "-q", "-m", "um"]));
    // Sem tag: a publicação para com o motivo e o que fazer.
    let error = project.published_version().unwrap_err();
    assert!(error.contains("não tem tag") && error.contains("git tag v1.0.0"), "{error}");
    assert!(project.with_published_version().is_err());
    assert!(git(&dir, &["tag", "v3.0.1"]));
    assert_eq!(project.published_version().unwrap(), "3.0.1");
    let published = project.with_published_version().unwrap();
    assert_eq!(published.version(), "3.0.1");
    assert_eq!(project.version(), "1.2.0", "o projeto em si não muda");
    let _ = std::fs::remove_dir_all(&base);
}
