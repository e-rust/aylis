//! A versão do app publicado (pedido do autor em 17/11: o NoOBS usa `x.y.z-beta`): a pré-release
//! e o build são aceitos, e o que só aceita números (o `.msi`, o iOS, os dados de versão do
//! `.exe`) recebe a parte de antes do `-`, sem que a beta passe a final.

#[test]
fn prerelease_versions_are_accepted() {
    for ok in ["1.0.0", "1.2", "0.52.7", "1.2.0-beta", "1.2.0-beta.1", "2.0.0-rc.2", "1.0.0+20261117", "1.0.0-beta.1+build.7", "1.2.3.4"] {
        assert!(aylis_project::is_valid_version(ok), "{ok}");
    }
    for bad in ["", "beta", "1..2", "1.2.", "-beta", "1.2.0-", "1.2.0-beta..1", "1.2.0-be ta", "1.2.0.4.5", "v1.2.0", "123456.0.0"] {
        assert!(!aylis_project::is_valid_version(bad), "{bad}");
    }
}

#[test]
fn the_numeric_part_leaves_the_prerelease_out() {
    assert_eq!(aylis_build::version_core("1.2.0-beta.1+42"), "1.2.0");
    assert_eq!(aylis_build::version4("1.2.0-beta.1"), "1.2.0.0", "a beta não vira 1.2.0.1, que passaria a final");
    assert_eq!(aylis_build::version4("1.2.0"), "1.2.0.0");
    assert_eq!(aylis_build::version4("0.52.7-beta"), "0.52.7.0");
}
