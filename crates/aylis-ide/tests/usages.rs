//! Onde é usado (item 20), num exemplo de verdade: o campo `cliente` do `cdsPedidos` do
//! Relatórios aparece nas colunas da grade, nos textos do relatório (`[cliente]`) e no código.

use std::path::Path;

use aylis_ide::usages::{self, Target};
use aylis_project::Project;

#[test]
fn the_field_of_the_reports_example_is_found_in_the_grid_the_report_and_the_code() {
    let project = Project::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/Relatorios/Relatorios.aylisproj")).unwrap();
    let entry = project.forms.first().unwrap();
    let layout = serde_json::from_str(&std::fs::read_to_string(project.file(&entry.file)).unwrap()).unwrap();
    let target = usages::target_of(&layout, &entry.name, "cdsPedidosCliente");
    assert_eq!(target, Target::Field { form: entry.name.clone(), data_set: "cdsPedidos".into(), field: "cliente".into() });
    let found = usages::find(&project, &target, &Default::default()).unwrap();
    let props: Vec<String> = found.forms.iter().map(|h| format!("{}.{}", h.component, h.property)).collect();
    assert!(props.iter().any(|p| p.ends_with(".columns")), "as colunas da grade: {props:?}");
    assert!(found.forms.iter().filter(|h| h.value.contains("[cliente]")).count() >= 2, "os textos do relatório: {props:?}");
    assert!(!found.text.iter().any(|h| h.file.components().any(|c| c.as_os_str() == "generated")), "o código gerado fica de fora");

    // O dataset em si: o DataSource que aponta para ele e o nome no código.
    let cds = usages::find(&project, &Target::Component { form: entry.name.clone(), name: "cdsPedidos".into() }, &Default::default()).unwrap();
    assert!(cds.forms.iter().any(|h| h.property == "data_set"), "{:?}", cds.forms);
    assert!(cds.text.iter().any(|h| h.text.contains("self.cdsPedidos")), "no código (o acesso tipado): {:?}", cds.text);
}
