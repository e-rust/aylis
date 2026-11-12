//! O que a IDE do Aylis refaz a cada "Gerar código" (não edite): o acesso tipado aos
//! componentes, a struct de cada form e o registro. O código dos forms é seu, em
//! `src/forms/`.

use aylis::prelude::Ui;

pub mod components;
pub use components::*;

#[path = "../src/forms/principal.rs"]
pub mod principal;

#[path = "../src/forms/rh/rh_funcionarios_cad.rs"]
pub mod rh_rh_funcionarios_cad;

#[path = "../src/forms/rh/rh_funcionarios_lista.rs"]
pub mod rh_rh_funcionarios_lista;

#[path = "../src/forms/rh/rh_funcionarios_consulta.rs"]
pub mod rh_rh_funcionarios_consulta;

#[path = "../src/forms/rh/rh_funcionarios_hist.rs"]
pub mod rh_rh_funcionarios_hist;

#[path = "../src/forms/rh/rh_cargos_cad.rs"]
pub mod rh_rh_cargos_cad;

#[path = "../src/forms/rh/rh_cargos_lista.rs"]
pub mod rh_rh_cargos_lista;

#[path = "../src/forms/rh/rh_cargos_consulta.rs"]
pub mod rh_rh_cargos_consulta;

#[path = "../src/forms/rh/rh_cargos_hist.rs"]
pub mod rh_rh_cargos_hist;

#[path = "../src/forms/rh/rh_departamentos_cad.rs"]
pub mod rh_rh_departamentos_cad;

#[path = "../src/forms/rh/rh_departamentos_lista.rs"]
pub mod rh_rh_departamentos_lista;

#[path = "../src/forms/rh/rh_departamentos_consulta.rs"]
pub mod rh_rh_departamentos_consulta;

#[path = "../src/forms/rh/rh_departamentos_hist.rs"]
pub mod rh_rh_departamentos_hist;

#[path = "../src/forms/rh/rh_centros_de_custo_cad.rs"]
pub mod rh_rh_centros_de_custo_cad;

#[path = "../src/forms/rh/rh_centros_de_custo_lista.rs"]
pub mod rh_rh_centros_de_custo_lista;

#[path = "../src/forms/rh/rh_centros_de_custo_consulta.rs"]
pub mod rh_rh_centros_de_custo_consulta;

#[path = "../src/forms/rh/rh_centros_de_custo_hist.rs"]
pub mod rh_rh_centros_de_custo_hist;

#[path = "../src/forms/rh/rh_sindicatos_cad.rs"]
pub mod rh_rh_sindicatos_cad;

#[path = "../src/forms/rh/rh_sindicatos_lista.rs"]
pub mod rh_rh_sindicatos_lista;

#[path = "../src/forms/rh/rh_sindicatos_consulta.rs"]
pub mod rh_rh_sindicatos_consulta;

#[path = "../src/forms/rh/rh_sindicatos_hist.rs"]
pub mod rh_rh_sindicatos_hist;

#[path = "../src/forms/rh/rh_beneficios_cad.rs"]
pub mod rh_rh_beneficios_cad;

#[path = "../src/forms/rh/rh_beneficios_lista.rs"]
pub mod rh_rh_beneficios_lista;

#[path = "../src/forms/rh/rh_beneficios_consulta.rs"]
pub mod rh_rh_beneficios_consulta;

#[path = "../src/forms/rh/rh_beneficios_hist.rs"]
pub mod rh_rh_beneficios_hist;

#[path = "../src/forms/rh/rh_planos_de_saude_cad.rs"]
pub mod rh_rh_planos_de_saude_cad;

#[path = "../src/forms/rh/rh_planos_de_saude_lista.rs"]
pub mod rh_rh_planos_de_saude_lista;

#[path = "../src/forms/rh/rh_planos_de_saude_consulta.rs"]
pub mod rh_rh_planos_de_saude_consulta;

#[path = "../src/forms/rh/rh_planos_de_saude_hist.rs"]
pub mod rh_rh_planos_de_saude_hist;

#[path = "../src/forms/rh/rh_escalas_de_trabalho_cad.rs"]
pub mod rh_rh_escalas_de_trabalho_cad;

#[path = "../src/forms/rh/rh_escalas_de_trabalho_lista.rs"]
pub mod rh_rh_escalas_de_trabalho_lista;

#[path = "../src/forms/rh/rh_escalas_de_trabalho_consulta.rs"]
pub mod rh_rh_escalas_de_trabalho_consulta;

#[path = "../src/forms/rh/rh_escalas_de_trabalho_hist.rs"]
pub mod rh_rh_escalas_de_trabalho_hist;

#[path = "../src/forms/rh/rh_turnos_cad.rs"]
pub mod rh_rh_turnos_cad;

#[path = "../src/forms/rh/rh_turnos_lista.rs"]
pub mod rh_rh_turnos_lista;

#[path = "../src/forms/rh/rh_turnos_consulta.rs"]
pub mod rh_rh_turnos_consulta;

#[path = "../src/forms/rh/rh_turnos_hist.rs"]
pub mod rh_rh_turnos_hist;

#[path = "../src/forms/rh/rh_feriados_cad.rs"]
pub mod rh_rh_feriados_cad;

#[path = "../src/forms/rh/rh_feriados_lista.rs"]
pub mod rh_rh_feriados_lista;

#[path = "../src/forms/rh/rh_feriados_consulta.rs"]
pub mod rh_rh_feriados_consulta;

#[path = "../src/forms/rh/rh_feriados_hist.rs"]
pub mod rh_rh_feriados_hist;

#[path = "../src/forms/rh/rh_cursos_cad.rs"]
pub mod rh_rh_cursos_cad;

#[path = "../src/forms/rh/rh_cursos_lista.rs"]
pub mod rh_rh_cursos_lista;

#[path = "../src/forms/rh/rh_cursos_consulta.rs"]
pub mod rh_rh_cursos_consulta;

#[path = "../src/forms/rh/rh_cursos_hist.rs"]
pub mod rh_rh_cursos_hist;

#[path = "../src/forms/rh/rh_instrutores_cad.rs"]
pub mod rh_rh_instrutores_cad;

#[path = "../src/forms/rh/rh_instrutores_lista.rs"]
pub mod rh_rh_instrutores_lista;

#[path = "../src/forms/rh/rh_instrutores_consulta.rs"]
pub mod rh_rh_instrutores_consulta;

#[path = "../src/forms/rh/rh_instrutores_hist.rs"]
pub mod rh_rh_instrutores_hist;

#[path = "../src/forms/rh/rh_competencias_cad.rs"]
pub mod rh_rh_competencias_cad;

#[path = "../src/forms/rh/rh_competencias_lista.rs"]
pub mod rh_rh_competencias_lista;

#[path = "../src/forms/rh/rh_competencias_consulta.rs"]
pub mod rh_rh_competencias_consulta;

#[path = "../src/forms/rh/rh_competencias_hist.rs"]
pub mod rh_rh_competencias_hist;

#[path = "../src/forms/rh/rh_niveis_salariais_cad.rs"]
pub mod rh_rh_niveis_salariais_cad;

#[path = "../src/forms/rh/rh_niveis_salariais_lista.rs"]
pub mod rh_rh_niveis_salariais_lista;

#[path = "../src/forms/rh/rh_niveis_salariais_consulta.rs"]
pub mod rh_rh_niveis_salariais_consulta;

#[path = "../src/forms/rh/rh_niveis_salariais_hist.rs"]
pub mod rh_rh_niveis_salariais_hist;

#[path = "../src/forms/rh/rh_eventos_da_folha_cad.rs"]
pub mod rh_rh_eventos_da_folha_cad;

#[path = "../src/forms/rh/rh_eventos_da_folha_lista.rs"]
pub mod rh_rh_eventos_da_folha_lista;

#[path = "../src/forms/rh/rh_eventos_da_folha_consulta.rs"]
pub mod rh_rh_eventos_da_folha_consulta;

#[path = "../src/forms/rh/rh_eventos_da_folha_hist.rs"]
pub mod rh_rh_eventos_da_folha_hist;

#[path = "../src/forms/rh/rh_tipos_de_afastamento_cad.rs"]
pub mod rh_rh_tipos_de_afastamento_cad;

#[path = "../src/forms/rh/rh_tipos_de_afastamento_lista.rs"]
pub mod rh_rh_tipos_de_afastamento_lista;

#[path = "../src/forms/rh/rh_tipos_de_afastamento_consulta.rs"]
pub mod rh_rh_tipos_de_afastamento_consulta;

#[path = "../src/forms/rh/rh_tipos_de_afastamento_hist.rs"]
pub mod rh_rh_tipos_de_afastamento_hist;

#[path = "../src/forms/rh/rh_motivos_de_desligamento_cad.rs"]
pub mod rh_rh_motivos_de_desligamento_cad;

#[path = "../src/forms/rh/rh_motivos_de_desligamento_lista.rs"]
pub mod rh_rh_motivos_de_desligamento_lista;

#[path = "../src/forms/rh/rh_motivos_de_desligamento_consulta.rs"]
pub mod rh_rh_motivos_de_desligamento_consulta;

#[path = "../src/forms/rh/rh_motivos_de_desligamento_hist.rs"]
pub mod rh_rh_motivos_de_desligamento_hist;

#[path = "../src/forms/rh/rh_bancos_de_horas_cad.rs"]
pub mod rh_rh_bancos_de_horas_cad;

#[path = "../src/forms/rh/rh_bancos_de_horas_lista.rs"]
pub mod rh_rh_bancos_de_horas_lista;

#[path = "../src/forms/rh/rh_bancos_de_horas_consulta.rs"]
pub mod rh_rh_bancos_de_horas_consulta;

#[path = "../src/forms/rh/rh_bancos_de_horas_hist.rs"]
pub mod rh_rh_bancos_de_horas_hist;

#[path = "../src/forms/rh/rh_epis_cad.rs"]
pub mod rh_rh_epis_cad;

#[path = "../src/forms/rh/rh_epis_lista.rs"]
pub mod rh_rh_epis_lista;

#[path = "../src/forms/rh/rh_epis_consulta.rs"]
pub mod rh_rh_epis_consulta;

#[path = "../src/forms/rh/rh_epis_hist.rs"]
pub mod rh_rh_epis_hist;

#[path = "../src/forms/rh/rh_exames_ocupacionais_cad.rs"]
pub mod rh_rh_exames_ocupacionais_cad;

#[path = "../src/forms/rh/rh_exames_ocupacionais_lista.rs"]
pub mod rh_rh_exames_ocupacionais_lista;

#[path = "../src/forms/rh/rh_exames_ocupacionais_consulta.rs"]
pub mod rh_rh_exames_ocupacionais_consulta;

#[path = "../src/forms/rh/rh_exames_ocupacionais_hist.rs"]
pub mod rh_rh_exames_ocupacionais_hist;

#[path = "../src/forms/rh/rh_vagas_cad.rs"]
pub mod rh_rh_vagas_cad;

#[path = "../src/forms/rh/rh_vagas_lista.rs"]
pub mod rh_rh_vagas_lista;

#[path = "../src/forms/rh/rh_vagas_consulta.rs"]
pub mod rh_rh_vagas_consulta;

#[path = "../src/forms/rh/rh_vagas_hist.rs"]
pub mod rh_rh_vagas_hist;

#[path = "../src/forms/rh/rh_admissoes_cad.rs"]
pub mod rh_rh_admissoes_cad;

#[path = "../src/forms/rh/rh_admissoes_lista.rs"]
pub mod rh_rh_admissoes_lista;

#[path = "../src/forms/rh/rh_admissoes_consulta.rs"]
pub mod rh_rh_admissoes_consulta;

#[path = "../src/forms/rh/rh_admissoes_hist.rs"]
pub mod rh_rh_admissoes_hist;

#[path = "../src/forms/rh/rh_desligamentos_cad.rs"]
pub mod rh_rh_desligamentos_cad;

#[path = "../src/forms/rh/rh_desligamentos_lista.rs"]
pub mod rh_rh_desligamentos_lista;

#[path = "../src/forms/rh/rh_desligamentos_consulta.rs"]
pub mod rh_rh_desligamentos_consulta;

#[path = "../src/forms/rh/rh_desligamentos_hist.rs"]
pub mod rh_rh_desligamentos_hist;

#[path = "../src/forms/rh/rh_ferias_cad.rs"]
pub mod rh_rh_ferias_cad;

#[path = "../src/forms/rh/rh_ferias_lista.rs"]
pub mod rh_rh_ferias_lista;

#[path = "../src/forms/rh/rh_ferias_consulta.rs"]
pub mod rh_rh_ferias_consulta;

#[path = "../src/forms/rh/rh_ferias_hist.rs"]
pub mod rh_rh_ferias_hist;

#[path = "../src/forms/rh/rh_folhas_de_pagamento_cad.rs"]
pub mod rh_rh_folhas_de_pagamento_cad;

#[path = "../src/forms/rh/rh_folhas_de_pagamento_lista.rs"]
pub mod rh_rh_folhas_de_pagamento_lista;

#[path = "../src/forms/rh/rh_folhas_de_pagamento_consulta.rs"]
pub mod rh_rh_folhas_de_pagamento_consulta;

#[path = "../src/forms/rh/rh_folhas_de_pagamento_hist.rs"]
pub mod rh_rh_folhas_de_pagamento_hist;

#[path = "../src/forms/rh/rh_adiantamentos_salariais_cad.rs"]
pub mod rh_rh_adiantamentos_salariais_cad;

#[path = "../src/forms/rh/rh_adiantamentos_salariais_lista.rs"]
pub mod rh_rh_adiantamentos_salariais_lista;

#[path = "../src/forms/rh/rh_adiantamentos_salariais_consulta.rs"]
pub mod rh_rh_adiantamentos_salariais_consulta;

#[path = "../src/forms/rh/rh_adiantamentos_salariais_hist.rs"]
pub mod rh_rh_adiantamentos_salariais_hist;

#[path = "../src/forms/rh/rh_decimo_terceiro_cad.rs"]
pub mod rh_rh_decimo_terceiro_cad;

#[path = "../src/forms/rh/rh_decimo_terceiro_lista.rs"]
pub mod rh_rh_decimo_terceiro_lista;

#[path = "../src/forms/rh/rh_decimo_terceiro_consulta.rs"]
pub mod rh_rh_decimo_terceiro_consulta;

#[path = "../src/forms/rh/rh_decimo_terceiro_hist.rs"]
pub mod rh_rh_decimo_terceiro_hist;

#[path = "../src/forms/rh/rh_rescisoes_cad.rs"]
pub mod rh_rh_rescisoes_cad;

#[path = "../src/forms/rh/rh_rescisoes_lista.rs"]
pub mod rh_rh_rescisoes_lista;

#[path = "../src/forms/rh/rh_rescisoes_consulta.rs"]
pub mod rh_rh_rescisoes_consulta;

#[path = "../src/forms/rh/rh_rescisoes_hist.rs"]
pub mod rh_rh_rescisoes_hist;

#[path = "../src/forms/rh/rh_afastamentos_cad.rs"]
pub mod rh_rh_afastamentos_cad;

#[path = "../src/forms/rh/rh_afastamentos_lista.rs"]
pub mod rh_rh_afastamentos_lista;

#[path = "../src/forms/rh/rh_afastamentos_consulta.rs"]
pub mod rh_rh_afastamentos_consulta;

#[path = "../src/forms/rh/rh_afastamentos_hist.rs"]
pub mod rh_rh_afastamentos_hist;

#[path = "../src/forms/rh/rh_avaliacoes_de_desempenho_cad.rs"]
pub mod rh_rh_avaliacoes_de_desempenho_cad;

#[path = "../src/forms/rh/rh_avaliacoes_de_desempenho_lista.rs"]
pub mod rh_rh_avaliacoes_de_desempenho_lista;

#[path = "../src/forms/rh/rh_avaliacoes_de_desempenho_consulta.rs"]
pub mod rh_rh_avaliacoes_de_desempenho_consulta;

#[path = "../src/forms/rh/rh_avaliacoes_de_desempenho_hist.rs"]
pub mod rh_rh_avaliacoes_de_desempenho_hist;

#[path = "../src/forms/rh/rh_treinamentos_realizados_cad.rs"]
pub mod rh_rh_treinamentos_realizados_cad;

#[path = "../src/forms/rh/rh_treinamentos_realizados_lista.rs"]
pub mod rh_rh_treinamentos_realizados_lista;

#[path = "../src/forms/rh/rh_treinamentos_realizados_consulta.rs"]
pub mod rh_rh_treinamentos_realizados_consulta;

#[path = "../src/forms/rh/rh_treinamentos_realizados_hist.rs"]
pub mod rh_rh_treinamentos_realizados_hist;

#[path = "../src/forms/rh/rh_requisicoes_de_pessoal_cad.rs"]
pub mod rh_rh_requisicoes_de_pessoal_cad;

#[path = "../src/forms/rh/rh_requisicoes_de_pessoal_lista.rs"]
pub mod rh_rh_requisicoes_de_pessoal_lista;

#[path = "../src/forms/rh/rh_requisicoes_de_pessoal_consulta.rs"]
pub mod rh_rh_requisicoes_de_pessoal_consulta;

#[path = "../src/forms/rh/rh_requisicoes_de_pessoal_hist.rs"]
pub mod rh_rh_requisicoes_de_pessoal_hist;

#[path = "../src/forms/rh/rh_entregas_de_epi_cad.rs"]
pub mod rh_rh_entregas_de_epi_cad;

#[path = "../src/forms/rh/rh_entregas_de_epi_lista.rs"]
pub mod rh_rh_entregas_de_epi_lista;

#[path = "../src/forms/rh/rh_entregas_de_epi_consulta.rs"]
pub mod rh_rh_entregas_de_epi_consulta;

#[path = "../src/forms/rh/rh_entregas_de_epi_hist.rs"]
pub mod rh_rh_entregas_de_epi_hist;

#[path = "../src/forms/rh/rh_exames_realizados_cad.rs"]
pub mod rh_rh_exames_realizados_cad;

#[path = "../src/forms/rh/rh_exames_realizados_lista.rs"]
pub mod rh_rh_exames_realizados_lista;

#[path = "../src/forms/rh/rh_exames_realizados_consulta.rs"]
pub mod rh_rh_exames_realizados_consulta;

#[path = "../src/forms/rh/rh_exames_realizados_hist.rs"]
pub mod rh_rh_exames_realizados_hist;

#[path = "../src/forms/rh/rh_lancamentos_de_ponto_cad.rs"]
pub mod rh_rh_lancamentos_de_ponto_cad;

#[path = "../src/forms/rh/rh_lancamentos_de_ponto_lista.rs"]
pub mod rh_rh_lancamentos_de_ponto_lista;

#[path = "../src/forms/rh/rh_lancamentos_de_ponto_consulta.rs"]
pub mod rh_rh_lancamentos_de_ponto_consulta;

#[path = "../src/forms/rh/rh_lancamentos_de_ponto_hist.rs"]
pub mod rh_rh_lancamentos_de_ponto_hist;

#[path = "../src/forms/rh/rh_horas_extras_cad.rs"]
pub mod rh_rh_horas_extras_cad;

#[path = "../src/forms/rh/rh_horas_extras_lista.rs"]
pub mod rh_rh_horas_extras_lista;

#[path = "../src/forms/rh/rh_horas_extras_consulta.rs"]
pub mod rh_rh_horas_extras_consulta;

#[path = "../src/forms/rh/rh_horas_extras_hist.rs"]
pub mod rh_rh_horas_extras_hist;

#[path = "../src/forms/rh/rh_promocoes_cad.rs"]
pub mod rh_rh_promocoes_cad;

#[path = "../src/forms/rh/rh_promocoes_lista.rs"]
pub mod rh_rh_promocoes_lista;

#[path = "../src/forms/rh/rh_promocoes_consulta.rs"]
pub mod rh_rh_promocoes_consulta;

#[path = "../src/forms/rh/rh_promocoes_hist.rs"]
pub mod rh_rh_promocoes_hist;

#[path = "../src/forms/rh/rh_transferencias_cad.rs"]
pub mod rh_rh_transferencias_cad;

#[path = "../src/forms/rh/rh_transferencias_lista.rs"]
pub mod rh_rh_transferencias_lista;

#[path = "../src/forms/rh/rh_transferencias_consulta.rs"]
pub mod rh_rh_transferencias_consulta;

#[path = "../src/forms/rh/rh_transferencias_hist.rs"]
pub mod rh_rh_transferencias_hist;

#[path = "../src/forms/rh/rh_advertencias_cad.rs"]
pub mod rh_rh_advertencias_cad;

#[path = "../src/forms/rh/rh_advertencias_lista.rs"]
pub mod rh_rh_advertencias_lista;

#[path = "../src/forms/rh/rh_advertencias_consulta.rs"]
pub mod rh_rh_advertencias_consulta;

#[path = "../src/forms/rh/rh_advertencias_hist.rs"]
pub mod rh_rh_advertencias_hist;

#[path = "../src/forms/rh/rh_recrutamentos_cad.rs"]
pub mod rh_rh_recrutamentos_cad;

#[path = "../src/forms/rh/rh_recrutamentos_lista.rs"]
pub mod rh_rh_recrutamentos_lista;

#[path = "../src/forms/rh/rh_recrutamentos_consulta.rs"]
pub mod rh_rh_recrutamentos_consulta;

#[path = "../src/forms/rh/rh_recrutamentos_hist.rs"]
pub mod rh_rh_recrutamentos_hist;

#[path = "../src/forms/rh/rh_entrevistas_cad.rs"]
pub mod rh_rh_entrevistas_cad;

#[path = "../src/forms/rh/rh_entrevistas_lista.rs"]
pub mod rh_rh_entrevistas_lista;

#[path = "../src/forms/rh/rh_entrevistas_consulta.rs"]
pub mod rh_rh_entrevistas_consulta;

#[path = "../src/forms/rh/rh_entrevistas_hist.rs"]
pub mod rh_rh_entrevistas_hist;

#[path = "../src/forms/rh/rh_reembolsos_cad.rs"]
pub mod rh_rh_reembolsos_cad;

#[path = "../src/forms/rh/rh_reembolsos_lista.rs"]
pub mod rh_rh_reembolsos_lista;

#[path = "../src/forms/rh/rh_reembolsos_consulta.rs"]
pub mod rh_rh_reembolsos_consulta;

#[path = "../src/forms/rh/rh_reembolsos_hist.rs"]
pub mod rh_rh_reembolsos_hist;

#[path = "../src/forms/fin/fin_bancos_cad.rs"]
pub mod fin_fin_bancos_cad;

#[path = "../src/forms/fin/fin_bancos_lista.rs"]
pub mod fin_fin_bancos_lista;

#[path = "../src/forms/fin/fin_bancos_consulta.rs"]
pub mod fin_fin_bancos_consulta;

#[path = "../src/forms/fin/fin_bancos_hist.rs"]
pub mod fin_fin_bancos_hist;

#[path = "../src/forms/fin/fin_contas_bancarias_cad.rs"]
pub mod fin_fin_contas_bancarias_cad;

#[path = "../src/forms/fin/fin_contas_bancarias_lista.rs"]
pub mod fin_fin_contas_bancarias_lista;

#[path = "../src/forms/fin/fin_contas_bancarias_consulta.rs"]
pub mod fin_fin_contas_bancarias_consulta;

#[path = "../src/forms/fin/fin_contas_bancarias_hist.rs"]
pub mod fin_fin_contas_bancarias_hist;

#[path = "../src/forms/fin/fin_formas_de_pagamento_cad.rs"]
pub mod fin_fin_formas_de_pagamento_cad;

#[path = "../src/forms/fin/fin_formas_de_pagamento_lista.rs"]
pub mod fin_fin_formas_de_pagamento_lista;

#[path = "../src/forms/fin/fin_formas_de_pagamento_consulta.rs"]
pub mod fin_fin_formas_de_pagamento_consulta;

#[path = "../src/forms/fin/fin_formas_de_pagamento_hist.rs"]
pub mod fin_fin_formas_de_pagamento_hist;

#[path = "../src/forms/fin/fin_condicoes_de_pagamento_cad.rs"]
pub mod fin_fin_condicoes_de_pagamento_cad;

#[path = "../src/forms/fin/fin_condicoes_de_pagamento_lista.rs"]
pub mod fin_fin_condicoes_de_pagamento_lista;

#[path = "../src/forms/fin/fin_condicoes_de_pagamento_consulta.rs"]
pub mod fin_fin_condicoes_de_pagamento_consulta;

#[path = "../src/forms/fin/fin_condicoes_de_pagamento_hist.rs"]
pub mod fin_fin_condicoes_de_pagamento_hist;

#[path = "../src/forms/fin/fin_plano_financeiro_cad.rs"]
pub mod fin_fin_plano_financeiro_cad;

#[path = "../src/forms/fin/fin_plano_financeiro_lista.rs"]
pub mod fin_fin_plano_financeiro_lista;

#[path = "../src/forms/fin/fin_plano_financeiro_consulta.rs"]
pub mod fin_fin_plano_financeiro_consulta;

#[path = "../src/forms/fin/fin_plano_financeiro_hist.rs"]
pub mod fin_fin_plano_financeiro_hist;

#[path = "../src/forms/fin/fin_centros_de_resultado_cad.rs"]
pub mod fin_fin_centros_de_resultado_cad;

#[path = "../src/forms/fin/fin_centros_de_resultado_lista.rs"]
pub mod fin_fin_centros_de_resultado_lista;

#[path = "../src/forms/fin/fin_centros_de_resultado_consulta.rs"]
pub mod fin_fin_centros_de_resultado_consulta;

#[path = "../src/forms/fin/fin_centros_de_resultado_hist.rs"]
pub mod fin_fin_centros_de_resultado_hist;

#[path = "../src/forms/fin/fin_carteiras_de_cobranca_cad.rs"]
pub mod fin_fin_carteiras_de_cobranca_cad;

#[path = "../src/forms/fin/fin_carteiras_de_cobranca_lista.rs"]
pub mod fin_fin_carteiras_de_cobranca_lista;

#[path = "../src/forms/fin/fin_carteiras_de_cobranca_consulta.rs"]
pub mod fin_fin_carteiras_de_cobranca_consulta;

#[path = "../src/forms/fin/fin_carteiras_de_cobranca_hist.rs"]
pub mod fin_fin_carteiras_de_cobranca_hist;

#[path = "../src/forms/fin/fin_moedas_cad.rs"]
pub mod fin_fin_moedas_cad;

#[path = "../src/forms/fin/fin_moedas_lista.rs"]
pub mod fin_fin_moedas_lista;

#[path = "../src/forms/fin/fin_moedas_consulta.rs"]
pub mod fin_fin_moedas_consulta;

#[path = "../src/forms/fin/fin_moedas_hist.rs"]
pub mod fin_fin_moedas_hist;

#[path = "../src/forms/fin/fin_indices_de_correcao_cad.rs"]
pub mod fin_fin_indices_de_correcao_cad;

#[path = "../src/forms/fin/fin_indices_de_correcao_lista.rs"]
pub mod fin_fin_indices_de_correcao_lista;

#[path = "../src/forms/fin/fin_indices_de_correcao_consulta.rs"]
pub mod fin_fin_indices_de_correcao_consulta;

#[path = "../src/forms/fin/fin_indices_de_correcao_hist.rs"]
pub mod fin_fin_indices_de_correcao_hist;

#[path = "../src/forms/fin/fin_tipos_de_documento_cad.rs"]
pub mod fin_fin_tipos_de_documento_cad;

#[path = "../src/forms/fin/fin_tipos_de_documento_lista.rs"]
pub mod fin_fin_tipos_de_documento_lista;

#[path = "../src/forms/fin/fin_tipos_de_documento_consulta.rs"]
pub mod fin_fin_tipos_de_documento_consulta;

#[path = "../src/forms/fin/fin_tipos_de_documento_hist.rs"]
pub mod fin_fin_tipos_de_documento_hist;

#[path = "../src/forms/fin/fin_portadores_cad.rs"]
pub mod fin_fin_portadores_cad;

#[path = "../src/forms/fin/fin_portadores_lista.rs"]
pub mod fin_fin_portadores_lista;

#[path = "../src/forms/fin/fin_portadores_consulta.rs"]
pub mod fin_fin_portadores_consulta;

#[path = "../src/forms/fin/fin_portadores_hist.rs"]
pub mod fin_fin_portadores_hist;

#[path = "../src/forms/fin/fin_favorecidos_cad.rs"]
pub mod fin_fin_favorecidos_cad;

#[path = "../src/forms/fin/fin_favorecidos_lista.rs"]
pub mod fin_fin_favorecidos_lista;

#[path = "../src/forms/fin/fin_favorecidos_consulta.rs"]
pub mod fin_fin_favorecidos_consulta;

#[path = "../src/forms/fin/fin_favorecidos_hist.rs"]
pub mod fin_fin_favorecidos_hist;

#[path = "../src/forms/fin/fin_cartoes_corporativos_cad.rs"]
pub mod fin_fin_cartoes_corporativos_cad;

#[path = "../src/forms/fin/fin_cartoes_corporativos_lista.rs"]
pub mod fin_fin_cartoes_corporativos_lista;

#[path = "../src/forms/fin/fin_cartoes_corporativos_consulta.rs"]
pub mod fin_fin_cartoes_corporativos_consulta;

#[path = "../src/forms/fin/fin_cartoes_corporativos_hist.rs"]
pub mod fin_fin_cartoes_corporativos_hist;

#[path = "../src/forms/fin/fin_operadoras_de_cartao_cad.rs"]
pub mod fin_fin_operadoras_de_cartao_cad;

#[path = "../src/forms/fin/fin_operadoras_de_cartao_lista.rs"]
pub mod fin_fin_operadoras_de_cartao_lista;

#[path = "../src/forms/fin/fin_operadoras_de_cartao_consulta.rs"]
pub mod fin_fin_operadoras_de_cartao_consulta;

#[path = "../src/forms/fin/fin_operadoras_de_cartao_hist.rs"]
pub mod fin_fin_operadoras_de_cartao_hist;

#[path = "../src/forms/fin/fin_tarifas_bancarias_cad.rs"]
pub mod fin_fin_tarifas_bancarias_cad;

#[path = "../src/forms/fin/fin_tarifas_bancarias_lista.rs"]
pub mod fin_fin_tarifas_bancarias_lista;

#[path = "../src/forms/fin/fin_tarifas_bancarias_consulta.rs"]
pub mod fin_fin_tarifas_bancarias_consulta;

#[path = "../src/forms/fin/fin_tarifas_bancarias_hist.rs"]
pub mod fin_fin_tarifas_bancarias_hist;

#[path = "../src/forms/fin/fin_categorias_de_despesa_cad.rs"]
pub mod fin_fin_categorias_de_despesa_cad;

#[path = "../src/forms/fin/fin_categorias_de_despesa_lista.rs"]
pub mod fin_fin_categorias_de_despesa_lista;

#[path = "../src/forms/fin/fin_categorias_de_despesa_consulta.rs"]
pub mod fin_fin_categorias_de_despesa_consulta;

#[path = "../src/forms/fin/fin_categorias_de_despesa_hist.rs"]
pub mod fin_fin_categorias_de_despesa_hist;

#[path = "../src/forms/fin/fin_categorias_de_receita_cad.rs"]
pub mod fin_fin_categorias_de_receita_cad;

#[path = "../src/forms/fin/fin_categorias_de_receita_lista.rs"]
pub mod fin_fin_categorias_de_receita_lista;

#[path = "../src/forms/fin/fin_categorias_de_receita_consulta.rs"]
pub mod fin_fin_categorias_de_receita_consulta;

#[path = "../src/forms/fin/fin_categorias_de_receita_hist.rs"]
pub mod fin_fin_categorias_de_receita_hist;

#[path = "../src/forms/fin/fin_projetos_cad.rs"]
pub mod fin_fin_projetos_cad;

#[path = "../src/forms/fin/fin_projetos_lista.rs"]
pub mod fin_fin_projetos_lista;

#[path = "../src/forms/fin/fin_projetos_consulta.rs"]
pub mod fin_fin_projetos_consulta;

#[path = "../src/forms/fin/fin_projetos_hist.rs"]
pub mod fin_fin_projetos_hist;

#[path = "../src/forms/fin/fin_contratos_financeiros_cad.rs"]
pub mod fin_fin_contratos_financeiros_cad;

#[path = "../src/forms/fin/fin_contratos_financeiros_lista.rs"]
pub mod fin_fin_contratos_financeiros_lista;

#[path = "../src/forms/fin/fin_contratos_financeiros_consulta.rs"]
pub mod fin_fin_contratos_financeiros_consulta;

#[path = "../src/forms/fin/fin_contratos_financeiros_hist.rs"]
pub mod fin_fin_contratos_financeiros_hist;

#[path = "../src/forms/fin/fin_avalistas_cad.rs"]
pub mod fin_fin_avalistas_cad;

#[path = "../src/forms/fin/fin_avalistas_lista.rs"]
pub mod fin_fin_avalistas_lista;

#[path = "../src/forms/fin/fin_avalistas_consulta.rs"]
pub mod fin_fin_avalistas_consulta;

#[path = "../src/forms/fin/fin_avalistas_hist.rs"]
pub mod fin_fin_avalistas_hist;

#[path = "../src/forms/fin/fin_agencias_cad.rs"]
pub mod fin_fin_agencias_cad;

#[path = "../src/forms/fin/fin_agencias_lista.rs"]
pub mod fin_fin_agencias_lista;

#[path = "../src/forms/fin/fin_agencias_consulta.rs"]
pub mod fin_fin_agencias_consulta;

#[path = "../src/forms/fin/fin_agencias_hist.rs"]
pub mod fin_fin_agencias_hist;

#[path = "../src/forms/fin/fin_contas_a_pagar_cad.rs"]
pub mod fin_fin_contas_a_pagar_cad;

#[path = "../src/forms/fin/fin_contas_a_pagar_lista.rs"]
pub mod fin_fin_contas_a_pagar_lista;

#[path = "../src/forms/fin/fin_contas_a_pagar_consulta.rs"]
pub mod fin_fin_contas_a_pagar_consulta;

#[path = "../src/forms/fin/fin_contas_a_pagar_hist.rs"]
pub mod fin_fin_contas_a_pagar_hist;

#[path = "../src/forms/fin/fin_contas_a_receber_cad.rs"]
pub mod fin_fin_contas_a_receber_cad;

#[path = "../src/forms/fin/fin_contas_a_receber_lista.rs"]
pub mod fin_fin_contas_a_receber_lista;

#[path = "../src/forms/fin/fin_contas_a_receber_consulta.rs"]
pub mod fin_fin_contas_a_receber_consulta;

#[path = "../src/forms/fin/fin_contas_a_receber_hist.rs"]
pub mod fin_fin_contas_a_receber_hist;

#[path = "../src/forms/fin/fin_baixas_de_pagamento_cad.rs"]
pub mod fin_fin_baixas_de_pagamento_cad;

#[path = "../src/forms/fin/fin_baixas_de_pagamento_lista.rs"]
pub mod fin_fin_baixas_de_pagamento_lista;

#[path = "../src/forms/fin/fin_baixas_de_pagamento_consulta.rs"]
pub mod fin_fin_baixas_de_pagamento_consulta;

#[path = "../src/forms/fin/fin_baixas_de_pagamento_hist.rs"]
pub mod fin_fin_baixas_de_pagamento_hist;

#[path = "../src/forms/fin/fin_baixas_de_recebimento_cad.rs"]
pub mod fin_fin_baixas_de_recebimento_cad;

#[path = "../src/forms/fin/fin_baixas_de_recebimento_lista.rs"]
pub mod fin_fin_baixas_de_recebimento_lista;

#[path = "../src/forms/fin/fin_baixas_de_recebimento_consulta.rs"]
pub mod fin_fin_baixas_de_recebimento_consulta;

#[path = "../src/forms/fin/fin_baixas_de_recebimento_hist.rs"]
pub mod fin_fin_baixas_de_recebimento_hist;

#[path = "../src/forms/fin/fin_movimentos_bancarios_cad.rs"]
pub mod fin_fin_movimentos_bancarios_cad;

#[path = "../src/forms/fin/fin_movimentos_bancarios_lista.rs"]
pub mod fin_fin_movimentos_bancarios_lista;

#[path = "../src/forms/fin/fin_movimentos_bancarios_consulta.rs"]
pub mod fin_fin_movimentos_bancarios_consulta;

#[path = "../src/forms/fin/fin_movimentos_bancarios_hist.rs"]
pub mod fin_fin_movimentos_bancarios_hist;

#[path = "../src/forms/fin/fin_conciliacoes_bancarias_cad.rs"]
pub mod fin_fin_conciliacoes_bancarias_cad;

#[path = "../src/forms/fin/fin_conciliacoes_bancarias_lista.rs"]
pub mod fin_fin_conciliacoes_bancarias_lista;

#[path = "../src/forms/fin/fin_conciliacoes_bancarias_consulta.rs"]
pub mod fin_fin_conciliacoes_bancarias_consulta;

#[path = "../src/forms/fin/fin_conciliacoes_bancarias_hist.rs"]
pub mod fin_fin_conciliacoes_bancarias_hist;

#[path = "../src/forms/fin/fin_transferencias_entre_contas_cad.rs"]
pub mod fin_fin_transferencias_entre_contas_cad;

#[path = "../src/forms/fin/fin_transferencias_entre_contas_lista.rs"]
pub mod fin_fin_transferencias_entre_contas_lista;

#[path = "../src/forms/fin/fin_transferencias_entre_contas_consulta.rs"]
pub mod fin_fin_transferencias_entre_contas_consulta;

#[path = "../src/forms/fin/fin_transferencias_entre_contas_hist.rs"]
pub mod fin_fin_transferencias_entre_contas_hist;

#[path = "../src/forms/fin/fin_borderos_de_pagamento_cad.rs"]
pub mod fin_fin_borderos_de_pagamento_cad;

#[path = "../src/forms/fin/fin_borderos_de_pagamento_lista.rs"]
pub mod fin_fin_borderos_de_pagamento_lista;

#[path = "../src/forms/fin/fin_borderos_de_pagamento_consulta.rs"]
pub mod fin_fin_borderos_de_pagamento_consulta;

#[path = "../src/forms/fin/fin_borderos_de_pagamento_hist.rs"]
pub mod fin_fin_borderos_de_pagamento_hist;

#[path = "../src/forms/fin/fin_borderos_de_cobranca_cad.rs"]
pub mod fin_fin_borderos_de_cobranca_cad;

#[path = "../src/forms/fin/fin_borderos_de_cobranca_lista.rs"]
pub mod fin_fin_borderos_de_cobranca_lista;

#[path = "../src/forms/fin/fin_borderos_de_cobranca_consulta.rs"]
pub mod fin_fin_borderos_de_cobranca_consulta;

#[path = "../src/forms/fin/fin_borderos_de_cobranca_hist.rs"]
pub mod fin_fin_borderos_de_cobranca_hist;

#[path = "../src/forms/fin/fin_remessas_cnab_cad.rs"]
pub mod fin_fin_remessas_cnab_cad;

#[path = "../src/forms/fin/fin_remessas_cnab_lista.rs"]
pub mod fin_fin_remessas_cnab_lista;

#[path = "../src/forms/fin/fin_remessas_cnab_consulta.rs"]
pub mod fin_fin_remessas_cnab_consulta;

#[path = "../src/forms/fin/fin_remessas_cnab_hist.rs"]
pub mod fin_fin_remessas_cnab_hist;

#[path = "../src/forms/fin/fin_retornos_cnab_cad.rs"]
pub mod fin_fin_retornos_cnab_cad;

#[path = "../src/forms/fin/fin_retornos_cnab_lista.rs"]
pub mod fin_fin_retornos_cnab_lista;

#[path = "../src/forms/fin/fin_retornos_cnab_consulta.rs"]
pub mod fin_fin_retornos_cnab_consulta;

#[path = "../src/forms/fin/fin_retornos_cnab_hist.rs"]
pub mod fin_fin_retornos_cnab_hist;

#[path = "../src/forms/fin/fin_adiantamentos_a_fornecedores_cad.rs"]
pub mod fin_fin_adiantamentos_a_fornecedores_cad;

#[path = "../src/forms/fin/fin_adiantamentos_a_fornecedores_lista.rs"]
pub mod fin_fin_adiantamentos_a_fornecedores_lista;

#[path = "../src/forms/fin/fin_adiantamentos_a_fornecedores_consulta.rs"]
pub mod fin_fin_adiantamentos_a_fornecedores_consulta;

#[path = "../src/forms/fin/fin_adiantamentos_a_fornecedores_hist.rs"]
pub mod fin_fin_adiantamentos_a_fornecedores_hist;

#[path = "../src/forms/fin/fin_adiantamentos_de_clientes_cad.rs"]
pub mod fin_fin_adiantamentos_de_clientes_cad;

#[path = "../src/forms/fin/fin_adiantamentos_de_clientes_lista.rs"]
pub mod fin_fin_adiantamentos_de_clientes_lista;

#[path = "../src/forms/fin/fin_adiantamentos_de_clientes_consulta.rs"]
pub mod fin_fin_adiantamentos_de_clientes_consulta;

#[path = "../src/forms/fin/fin_adiantamentos_de_clientes_hist.rs"]
pub mod fin_fin_adiantamentos_de_clientes_hist;

#[path = "../src/forms/fin/fin_emprestimos_cad.rs"]
pub mod fin_fin_emprestimos_cad;

#[path = "../src/forms/fin/fin_emprestimos_lista.rs"]
pub mod fin_fin_emprestimos_lista;

#[path = "../src/forms/fin/fin_emprestimos_consulta.rs"]
pub mod fin_fin_emprestimos_consulta;

#[path = "../src/forms/fin/fin_emprestimos_hist.rs"]
pub mod fin_fin_emprestimos_hist;

#[path = "../src/forms/fin/fin_aplicacoes_financeiras_cad.rs"]
pub mod fin_fin_aplicacoes_financeiras_cad;

#[path = "../src/forms/fin/fin_aplicacoes_financeiras_lista.rs"]
pub mod fin_fin_aplicacoes_financeiras_lista;

#[path = "../src/forms/fin/fin_aplicacoes_financeiras_consulta.rs"]
pub mod fin_fin_aplicacoes_financeiras_consulta;

#[path = "../src/forms/fin/fin_aplicacoes_financeiras_hist.rs"]
pub mod fin_fin_aplicacoes_financeiras_hist;

#[path = "../src/forms/fin/fin_fluxos_de_caixa_cad.rs"]
pub mod fin_fin_fluxos_de_caixa_cad;

#[path = "../src/forms/fin/fin_fluxos_de_caixa_lista.rs"]
pub mod fin_fin_fluxos_de_caixa_lista;

#[path = "../src/forms/fin/fin_fluxos_de_caixa_consulta.rs"]
pub mod fin_fin_fluxos_de_caixa_consulta;

#[path = "../src/forms/fin/fin_fluxos_de_caixa_hist.rs"]
pub mod fin_fin_fluxos_de_caixa_hist;

#[path = "../src/forms/fin/fin_orcamentos_financeiros_cad.rs"]
pub mod fin_fin_orcamentos_financeiros_cad;

#[path = "../src/forms/fin/fin_orcamentos_financeiros_lista.rs"]
pub mod fin_fin_orcamentos_financeiros_lista;

#[path = "../src/forms/fin/fin_orcamentos_financeiros_consulta.rs"]
pub mod fin_fin_orcamentos_financeiros_consulta;

#[path = "../src/forms/fin/fin_orcamentos_financeiros_hist.rs"]
pub mod fin_fin_orcamentos_financeiros_hist;

#[path = "../src/forms/fin/fin_cheques_emitidos_cad.rs"]
pub mod fin_fin_cheques_emitidos_cad;

#[path = "../src/forms/fin/fin_cheques_emitidos_lista.rs"]
pub mod fin_fin_cheques_emitidos_lista;

#[path = "../src/forms/fin/fin_cheques_emitidos_consulta.rs"]
pub mod fin_fin_cheques_emitidos_consulta;

#[path = "../src/forms/fin/fin_cheques_emitidos_hist.rs"]
pub mod fin_fin_cheques_emitidos_hist;

#[path = "../src/forms/fin/fin_cheques_recebidos_cad.rs"]
pub mod fin_fin_cheques_recebidos_cad;

#[path = "../src/forms/fin/fin_cheques_recebidos_lista.rs"]
pub mod fin_fin_cheques_recebidos_lista;

#[path = "../src/forms/fin/fin_cheques_recebidos_consulta.rs"]
pub mod fin_fin_cheques_recebidos_consulta;

#[path = "../src/forms/fin/fin_cheques_recebidos_hist.rs"]
pub mod fin_fin_cheques_recebidos_hist;

#[path = "../src/forms/fin/fin_renegociacoes_cad.rs"]
pub mod fin_fin_renegociacoes_cad;

#[path = "../src/forms/fin/fin_renegociacoes_lista.rs"]
pub mod fin_fin_renegociacoes_lista;

#[path = "../src/forms/fin/fin_renegociacoes_consulta.rs"]
pub mod fin_fin_renegociacoes_consulta;

#[path = "../src/forms/fin/fin_renegociacoes_hist.rs"]
pub mod fin_fin_renegociacoes_hist;

#[path = "../src/forms/fin/fin_prestacoes_de_contas_cad.rs"]
pub mod fin_fin_prestacoes_de_contas_cad;

#[path = "../src/forms/fin/fin_prestacoes_de_contas_lista.rs"]
pub mod fin_fin_prestacoes_de_contas_lista;

#[path = "../src/forms/fin/fin_prestacoes_de_contas_consulta.rs"]
pub mod fin_fin_prestacoes_de_contas_consulta;

#[path = "../src/forms/fin/fin_prestacoes_de_contas_hist.rs"]
pub mod fin_fin_prestacoes_de_contas_hist;

#[path = "../src/forms/ctb/ctb_plano_de_contas_cad.rs"]
pub mod ctb_ctb_plano_de_contas_cad;

#[path = "../src/forms/ctb/ctb_plano_de_contas_lista.rs"]
pub mod ctb_ctb_plano_de_contas_lista;

#[path = "../src/forms/ctb/ctb_plano_de_contas_consulta.rs"]
pub mod ctb_ctb_plano_de_contas_consulta;

#[path = "../src/forms/ctb/ctb_plano_de_contas_hist.rs"]
pub mod ctb_ctb_plano_de_contas_hist;

#[path = "../src/forms/ctb/ctb_historicos_padrao_cad.rs"]
pub mod ctb_ctb_historicos_padrao_cad;

#[path = "../src/forms/ctb/ctb_historicos_padrao_lista.rs"]
pub mod ctb_ctb_historicos_padrao_lista;

#[path = "../src/forms/ctb/ctb_historicos_padrao_consulta.rs"]
pub mod ctb_ctb_historicos_padrao_consulta;

#[path = "../src/forms/ctb/ctb_historicos_padrao_hist.rs"]
pub mod ctb_ctb_historicos_padrao_hist;

#[path = "../src/forms/ctb/ctb_centros_de_custo_contabeis_cad.rs"]
pub mod ctb_ctb_centros_de_custo_contabeis_cad;

#[path = "../src/forms/ctb/ctb_centros_de_custo_contabeis_lista.rs"]
pub mod ctb_ctb_centros_de_custo_contabeis_lista;

#[path = "../src/forms/ctb/ctb_centros_de_custo_contabeis_consulta.rs"]
pub mod ctb_ctb_centros_de_custo_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_centros_de_custo_contabeis_hist.rs"]
pub mod ctb_ctb_centros_de_custo_contabeis_hist;

#[path = "../src/forms/ctb/ctb_exercicios_cad.rs"]
pub mod ctb_ctb_exercicios_cad;

#[path = "../src/forms/ctb/ctb_exercicios_lista.rs"]
pub mod ctb_ctb_exercicios_lista;

#[path = "../src/forms/ctb/ctb_exercicios_consulta.rs"]
pub mod ctb_ctb_exercicios_consulta;

#[path = "../src/forms/ctb/ctb_exercicios_hist.rs"]
pub mod ctb_ctb_exercicios_hist;

#[path = "../src/forms/ctb/ctb_periodos_contabeis_cad.rs"]
pub mod ctb_ctb_periodos_contabeis_cad;

#[path = "../src/forms/ctb/ctb_periodos_contabeis_lista.rs"]
pub mod ctb_ctb_periodos_contabeis_lista;

#[path = "../src/forms/ctb/ctb_periodos_contabeis_consulta.rs"]
pub mod ctb_ctb_periodos_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_periodos_contabeis_hist.rs"]
pub mod ctb_ctb_periodos_contabeis_hist;

#[path = "../src/forms/ctb/ctb_empresas_cad.rs"]
pub mod ctb_ctb_empresas_cad;

#[path = "../src/forms/ctb/ctb_empresas_lista.rs"]
pub mod ctb_ctb_empresas_lista;

#[path = "../src/forms/ctb/ctb_empresas_consulta.rs"]
pub mod ctb_ctb_empresas_consulta;

#[path = "../src/forms/ctb/ctb_empresas_hist.rs"]
pub mod ctb_ctb_empresas_hist;

#[path = "../src/forms/ctb/ctb_filiais_cad.rs"]
pub mod ctb_ctb_filiais_cad;

#[path = "../src/forms/ctb/ctb_filiais_lista.rs"]
pub mod ctb_ctb_filiais_lista;

#[path = "../src/forms/ctb/ctb_filiais_consulta.rs"]
pub mod ctb_ctb_filiais_consulta;

#[path = "../src/forms/ctb/ctb_filiais_hist.rs"]
pub mod ctb_ctb_filiais_hist;

#[path = "../src/forms/ctb/ctb_moedas_contabeis_cad.rs"]
pub mod ctb_ctb_moedas_contabeis_cad;

#[path = "../src/forms/ctb/ctb_moedas_contabeis_lista.rs"]
pub mod ctb_ctb_moedas_contabeis_lista;

#[path = "../src/forms/ctb/ctb_moedas_contabeis_consulta.rs"]
pub mod ctb_ctb_moedas_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_moedas_contabeis_hist.rs"]
pub mod ctb_ctb_moedas_contabeis_hist;

#[path = "../src/forms/ctb/ctb_grupos_de_contas_cad.rs"]
pub mod ctb_ctb_grupos_de_contas_cad;

#[path = "../src/forms/ctb/ctb_grupos_de_contas_lista.rs"]
pub mod ctb_ctb_grupos_de_contas_lista;

#[path = "../src/forms/ctb/ctb_grupos_de_contas_consulta.rs"]
pub mod ctb_ctb_grupos_de_contas_consulta;

#[path = "../src/forms/ctb/ctb_grupos_de_contas_hist.rs"]
pub mod ctb_ctb_grupos_de_contas_hist;

#[path = "../src/forms/ctb/ctb_contas_referenciais_cad.rs"]
pub mod ctb_ctb_contas_referenciais_cad;

#[path = "../src/forms/ctb/ctb_contas_referenciais_lista.rs"]
pub mod ctb_ctb_contas_referenciais_lista;

#[path = "../src/forms/ctb/ctb_contas_referenciais_consulta.rs"]
pub mod ctb_ctb_contas_referenciais_consulta;

#[path = "../src/forms/ctb/ctb_contas_referenciais_hist.rs"]
pub mod ctb_ctb_contas_referenciais_hist;

#[path = "../src/forms/ctb/ctb_responsaveis_contabeis_cad.rs"]
pub mod ctb_ctb_responsaveis_contabeis_cad;

#[path = "../src/forms/ctb/ctb_responsaveis_contabeis_lista.rs"]
pub mod ctb_ctb_responsaveis_contabeis_lista;

#[path = "../src/forms/ctb/ctb_responsaveis_contabeis_consulta.rs"]
pub mod ctb_ctb_responsaveis_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_responsaveis_contabeis_hist.rs"]
pub mod ctb_ctb_responsaveis_contabeis_hist;

#[path = "../src/forms/ctb/ctb_livros_contabeis_cad.rs"]
pub mod ctb_ctb_livros_contabeis_cad;

#[path = "../src/forms/ctb/ctb_livros_contabeis_lista.rs"]
pub mod ctb_ctb_livros_contabeis_lista;

#[path = "../src/forms/ctb/ctb_livros_contabeis_consulta.rs"]
pub mod ctb_ctb_livros_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_livros_contabeis_hist.rs"]
pub mod ctb_ctb_livros_contabeis_hist;

#[path = "../src/forms/ctb/ctb_naturezas_de_saldo_cad.rs"]
pub mod ctb_ctb_naturezas_de_saldo_cad;

#[path = "../src/forms/ctb/ctb_naturezas_de_saldo_lista.rs"]
pub mod ctb_ctb_naturezas_de_saldo_lista;

#[path = "../src/forms/ctb/ctb_naturezas_de_saldo_consulta.rs"]
pub mod ctb_ctb_naturezas_de_saldo_consulta;

#[path = "../src/forms/ctb/ctb_naturezas_de_saldo_hist.rs"]
pub mod ctb_ctb_naturezas_de_saldo_hist;

#[path = "../src/forms/ctb/ctb_regras_de_rateio_cad.rs"]
pub mod ctb_ctb_regras_de_rateio_cad;

#[path = "../src/forms/ctb/ctb_regras_de_rateio_lista.rs"]
pub mod ctb_ctb_regras_de_rateio_lista;

#[path = "../src/forms/ctb/ctb_regras_de_rateio_consulta.rs"]
pub mod ctb_ctb_regras_de_rateio_consulta;

#[path = "../src/forms/ctb/ctb_regras_de_rateio_hist.rs"]
pub mod ctb_ctb_regras_de_rateio_hist;

#[path = "../src/forms/ctb/ctb_criterios_de_depreciacao_cad.rs"]
pub mod ctb_ctb_criterios_de_depreciacao_cad;

#[path = "../src/forms/ctb/ctb_criterios_de_depreciacao_lista.rs"]
pub mod ctb_ctb_criterios_de_depreciacao_lista;

#[path = "../src/forms/ctb/ctb_criterios_de_depreciacao_consulta.rs"]
pub mod ctb_ctb_criterios_de_depreciacao_consulta;

#[path = "../src/forms/ctb/ctb_criterios_de_depreciacao_hist.rs"]
pub mod ctb_ctb_criterios_de_depreciacao_hist;

#[path = "../src/forms/ctb/ctb_grupos_de_bens_cad.rs"]
pub mod ctb_ctb_grupos_de_bens_cad;

#[path = "../src/forms/ctb/ctb_grupos_de_bens_lista.rs"]
pub mod ctb_ctb_grupos_de_bens_lista;

#[path = "../src/forms/ctb/ctb_grupos_de_bens_consulta.rs"]
pub mod ctb_ctb_grupos_de_bens_consulta;

#[path = "../src/forms/ctb/ctb_grupos_de_bens_hist.rs"]
pub mod ctb_ctb_grupos_de_bens_hist;

#[path = "../src/forms/ctb/ctb_locais_de_bens_cad.rs"]
pub mod ctb_ctb_locais_de_bens_cad;

#[path = "../src/forms/ctb/ctb_locais_de_bens_lista.rs"]
pub mod ctb_ctb_locais_de_bens_lista;

#[path = "../src/forms/ctb/ctb_locais_de_bens_consulta.rs"]
pub mod ctb_ctb_locais_de_bens_consulta;

#[path = "../src/forms/ctb/ctb_locais_de_bens_hist.rs"]
pub mod ctb_ctb_locais_de_bens_hist;

#[path = "../src/forms/ctb/ctb_tipos_de_lancamento_cad.rs"]
pub mod ctb_ctb_tipos_de_lancamento_cad;

#[path = "../src/forms/ctb/ctb_tipos_de_lancamento_lista.rs"]
pub mod ctb_ctb_tipos_de_lancamento_lista;

#[path = "../src/forms/ctb/ctb_tipos_de_lancamento_consulta.rs"]
pub mod ctb_ctb_tipos_de_lancamento_consulta;

#[path = "../src/forms/ctb/ctb_tipos_de_lancamento_hist.rs"]
pub mod ctb_ctb_tipos_de_lancamento_hist;

#[path = "../src/forms/ctb/ctb_agrupadores_de_dre_cad.rs"]
pub mod ctb_ctb_agrupadores_de_dre_cad;

#[path = "../src/forms/ctb/ctb_agrupadores_de_dre_lista.rs"]
pub mod ctb_ctb_agrupadores_de_dre_lista;

#[path = "../src/forms/ctb/ctb_agrupadores_de_dre_consulta.rs"]
pub mod ctb_ctb_agrupadores_de_dre_consulta;

#[path = "../src/forms/ctb/ctb_agrupadores_de_dre_hist.rs"]
pub mod ctb_ctb_agrupadores_de_dre_hist;

#[path = "../src/forms/ctb/ctb_notas_explicativas_cad.rs"]
pub mod ctb_ctb_notas_explicativas_cad;

#[path = "../src/forms/ctb/ctb_notas_explicativas_lista.rs"]
pub mod ctb_ctb_notas_explicativas_lista;

#[path = "../src/forms/ctb/ctb_notas_explicativas_consulta.rs"]
pub mod ctb_ctb_notas_explicativas_consulta;

#[path = "../src/forms/ctb/ctb_notas_explicativas_hist.rs"]
pub mod ctb_ctb_notas_explicativas_hist;

#[path = "../src/forms/ctb/ctb_calendarios_contabeis_cad.rs"]
pub mod ctb_ctb_calendarios_contabeis_cad;

#[path = "../src/forms/ctb/ctb_calendarios_contabeis_lista.rs"]
pub mod ctb_ctb_calendarios_contabeis_lista;

#[path = "../src/forms/ctb/ctb_calendarios_contabeis_consulta.rs"]
pub mod ctb_ctb_calendarios_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_calendarios_contabeis_hist.rs"]
pub mod ctb_ctb_calendarios_contabeis_hist;

#[path = "../src/forms/ctb/ctb_lancamentos_contabeis_cad.rs"]
pub mod ctb_ctb_lancamentos_contabeis_cad;

#[path = "../src/forms/ctb/ctb_lancamentos_contabeis_lista.rs"]
pub mod ctb_ctb_lancamentos_contabeis_lista;

#[path = "../src/forms/ctb/ctb_lancamentos_contabeis_consulta.rs"]
pub mod ctb_ctb_lancamentos_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_lancamentos_contabeis_hist.rs"]
pub mod ctb_ctb_lancamentos_contabeis_hist;

#[path = "../src/forms/ctb/ctb_lotes_de_lancamento_cad.rs"]
pub mod ctb_ctb_lotes_de_lancamento_cad;

#[path = "../src/forms/ctb/ctb_lotes_de_lancamento_lista.rs"]
pub mod ctb_ctb_lotes_de_lancamento_lista;

#[path = "../src/forms/ctb/ctb_lotes_de_lancamento_consulta.rs"]
pub mod ctb_ctb_lotes_de_lancamento_consulta;

#[path = "../src/forms/ctb/ctb_lotes_de_lancamento_hist.rs"]
pub mod ctb_ctb_lotes_de_lancamento_hist;

#[path = "../src/forms/ctb/ctb_rateios_cad.rs"]
pub mod ctb_ctb_rateios_cad;

#[path = "../src/forms/ctb/ctb_rateios_lista.rs"]
pub mod ctb_ctb_rateios_lista;

#[path = "../src/forms/ctb/ctb_rateios_consulta.rs"]
pub mod ctb_ctb_rateios_consulta;

#[path = "../src/forms/ctb/ctb_rateios_hist.rs"]
pub mod ctb_ctb_rateios_hist;

#[path = "../src/forms/ctb/ctb_encerramentos_cad.rs"]
pub mod ctb_ctb_encerramentos_cad;

#[path = "../src/forms/ctb/ctb_encerramentos_lista.rs"]
pub mod ctb_ctb_encerramentos_lista;

#[path = "../src/forms/ctb/ctb_encerramentos_consulta.rs"]
pub mod ctb_ctb_encerramentos_consulta;

#[path = "../src/forms/ctb/ctb_encerramentos_hist.rs"]
pub mod ctb_ctb_encerramentos_hist;

#[path = "../src/forms/ctb/ctb_apuracoes_de_resultado_cad.rs"]
pub mod ctb_ctb_apuracoes_de_resultado_cad;

#[path = "../src/forms/ctb/ctb_apuracoes_de_resultado_lista.rs"]
pub mod ctb_ctb_apuracoes_de_resultado_lista;

#[path = "../src/forms/ctb/ctb_apuracoes_de_resultado_consulta.rs"]
pub mod ctb_ctb_apuracoes_de_resultado_consulta;

#[path = "../src/forms/ctb/ctb_apuracoes_de_resultado_hist.rs"]
pub mod ctb_ctb_apuracoes_de_resultado_hist;

#[path = "../src/forms/ctb/ctb_balancetes_cad.rs"]
pub mod ctb_ctb_balancetes_cad;

#[path = "../src/forms/ctb/ctb_balancetes_lista.rs"]
pub mod ctb_ctb_balancetes_lista;

#[path = "../src/forms/ctb/ctb_balancetes_consulta.rs"]
pub mod ctb_ctb_balancetes_consulta;

#[path = "../src/forms/ctb/ctb_balancetes_hist.rs"]
pub mod ctb_ctb_balancetes_hist;

#[path = "../src/forms/ctb/ctb_balancos_cad.rs"]
pub mod ctb_ctb_balancos_cad;

#[path = "../src/forms/ctb/ctb_balancos_lista.rs"]
pub mod ctb_ctb_balancos_lista;

#[path = "../src/forms/ctb/ctb_balancos_consulta.rs"]
pub mod ctb_ctb_balancos_consulta;

#[path = "../src/forms/ctb/ctb_balancos_hist.rs"]
pub mod ctb_ctb_balancos_hist;

#[path = "../src/forms/ctb/ctb_dres_cad.rs"]
pub mod ctb_ctb_dres_cad;

#[path = "../src/forms/ctb/ctb_dres_lista.rs"]
pub mod ctb_ctb_dres_lista;

#[path = "../src/forms/ctb/ctb_dres_consulta.rs"]
pub mod ctb_ctb_dres_consulta;

#[path = "../src/forms/ctb/ctb_dres_hist.rs"]
pub mod ctb_ctb_dres_hist;

#[path = "../src/forms/ctb/ctb_razoes_cad.rs"]
pub mod ctb_ctb_razoes_cad;

#[path = "../src/forms/ctb/ctb_razoes_lista.rs"]
pub mod ctb_ctb_razoes_lista;

#[path = "../src/forms/ctb/ctb_razoes_consulta.rs"]
pub mod ctb_ctb_razoes_consulta;

#[path = "../src/forms/ctb/ctb_razoes_hist.rs"]
pub mod ctb_ctb_razoes_hist;

#[path = "../src/forms/ctb/ctb_diarios_cad.rs"]
pub mod ctb_ctb_diarios_cad;

#[path = "../src/forms/ctb/ctb_diarios_lista.rs"]
pub mod ctb_ctb_diarios_lista;

#[path = "../src/forms/ctb/ctb_diarios_consulta.rs"]
pub mod ctb_ctb_diarios_consulta;

#[path = "../src/forms/ctb/ctb_diarios_hist.rs"]
pub mod ctb_ctb_diarios_hist;

#[path = "../src/forms/ctb/ctb_conciliacoes_contabeis_cad.rs"]
pub mod ctb_ctb_conciliacoes_contabeis_cad;

#[path = "../src/forms/ctb/ctb_conciliacoes_contabeis_lista.rs"]
pub mod ctb_ctb_conciliacoes_contabeis_lista;

#[path = "../src/forms/ctb/ctb_conciliacoes_contabeis_consulta.rs"]
pub mod ctb_ctb_conciliacoes_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_conciliacoes_contabeis_hist.rs"]
pub mod ctb_ctb_conciliacoes_contabeis_hist;

#[path = "../src/forms/ctb/ctb_bens_patrimoniais_cad.rs"]
pub mod ctb_ctb_bens_patrimoniais_cad;

#[path = "../src/forms/ctb/ctb_bens_patrimoniais_lista.rs"]
pub mod ctb_ctb_bens_patrimoniais_lista;

#[path = "../src/forms/ctb/ctb_bens_patrimoniais_consulta.rs"]
pub mod ctb_ctb_bens_patrimoniais_consulta;

#[path = "../src/forms/ctb/ctb_bens_patrimoniais_hist.rs"]
pub mod ctb_ctb_bens_patrimoniais_hist;

#[path = "../src/forms/ctb/ctb_depreciacoes_cad.rs"]
pub mod ctb_ctb_depreciacoes_cad;

#[path = "../src/forms/ctb/ctb_depreciacoes_lista.rs"]
pub mod ctb_ctb_depreciacoes_lista;

#[path = "../src/forms/ctb/ctb_depreciacoes_consulta.rs"]
pub mod ctb_ctb_depreciacoes_consulta;

#[path = "../src/forms/ctb/ctb_depreciacoes_hist.rs"]
pub mod ctb_ctb_depreciacoes_hist;

#[path = "../src/forms/ctb/ctb_baixas_de_bens_cad.rs"]
pub mod ctb_ctb_baixas_de_bens_cad;

#[path = "../src/forms/ctb/ctb_baixas_de_bens_lista.rs"]
pub mod ctb_ctb_baixas_de_bens_lista;

#[path = "../src/forms/ctb/ctb_baixas_de_bens_consulta.rs"]
pub mod ctb_ctb_baixas_de_bens_consulta;

#[path = "../src/forms/ctb/ctb_baixas_de_bens_hist.rs"]
pub mod ctb_ctb_baixas_de_bens_hist;

#[path = "../src/forms/ctb/ctb_transferencias_de_bens_cad.rs"]
pub mod ctb_ctb_transferencias_de_bens_cad;

#[path = "../src/forms/ctb/ctb_transferencias_de_bens_lista.rs"]
pub mod ctb_ctb_transferencias_de_bens_lista;

#[path = "../src/forms/ctb/ctb_transferencias_de_bens_consulta.rs"]
pub mod ctb_ctb_transferencias_de_bens_consulta;

#[path = "../src/forms/ctb/ctb_transferencias_de_bens_hist.rs"]
pub mod ctb_ctb_transferencias_de_bens_hist;

#[path = "../src/forms/ctb/ctb_inventarios_patrimoniais_cad.rs"]
pub mod ctb_ctb_inventarios_patrimoniais_cad;

#[path = "../src/forms/ctb/ctb_inventarios_patrimoniais_lista.rs"]
pub mod ctb_ctb_inventarios_patrimoniais_lista;

#[path = "../src/forms/ctb/ctb_inventarios_patrimoniais_consulta.rs"]
pub mod ctb_ctb_inventarios_patrimoniais_consulta;

#[path = "../src/forms/ctb/ctb_inventarios_patrimoniais_hist.rs"]
pub mod ctb_ctb_inventarios_patrimoniais_hist;

#[path = "../src/forms/ctb/ctb_provisoes_cad.rs"]
pub mod ctb_ctb_provisoes_cad;

#[path = "../src/forms/ctb/ctb_provisoes_lista.rs"]
pub mod ctb_ctb_provisoes_lista;

#[path = "../src/forms/ctb/ctb_provisoes_consulta.rs"]
pub mod ctb_ctb_provisoes_consulta;

#[path = "../src/forms/ctb/ctb_provisoes_hist.rs"]
pub mod ctb_ctb_provisoes_hist;

#[path = "../src/forms/ctb/ctb_ajustes_de_saldo_cad.rs"]
pub mod ctb_ctb_ajustes_de_saldo_cad;

#[path = "../src/forms/ctb/ctb_ajustes_de_saldo_lista.rs"]
pub mod ctb_ctb_ajustes_de_saldo_lista;

#[path = "../src/forms/ctb/ctb_ajustes_de_saldo_consulta.rs"]
pub mod ctb_ctb_ajustes_de_saldo_consulta;

#[path = "../src/forms/ctb/ctb_ajustes_de_saldo_hist.rs"]
pub mod ctb_ctb_ajustes_de_saldo_hist;

#[path = "../src/forms/ctb/ctb_integracoes_contabeis_cad.rs"]
pub mod ctb_ctb_integracoes_contabeis_cad;

#[path = "../src/forms/ctb/ctb_integracoes_contabeis_lista.rs"]
pub mod ctb_ctb_integracoes_contabeis_lista;

#[path = "../src/forms/ctb/ctb_integracoes_contabeis_consulta.rs"]
pub mod ctb_ctb_integracoes_contabeis_consulta;

#[path = "../src/forms/ctb/ctb_integracoes_contabeis_hist.rs"]
pub mod ctb_ctb_integracoes_contabeis_hist;

#[path = "../src/forms/ctb/ctb_fechamentos_mensais_cad.rs"]
pub mod ctb_ctb_fechamentos_mensais_cad;

#[path = "../src/forms/ctb/ctb_fechamentos_mensais_lista.rs"]
pub mod ctb_ctb_fechamentos_mensais_lista;

#[path = "../src/forms/ctb/ctb_fechamentos_mensais_consulta.rs"]
pub mod ctb_ctb_fechamentos_mensais_consulta;

#[path = "../src/forms/ctb/ctb_fechamentos_mensais_hist.rs"]
pub mod ctb_ctb_fechamentos_mensais_hist;

#[path = "../src/forms/ctb/ctb_ecds_cad.rs"]
pub mod ctb_ctb_ecds_cad;

#[path = "../src/forms/ctb/ctb_ecds_lista.rs"]
pub mod ctb_ctb_ecds_lista;

#[path = "../src/forms/ctb/ctb_ecds_consulta.rs"]
pub mod ctb_ctb_ecds_consulta;

#[path = "../src/forms/ctb/ctb_ecds_hist.rs"]
pub mod ctb_ctb_ecds_hist;

#[path = "../src/forms/com/com_fornecedores_cad.rs"]
pub mod com_com_fornecedores_cad;

#[path = "../src/forms/com/com_fornecedores_lista.rs"]
pub mod com_com_fornecedores_lista;

#[path = "../src/forms/com/com_fornecedores_consulta.rs"]
pub mod com_com_fornecedores_consulta;

#[path = "../src/forms/com/com_fornecedores_hist.rs"]
pub mod com_com_fornecedores_hist;

#[path = "../src/forms/com/com_compradores_cad.rs"]
pub mod com_com_compradores_cad;

#[path = "../src/forms/com/com_compradores_lista.rs"]
pub mod com_com_compradores_lista;

#[path = "../src/forms/com/com_compradores_consulta.rs"]
pub mod com_com_compradores_consulta;

#[path = "../src/forms/com/com_compradores_hist.rs"]
pub mod com_com_compradores_hist;

#[path = "../src/forms/com/com_produtos_cad.rs"]
pub mod com_com_produtos_cad;

#[path = "../src/forms/com/com_produtos_lista.rs"]
pub mod com_com_produtos_lista;

#[path = "../src/forms/com/com_produtos_consulta.rs"]
pub mod com_com_produtos_consulta;

#[path = "../src/forms/com/com_produtos_hist.rs"]
pub mod com_com_produtos_hist;

#[path = "../src/forms/com/com_grupos_de_produtos_cad.rs"]
pub mod com_com_grupos_de_produtos_cad;

#[path = "../src/forms/com/com_grupos_de_produtos_lista.rs"]
pub mod com_com_grupos_de_produtos_lista;

#[path = "../src/forms/com/com_grupos_de_produtos_consulta.rs"]
pub mod com_com_grupos_de_produtos_consulta;

#[path = "../src/forms/com/com_grupos_de_produtos_hist.rs"]
pub mod com_com_grupos_de_produtos_hist;

#[path = "../src/forms/com/com_unidades_de_medida_cad.rs"]
pub mod com_com_unidades_de_medida_cad;

#[path = "../src/forms/com/com_unidades_de_medida_lista.rs"]
pub mod com_com_unidades_de_medida_lista;

#[path = "../src/forms/com/com_unidades_de_medida_consulta.rs"]
pub mod com_com_unidades_de_medida_consulta;

#[path = "../src/forms/com/com_unidades_de_medida_hist.rs"]
pub mod com_com_unidades_de_medida_hist;

#[path = "../src/forms/com/com_marcas_cad.rs"]
pub mod com_com_marcas_cad;

#[path = "../src/forms/com/com_marcas_lista.rs"]
pub mod com_com_marcas_lista;

#[path = "../src/forms/com/com_marcas_consulta.rs"]
pub mod com_com_marcas_consulta;

#[path = "../src/forms/com/com_marcas_hist.rs"]
pub mod com_com_marcas_hist;

#[path = "../src/forms/com/com_categorias_de_compra_cad.rs"]
pub mod com_com_categorias_de_compra_cad;

#[path = "../src/forms/com/com_categorias_de_compra_lista.rs"]
pub mod com_com_categorias_de_compra_lista;

#[path = "../src/forms/com/com_categorias_de_compra_consulta.rs"]
pub mod com_com_categorias_de_compra_consulta;

#[path = "../src/forms/com/com_categorias_de_compra_hist.rs"]
pub mod com_com_categorias_de_compra_hist;

#[path = "../src/forms/com/com_locais_de_entrega_cad.rs"]
pub mod com_com_locais_de_entrega_cad;

#[path = "../src/forms/com/com_locais_de_entrega_lista.rs"]
pub mod com_com_locais_de_entrega_lista;

#[path = "../src/forms/com/com_locais_de_entrega_consulta.rs"]
pub mod com_com_locais_de_entrega_consulta;

#[path = "../src/forms/com/com_locais_de_entrega_hist.rs"]
pub mod com_com_locais_de_entrega_hist;

#[path = "../src/forms/com/com_transportadoras_cad.rs"]
pub mod com_com_transportadoras_cad;

#[path = "../src/forms/com/com_transportadoras_lista.rs"]
pub mod com_com_transportadoras_lista;

#[path = "../src/forms/com/com_transportadoras_consulta.rs"]
pub mod com_com_transportadoras_consulta;

#[path = "../src/forms/com/com_transportadoras_hist.rs"]
pub mod com_com_transportadoras_hist;

#[path = "../src/forms/com/com_tabelas_de_preco_de_compra_cad.rs"]
pub mod com_com_tabelas_de_preco_de_compra_cad;

#[path = "../src/forms/com/com_tabelas_de_preco_de_compra_lista.rs"]
pub mod com_com_tabelas_de_preco_de_compra_lista;

#[path = "../src/forms/com/com_tabelas_de_preco_de_compra_consulta.rs"]
pub mod com_com_tabelas_de_preco_de_compra_consulta;

#[path = "../src/forms/com/com_tabelas_de_preco_de_compra_hist.rs"]
pub mod com_com_tabelas_de_preco_de_compra_hist;

#[path = "../src/forms/com/com_condicoes_de_compra_cad.rs"]
pub mod com_com_condicoes_de_compra_cad;

#[path = "../src/forms/com/com_condicoes_de_compra_lista.rs"]
pub mod com_com_condicoes_de_compra_lista;

#[path = "../src/forms/com/com_condicoes_de_compra_consulta.rs"]
pub mod com_com_condicoes_de_compra_consulta;

#[path = "../src/forms/com/com_condicoes_de_compra_hist.rs"]
pub mod com_com_condicoes_de_compra_hist;

#[path = "../src/forms/com/com_aprovadores_cad.rs"]
pub mod com_com_aprovadores_cad;

#[path = "../src/forms/com/com_aprovadores_lista.rs"]
pub mod com_com_aprovadores_lista;

#[path = "../src/forms/com/com_aprovadores_consulta.rs"]
pub mod com_com_aprovadores_consulta;

#[path = "../src/forms/com/com_aprovadores_hist.rs"]
pub mod com_com_aprovadores_hist;

#[path = "../src/forms/com/com_alcadas_de_aprovacao_cad.rs"]
pub mod com_com_alcadas_de_aprovacao_cad;

#[path = "../src/forms/com/com_alcadas_de_aprovacao_lista.rs"]
pub mod com_com_alcadas_de_aprovacao_lista;

#[path = "../src/forms/com/com_alcadas_de_aprovacao_consulta.rs"]
pub mod com_com_alcadas_de_aprovacao_consulta;

#[path = "../src/forms/com/com_alcadas_de_aprovacao_hist.rs"]
pub mod com_com_alcadas_de_aprovacao_hist;

#[path = "../src/forms/com/com_contratos_de_fornecimento_cad.rs"]
pub mod com_com_contratos_de_fornecimento_cad;

#[path = "../src/forms/com/com_contratos_de_fornecimento_lista.rs"]
pub mod com_com_contratos_de_fornecimento_lista;

#[path = "../src/forms/com/com_contratos_de_fornecimento_consulta.rs"]
pub mod com_com_contratos_de_fornecimento_consulta;

#[path = "../src/forms/com/com_contratos_de_fornecimento_hist.rs"]
pub mod com_com_contratos_de_fornecimento_hist;

#[path = "../src/forms/com/com_homologacoes_cad.rs"]
pub mod com_com_homologacoes_cad;

#[path = "../src/forms/com/com_homologacoes_lista.rs"]
pub mod com_com_homologacoes_lista;

#[path = "../src/forms/com/com_homologacoes_consulta.rs"]
pub mod com_com_homologacoes_consulta;

#[path = "../src/forms/com/com_homologacoes_hist.rs"]
pub mod com_com_homologacoes_hist;

#[path = "../src/forms/com/com_qualificacoes_de_fornecedor_cad.rs"]
pub mod com_com_qualificacoes_de_fornecedor_cad;

#[path = "../src/forms/com/com_qualificacoes_de_fornecedor_lista.rs"]
pub mod com_com_qualificacoes_de_fornecedor_lista;

#[path = "../src/forms/com/com_qualificacoes_de_fornecedor_consulta.rs"]
pub mod com_com_qualificacoes_de_fornecedor_consulta;

#[path = "../src/forms/com/com_qualificacoes_de_fornecedor_hist.rs"]
pub mod com_com_qualificacoes_de_fornecedor_hist;

#[path = "../src/forms/com/com_tipos_de_frete_cad.rs"]
pub mod com_com_tipos_de_frete_cad;

#[path = "../src/forms/com/com_tipos_de_frete_lista.rs"]
pub mod com_com_tipos_de_frete_lista;

#[path = "../src/forms/com/com_tipos_de_frete_consulta.rs"]
pub mod com_com_tipos_de_frete_consulta;

#[path = "../src/forms/com/com_tipos_de_frete_hist.rs"]
pub mod com_com_tipos_de_frete_hist;

#[path = "../src/forms/com/com_embalagens_cad.rs"]
pub mod com_com_embalagens_cad;

#[path = "../src/forms/com/com_embalagens_lista.rs"]
pub mod com_com_embalagens_lista;

#[path = "../src/forms/com/com_embalagens_consulta.rs"]
pub mod com_com_embalagens_consulta;

#[path = "../src/forms/com/com_embalagens_hist.rs"]
pub mod com_com_embalagens_hist;

#[path = "../src/forms/com/com_almoxarifados_cad.rs"]
pub mod com_com_almoxarifados_cad;

#[path = "../src/forms/com/com_almoxarifados_lista.rs"]
pub mod com_com_almoxarifados_lista;

#[path = "../src/forms/com/com_almoxarifados_consulta.rs"]
pub mod com_com_almoxarifados_consulta;

#[path = "../src/forms/com/com_almoxarifados_hist.rs"]
pub mod com_com_almoxarifados_hist;

#[path = "../src/forms/com/com_classes_de_material_cad.rs"]
pub mod com_com_classes_de_material_cad;

#[path = "../src/forms/com/com_classes_de_material_lista.rs"]
pub mod com_com_classes_de_material_lista;

#[path = "../src/forms/com/com_classes_de_material_consulta.rs"]
pub mod com_com_classes_de_material_consulta;

#[path = "../src/forms/com/com_classes_de_material_hist.rs"]
pub mod com_com_classes_de_material_hist;

#[path = "../src/forms/com/com_catalogos_de_fornecedor_cad.rs"]
pub mod com_com_catalogos_de_fornecedor_cad;

#[path = "../src/forms/com/com_catalogos_de_fornecedor_lista.rs"]
pub mod com_com_catalogos_de_fornecedor_lista;

#[path = "../src/forms/com/com_catalogos_de_fornecedor_consulta.rs"]
pub mod com_com_catalogos_de_fornecedor_consulta;

#[path = "../src/forms/com/com_catalogos_de_fornecedor_hist.rs"]
pub mod com_com_catalogos_de_fornecedor_hist;

#[path = "../src/forms/com/com_solicitacoes_de_compra_cad.rs"]
pub mod com_com_solicitacoes_de_compra_cad;

#[path = "../src/forms/com/com_solicitacoes_de_compra_lista.rs"]
pub mod com_com_solicitacoes_de_compra_lista;

#[path = "../src/forms/com/com_solicitacoes_de_compra_consulta.rs"]
pub mod com_com_solicitacoes_de_compra_consulta;

#[path = "../src/forms/com/com_solicitacoes_de_compra_hist.rs"]
pub mod com_com_solicitacoes_de_compra_hist;

#[path = "../src/forms/com/com_cotacoes_cad.rs"]
pub mod com_com_cotacoes_cad;

#[path = "../src/forms/com/com_cotacoes_lista.rs"]
pub mod com_com_cotacoes_lista;

#[path = "../src/forms/com/com_cotacoes_consulta.rs"]
pub mod com_com_cotacoes_consulta;

#[path = "../src/forms/com/com_cotacoes_hist.rs"]
pub mod com_com_cotacoes_hist;

#[path = "../src/forms/com/com_mapas_de_cotacao_cad.rs"]
pub mod com_com_mapas_de_cotacao_cad;

#[path = "../src/forms/com/com_mapas_de_cotacao_lista.rs"]
pub mod com_com_mapas_de_cotacao_lista;

#[path = "../src/forms/com/com_mapas_de_cotacao_consulta.rs"]
pub mod com_com_mapas_de_cotacao_consulta;

#[path = "../src/forms/com/com_mapas_de_cotacao_hist.rs"]
pub mod com_com_mapas_de_cotacao_hist;

#[path = "../src/forms/com/com_pedidos_de_compra_cad.rs"]
pub mod com_com_pedidos_de_compra_cad;

#[path = "../src/forms/com/com_pedidos_de_compra_lista.rs"]
pub mod com_com_pedidos_de_compra_lista;

#[path = "../src/forms/com/com_pedidos_de_compra_consulta.rs"]
pub mod com_com_pedidos_de_compra_consulta;

#[path = "../src/forms/com/com_pedidos_de_compra_hist.rs"]
pub mod com_com_pedidos_de_compra_hist;

#[path = "../src/forms/com/com_recebimentos_cad.rs"]
pub mod com_com_recebimentos_cad;

#[path = "../src/forms/com/com_recebimentos_lista.rs"]
pub mod com_com_recebimentos_lista;

#[path = "../src/forms/com/com_recebimentos_consulta.rs"]
pub mod com_com_recebimentos_consulta;

#[path = "../src/forms/com/com_recebimentos_hist.rs"]
pub mod com_com_recebimentos_hist;

#[path = "../src/forms/com/com_notas_de_entrada_cad.rs"]
pub mod com_com_notas_de_entrada_cad;

#[path = "../src/forms/com/com_notas_de_entrada_lista.rs"]
pub mod com_com_notas_de_entrada_lista;

#[path = "../src/forms/com/com_notas_de_entrada_consulta.rs"]
pub mod com_com_notas_de_entrada_consulta;

#[path = "../src/forms/com/com_notas_de_entrada_hist.rs"]
pub mod com_com_notas_de_entrada_hist;

#[path = "../src/forms/com/com_devolucoes_a_fornecedor_cad.rs"]
pub mod com_com_devolucoes_a_fornecedor_cad;

#[path = "../src/forms/com/com_devolucoes_a_fornecedor_lista.rs"]
pub mod com_com_devolucoes_a_fornecedor_lista;

#[path = "../src/forms/com/com_devolucoes_a_fornecedor_consulta.rs"]
pub mod com_com_devolucoes_a_fornecedor_consulta;

#[path = "../src/forms/com/com_devolucoes_a_fornecedor_hist.rs"]
pub mod com_com_devolucoes_a_fornecedor_hist;

#[path = "../src/forms/com/com_avaliacoes_de_fornecedor_cad.rs"]
pub mod com_com_avaliacoes_de_fornecedor_cad;

#[path = "../src/forms/com/com_avaliacoes_de_fornecedor_lista.rs"]
pub mod com_com_avaliacoes_de_fornecedor_lista;

#[path = "../src/forms/com/com_avaliacoes_de_fornecedor_consulta.rs"]
pub mod com_com_avaliacoes_de_fornecedor_consulta;

#[path = "../src/forms/com/com_avaliacoes_de_fornecedor_hist.rs"]
pub mod com_com_avaliacoes_de_fornecedor_hist;

#[path = "../src/forms/com/com_requisicoes_ao_almoxarifado_cad.rs"]
pub mod com_com_requisicoes_ao_almoxarifado_cad;

#[path = "../src/forms/com/com_requisicoes_ao_almoxarifado_lista.rs"]
pub mod com_com_requisicoes_ao_almoxarifado_lista;

#[path = "../src/forms/com/com_requisicoes_ao_almoxarifado_consulta.rs"]
pub mod com_com_requisicoes_ao_almoxarifado_consulta;

#[path = "../src/forms/com/com_requisicoes_ao_almoxarifado_hist.rs"]
pub mod com_com_requisicoes_ao_almoxarifado_hist;

#[path = "../src/forms/com/com_entradas_de_estoque_cad.rs"]
pub mod com_com_entradas_de_estoque_cad;

#[path = "../src/forms/com/com_entradas_de_estoque_lista.rs"]
pub mod com_com_entradas_de_estoque_lista;

#[path = "../src/forms/com/com_entradas_de_estoque_consulta.rs"]
pub mod com_com_entradas_de_estoque_consulta;

#[path = "../src/forms/com/com_entradas_de_estoque_hist.rs"]
pub mod com_com_entradas_de_estoque_hist;

#[path = "../src/forms/com/com_saidas_de_estoque_cad.rs"]
pub mod com_com_saidas_de_estoque_cad;

#[path = "../src/forms/com/com_saidas_de_estoque_lista.rs"]
pub mod com_com_saidas_de_estoque_lista;

#[path = "../src/forms/com/com_saidas_de_estoque_consulta.rs"]
pub mod com_com_saidas_de_estoque_consulta;

#[path = "../src/forms/com/com_saidas_de_estoque_hist.rs"]
pub mod com_com_saidas_de_estoque_hist;

#[path = "../src/forms/com/com_transferencias_de_estoque_cad.rs"]
pub mod com_com_transferencias_de_estoque_cad;

#[path = "../src/forms/com/com_transferencias_de_estoque_lista.rs"]
pub mod com_com_transferencias_de_estoque_lista;

#[path = "../src/forms/com/com_transferencias_de_estoque_consulta.rs"]
pub mod com_com_transferencias_de_estoque_consulta;

#[path = "../src/forms/com/com_transferencias_de_estoque_hist.rs"]
pub mod com_com_transferencias_de_estoque_hist;

#[path = "../src/forms/com/com_inventarios_cad.rs"]
pub mod com_com_inventarios_cad;

#[path = "../src/forms/com/com_inventarios_lista.rs"]
pub mod com_com_inventarios_lista;

#[path = "../src/forms/com/com_inventarios_consulta.rs"]
pub mod com_com_inventarios_consulta;

#[path = "../src/forms/com/com_inventarios_hist.rs"]
pub mod com_com_inventarios_hist;

#[path = "../src/forms/com/com_ajustes_de_estoque_cad.rs"]
pub mod com_com_ajustes_de_estoque_cad;

#[path = "../src/forms/com/com_ajustes_de_estoque_lista.rs"]
pub mod com_com_ajustes_de_estoque_lista;

#[path = "../src/forms/com/com_ajustes_de_estoque_consulta.rs"]
pub mod com_com_ajustes_de_estoque_consulta;

#[path = "../src/forms/com/com_ajustes_de_estoque_hist.rs"]
pub mod com_com_ajustes_de_estoque_hist;

#[path = "../src/forms/com/com_aprovacoes_de_compra_cad.rs"]
pub mod com_com_aprovacoes_de_compra_cad;

#[path = "../src/forms/com/com_aprovacoes_de_compra_lista.rs"]
pub mod com_com_aprovacoes_de_compra_lista;

#[path = "../src/forms/com/com_aprovacoes_de_compra_consulta.rs"]
pub mod com_com_aprovacoes_de_compra_consulta;

#[path = "../src/forms/com/com_aprovacoes_de_compra_hist.rs"]
pub mod com_com_aprovacoes_de_compra_hist;

#[path = "../src/forms/com/com_ordens_de_servico_externas_cad.rs"]
pub mod com_com_ordens_de_servico_externas_cad;

#[path = "../src/forms/com/com_ordens_de_servico_externas_lista.rs"]
pub mod com_com_ordens_de_servico_externas_lista;

#[path = "../src/forms/com/com_ordens_de_servico_externas_consulta.rs"]
pub mod com_com_ordens_de_servico_externas_consulta;

#[path = "../src/forms/com/com_ordens_de_servico_externas_hist.rs"]
pub mod com_com_ordens_de_servico_externas_hist;

#[path = "../src/forms/com/com_contratos_de_compra_cad.rs"]
pub mod com_com_contratos_de_compra_cad;

#[path = "../src/forms/com/com_contratos_de_compra_lista.rs"]
pub mod com_com_contratos_de_compra_lista;

#[path = "../src/forms/com/com_contratos_de_compra_consulta.rs"]
pub mod com_com_contratos_de_compra_consulta;

#[path = "../src/forms/com/com_contratos_de_compra_hist.rs"]
pub mod com_com_contratos_de_compra_hist;

#[path = "../src/forms/com/com_previsoes_de_compra_cad.rs"]
pub mod com_com_previsoes_de_compra_cad;

#[path = "../src/forms/com/com_previsoes_de_compra_lista.rs"]
pub mod com_com_previsoes_de_compra_lista;

#[path = "../src/forms/com/com_previsoes_de_compra_consulta.rs"]
pub mod com_com_previsoes_de_compra_consulta;

#[path = "../src/forms/com/com_previsoes_de_compra_hist.rs"]
pub mod com_com_previsoes_de_compra_hist;

#[path = "../src/forms/com/com_importacoes_cad.rs"]
pub mod com_com_importacoes_cad;

#[path = "../src/forms/com/com_importacoes_lista.rs"]
pub mod com_com_importacoes_lista;

#[path = "../src/forms/com/com_importacoes_consulta.rs"]
pub mod com_com_importacoes_consulta;

#[path = "../src/forms/com/com_importacoes_hist.rs"]
pub mod com_com_importacoes_hist;

#[path = "../src/forms/com/com_despesas_de_importacao_cad.rs"]
pub mod com_com_despesas_de_importacao_cad;

#[path = "../src/forms/com/com_despesas_de_importacao_lista.rs"]
pub mod com_com_despesas_de_importacao_lista;

#[path = "../src/forms/com/com_despesas_de_importacao_consulta.rs"]
pub mod com_com_despesas_de_importacao_consulta;

#[path = "../src/forms/com/com_despesas_de_importacao_hist.rs"]
pub mod com_com_despesas_de_importacao_hist;

#[path = "../src/forms/com/com_conferencias_de_recebimento_cad.rs"]
pub mod com_com_conferencias_de_recebimento_cad;

#[path = "../src/forms/com/com_conferencias_de_recebimento_lista.rs"]
pub mod com_com_conferencias_de_recebimento_lista;

#[path = "../src/forms/com/com_conferencias_de_recebimento_consulta.rs"]
pub mod com_com_conferencias_de_recebimento_consulta;

#[path = "../src/forms/com/com_conferencias_de_recebimento_hist.rs"]
pub mod com_com_conferencias_de_recebimento_hist;

#[path = "../src/forms/ven/ven_clientes_cad.rs"]
pub mod ven_ven_clientes_cad;

#[path = "../src/forms/ven/ven_clientes_lista.rs"]
pub mod ven_ven_clientes_lista;

#[path = "../src/forms/ven/ven_clientes_consulta.rs"]
pub mod ven_ven_clientes_consulta;

#[path = "../src/forms/ven/ven_clientes_hist.rs"]
pub mod ven_ven_clientes_hist;

#[path = "../src/forms/ven/ven_vendedores_cad.rs"]
pub mod ven_ven_vendedores_cad;

#[path = "../src/forms/ven/ven_vendedores_lista.rs"]
pub mod ven_ven_vendedores_lista;

#[path = "../src/forms/ven/ven_vendedores_consulta.rs"]
pub mod ven_ven_vendedores_consulta;

#[path = "../src/forms/ven/ven_vendedores_hist.rs"]
pub mod ven_ven_vendedores_hist;

#[path = "../src/forms/ven/ven_representantes_cad.rs"]
pub mod ven_ven_representantes_cad;

#[path = "../src/forms/ven/ven_representantes_lista.rs"]
pub mod ven_ven_representantes_lista;

#[path = "../src/forms/ven/ven_representantes_consulta.rs"]
pub mod ven_ven_representantes_consulta;

#[path = "../src/forms/ven/ven_representantes_hist.rs"]
pub mod ven_ven_representantes_hist;

#[path = "../src/forms/ven/ven_regioes_de_venda_cad.rs"]
pub mod ven_ven_regioes_de_venda_cad;

#[path = "../src/forms/ven/ven_regioes_de_venda_lista.rs"]
pub mod ven_ven_regioes_de_venda_lista;

#[path = "../src/forms/ven/ven_regioes_de_venda_consulta.rs"]
pub mod ven_ven_regioes_de_venda_consulta;

#[path = "../src/forms/ven/ven_regioes_de_venda_hist.rs"]
pub mod ven_ven_regioes_de_venda_hist;

#[path = "../src/forms/ven/ven_tabelas_de_preco_cad.rs"]
pub mod ven_ven_tabelas_de_preco_cad;

#[path = "../src/forms/ven/ven_tabelas_de_preco_lista.rs"]
pub mod ven_ven_tabelas_de_preco_lista;

#[path = "../src/forms/ven/ven_tabelas_de_preco_consulta.rs"]
pub mod ven_ven_tabelas_de_preco_consulta;

#[path = "../src/forms/ven/ven_tabelas_de_preco_hist.rs"]
pub mod ven_ven_tabelas_de_preco_hist;

#[path = "../src/forms/ven/ven_campanhas_cad.rs"]
pub mod ven_ven_campanhas_cad;

#[path = "../src/forms/ven/ven_campanhas_lista.rs"]
pub mod ven_ven_campanhas_lista;

#[path = "../src/forms/ven/ven_campanhas_consulta.rs"]
pub mod ven_ven_campanhas_consulta;

#[path = "../src/forms/ven/ven_campanhas_hist.rs"]
pub mod ven_ven_campanhas_hist;

#[path = "../src/forms/ven/ven_promocoes_cad.rs"]
pub mod ven_ven_promocoes_cad;

#[path = "../src/forms/ven/ven_promocoes_lista.rs"]
pub mod ven_ven_promocoes_lista;

#[path = "../src/forms/ven/ven_promocoes_consulta.rs"]
pub mod ven_ven_promocoes_consulta;

#[path = "../src/forms/ven/ven_promocoes_hist.rs"]
pub mod ven_ven_promocoes_hist;

#[path = "../src/forms/ven/ven_metas_de_venda_cad.rs"]
pub mod ven_ven_metas_de_venda_cad;

#[path = "../src/forms/ven/ven_metas_de_venda_lista.rs"]
pub mod ven_ven_metas_de_venda_lista;

#[path = "../src/forms/ven/ven_metas_de_venda_consulta.rs"]
pub mod ven_ven_metas_de_venda_consulta;

#[path = "../src/forms/ven/ven_metas_de_venda_hist.rs"]
pub mod ven_ven_metas_de_venda_hist;

#[path = "../src/forms/ven/ven_canais_de_venda_cad.rs"]
pub mod ven_ven_canais_de_venda_cad;

#[path = "../src/forms/ven/ven_canais_de_venda_lista.rs"]
pub mod ven_ven_canais_de_venda_lista;

#[path = "../src/forms/ven/ven_canais_de_venda_consulta.rs"]
pub mod ven_ven_canais_de_venda_consulta;

#[path = "../src/forms/ven/ven_canais_de_venda_hist.rs"]
pub mod ven_ven_canais_de_venda_hist;

#[path = "../src/forms/ven/ven_segmentos_de_cliente_cad.rs"]
pub mod ven_ven_segmentos_de_cliente_cad;

#[path = "../src/forms/ven/ven_segmentos_de_cliente_lista.rs"]
pub mod ven_ven_segmentos_de_cliente_lista;

#[path = "../src/forms/ven/ven_segmentos_de_cliente_consulta.rs"]
pub mod ven_ven_segmentos_de_cliente_consulta;

#[path = "../src/forms/ven/ven_segmentos_de_cliente_hist.rs"]
pub mod ven_ven_segmentos_de_cliente_hist;

#[path = "../src/forms/ven/ven_comissoes_cad.rs"]
pub mod ven_ven_comissoes_cad;

#[path = "../src/forms/ven/ven_comissoes_lista.rs"]
pub mod ven_ven_comissoes_lista;

#[path = "../src/forms/ven/ven_comissoes_consulta.rs"]
pub mod ven_ven_comissoes_consulta;

#[path = "../src/forms/ven/ven_comissoes_hist.rs"]
pub mod ven_ven_comissoes_hist;

#[path = "../src/forms/ven/ven_rotas_de_entrega_cad.rs"]
pub mod ven_ven_rotas_de_entrega_cad;

#[path = "../src/forms/ven/ven_rotas_de_entrega_lista.rs"]
pub mod ven_ven_rotas_de_entrega_lista;

#[path = "../src/forms/ven/ven_rotas_de_entrega_consulta.rs"]
pub mod ven_ven_rotas_de_entrega_consulta;

#[path = "../src/forms/ven/ven_rotas_de_entrega_hist.rs"]
pub mod ven_ven_rotas_de_entrega_hist;

#[path = "../src/forms/ven/ven_motivos_de_perda_cad.rs"]
pub mod ven_ven_motivos_de_perda_cad;

#[path = "../src/forms/ven/ven_motivos_de_perda_lista.rs"]
pub mod ven_ven_motivos_de_perda_lista;

#[path = "../src/forms/ven/ven_motivos_de_perda_consulta.rs"]
pub mod ven_ven_motivos_de_perda_consulta;

#[path = "../src/forms/ven/ven_motivos_de_perda_hist.rs"]
pub mod ven_ven_motivos_de_perda_hist;

#[path = "../src/forms/ven/ven_tipos_de_pedido_cad.rs"]
pub mod ven_ven_tipos_de_pedido_cad;

#[path = "../src/forms/ven/ven_tipos_de_pedido_lista.rs"]
pub mod ven_ven_tipos_de_pedido_lista;

#[path = "../src/forms/ven/ven_tipos_de_pedido_consulta.rs"]
pub mod ven_ven_tipos_de_pedido_consulta;

#[path = "../src/forms/ven/ven_tipos_de_pedido_hist.rs"]
pub mod ven_ven_tipos_de_pedido_hist;

#[path = "../src/forms/ven/ven_prazos_de_entrega_cad.rs"]
pub mod ven_ven_prazos_de_entrega_cad;

#[path = "../src/forms/ven/ven_prazos_de_entrega_lista.rs"]
pub mod ven_ven_prazos_de_entrega_lista;

#[path = "../src/forms/ven/ven_prazos_de_entrega_consulta.rs"]
pub mod ven_ven_prazos_de_entrega_consulta;

#[path = "../src/forms/ven/ven_prazos_de_entrega_hist.rs"]
pub mod ven_ven_prazos_de_entrega_hist;

#[path = "../src/forms/ven/ven_contatos_cad.rs"]
pub mod ven_ven_contatos_cad;

#[path = "../src/forms/ven/ven_contatos_lista.rs"]
pub mod ven_ven_contatos_lista;

#[path = "../src/forms/ven/ven_contatos_consulta.rs"]
pub mod ven_ven_contatos_consulta;

#[path = "../src/forms/ven/ven_contatos_hist.rs"]
pub mod ven_ven_contatos_hist;

#[path = "../src/forms/ven/ven_concorrentes_cad.rs"]
pub mod ven_ven_concorrentes_cad;

#[path = "../src/forms/ven/ven_concorrentes_lista.rs"]
pub mod ven_ven_concorrentes_lista;

#[path = "../src/forms/ven/ven_concorrentes_consulta.rs"]
pub mod ven_ven_concorrentes_consulta;

#[path = "../src/forms/ven/ven_concorrentes_hist.rs"]
pub mod ven_ven_concorrentes_hist;

#[path = "../src/forms/ven/ven_grupos_de_clientes_cad.rs"]
pub mod ven_ven_grupos_de_clientes_cad;

#[path = "../src/forms/ven/ven_grupos_de_clientes_lista.rs"]
pub mod ven_ven_grupos_de_clientes_lista;

#[path = "../src/forms/ven/ven_grupos_de_clientes_consulta.rs"]
pub mod ven_ven_grupos_de_clientes_consulta;

#[path = "../src/forms/ven/ven_grupos_de_clientes_hist.rs"]
pub mod ven_ven_grupos_de_clientes_hist;

#[path = "../src/forms/ven/ven_descontos_cad.rs"]
pub mod ven_ven_descontos_cad;

#[path = "../src/forms/ven/ven_descontos_lista.rs"]
pub mod ven_ven_descontos_lista;

#[path = "../src/forms/ven/ven_descontos_consulta.rs"]
pub mod ven_ven_descontos_consulta;

#[path = "../src/forms/ven/ven_descontos_hist.rs"]
pub mod ven_ven_descontos_hist;

#[path = "../src/forms/ven/ven_kits_de_produtos_cad.rs"]
pub mod ven_ven_kits_de_produtos_cad;

#[path = "../src/forms/ven/ven_kits_de_produtos_lista.rs"]
pub mod ven_ven_kits_de_produtos_lista;

#[path = "../src/forms/ven/ven_kits_de_produtos_consulta.rs"]
pub mod ven_ven_kits_de_produtos_consulta;

#[path = "../src/forms/ven/ven_kits_de_produtos_hist.rs"]
pub mod ven_ven_kits_de_produtos_hist;

#[path = "../src/forms/ven/ven_pontos_de_venda_cad.rs"]
pub mod ven_ven_pontos_de_venda_cad;

#[path = "../src/forms/ven/ven_pontos_de_venda_lista.rs"]
pub mod ven_ven_pontos_de_venda_lista;

#[path = "../src/forms/ven/ven_pontos_de_venda_consulta.rs"]
pub mod ven_ven_pontos_de_venda_consulta;

#[path = "../src/forms/ven/ven_pontos_de_venda_hist.rs"]
pub mod ven_ven_pontos_de_venda_hist;

#[path = "../src/forms/ven/ven_orcamentos_de_venda_cad.rs"]
pub mod ven_ven_orcamentos_de_venda_cad;

#[path = "../src/forms/ven/ven_orcamentos_de_venda_lista.rs"]
pub mod ven_ven_orcamentos_de_venda_lista;

#[path = "../src/forms/ven/ven_orcamentos_de_venda_consulta.rs"]
pub mod ven_ven_orcamentos_de_venda_consulta;

#[path = "../src/forms/ven/ven_orcamentos_de_venda_hist.rs"]
pub mod ven_ven_orcamentos_de_venda_hist;

#[path = "../src/forms/ven/ven_pedidos_de_venda_cad.rs"]
pub mod ven_ven_pedidos_de_venda_cad;

#[path = "../src/forms/ven/ven_pedidos_de_venda_lista.rs"]
pub mod ven_ven_pedidos_de_venda_lista;

#[path = "../src/forms/ven/ven_pedidos_de_venda_consulta.rs"]
pub mod ven_ven_pedidos_de_venda_consulta;

#[path = "../src/forms/ven/ven_pedidos_de_venda_hist.rs"]
pub mod ven_ven_pedidos_de_venda_hist;

#[path = "../src/forms/ven/ven_faturamentos_cad.rs"]
pub mod ven_ven_faturamentos_cad;

#[path = "../src/forms/ven/ven_faturamentos_lista.rs"]
pub mod ven_ven_faturamentos_lista;

#[path = "../src/forms/ven/ven_faturamentos_consulta.rs"]
pub mod ven_ven_faturamentos_consulta;

#[path = "../src/forms/ven/ven_faturamentos_hist.rs"]
pub mod ven_ven_faturamentos_hist;

#[path = "../src/forms/ven/ven_notas_de_saida_cad.rs"]
pub mod ven_ven_notas_de_saida_cad;

#[path = "../src/forms/ven/ven_notas_de_saida_lista.rs"]
pub mod ven_ven_notas_de_saida_lista;

#[path = "../src/forms/ven/ven_notas_de_saida_consulta.rs"]
pub mod ven_ven_notas_de_saida_consulta;

#[path = "../src/forms/ven/ven_notas_de_saida_hist.rs"]
pub mod ven_ven_notas_de_saida_hist;

#[path = "../src/forms/ven/ven_devolucoes_de_venda_cad.rs"]
pub mod ven_ven_devolucoes_de_venda_cad;

#[path = "../src/forms/ven/ven_devolucoes_de_venda_lista.rs"]
pub mod ven_ven_devolucoes_de_venda_lista;

#[path = "../src/forms/ven/ven_devolucoes_de_venda_consulta.rs"]
pub mod ven_ven_devolucoes_de_venda_consulta;

#[path = "../src/forms/ven/ven_devolucoes_de_venda_hist.rs"]
pub mod ven_ven_devolucoes_de_venda_hist;

#[path = "../src/forms/ven/ven_expedicoes_cad.rs"]
pub mod ven_ven_expedicoes_cad;

#[path = "../src/forms/ven/ven_expedicoes_lista.rs"]
pub mod ven_ven_expedicoes_lista;

#[path = "../src/forms/ven/ven_expedicoes_consulta.rs"]
pub mod ven_ven_expedicoes_consulta;

#[path = "../src/forms/ven/ven_expedicoes_hist.rs"]
pub mod ven_ven_expedicoes_hist;

#[path = "../src/forms/ven/ven_romaneios_cad.rs"]
pub mod ven_ven_romaneios_cad;

#[path = "../src/forms/ven/ven_romaneios_lista.rs"]
pub mod ven_ven_romaneios_lista;

#[path = "../src/forms/ven/ven_romaneios_consulta.rs"]
pub mod ven_ven_romaneios_consulta;

#[path = "../src/forms/ven/ven_romaneios_hist.rs"]
pub mod ven_ven_romaneios_hist;

#[path = "../src/forms/ven/ven_entregas_cad.rs"]
pub mod ven_ven_entregas_cad;

#[path = "../src/forms/ven/ven_entregas_lista.rs"]
pub mod ven_ven_entregas_lista;

#[path = "../src/forms/ven/ven_entregas_consulta.rs"]
pub mod ven_ven_entregas_consulta;

#[path = "../src/forms/ven/ven_entregas_hist.rs"]
pub mod ven_ven_entregas_hist;

#[path = "../src/forms/ven/ven_oportunidades_cad.rs"]
pub mod ven_ven_oportunidades_cad;

#[path = "../src/forms/ven/ven_oportunidades_lista.rs"]
pub mod ven_ven_oportunidades_lista;

#[path = "../src/forms/ven/ven_oportunidades_consulta.rs"]
pub mod ven_ven_oportunidades_consulta;

#[path = "../src/forms/ven/ven_oportunidades_hist.rs"]
pub mod ven_ven_oportunidades_hist;

#[path = "../src/forms/ven/ven_propostas_comerciais_cad.rs"]
pub mod ven_ven_propostas_comerciais_cad;

#[path = "../src/forms/ven/ven_propostas_comerciais_lista.rs"]
pub mod ven_ven_propostas_comerciais_lista;

#[path = "../src/forms/ven/ven_propostas_comerciais_consulta.rs"]
pub mod ven_ven_propostas_comerciais_consulta;

#[path = "../src/forms/ven/ven_propostas_comerciais_hist.rs"]
pub mod ven_ven_propostas_comerciais_hist;

#[path = "../src/forms/ven/ven_visitas_cad.rs"]
pub mod ven_ven_visitas_cad;

#[path = "../src/forms/ven/ven_visitas_lista.rs"]
pub mod ven_ven_visitas_lista;

#[path = "../src/forms/ven/ven_visitas_consulta.rs"]
pub mod ven_ven_visitas_consulta;

#[path = "../src/forms/ven/ven_visitas_hist.rs"]
pub mod ven_ven_visitas_hist;

#[path = "../src/forms/ven/ven_pos_vendas_cad.rs"]
pub mod ven_ven_pos_vendas_cad;

#[path = "../src/forms/ven/ven_pos_vendas_lista.rs"]
pub mod ven_ven_pos_vendas_lista;

#[path = "../src/forms/ven/ven_pos_vendas_consulta.rs"]
pub mod ven_ven_pos_vendas_consulta;

#[path = "../src/forms/ven/ven_pos_vendas_hist.rs"]
pub mod ven_ven_pos_vendas_hist;

#[path = "../src/forms/ven/ven_reclamacoes_cad.rs"]
pub mod ven_ven_reclamacoes_cad;

#[path = "../src/forms/ven/ven_reclamacoes_lista.rs"]
pub mod ven_ven_reclamacoes_lista;

#[path = "../src/forms/ven/ven_reclamacoes_consulta.rs"]
pub mod ven_ven_reclamacoes_consulta;

#[path = "../src/forms/ven/ven_reclamacoes_hist.rs"]
pub mod ven_ven_reclamacoes_hist;

#[path = "../src/forms/ven/ven_trocas_cad.rs"]
pub mod ven_ven_trocas_cad;

#[path = "../src/forms/ven/ven_trocas_lista.rs"]
pub mod ven_ven_trocas_lista;

#[path = "../src/forms/ven/ven_trocas_consulta.rs"]
pub mod ven_ven_trocas_consulta;

#[path = "../src/forms/ven/ven_trocas_hist.rs"]
pub mod ven_ven_trocas_hist;

#[path = "../src/forms/ven/ven_bonificacoes_cad.rs"]
pub mod ven_ven_bonificacoes_cad;

#[path = "../src/forms/ven/ven_bonificacoes_lista.rs"]
pub mod ven_ven_bonificacoes_lista;

#[path = "../src/forms/ven/ven_bonificacoes_consulta.rs"]
pub mod ven_ven_bonificacoes_consulta;

#[path = "../src/forms/ven/ven_bonificacoes_hist.rs"]
pub mod ven_ven_bonificacoes_hist;

#[path = "../src/forms/ven/ven_consignacoes_cad.rs"]
pub mod ven_ven_consignacoes_cad;

#[path = "../src/forms/ven/ven_consignacoes_lista.rs"]
pub mod ven_ven_consignacoes_lista;

#[path = "../src/forms/ven/ven_consignacoes_consulta.rs"]
pub mod ven_ven_consignacoes_consulta;

#[path = "../src/forms/ven/ven_consignacoes_hist.rs"]
pub mod ven_ven_consignacoes_hist;

#[path = "../src/forms/ven/ven_contratos_de_venda_cad.rs"]
pub mod ven_ven_contratos_de_venda_cad;

#[path = "../src/forms/ven/ven_contratos_de_venda_lista.rs"]
pub mod ven_ven_contratos_de_venda_lista;

#[path = "../src/forms/ven/ven_contratos_de_venda_consulta.rs"]
pub mod ven_ven_contratos_de_venda_consulta;

#[path = "../src/forms/ven/ven_contratos_de_venda_hist.rs"]
pub mod ven_ven_contratos_de_venda_hist;

#[path = "../src/forms/ven/ven_assinaturas_cad.rs"]
pub mod ven_ven_assinaturas_cad;

#[path = "../src/forms/ven/ven_assinaturas_lista.rs"]
pub mod ven_ven_assinaturas_lista;

#[path = "../src/forms/ven/ven_assinaturas_consulta.rs"]
pub mod ven_ven_assinaturas_consulta;

#[path = "../src/forms/ven/ven_assinaturas_hist.rs"]
pub mod ven_ven_assinaturas_hist;

#[path = "../src/forms/ven/ven_pedidos_de_e_commerce_cad.rs"]
pub mod ven_ven_pedidos_de_e_commerce_cad;

#[path = "../src/forms/ven/ven_pedidos_de_e_commerce_lista.rs"]
pub mod ven_ven_pedidos_de_e_commerce_lista;

#[path = "../src/forms/ven/ven_pedidos_de_e_commerce_consulta.rs"]
pub mod ven_ven_pedidos_de_e_commerce_consulta;

#[path = "../src/forms/ven/ven_pedidos_de_e_commerce_hist.rs"]
pub mod ven_ven_pedidos_de_e_commerce_hist;

#[path = "../src/forms/ven/ven_apuracoes_de_comissao_cad.rs"]
pub mod ven_ven_apuracoes_de_comissao_cad;

#[path = "../src/forms/ven/ven_apuracoes_de_comissao_lista.rs"]
pub mod ven_ven_apuracoes_de_comissao_lista;

#[path = "../src/forms/ven/ven_apuracoes_de_comissao_consulta.rs"]
pub mod ven_ven_apuracoes_de_comissao_consulta;

#[path = "../src/forms/ven/ven_apuracoes_de_comissao_hist.rs"]
pub mod ven_ven_apuracoes_de_comissao_hist;

#[path = "../src/forms/ven/ven_previsoes_de_venda_cad.rs"]
pub mod ven_ven_previsoes_de_venda_cad;

#[path = "../src/forms/ven/ven_previsoes_de_venda_lista.rs"]
pub mod ven_ven_previsoes_de_venda_lista;

#[path = "../src/forms/ven/ven_previsoes_de_venda_consulta.rs"]
pub mod ven_ven_previsoes_de_venda_consulta;

#[path = "../src/forms/ven/ven_previsoes_de_venda_hist.rs"]
pub mod ven_ven_previsoes_de_venda_hist;

#[path = "../src/forms/fis/fis_cfops_cad.rs"]
pub mod fis_fis_cfops_cad;

#[path = "../src/forms/fis/fis_cfops_lista.rs"]
pub mod fis_fis_cfops_lista;

#[path = "../src/forms/fis/fis_cfops_consulta.rs"]
pub mod fis_fis_cfops_consulta;

#[path = "../src/forms/fis/fis_cfops_hist.rs"]
pub mod fis_fis_cfops_hist;

#[path = "../src/forms/fis/fis_ncms_cad.rs"]
pub mod fis_fis_ncms_cad;

#[path = "../src/forms/fis/fis_ncms_lista.rs"]
pub mod fis_fis_ncms_lista;

#[path = "../src/forms/fis/fis_ncms_consulta.rs"]
pub mod fis_fis_ncms_consulta;

#[path = "../src/forms/fis/fis_ncms_hist.rs"]
pub mod fis_fis_ncms_hist;

#[path = "../src/forms/fis/fis_cests_cad.rs"]
pub mod fis_fis_cests_cad;

#[path = "../src/forms/fis/fis_cests_lista.rs"]
pub mod fis_fis_cests_lista;

#[path = "../src/forms/fis/fis_cests_consulta.rs"]
pub mod fis_fis_cests_consulta;

#[path = "../src/forms/fis/fis_cests_hist.rs"]
pub mod fis_fis_cests_hist;

#[path = "../src/forms/fis/fis_naturezas_de_operacao_cad.rs"]
pub mod fis_fis_naturezas_de_operacao_cad;

#[path = "../src/forms/fis/fis_naturezas_de_operacao_lista.rs"]
pub mod fis_fis_naturezas_de_operacao_lista;

#[path = "../src/forms/fis/fis_naturezas_de_operacao_consulta.rs"]
pub mod fis_fis_naturezas_de_operacao_consulta;

#[path = "../src/forms/fis/fis_naturezas_de_operacao_hist.rs"]
pub mod fis_fis_naturezas_de_operacao_hist;

#[path = "../src/forms/fis/fis_regras_de_icms_cad.rs"]
pub mod fis_fis_regras_de_icms_cad;

#[path = "../src/forms/fis/fis_regras_de_icms_lista.rs"]
pub mod fis_fis_regras_de_icms_lista;

#[path = "../src/forms/fis/fis_regras_de_icms_consulta.rs"]
pub mod fis_fis_regras_de_icms_consulta;

#[path = "../src/forms/fis/fis_regras_de_icms_hist.rs"]
pub mod fis_fis_regras_de_icms_hist;

#[path = "../src/forms/fis/fis_regras_de_ipi_cad.rs"]
pub mod fis_fis_regras_de_ipi_cad;

#[path = "../src/forms/fis/fis_regras_de_ipi_lista.rs"]
pub mod fis_fis_regras_de_ipi_lista;

#[path = "../src/forms/fis/fis_regras_de_ipi_consulta.rs"]
pub mod fis_fis_regras_de_ipi_consulta;

#[path = "../src/forms/fis/fis_regras_de_ipi_hist.rs"]
pub mod fis_fis_regras_de_ipi_hist;

#[path = "../src/forms/fis/fis_regras_de_pis_cad.rs"]
pub mod fis_fis_regras_de_pis_cad;

#[path = "../src/forms/fis/fis_regras_de_pis_lista.rs"]
pub mod fis_fis_regras_de_pis_lista;

#[path = "../src/forms/fis/fis_regras_de_pis_consulta.rs"]
pub mod fis_fis_regras_de_pis_consulta;

#[path = "../src/forms/fis/fis_regras_de_pis_hist.rs"]
pub mod fis_fis_regras_de_pis_hist;

#[path = "../src/forms/fis/fis_regras_de_cofins_cad.rs"]
pub mod fis_fis_regras_de_cofins_cad;

#[path = "../src/forms/fis/fis_regras_de_cofins_lista.rs"]
pub mod fis_fis_regras_de_cofins_lista;

#[path = "../src/forms/fis/fis_regras_de_cofins_consulta.rs"]
pub mod fis_fis_regras_de_cofins_consulta;

#[path = "../src/forms/fis/fis_regras_de_cofins_hist.rs"]
pub mod fis_fis_regras_de_cofins_hist;

#[path = "../src/forms/fis/fis_regras_de_iss_cad.rs"]
pub mod fis_fis_regras_de_iss_cad;

#[path = "../src/forms/fis/fis_regras_de_iss_lista.rs"]
pub mod fis_fis_regras_de_iss_lista;

#[path = "../src/forms/fis/fis_regras_de_iss_consulta.rs"]
pub mod fis_fis_regras_de_iss_consulta;

#[path = "../src/forms/fis/fis_regras_de_iss_hist.rs"]
pub mod fis_fis_regras_de_iss_hist;

#[path = "../src/forms/fis/fis_aliquotas_interestaduais_cad.rs"]
pub mod fis_fis_aliquotas_interestaduais_cad;

#[path = "../src/forms/fis/fis_aliquotas_interestaduais_lista.rs"]
pub mod fis_fis_aliquotas_interestaduais_lista;

#[path = "../src/forms/fis/fis_aliquotas_interestaduais_consulta.rs"]
pub mod fis_fis_aliquotas_interestaduais_consulta;

#[path = "../src/forms/fis/fis_aliquotas_interestaduais_hist.rs"]
pub mod fis_fis_aliquotas_interestaduais_hist;

#[path = "../src/forms/fis/fis_beneficios_fiscais_cad.rs"]
pub mod fis_fis_beneficios_fiscais_cad;

#[path = "../src/forms/fis/fis_beneficios_fiscais_lista.rs"]
pub mod fis_fis_beneficios_fiscais_lista;

#[path = "../src/forms/fis/fis_beneficios_fiscais_consulta.rs"]
pub mod fis_fis_beneficios_fiscais_consulta;

#[path = "../src/forms/fis/fis_beneficios_fiscais_hist.rs"]
pub mod fis_fis_beneficios_fiscais_hist;

#[path = "../src/forms/fis/fis_regimes_tributarios_cad.rs"]
pub mod fis_fis_regimes_tributarios_cad;

#[path = "../src/forms/fis/fis_regimes_tributarios_lista.rs"]
pub mod fis_fis_regimes_tributarios_lista;

#[path = "../src/forms/fis/fis_regimes_tributarios_consulta.rs"]
pub mod fis_fis_regimes_tributarios_consulta;

#[path = "../src/forms/fis/fis_regimes_tributarios_hist.rs"]
pub mod fis_fis_regimes_tributarios_hist;

#[path = "../src/forms/fis/fis_municipios_cad.rs"]
pub mod fis_fis_municipios_cad;

#[path = "../src/forms/fis/fis_municipios_lista.rs"]
pub mod fis_fis_municipios_lista;

#[path = "../src/forms/fis/fis_municipios_consulta.rs"]
pub mod fis_fis_municipios_consulta;

#[path = "../src/forms/fis/fis_municipios_hist.rs"]
pub mod fis_fis_municipios_hist;

#[path = "../src/forms/fis/fis_estados_cad.rs"]
pub mod fis_fis_estados_cad;

#[path = "../src/forms/fis/fis_estados_lista.rs"]
pub mod fis_fis_estados_lista;

#[path = "../src/forms/fis/fis_estados_consulta.rs"]
pub mod fis_fis_estados_consulta;

#[path = "../src/forms/fis/fis_estados_hist.rs"]
pub mod fis_fis_estados_hist;

#[path = "../src/forms/fis/fis_paises_cad.rs"]
pub mod fis_fis_paises_cad;

#[path = "../src/forms/fis/fis_paises_lista.rs"]
pub mod fis_fis_paises_lista;

#[path = "../src/forms/fis/fis_paises_consulta.rs"]
pub mod fis_fis_paises_consulta;

#[path = "../src/forms/fis/fis_paises_hist.rs"]
pub mod fis_fis_paises_hist;

#[path = "../src/forms/fis/fis_servicos_da_lc_116_cad.rs"]
pub mod fis_fis_servicos_da_lc_116_cad;

#[path = "../src/forms/fis/fis_servicos_da_lc_116_lista.rs"]
pub mod fis_fis_servicos_da_lc_116_lista;

#[path = "../src/forms/fis/fis_servicos_da_lc_116_consulta.rs"]
pub mod fis_fis_servicos_da_lc_116_consulta;

#[path = "../src/forms/fis/fis_servicos_da_lc_116_hist.rs"]
pub mod fis_fis_servicos_da_lc_116_hist;

#[path = "../src/forms/fis/fis_obrigacoes_acessorias_cad.rs"]
pub mod fis_fis_obrigacoes_acessorias_cad;

#[path = "../src/forms/fis/fis_obrigacoes_acessorias_lista.rs"]
pub mod fis_fis_obrigacoes_acessorias_lista;

#[path = "../src/forms/fis/fis_obrigacoes_acessorias_consulta.rs"]
pub mod fis_fis_obrigacoes_acessorias_consulta;

#[path = "../src/forms/fis/fis_obrigacoes_acessorias_hist.rs"]
pub mod fis_fis_obrigacoes_acessorias_hist;

#[path = "../src/forms/fis/fis_contabilistas_cad.rs"]
pub mod fis_fis_contabilistas_cad;

#[path = "../src/forms/fis/fis_contabilistas_lista.rs"]
pub mod fis_fis_contabilistas_lista;

#[path = "../src/forms/fis/fis_contabilistas_consulta.rs"]
pub mod fis_fis_contabilistas_consulta;

#[path = "../src/forms/fis/fis_contabilistas_hist.rs"]
pub mod fis_fis_contabilistas_hist;

#[path = "../src/forms/fis/fis_certificados_digitais_cad.rs"]
pub mod fis_fis_certificados_digitais_cad;

#[path = "../src/forms/fis/fis_certificados_digitais_lista.rs"]
pub mod fis_fis_certificados_digitais_lista;

#[path = "../src/forms/fis/fis_certificados_digitais_consulta.rs"]
pub mod fis_fis_certificados_digitais_consulta;

#[path = "../src/forms/fis/fis_certificados_digitais_hist.rs"]
pub mod fis_fis_certificados_digitais_hist;

#[path = "../src/forms/fis/fis_series_de_notas_cad.rs"]
pub mod fis_fis_series_de_notas_cad;

#[path = "../src/forms/fis/fis_series_de_notas_lista.rs"]
pub mod fis_fis_series_de_notas_lista;

#[path = "../src/forms/fis/fis_series_de_notas_consulta.rs"]
pub mod fis_fis_series_de_notas_consulta;

#[path = "../src/forms/fis/fis_series_de_notas_hist.rs"]
pub mod fis_fis_series_de_notas_hist;

#[path = "../src/forms/fis/fis_mensagens_fiscais_cad.rs"]
pub mod fis_fis_mensagens_fiscais_cad;

#[path = "../src/forms/fis/fis_mensagens_fiscais_lista.rs"]
pub mod fis_fis_mensagens_fiscais_lista;

#[path = "../src/forms/fis/fis_mensagens_fiscais_consulta.rs"]
pub mod fis_fis_mensagens_fiscais_consulta;

#[path = "../src/forms/fis/fis_mensagens_fiscais_hist.rs"]
pub mod fis_fis_mensagens_fiscais_hist;

#[path = "../src/forms/fis/fis_notas_fiscais_eletronicas_cad.rs"]
pub mod fis_fis_notas_fiscais_eletronicas_cad;

#[path = "../src/forms/fis/fis_notas_fiscais_eletronicas_lista.rs"]
pub mod fis_fis_notas_fiscais_eletronicas_lista;

#[path = "../src/forms/fis/fis_notas_fiscais_eletronicas_consulta.rs"]
pub mod fis_fis_notas_fiscais_eletronicas_consulta;

#[path = "../src/forms/fis/fis_notas_fiscais_eletronicas_hist.rs"]
pub mod fis_fis_notas_fiscais_eletronicas_hist;

#[path = "../src/forms/fis/fis_notas_de_servico_cad.rs"]
pub mod fis_fis_notas_de_servico_cad;

#[path = "../src/forms/fis/fis_notas_de_servico_lista.rs"]
pub mod fis_fis_notas_de_servico_lista;

#[path = "../src/forms/fis/fis_notas_de_servico_consulta.rs"]
pub mod fis_fis_notas_de_servico_consulta;

#[path = "../src/forms/fis/fis_notas_de_servico_hist.rs"]
pub mod fis_fis_notas_de_servico_hist;

#[path = "../src/forms/fis/fis_conhecimentos_de_transporte_cad.rs"]
pub mod fis_fis_conhecimentos_de_transporte_cad;

#[path = "../src/forms/fis/fis_conhecimentos_de_transporte_lista.rs"]
pub mod fis_fis_conhecimentos_de_transporte_lista;

#[path = "../src/forms/fis/fis_conhecimentos_de_transporte_consulta.rs"]
pub mod fis_fis_conhecimentos_de_transporte_consulta;

#[path = "../src/forms/fis/fis_conhecimentos_de_transporte_hist.rs"]
pub mod fis_fis_conhecimentos_de_transporte_hist;

#[path = "../src/forms/fis/fis_manifestos_de_carga_cad.rs"]
pub mod fis_fis_manifestos_de_carga_cad;

#[path = "../src/forms/fis/fis_manifestos_de_carga_lista.rs"]
pub mod fis_fis_manifestos_de_carga_lista;

#[path = "../src/forms/fis/fis_manifestos_de_carga_consulta.rs"]
pub mod fis_fis_manifestos_de_carga_consulta;

#[path = "../src/forms/fis/fis_manifestos_de_carga_hist.rs"]
pub mod fis_fis_manifestos_de_carga_hist;

#[path = "../src/forms/fis/fis_cartas_de_correcao_cad.rs"]
pub mod fis_fis_cartas_de_correcao_cad;

#[path = "../src/forms/fis/fis_cartas_de_correcao_lista.rs"]
pub mod fis_fis_cartas_de_correcao_lista;

#[path = "../src/forms/fis/fis_cartas_de_correcao_consulta.rs"]
pub mod fis_fis_cartas_de_correcao_consulta;

#[path = "../src/forms/fis/fis_cartas_de_correcao_hist.rs"]
pub mod fis_fis_cartas_de_correcao_hist;

#[path = "../src/forms/fis/fis_cancelamentos_de_nota_cad.rs"]
pub mod fis_fis_cancelamentos_de_nota_cad;

#[path = "../src/forms/fis/fis_cancelamentos_de_nota_lista.rs"]
pub mod fis_fis_cancelamentos_de_nota_lista;

#[path = "../src/forms/fis/fis_cancelamentos_de_nota_consulta.rs"]
pub mod fis_fis_cancelamentos_de_nota_consulta;

#[path = "../src/forms/fis/fis_cancelamentos_de_nota_hist.rs"]
pub mod fis_fis_cancelamentos_de_nota_hist;

#[path = "../src/forms/fis/fis_inutilizacoes_cad.rs"]
pub mod fis_fis_inutilizacoes_cad;

#[path = "../src/forms/fis/fis_inutilizacoes_lista.rs"]
pub mod fis_fis_inutilizacoes_lista;

#[path = "../src/forms/fis/fis_inutilizacoes_consulta.rs"]
pub mod fis_fis_inutilizacoes_consulta;

#[path = "../src/forms/fis/fis_inutilizacoes_hist.rs"]
pub mod fis_fis_inutilizacoes_hist;

#[path = "../src/forms/fis/fis_apuracoes_de_icms_cad.rs"]
pub mod fis_fis_apuracoes_de_icms_cad;

#[path = "../src/forms/fis/fis_apuracoes_de_icms_lista.rs"]
pub mod fis_fis_apuracoes_de_icms_lista;

#[path = "../src/forms/fis/fis_apuracoes_de_icms_consulta.rs"]
pub mod fis_fis_apuracoes_de_icms_consulta;

#[path = "../src/forms/fis/fis_apuracoes_de_icms_hist.rs"]
pub mod fis_fis_apuracoes_de_icms_hist;

#[path = "../src/forms/fis/fis_apuracoes_de_ipi_cad.rs"]
pub mod fis_fis_apuracoes_de_ipi_cad;

#[path = "../src/forms/fis/fis_apuracoes_de_ipi_lista.rs"]
pub mod fis_fis_apuracoes_de_ipi_lista;

#[path = "../src/forms/fis/fis_apuracoes_de_ipi_consulta.rs"]
pub mod fis_fis_apuracoes_de_ipi_consulta;

#[path = "../src/forms/fis/fis_apuracoes_de_ipi_hist.rs"]
pub mod fis_fis_apuracoes_de_ipi_hist;

#[path = "../src/forms/fis/fis_apuracoes_de_pis_e_cofins_cad.rs"]
pub mod fis_fis_apuracoes_de_pis_e_cofins_cad;

#[path = "../src/forms/fis/fis_apuracoes_de_pis_e_cofins_lista.rs"]
pub mod fis_fis_apuracoes_de_pis_e_cofins_lista;

#[path = "../src/forms/fis/fis_apuracoes_de_pis_e_cofins_consulta.rs"]
pub mod fis_fis_apuracoes_de_pis_e_cofins_consulta;

#[path = "../src/forms/fis/fis_apuracoes_de_pis_e_cofins_hist.rs"]
pub mod fis_fis_apuracoes_de_pis_e_cofins_hist;

#[path = "../src/forms/fis/fis_guias_de_recolhimento_cad.rs"]
pub mod fis_fis_guias_de_recolhimento_cad;

#[path = "../src/forms/fis/fis_guias_de_recolhimento_lista.rs"]
pub mod fis_fis_guias_de_recolhimento_lista;

#[path = "../src/forms/fis/fis_guias_de_recolhimento_consulta.rs"]
pub mod fis_fis_guias_de_recolhimento_consulta;

#[path = "../src/forms/fis/fis_guias_de_recolhimento_hist.rs"]
pub mod fis_fis_guias_de_recolhimento_hist;

#[path = "../src/forms/fis/fis_sped_fiscal_cad.rs"]
pub mod fis_fis_sped_fiscal_cad;

#[path = "../src/forms/fis/fis_sped_fiscal_lista.rs"]
pub mod fis_fis_sped_fiscal_lista;

#[path = "../src/forms/fis/fis_sped_fiscal_consulta.rs"]
pub mod fis_fis_sped_fiscal_consulta;

#[path = "../src/forms/fis/fis_sped_fiscal_hist.rs"]
pub mod fis_fis_sped_fiscal_hist;

#[path = "../src/forms/fis/fis_sped_contribuicoes_cad.rs"]
pub mod fis_fis_sped_contribuicoes_cad;

#[path = "../src/forms/fis/fis_sped_contribuicoes_lista.rs"]
pub mod fis_fis_sped_contribuicoes_lista;

#[path = "../src/forms/fis/fis_sped_contribuicoes_consulta.rs"]
pub mod fis_fis_sped_contribuicoes_consulta;

#[path = "../src/forms/fis/fis_sped_contribuicoes_hist.rs"]
pub mod fis_fis_sped_contribuicoes_hist;

#[path = "../src/forms/fis/fis_livros_de_entrada_cad.rs"]
pub mod fis_fis_livros_de_entrada_cad;

#[path = "../src/forms/fis/fis_livros_de_entrada_lista.rs"]
pub mod fis_fis_livros_de_entrada_lista;

#[path = "../src/forms/fis/fis_livros_de_entrada_consulta.rs"]
pub mod fis_fis_livros_de_entrada_consulta;

#[path = "../src/forms/fis/fis_livros_de_entrada_hist.rs"]
pub mod fis_fis_livros_de_entrada_hist;

#[path = "../src/forms/fis/fis_livros_de_saida_cad.rs"]
pub mod fis_fis_livros_de_saida_cad;

#[path = "../src/forms/fis/fis_livros_de_saida_lista.rs"]
pub mod fis_fis_livros_de_saida_lista;

#[path = "../src/forms/fis/fis_livros_de_saida_consulta.rs"]
pub mod fis_fis_livros_de_saida_consulta;

#[path = "../src/forms/fis/fis_livros_de_saida_hist.rs"]
pub mod fis_fis_livros_de_saida_hist;

#[path = "../src/forms/fis/fis_livros_de_servico_cad.rs"]
pub mod fis_fis_livros_de_servico_cad;

#[path = "../src/forms/fis/fis_livros_de_servico_lista.rs"]
pub mod fis_fis_livros_de_servico_lista;

#[path = "../src/forms/fis/fis_livros_de_servico_consulta.rs"]
pub mod fis_fis_livros_de_servico_consulta;

#[path = "../src/forms/fis/fis_livros_de_servico_hist.rs"]
pub mod fis_fis_livros_de_servico_hist;

#[path = "../src/forms/fis/fis_retencoes_na_fonte_cad.rs"]
pub mod fis_fis_retencoes_na_fonte_cad;

#[path = "../src/forms/fis/fis_retencoes_na_fonte_lista.rs"]
pub mod fis_fis_retencoes_na_fonte_lista;

#[path = "../src/forms/fis/fis_retencoes_na_fonte_consulta.rs"]
pub mod fis_fis_retencoes_na_fonte_consulta;

#[path = "../src/forms/fis/fis_retencoes_na_fonte_hist.rs"]
pub mod fis_fis_retencoes_na_fonte_hist;

#[path = "../src/forms/fis/fis_creditos_tributarios_cad.rs"]
pub mod fis_fis_creditos_tributarios_cad;

#[path = "../src/forms/fis/fis_creditos_tributarios_lista.rs"]
pub mod fis_fis_creditos_tributarios_lista;

#[path = "../src/forms/fis/fis_creditos_tributarios_consulta.rs"]
pub mod fis_fis_creditos_tributarios_consulta;

#[path = "../src/forms/fis/fis_creditos_tributarios_hist.rs"]
pub mod fis_fis_creditos_tributarios_hist;

#[path = "../src/forms/fis/fis_ajustes_de_apuracao_cad.rs"]
pub mod fis_fis_ajustes_de_apuracao_cad;

#[path = "../src/forms/fis/fis_ajustes_de_apuracao_lista.rs"]
pub mod fis_fis_ajustes_de_apuracao_lista;

#[path = "../src/forms/fis/fis_ajustes_de_apuracao_consulta.rs"]
pub mod fis_fis_ajustes_de_apuracao_consulta;

#[path = "../src/forms/fis/fis_ajustes_de_apuracao_hist.rs"]
pub mod fis_fis_ajustes_de_apuracao_hist;

#[path = "../src/forms/fis/fis_escrituracoes_cad.rs"]
pub mod fis_fis_escrituracoes_cad;

#[path = "../src/forms/fis/fis_escrituracoes_lista.rs"]
pub mod fis_fis_escrituracoes_lista;

#[path = "../src/forms/fis/fis_escrituracoes_consulta.rs"]
pub mod fis_fis_escrituracoes_consulta;

#[path = "../src/forms/fis/fis_escrituracoes_hist.rs"]
pub mod fis_fis_escrituracoes_hist;

#[path = "../src/forms/fis/fis_declaracoes_cad.rs"]
pub mod fis_fis_declaracoes_cad;

#[path = "../src/forms/fis/fis_declaracoes_lista.rs"]
pub mod fis_fis_declaracoes_lista;

#[path = "../src/forms/fis/fis_declaracoes_consulta.rs"]
pub mod fis_fis_declaracoes_consulta;

#[path = "../src/forms/fis/fis_declaracoes_hist.rs"]
pub mod fis_fis_declaracoes_hist;

/// Registra as classes dos forms (cada form aberto ganha a instância dele), os handlers e
/// as classes dos pacotes de código.
pub fn register(ui: &mut Ui) {
    ui.register_form_class("frmPrincipal", FrmPrincipal::bind);
    ui.register_handler("frmPrincipal.form_show", aylis::prelude::instances::form_handler(FrmPrincipal::form_show));
    ui.register_handler("frmPrincipal.edt_busca_change", aylis::prelude::instances::form_handler(FrmPrincipal::edt_busca_change));
    ui.register_handler("frmPrincipal.btn_abrir_click", aylis::prelude::instances::form_handler(FrmPrincipal::btn_abrir_click));
    ui.register_handler("frmPrincipal.grd_telas_dbl_click", aylis::prelude::instances::form_handler(FrmPrincipal::grd_telas_dbl_click));
    ui.register_form_class("frmRhFuncionariosCad", FrmRhFuncionariosCad::bind);
    ui.register_handler("frmRhFuncionariosCad.form_show", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::form_show));
    ui.register_handler("frmRhFuncionariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_novo_click));
    ui.register_handler("frmRhFuncionariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_salvar_click));
    ui.register_handler("frmRhFuncionariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_excluir_click));
    ui.register_handler("frmRhFuncionariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_cancelar_click));
    ui.register_handler("frmRhFuncionariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_novo_item_click));
    ui.register_handler("frmRhFuncionariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::btn_fechar_click));
    ui.register_handler("frmRhFuncionariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFuncionariosCad::edt_busca_change));
    ui.register_form_class("frmRhFuncionariosLista", FrmRhFuncionariosLista::bind);
    ui.register_handler("frmRhFuncionariosLista.form_show", aylis::prelude::instances::form_handler(FrmRhFuncionariosLista::form_show));
    ui.register_handler("frmRhFuncionariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosLista::btn_abrir_click));
    ui.register_handler("frmRhFuncionariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFuncionariosLista::edt_busca_change));
    ui.register_handler("frmRhFuncionariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhFuncionariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhFuncionariosConsulta", FrmRhFuncionariosConsulta::bind);
    ui.register_handler("frmRhFuncionariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhFuncionariosConsulta::form_show));
    ui.register_handler("frmRhFuncionariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFuncionariosConsulta::edt_busca_change));
    ui.register_form_class("frmRhFuncionariosHist", FrmRhFuncionariosHist::bind);
    ui.register_handler("frmRhFuncionariosHist.form_show", aylis::prelude::instances::form_handler(FrmRhFuncionariosHist::form_show));
    ui.register_handler("frmRhFuncionariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFuncionariosHist::edt_busca_change));
    ui.register_form_class("frmRhCargosCad", FrmRhCargosCad::bind);
    ui.register_handler("frmRhCargosCad.form_show", aylis::prelude::instances::form_handler(FrmRhCargosCad::form_show));
    ui.register_handler("frmRhCargosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_novo_click));
    ui.register_handler("frmRhCargosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_salvar_click));
    ui.register_handler("frmRhCargosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_excluir_click));
    ui.register_handler("frmRhCargosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_cancelar_click));
    ui.register_handler("frmRhCargosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_novo_item_click));
    ui.register_handler("frmRhCargosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhCargosCad::btn_fechar_click));
    ui.register_handler("frmRhCargosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCargosCad::edt_busca_change));
    ui.register_form_class("frmRhCargosLista", FrmRhCargosLista::bind);
    ui.register_handler("frmRhCargosLista.form_show", aylis::prelude::instances::form_handler(FrmRhCargosLista::form_show));
    ui.register_handler("frmRhCargosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhCargosLista::btn_abrir_click));
    ui.register_handler("frmRhCargosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCargosLista::edt_busca_change));
    ui.register_handler("frmRhCargosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhCargosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhCargosConsulta", FrmRhCargosConsulta::bind);
    ui.register_handler("frmRhCargosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhCargosConsulta::form_show));
    ui.register_handler("frmRhCargosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCargosConsulta::edt_busca_change));
    ui.register_form_class("frmRhCargosHist", FrmRhCargosHist::bind);
    ui.register_handler("frmRhCargosHist.form_show", aylis::prelude::instances::form_handler(FrmRhCargosHist::form_show));
    ui.register_handler("frmRhCargosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCargosHist::edt_busca_change));
    ui.register_form_class("frmRhDepartamentosCad", FrmRhDepartamentosCad::bind);
    ui.register_handler("frmRhDepartamentosCad.form_show", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::form_show));
    ui.register_handler("frmRhDepartamentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_novo_click));
    ui.register_handler("frmRhDepartamentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_salvar_click));
    ui.register_handler("frmRhDepartamentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_excluir_click));
    ui.register_handler("frmRhDepartamentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_cancelar_click));
    ui.register_handler("frmRhDepartamentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_novo_item_click));
    ui.register_handler("frmRhDepartamentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::btn_fechar_click));
    ui.register_handler("frmRhDepartamentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDepartamentosCad::edt_busca_change));
    ui.register_form_class("frmRhDepartamentosLista", FrmRhDepartamentosLista::bind);
    ui.register_handler("frmRhDepartamentosLista.form_show", aylis::prelude::instances::form_handler(FrmRhDepartamentosLista::form_show));
    ui.register_handler("frmRhDepartamentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosLista::btn_abrir_click));
    ui.register_handler("frmRhDepartamentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDepartamentosLista::edt_busca_change));
    ui.register_handler("frmRhDepartamentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhDepartamentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhDepartamentosConsulta", FrmRhDepartamentosConsulta::bind);
    ui.register_handler("frmRhDepartamentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhDepartamentosConsulta::form_show));
    ui.register_handler("frmRhDepartamentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDepartamentosConsulta::edt_busca_change));
    ui.register_form_class("frmRhDepartamentosHist", FrmRhDepartamentosHist::bind);
    ui.register_handler("frmRhDepartamentosHist.form_show", aylis::prelude::instances::form_handler(FrmRhDepartamentosHist::form_show));
    ui.register_handler("frmRhDepartamentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDepartamentosHist::edt_busca_change));
    ui.register_form_class("frmRhCentrosDeCustoCad", FrmRhCentrosDeCustoCad::bind);
    ui.register_handler("frmRhCentrosDeCustoCad.form_show", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::form_show));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_novo_click));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_salvar_click));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_excluir_click));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_cancelar_click));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_novo_item_click));
    ui.register_handler("frmRhCentrosDeCustoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::btn_fechar_click));
    ui.register_handler("frmRhCentrosDeCustoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoCad::edt_busca_change));
    ui.register_form_class("frmRhCentrosDeCustoLista", FrmRhCentrosDeCustoLista::bind);
    ui.register_handler("frmRhCentrosDeCustoLista.form_show", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoLista::form_show));
    ui.register_handler("frmRhCentrosDeCustoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoLista::btn_abrir_click));
    ui.register_handler("frmRhCentrosDeCustoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoLista::edt_busca_change));
    ui.register_handler("frmRhCentrosDeCustoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhCentrosDeCustoConsulta", FrmRhCentrosDeCustoConsulta::bind);
    ui.register_handler("frmRhCentrosDeCustoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoConsulta::form_show));
    ui.register_handler("frmRhCentrosDeCustoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoConsulta::edt_busca_change));
    ui.register_form_class("frmRhCentrosDeCustoHist", FrmRhCentrosDeCustoHist::bind);
    ui.register_handler("frmRhCentrosDeCustoHist.form_show", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoHist::form_show));
    ui.register_handler("frmRhCentrosDeCustoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCentrosDeCustoHist::edt_busca_change));
    ui.register_form_class("frmRhSindicatosCad", FrmRhSindicatosCad::bind);
    ui.register_handler("frmRhSindicatosCad.form_show", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::form_show));
    ui.register_handler("frmRhSindicatosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_novo_click));
    ui.register_handler("frmRhSindicatosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_salvar_click));
    ui.register_handler("frmRhSindicatosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_excluir_click));
    ui.register_handler("frmRhSindicatosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_cancelar_click));
    ui.register_handler("frmRhSindicatosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_novo_item_click));
    ui.register_handler("frmRhSindicatosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::btn_fechar_click));
    ui.register_handler("frmRhSindicatosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhSindicatosCad::edt_busca_change));
    ui.register_form_class("frmRhSindicatosLista", FrmRhSindicatosLista::bind);
    ui.register_handler("frmRhSindicatosLista.form_show", aylis::prelude::instances::form_handler(FrmRhSindicatosLista::form_show));
    ui.register_handler("frmRhSindicatosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhSindicatosLista::btn_abrir_click));
    ui.register_handler("frmRhSindicatosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhSindicatosLista::edt_busca_change));
    ui.register_handler("frmRhSindicatosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhSindicatosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhSindicatosConsulta", FrmRhSindicatosConsulta::bind);
    ui.register_handler("frmRhSindicatosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhSindicatosConsulta::form_show));
    ui.register_handler("frmRhSindicatosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhSindicatosConsulta::edt_busca_change));
    ui.register_form_class("frmRhSindicatosHist", FrmRhSindicatosHist::bind);
    ui.register_handler("frmRhSindicatosHist.form_show", aylis::prelude::instances::form_handler(FrmRhSindicatosHist::form_show));
    ui.register_handler("frmRhSindicatosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhSindicatosHist::edt_busca_change));
    ui.register_form_class("frmRhBeneficiosCad", FrmRhBeneficiosCad::bind);
    ui.register_handler("frmRhBeneficiosCad.form_show", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::form_show));
    ui.register_handler("frmRhBeneficiosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_novo_click));
    ui.register_handler("frmRhBeneficiosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_salvar_click));
    ui.register_handler("frmRhBeneficiosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_excluir_click));
    ui.register_handler("frmRhBeneficiosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_cancelar_click));
    ui.register_handler("frmRhBeneficiosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_novo_item_click));
    ui.register_handler("frmRhBeneficiosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::btn_fechar_click));
    ui.register_handler("frmRhBeneficiosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBeneficiosCad::edt_busca_change));
    ui.register_form_class("frmRhBeneficiosLista", FrmRhBeneficiosLista::bind);
    ui.register_handler("frmRhBeneficiosLista.form_show", aylis::prelude::instances::form_handler(FrmRhBeneficiosLista::form_show));
    ui.register_handler("frmRhBeneficiosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosLista::btn_abrir_click));
    ui.register_handler("frmRhBeneficiosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBeneficiosLista::edt_busca_change));
    ui.register_handler("frmRhBeneficiosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhBeneficiosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhBeneficiosConsulta", FrmRhBeneficiosConsulta::bind);
    ui.register_handler("frmRhBeneficiosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhBeneficiosConsulta::form_show));
    ui.register_handler("frmRhBeneficiosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBeneficiosConsulta::edt_busca_change));
    ui.register_form_class("frmRhBeneficiosHist", FrmRhBeneficiosHist::bind);
    ui.register_handler("frmRhBeneficiosHist.form_show", aylis::prelude::instances::form_handler(FrmRhBeneficiosHist::form_show));
    ui.register_handler("frmRhBeneficiosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBeneficiosHist::edt_busca_change));
    ui.register_form_class("frmRhPlanosDeSaudeCad", FrmRhPlanosDeSaudeCad::bind);
    ui.register_handler("frmRhPlanosDeSaudeCad.form_show", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::form_show));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_novo_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_salvar_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_excluir_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_cancelar_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_novo_item_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::btn_fechar_click));
    ui.register_handler("frmRhPlanosDeSaudeCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeCad::edt_busca_change));
    ui.register_form_class("frmRhPlanosDeSaudeLista", FrmRhPlanosDeSaudeLista::bind);
    ui.register_handler("frmRhPlanosDeSaudeLista.form_show", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeLista::form_show));
    ui.register_handler("frmRhPlanosDeSaudeLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeLista::btn_abrir_click));
    ui.register_handler("frmRhPlanosDeSaudeLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeLista::edt_busca_change));
    ui.register_handler("frmRhPlanosDeSaudeLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhPlanosDeSaudeConsulta", FrmRhPlanosDeSaudeConsulta::bind);
    ui.register_handler("frmRhPlanosDeSaudeConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeConsulta::form_show));
    ui.register_handler("frmRhPlanosDeSaudeConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeConsulta::edt_busca_change));
    ui.register_form_class("frmRhPlanosDeSaudeHist", FrmRhPlanosDeSaudeHist::bind);
    ui.register_handler("frmRhPlanosDeSaudeHist.form_show", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeHist::form_show));
    ui.register_handler("frmRhPlanosDeSaudeHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPlanosDeSaudeHist::edt_busca_change));
    ui.register_form_class("frmRhEscalasDeTrabalhoCad", FrmRhEscalasDeTrabalhoCad::bind);
    ui.register_handler("frmRhEscalasDeTrabalhoCad.form_show", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::form_show));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_novo_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_salvar_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_excluir_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_cancelar_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_novo_item_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::btn_fechar_click));
    ui.register_handler("frmRhEscalasDeTrabalhoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoCad::edt_busca_change));
    ui.register_form_class("frmRhEscalasDeTrabalhoLista", FrmRhEscalasDeTrabalhoLista::bind);
    ui.register_handler("frmRhEscalasDeTrabalhoLista.form_show", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoLista::form_show));
    ui.register_handler("frmRhEscalasDeTrabalhoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoLista::btn_abrir_click));
    ui.register_handler("frmRhEscalasDeTrabalhoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoLista::edt_busca_change));
    ui.register_handler("frmRhEscalasDeTrabalhoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhEscalasDeTrabalhoConsulta", FrmRhEscalasDeTrabalhoConsulta::bind);
    ui.register_handler("frmRhEscalasDeTrabalhoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoConsulta::form_show));
    ui.register_handler("frmRhEscalasDeTrabalhoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoConsulta::edt_busca_change));
    ui.register_form_class("frmRhEscalasDeTrabalhoHist", FrmRhEscalasDeTrabalhoHist::bind);
    ui.register_handler("frmRhEscalasDeTrabalhoHist.form_show", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoHist::form_show));
    ui.register_handler("frmRhEscalasDeTrabalhoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEscalasDeTrabalhoHist::edt_busca_change));
    ui.register_form_class("frmRhTurnosCad", FrmRhTurnosCad::bind);
    ui.register_handler("frmRhTurnosCad.form_show", aylis::prelude::instances::form_handler(FrmRhTurnosCad::form_show));
    ui.register_handler("frmRhTurnosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_novo_click));
    ui.register_handler("frmRhTurnosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_salvar_click));
    ui.register_handler("frmRhTurnosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_excluir_click));
    ui.register_handler("frmRhTurnosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_cancelar_click));
    ui.register_handler("frmRhTurnosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_novo_item_click));
    ui.register_handler("frmRhTurnosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhTurnosCad::btn_fechar_click));
    ui.register_handler("frmRhTurnosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTurnosCad::edt_busca_change));
    ui.register_form_class("frmRhTurnosLista", FrmRhTurnosLista::bind);
    ui.register_handler("frmRhTurnosLista.form_show", aylis::prelude::instances::form_handler(FrmRhTurnosLista::form_show));
    ui.register_handler("frmRhTurnosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhTurnosLista::btn_abrir_click));
    ui.register_handler("frmRhTurnosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTurnosLista::edt_busca_change));
    ui.register_handler("frmRhTurnosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhTurnosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhTurnosConsulta", FrmRhTurnosConsulta::bind);
    ui.register_handler("frmRhTurnosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhTurnosConsulta::form_show));
    ui.register_handler("frmRhTurnosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTurnosConsulta::edt_busca_change));
    ui.register_form_class("frmRhTurnosHist", FrmRhTurnosHist::bind);
    ui.register_handler("frmRhTurnosHist.form_show", aylis::prelude::instances::form_handler(FrmRhTurnosHist::form_show));
    ui.register_handler("frmRhTurnosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTurnosHist::edt_busca_change));
    ui.register_form_class("frmRhFeriadosCad", FrmRhFeriadosCad::bind);
    ui.register_handler("frmRhFeriadosCad.form_show", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::form_show));
    ui.register_handler("frmRhFeriadosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_novo_click));
    ui.register_handler("frmRhFeriadosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_salvar_click));
    ui.register_handler("frmRhFeriadosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_excluir_click));
    ui.register_handler("frmRhFeriadosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_cancelar_click));
    ui.register_handler("frmRhFeriadosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_novo_item_click));
    ui.register_handler("frmRhFeriadosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::btn_fechar_click));
    ui.register_handler("frmRhFeriadosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriadosCad::edt_busca_change));
    ui.register_form_class("frmRhFeriadosLista", FrmRhFeriadosLista::bind);
    ui.register_handler("frmRhFeriadosLista.form_show", aylis::prelude::instances::form_handler(FrmRhFeriadosLista::form_show));
    ui.register_handler("frmRhFeriadosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhFeriadosLista::btn_abrir_click));
    ui.register_handler("frmRhFeriadosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriadosLista::edt_busca_change));
    ui.register_handler("frmRhFeriadosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhFeriadosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhFeriadosConsulta", FrmRhFeriadosConsulta::bind);
    ui.register_handler("frmRhFeriadosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhFeriadosConsulta::form_show));
    ui.register_handler("frmRhFeriadosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriadosConsulta::edt_busca_change));
    ui.register_form_class("frmRhFeriadosHist", FrmRhFeriadosHist::bind);
    ui.register_handler("frmRhFeriadosHist.form_show", aylis::prelude::instances::form_handler(FrmRhFeriadosHist::form_show));
    ui.register_handler("frmRhFeriadosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriadosHist::edt_busca_change));
    ui.register_form_class("frmRhCursosCad", FrmRhCursosCad::bind);
    ui.register_handler("frmRhCursosCad.form_show", aylis::prelude::instances::form_handler(FrmRhCursosCad::form_show));
    ui.register_handler("frmRhCursosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_novo_click));
    ui.register_handler("frmRhCursosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_salvar_click));
    ui.register_handler("frmRhCursosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_excluir_click));
    ui.register_handler("frmRhCursosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_cancelar_click));
    ui.register_handler("frmRhCursosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_novo_item_click));
    ui.register_handler("frmRhCursosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhCursosCad::btn_fechar_click));
    ui.register_handler("frmRhCursosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCursosCad::edt_busca_change));
    ui.register_form_class("frmRhCursosLista", FrmRhCursosLista::bind);
    ui.register_handler("frmRhCursosLista.form_show", aylis::prelude::instances::form_handler(FrmRhCursosLista::form_show));
    ui.register_handler("frmRhCursosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhCursosLista::btn_abrir_click));
    ui.register_handler("frmRhCursosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCursosLista::edt_busca_change));
    ui.register_handler("frmRhCursosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhCursosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhCursosConsulta", FrmRhCursosConsulta::bind);
    ui.register_handler("frmRhCursosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhCursosConsulta::form_show));
    ui.register_handler("frmRhCursosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCursosConsulta::edt_busca_change));
    ui.register_form_class("frmRhCursosHist", FrmRhCursosHist::bind);
    ui.register_handler("frmRhCursosHist.form_show", aylis::prelude::instances::form_handler(FrmRhCursosHist::form_show));
    ui.register_handler("frmRhCursosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCursosHist::edt_busca_change));
    ui.register_form_class("frmRhInstrutoresCad", FrmRhInstrutoresCad::bind);
    ui.register_handler("frmRhInstrutoresCad.form_show", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::form_show));
    ui.register_handler("frmRhInstrutoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_novo_click));
    ui.register_handler("frmRhInstrutoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_salvar_click));
    ui.register_handler("frmRhInstrutoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_excluir_click));
    ui.register_handler("frmRhInstrutoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_cancelar_click));
    ui.register_handler("frmRhInstrutoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_novo_item_click));
    ui.register_handler("frmRhInstrutoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::btn_fechar_click));
    ui.register_handler("frmRhInstrutoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhInstrutoresCad::edt_busca_change));
    ui.register_form_class("frmRhInstrutoresLista", FrmRhInstrutoresLista::bind);
    ui.register_handler("frmRhInstrutoresLista.form_show", aylis::prelude::instances::form_handler(FrmRhInstrutoresLista::form_show));
    ui.register_handler("frmRhInstrutoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresLista::btn_abrir_click));
    ui.register_handler("frmRhInstrutoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhInstrutoresLista::edt_busca_change));
    ui.register_handler("frmRhInstrutoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhInstrutoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhInstrutoresConsulta", FrmRhInstrutoresConsulta::bind);
    ui.register_handler("frmRhInstrutoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhInstrutoresConsulta::form_show));
    ui.register_handler("frmRhInstrutoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhInstrutoresConsulta::edt_busca_change));
    ui.register_form_class("frmRhInstrutoresHist", FrmRhInstrutoresHist::bind);
    ui.register_handler("frmRhInstrutoresHist.form_show", aylis::prelude::instances::form_handler(FrmRhInstrutoresHist::form_show));
    ui.register_handler("frmRhInstrutoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhInstrutoresHist::edt_busca_change));
    ui.register_form_class("frmRhCompetenciasCad", FrmRhCompetenciasCad::bind);
    ui.register_handler("frmRhCompetenciasCad.form_show", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::form_show));
    ui.register_handler("frmRhCompetenciasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_novo_click));
    ui.register_handler("frmRhCompetenciasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_salvar_click));
    ui.register_handler("frmRhCompetenciasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_excluir_click));
    ui.register_handler("frmRhCompetenciasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_cancelar_click));
    ui.register_handler("frmRhCompetenciasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_novo_item_click));
    ui.register_handler("frmRhCompetenciasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::btn_fechar_click));
    ui.register_handler("frmRhCompetenciasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCompetenciasCad::edt_busca_change));
    ui.register_form_class("frmRhCompetenciasLista", FrmRhCompetenciasLista::bind);
    ui.register_handler("frmRhCompetenciasLista.form_show", aylis::prelude::instances::form_handler(FrmRhCompetenciasLista::form_show));
    ui.register_handler("frmRhCompetenciasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasLista::btn_abrir_click));
    ui.register_handler("frmRhCompetenciasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCompetenciasLista::edt_busca_change));
    ui.register_handler("frmRhCompetenciasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhCompetenciasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhCompetenciasConsulta", FrmRhCompetenciasConsulta::bind);
    ui.register_handler("frmRhCompetenciasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhCompetenciasConsulta::form_show));
    ui.register_handler("frmRhCompetenciasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCompetenciasConsulta::edt_busca_change));
    ui.register_form_class("frmRhCompetenciasHist", FrmRhCompetenciasHist::bind);
    ui.register_handler("frmRhCompetenciasHist.form_show", aylis::prelude::instances::form_handler(FrmRhCompetenciasHist::form_show));
    ui.register_handler("frmRhCompetenciasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhCompetenciasHist::edt_busca_change));
    ui.register_form_class("frmRhNiveisSalariaisCad", FrmRhNiveisSalariaisCad::bind);
    ui.register_handler("frmRhNiveisSalariaisCad.form_show", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::form_show));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_novo_click));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_salvar_click));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_excluir_click));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_cancelar_click));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_novo_item_click));
    ui.register_handler("frmRhNiveisSalariaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::btn_fechar_click));
    ui.register_handler("frmRhNiveisSalariaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisCad::edt_busca_change));
    ui.register_form_class("frmRhNiveisSalariaisLista", FrmRhNiveisSalariaisLista::bind);
    ui.register_handler("frmRhNiveisSalariaisLista.form_show", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisLista::form_show));
    ui.register_handler("frmRhNiveisSalariaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisLista::btn_abrir_click));
    ui.register_handler("frmRhNiveisSalariaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisLista::edt_busca_change));
    ui.register_handler("frmRhNiveisSalariaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhNiveisSalariaisConsulta", FrmRhNiveisSalariaisConsulta::bind);
    ui.register_handler("frmRhNiveisSalariaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisConsulta::form_show));
    ui.register_handler("frmRhNiveisSalariaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisConsulta::edt_busca_change));
    ui.register_form_class("frmRhNiveisSalariaisHist", FrmRhNiveisSalariaisHist::bind);
    ui.register_handler("frmRhNiveisSalariaisHist.form_show", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisHist::form_show));
    ui.register_handler("frmRhNiveisSalariaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhNiveisSalariaisHist::edt_busca_change));
    ui.register_form_class("frmRhEventosDaFolhaCad", FrmRhEventosDaFolhaCad::bind);
    ui.register_handler("frmRhEventosDaFolhaCad.form_show", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::form_show));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_novo_click));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_salvar_click));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_excluir_click));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_cancelar_click));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_novo_item_click));
    ui.register_handler("frmRhEventosDaFolhaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::btn_fechar_click));
    ui.register_handler("frmRhEventosDaFolhaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaCad::edt_busca_change));
    ui.register_form_class("frmRhEventosDaFolhaLista", FrmRhEventosDaFolhaLista::bind);
    ui.register_handler("frmRhEventosDaFolhaLista.form_show", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaLista::form_show));
    ui.register_handler("frmRhEventosDaFolhaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaLista::btn_abrir_click));
    ui.register_handler("frmRhEventosDaFolhaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaLista::edt_busca_change));
    ui.register_handler("frmRhEventosDaFolhaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhEventosDaFolhaConsulta", FrmRhEventosDaFolhaConsulta::bind);
    ui.register_handler("frmRhEventosDaFolhaConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaConsulta::form_show));
    ui.register_handler("frmRhEventosDaFolhaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaConsulta::edt_busca_change));
    ui.register_form_class("frmRhEventosDaFolhaHist", FrmRhEventosDaFolhaHist::bind);
    ui.register_handler("frmRhEventosDaFolhaHist.form_show", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaHist::form_show));
    ui.register_handler("frmRhEventosDaFolhaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEventosDaFolhaHist::edt_busca_change));
    ui.register_form_class("frmRhTiposDeAfastamentoCad", FrmRhTiposDeAfastamentoCad::bind);
    ui.register_handler("frmRhTiposDeAfastamentoCad.form_show", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::form_show));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_novo_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_salvar_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_excluir_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_cancelar_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_novo_item_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::btn_fechar_click));
    ui.register_handler("frmRhTiposDeAfastamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoCad::edt_busca_change));
    ui.register_form_class("frmRhTiposDeAfastamentoLista", FrmRhTiposDeAfastamentoLista::bind);
    ui.register_handler("frmRhTiposDeAfastamentoLista.form_show", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoLista::form_show));
    ui.register_handler("frmRhTiposDeAfastamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoLista::btn_abrir_click));
    ui.register_handler("frmRhTiposDeAfastamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoLista::edt_busca_change));
    ui.register_handler("frmRhTiposDeAfastamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhTiposDeAfastamentoConsulta", FrmRhTiposDeAfastamentoConsulta::bind);
    ui.register_handler("frmRhTiposDeAfastamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoConsulta::form_show));
    ui.register_handler("frmRhTiposDeAfastamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoConsulta::edt_busca_change));
    ui.register_form_class("frmRhTiposDeAfastamentoHist", FrmRhTiposDeAfastamentoHist::bind);
    ui.register_handler("frmRhTiposDeAfastamentoHist.form_show", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoHist::form_show));
    ui.register_handler("frmRhTiposDeAfastamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTiposDeAfastamentoHist::edt_busca_change));
    ui.register_form_class("frmRhMotivosDeDesligamentoCad", FrmRhMotivosDeDesligamentoCad::bind);
    ui.register_handler("frmRhMotivosDeDesligamentoCad.form_show", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::form_show));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_novo_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_salvar_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_excluir_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_cancelar_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_novo_item_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::btn_fechar_click));
    ui.register_handler("frmRhMotivosDeDesligamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoCad::edt_busca_change));
    ui.register_form_class("frmRhMotivosDeDesligamentoLista", FrmRhMotivosDeDesligamentoLista::bind);
    ui.register_handler("frmRhMotivosDeDesligamentoLista.form_show", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoLista::form_show));
    ui.register_handler("frmRhMotivosDeDesligamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoLista::btn_abrir_click));
    ui.register_handler("frmRhMotivosDeDesligamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoLista::edt_busca_change));
    ui.register_handler("frmRhMotivosDeDesligamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhMotivosDeDesligamentoConsulta", FrmRhMotivosDeDesligamentoConsulta::bind);
    ui.register_handler("frmRhMotivosDeDesligamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoConsulta::form_show));
    ui.register_handler("frmRhMotivosDeDesligamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoConsulta::edt_busca_change));
    ui.register_form_class("frmRhMotivosDeDesligamentoHist", FrmRhMotivosDeDesligamentoHist::bind);
    ui.register_handler("frmRhMotivosDeDesligamentoHist.form_show", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoHist::form_show));
    ui.register_handler("frmRhMotivosDeDesligamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhMotivosDeDesligamentoHist::edt_busca_change));
    ui.register_form_class("frmRhBancosDeHorasCad", FrmRhBancosDeHorasCad::bind);
    ui.register_handler("frmRhBancosDeHorasCad.form_show", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::form_show));
    ui.register_handler("frmRhBancosDeHorasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_novo_click));
    ui.register_handler("frmRhBancosDeHorasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_salvar_click));
    ui.register_handler("frmRhBancosDeHorasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_excluir_click));
    ui.register_handler("frmRhBancosDeHorasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_cancelar_click));
    ui.register_handler("frmRhBancosDeHorasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_novo_item_click));
    ui.register_handler("frmRhBancosDeHorasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::btn_fechar_click));
    ui.register_handler("frmRhBancosDeHorasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasCad::edt_busca_change));
    ui.register_form_class("frmRhBancosDeHorasLista", FrmRhBancosDeHorasLista::bind);
    ui.register_handler("frmRhBancosDeHorasLista.form_show", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasLista::form_show));
    ui.register_handler("frmRhBancosDeHorasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasLista::btn_abrir_click));
    ui.register_handler("frmRhBancosDeHorasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasLista::edt_busca_change));
    ui.register_handler("frmRhBancosDeHorasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhBancosDeHorasConsulta", FrmRhBancosDeHorasConsulta::bind);
    ui.register_handler("frmRhBancosDeHorasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasConsulta::form_show));
    ui.register_handler("frmRhBancosDeHorasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasConsulta::edt_busca_change));
    ui.register_form_class("frmRhBancosDeHorasHist", FrmRhBancosDeHorasHist::bind);
    ui.register_handler("frmRhBancosDeHorasHist.form_show", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasHist::form_show));
    ui.register_handler("frmRhBancosDeHorasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhBancosDeHorasHist::edt_busca_change));
    ui.register_form_class("frmRhEpisCad", FrmRhEpisCad::bind);
    ui.register_handler("frmRhEpisCad.form_show", aylis::prelude::instances::form_handler(FrmRhEpisCad::form_show));
    ui.register_handler("frmRhEpisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_novo_click));
    ui.register_handler("frmRhEpisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_salvar_click));
    ui.register_handler("frmRhEpisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_excluir_click));
    ui.register_handler("frmRhEpisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_cancelar_click));
    ui.register_handler("frmRhEpisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_novo_item_click));
    ui.register_handler("frmRhEpisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhEpisCad::btn_fechar_click));
    ui.register_handler("frmRhEpisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEpisCad::edt_busca_change));
    ui.register_form_class("frmRhEpisLista", FrmRhEpisLista::bind);
    ui.register_handler("frmRhEpisLista.form_show", aylis::prelude::instances::form_handler(FrmRhEpisLista::form_show));
    ui.register_handler("frmRhEpisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhEpisLista::btn_abrir_click));
    ui.register_handler("frmRhEpisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEpisLista::edt_busca_change));
    ui.register_handler("frmRhEpisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhEpisLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhEpisConsulta", FrmRhEpisConsulta::bind);
    ui.register_handler("frmRhEpisConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhEpisConsulta::form_show));
    ui.register_handler("frmRhEpisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEpisConsulta::edt_busca_change));
    ui.register_form_class("frmRhEpisHist", FrmRhEpisHist::bind);
    ui.register_handler("frmRhEpisHist.form_show", aylis::prelude::instances::form_handler(FrmRhEpisHist::form_show));
    ui.register_handler("frmRhEpisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEpisHist::edt_busca_change));
    ui.register_form_class("frmRhExamesOcupacionaisCad", FrmRhExamesOcupacionaisCad::bind);
    ui.register_handler("frmRhExamesOcupacionaisCad.form_show", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::form_show));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_novo_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_salvar_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_excluir_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_cancelar_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_novo_item_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::btn_fechar_click));
    ui.register_handler("frmRhExamesOcupacionaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisCad::edt_busca_change));
    ui.register_form_class("frmRhExamesOcupacionaisLista", FrmRhExamesOcupacionaisLista::bind);
    ui.register_handler("frmRhExamesOcupacionaisLista.form_show", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisLista::form_show));
    ui.register_handler("frmRhExamesOcupacionaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisLista::btn_abrir_click));
    ui.register_handler("frmRhExamesOcupacionaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisLista::edt_busca_change));
    ui.register_handler("frmRhExamesOcupacionaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhExamesOcupacionaisConsulta", FrmRhExamesOcupacionaisConsulta::bind);
    ui.register_handler("frmRhExamesOcupacionaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisConsulta::form_show));
    ui.register_handler("frmRhExamesOcupacionaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisConsulta::edt_busca_change));
    ui.register_form_class("frmRhExamesOcupacionaisHist", FrmRhExamesOcupacionaisHist::bind);
    ui.register_handler("frmRhExamesOcupacionaisHist.form_show", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisHist::form_show));
    ui.register_handler("frmRhExamesOcupacionaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesOcupacionaisHist::edt_busca_change));
    ui.register_form_class("frmRhVagasCad", FrmRhVagasCad::bind);
    ui.register_handler("frmRhVagasCad.form_show", aylis::prelude::instances::form_handler(FrmRhVagasCad::form_show));
    ui.register_handler("frmRhVagasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_novo_click));
    ui.register_handler("frmRhVagasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_salvar_click));
    ui.register_handler("frmRhVagasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_excluir_click));
    ui.register_handler("frmRhVagasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_cancelar_click));
    ui.register_handler("frmRhVagasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_novo_item_click));
    ui.register_handler("frmRhVagasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhVagasCad::btn_fechar_click));
    ui.register_handler("frmRhVagasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhVagasCad::edt_busca_change));
    ui.register_form_class("frmRhVagasLista", FrmRhVagasLista::bind);
    ui.register_handler("frmRhVagasLista.form_show", aylis::prelude::instances::form_handler(FrmRhVagasLista::form_show));
    ui.register_handler("frmRhVagasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhVagasLista::btn_abrir_click));
    ui.register_handler("frmRhVagasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhVagasLista::edt_busca_change));
    ui.register_handler("frmRhVagasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhVagasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhVagasConsulta", FrmRhVagasConsulta::bind);
    ui.register_handler("frmRhVagasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhVagasConsulta::form_show));
    ui.register_handler("frmRhVagasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhVagasConsulta::edt_busca_change));
    ui.register_form_class("frmRhVagasHist", FrmRhVagasHist::bind);
    ui.register_handler("frmRhVagasHist.form_show", aylis::prelude::instances::form_handler(FrmRhVagasHist::form_show));
    ui.register_handler("frmRhVagasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhVagasHist::edt_busca_change));
    ui.register_form_class("frmRhAdmissoesCad", FrmRhAdmissoesCad::bind);
    ui.register_handler("frmRhAdmissoesCad.form_show", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::form_show));
    ui.register_handler("frmRhAdmissoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_novo_click));
    ui.register_handler("frmRhAdmissoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_salvar_click));
    ui.register_handler("frmRhAdmissoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_excluir_click));
    ui.register_handler("frmRhAdmissoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_cancelar_click));
    ui.register_handler("frmRhAdmissoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_novo_item_click));
    ui.register_handler("frmRhAdmissoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::btn_fechar_click));
    ui.register_handler("frmRhAdmissoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdmissoesCad::edt_busca_change));
    ui.register_form_class("frmRhAdmissoesLista", FrmRhAdmissoesLista::bind);
    ui.register_handler("frmRhAdmissoesLista.form_show", aylis::prelude::instances::form_handler(FrmRhAdmissoesLista::form_show));
    ui.register_handler("frmRhAdmissoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesLista::btn_abrir_click));
    ui.register_handler("frmRhAdmissoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdmissoesLista::edt_busca_change));
    ui.register_handler("frmRhAdmissoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhAdmissoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhAdmissoesConsulta", FrmRhAdmissoesConsulta::bind);
    ui.register_handler("frmRhAdmissoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhAdmissoesConsulta::form_show));
    ui.register_handler("frmRhAdmissoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdmissoesConsulta::edt_busca_change));
    ui.register_form_class("frmRhAdmissoesHist", FrmRhAdmissoesHist::bind);
    ui.register_handler("frmRhAdmissoesHist.form_show", aylis::prelude::instances::form_handler(FrmRhAdmissoesHist::form_show));
    ui.register_handler("frmRhAdmissoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdmissoesHist::edt_busca_change));
    ui.register_form_class("frmRhDesligamentosCad", FrmRhDesligamentosCad::bind);
    ui.register_handler("frmRhDesligamentosCad.form_show", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::form_show));
    ui.register_handler("frmRhDesligamentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_novo_click));
    ui.register_handler("frmRhDesligamentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_salvar_click));
    ui.register_handler("frmRhDesligamentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_excluir_click));
    ui.register_handler("frmRhDesligamentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_cancelar_click));
    ui.register_handler("frmRhDesligamentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_novo_item_click));
    ui.register_handler("frmRhDesligamentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::btn_fechar_click));
    ui.register_handler("frmRhDesligamentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDesligamentosCad::edt_busca_change));
    ui.register_form_class("frmRhDesligamentosLista", FrmRhDesligamentosLista::bind);
    ui.register_handler("frmRhDesligamentosLista.form_show", aylis::prelude::instances::form_handler(FrmRhDesligamentosLista::form_show));
    ui.register_handler("frmRhDesligamentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosLista::btn_abrir_click));
    ui.register_handler("frmRhDesligamentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDesligamentosLista::edt_busca_change));
    ui.register_handler("frmRhDesligamentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhDesligamentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhDesligamentosConsulta", FrmRhDesligamentosConsulta::bind);
    ui.register_handler("frmRhDesligamentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhDesligamentosConsulta::form_show));
    ui.register_handler("frmRhDesligamentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDesligamentosConsulta::edt_busca_change));
    ui.register_form_class("frmRhDesligamentosHist", FrmRhDesligamentosHist::bind);
    ui.register_handler("frmRhDesligamentosHist.form_show", aylis::prelude::instances::form_handler(FrmRhDesligamentosHist::form_show));
    ui.register_handler("frmRhDesligamentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDesligamentosHist::edt_busca_change));
    ui.register_form_class("frmRhFeriasCad", FrmRhFeriasCad::bind);
    ui.register_handler("frmRhFeriasCad.form_show", aylis::prelude::instances::form_handler(FrmRhFeriasCad::form_show));
    ui.register_handler("frmRhFeriasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_novo_click));
    ui.register_handler("frmRhFeriasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_salvar_click));
    ui.register_handler("frmRhFeriasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_excluir_click));
    ui.register_handler("frmRhFeriasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_cancelar_click));
    ui.register_handler("frmRhFeriasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_novo_item_click));
    ui.register_handler("frmRhFeriasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhFeriasCad::btn_fechar_click));
    ui.register_handler("frmRhFeriasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriasCad::edt_busca_change));
    ui.register_form_class("frmRhFeriasLista", FrmRhFeriasLista::bind);
    ui.register_handler("frmRhFeriasLista.form_show", aylis::prelude::instances::form_handler(FrmRhFeriasLista::form_show));
    ui.register_handler("frmRhFeriasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhFeriasLista::btn_abrir_click));
    ui.register_handler("frmRhFeriasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriasLista::edt_busca_change));
    ui.register_handler("frmRhFeriasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhFeriasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhFeriasConsulta", FrmRhFeriasConsulta::bind);
    ui.register_handler("frmRhFeriasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhFeriasConsulta::form_show));
    ui.register_handler("frmRhFeriasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriasConsulta::edt_busca_change));
    ui.register_form_class("frmRhFeriasHist", FrmRhFeriasHist::bind);
    ui.register_handler("frmRhFeriasHist.form_show", aylis::prelude::instances::form_handler(FrmRhFeriasHist::form_show));
    ui.register_handler("frmRhFeriasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFeriasHist::edt_busca_change));
    ui.register_form_class("frmRhFolhasDePagamentoCad", FrmRhFolhasDePagamentoCad::bind);
    ui.register_handler("frmRhFolhasDePagamentoCad.form_show", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::form_show));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_novo_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_salvar_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_excluir_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_cancelar_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_novo_item_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::btn_fechar_click));
    ui.register_handler("frmRhFolhasDePagamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoCad::edt_busca_change));
    ui.register_form_class("frmRhFolhasDePagamentoLista", FrmRhFolhasDePagamentoLista::bind);
    ui.register_handler("frmRhFolhasDePagamentoLista.form_show", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoLista::form_show));
    ui.register_handler("frmRhFolhasDePagamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoLista::btn_abrir_click));
    ui.register_handler("frmRhFolhasDePagamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoLista::edt_busca_change));
    ui.register_handler("frmRhFolhasDePagamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhFolhasDePagamentoConsulta", FrmRhFolhasDePagamentoConsulta::bind);
    ui.register_handler("frmRhFolhasDePagamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoConsulta::form_show));
    ui.register_handler("frmRhFolhasDePagamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoConsulta::edt_busca_change));
    ui.register_form_class("frmRhFolhasDePagamentoHist", FrmRhFolhasDePagamentoHist::bind);
    ui.register_handler("frmRhFolhasDePagamentoHist.form_show", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoHist::form_show));
    ui.register_handler("frmRhFolhasDePagamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhFolhasDePagamentoHist::edt_busca_change));
    ui.register_form_class("frmRhAdiantamentosSalariaisCad", FrmRhAdiantamentosSalariaisCad::bind);
    ui.register_handler("frmRhAdiantamentosSalariaisCad.form_show", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::form_show));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_novo_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_salvar_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_excluir_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_cancelar_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_novo_item_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::btn_fechar_click));
    ui.register_handler("frmRhAdiantamentosSalariaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisCad::edt_busca_change));
    ui.register_form_class("frmRhAdiantamentosSalariaisLista", FrmRhAdiantamentosSalariaisLista::bind);
    ui.register_handler("frmRhAdiantamentosSalariaisLista.form_show", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisLista::form_show));
    ui.register_handler("frmRhAdiantamentosSalariaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisLista::btn_abrir_click));
    ui.register_handler("frmRhAdiantamentosSalariaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisLista::edt_busca_change));
    ui.register_handler("frmRhAdiantamentosSalariaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhAdiantamentosSalariaisConsulta", FrmRhAdiantamentosSalariaisConsulta::bind);
    ui.register_handler("frmRhAdiantamentosSalariaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisConsulta::form_show));
    ui.register_handler("frmRhAdiantamentosSalariaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisConsulta::edt_busca_change));
    ui.register_form_class("frmRhAdiantamentosSalariaisHist", FrmRhAdiantamentosSalariaisHist::bind);
    ui.register_handler("frmRhAdiantamentosSalariaisHist.form_show", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisHist::form_show));
    ui.register_handler("frmRhAdiantamentosSalariaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdiantamentosSalariaisHist::edt_busca_change));
    ui.register_form_class("frmRhDecimoTerceiroCad", FrmRhDecimoTerceiroCad::bind);
    ui.register_handler("frmRhDecimoTerceiroCad.form_show", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::form_show));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_novo_click));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_salvar_click));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_excluir_click));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_cancelar_click));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_novo_item_click));
    ui.register_handler("frmRhDecimoTerceiroCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::btn_fechar_click));
    ui.register_handler("frmRhDecimoTerceiroCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroCad::edt_busca_change));
    ui.register_form_class("frmRhDecimoTerceiroLista", FrmRhDecimoTerceiroLista::bind);
    ui.register_handler("frmRhDecimoTerceiroLista.form_show", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroLista::form_show));
    ui.register_handler("frmRhDecimoTerceiroLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroLista::btn_abrir_click));
    ui.register_handler("frmRhDecimoTerceiroLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroLista::edt_busca_change));
    ui.register_handler("frmRhDecimoTerceiroLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhDecimoTerceiroConsulta", FrmRhDecimoTerceiroConsulta::bind);
    ui.register_handler("frmRhDecimoTerceiroConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroConsulta::form_show));
    ui.register_handler("frmRhDecimoTerceiroConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroConsulta::edt_busca_change));
    ui.register_form_class("frmRhDecimoTerceiroHist", FrmRhDecimoTerceiroHist::bind);
    ui.register_handler("frmRhDecimoTerceiroHist.form_show", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroHist::form_show));
    ui.register_handler("frmRhDecimoTerceiroHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhDecimoTerceiroHist::edt_busca_change));
    ui.register_form_class("frmRhRescisoesCad", FrmRhRescisoesCad::bind);
    ui.register_handler("frmRhRescisoesCad.form_show", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::form_show));
    ui.register_handler("frmRhRescisoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_novo_click));
    ui.register_handler("frmRhRescisoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_salvar_click));
    ui.register_handler("frmRhRescisoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_excluir_click));
    ui.register_handler("frmRhRescisoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_cancelar_click));
    ui.register_handler("frmRhRescisoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_novo_item_click));
    ui.register_handler("frmRhRescisoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::btn_fechar_click));
    ui.register_handler("frmRhRescisoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRescisoesCad::edt_busca_change));
    ui.register_form_class("frmRhRescisoesLista", FrmRhRescisoesLista::bind);
    ui.register_handler("frmRhRescisoesLista.form_show", aylis::prelude::instances::form_handler(FrmRhRescisoesLista::form_show));
    ui.register_handler("frmRhRescisoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhRescisoesLista::btn_abrir_click));
    ui.register_handler("frmRhRescisoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRescisoesLista::edt_busca_change));
    ui.register_handler("frmRhRescisoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhRescisoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhRescisoesConsulta", FrmRhRescisoesConsulta::bind);
    ui.register_handler("frmRhRescisoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhRescisoesConsulta::form_show));
    ui.register_handler("frmRhRescisoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRescisoesConsulta::edt_busca_change));
    ui.register_form_class("frmRhRescisoesHist", FrmRhRescisoesHist::bind);
    ui.register_handler("frmRhRescisoesHist.form_show", aylis::prelude::instances::form_handler(FrmRhRescisoesHist::form_show));
    ui.register_handler("frmRhRescisoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRescisoesHist::edt_busca_change));
    ui.register_form_class("frmRhAfastamentosCad", FrmRhAfastamentosCad::bind);
    ui.register_handler("frmRhAfastamentosCad.form_show", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::form_show));
    ui.register_handler("frmRhAfastamentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_novo_click));
    ui.register_handler("frmRhAfastamentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_salvar_click));
    ui.register_handler("frmRhAfastamentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_excluir_click));
    ui.register_handler("frmRhAfastamentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_cancelar_click));
    ui.register_handler("frmRhAfastamentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_novo_item_click));
    ui.register_handler("frmRhAfastamentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::btn_fechar_click));
    ui.register_handler("frmRhAfastamentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAfastamentosCad::edt_busca_change));
    ui.register_form_class("frmRhAfastamentosLista", FrmRhAfastamentosLista::bind);
    ui.register_handler("frmRhAfastamentosLista.form_show", aylis::prelude::instances::form_handler(FrmRhAfastamentosLista::form_show));
    ui.register_handler("frmRhAfastamentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosLista::btn_abrir_click));
    ui.register_handler("frmRhAfastamentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAfastamentosLista::edt_busca_change));
    ui.register_handler("frmRhAfastamentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhAfastamentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhAfastamentosConsulta", FrmRhAfastamentosConsulta::bind);
    ui.register_handler("frmRhAfastamentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhAfastamentosConsulta::form_show));
    ui.register_handler("frmRhAfastamentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAfastamentosConsulta::edt_busca_change));
    ui.register_form_class("frmRhAfastamentosHist", FrmRhAfastamentosHist::bind);
    ui.register_handler("frmRhAfastamentosHist.form_show", aylis::prelude::instances::form_handler(FrmRhAfastamentosHist::form_show));
    ui.register_handler("frmRhAfastamentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAfastamentosHist::edt_busca_change));
    ui.register_form_class("frmRhAvaliacoesDeDesempenhoCad", FrmRhAvaliacoesDeDesempenhoCad::bind);
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.form_show", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::form_show));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_novo_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_salvar_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_excluir_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_cancelar_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_novo_item_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::btn_fechar_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoCad::edt_busca_change));
    ui.register_form_class("frmRhAvaliacoesDeDesempenhoLista", FrmRhAvaliacoesDeDesempenhoLista::bind);
    ui.register_handler("frmRhAvaliacoesDeDesempenhoLista.form_show", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoLista::form_show));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoLista::btn_abrir_click));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoLista::edt_busca_change));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhAvaliacoesDeDesempenhoConsulta", FrmRhAvaliacoesDeDesempenhoConsulta::bind);
    ui.register_handler("frmRhAvaliacoesDeDesempenhoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoConsulta::form_show));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoConsulta::edt_busca_change));
    ui.register_form_class("frmRhAvaliacoesDeDesempenhoHist", FrmRhAvaliacoesDeDesempenhoHist::bind);
    ui.register_handler("frmRhAvaliacoesDeDesempenhoHist.form_show", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoHist::form_show));
    ui.register_handler("frmRhAvaliacoesDeDesempenhoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAvaliacoesDeDesempenhoHist::edt_busca_change));
    ui.register_form_class("frmRhTreinamentosRealizadosCad", FrmRhTreinamentosRealizadosCad::bind);
    ui.register_handler("frmRhTreinamentosRealizadosCad.form_show", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::form_show));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_novo_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_salvar_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_excluir_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_cancelar_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_novo_item_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::btn_fechar_click));
    ui.register_handler("frmRhTreinamentosRealizadosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosCad::edt_busca_change));
    ui.register_form_class("frmRhTreinamentosRealizadosLista", FrmRhTreinamentosRealizadosLista::bind);
    ui.register_handler("frmRhTreinamentosRealizadosLista.form_show", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosLista::form_show));
    ui.register_handler("frmRhTreinamentosRealizadosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosLista::btn_abrir_click));
    ui.register_handler("frmRhTreinamentosRealizadosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosLista::edt_busca_change));
    ui.register_handler("frmRhTreinamentosRealizadosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhTreinamentosRealizadosConsulta", FrmRhTreinamentosRealizadosConsulta::bind);
    ui.register_handler("frmRhTreinamentosRealizadosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosConsulta::form_show));
    ui.register_handler("frmRhTreinamentosRealizadosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosConsulta::edt_busca_change));
    ui.register_form_class("frmRhTreinamentosRealizadosHist", FrmRhTreinamentosRealizadosHist::bind);
    ui.register_handler("frmRhTreinamentosRealizadosHist.form_show", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosHist::form_show));
    ui.register_handler("frmRhTreinamentosRealizadosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTreinamentosRealizadosHist::edt_busca_change));
    ui.register_form_class("frmRhRequisicoesDePessoalCad", FrmRhRequisicoesDePessoalCad::bind);
    ui.register_handler("frmRhRequisicoesDePessoalCad.form_show", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::form_show));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_novo_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_salvar_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_excluir_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_cancelar_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_novo_item_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::btn_fechar_click));
    ui.register_handler("frmRhRequisicoesDePessoalCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalCad::edt_busca_change));
    ui.register_form_class("frmRhRequisicoesDePessoalLista", FrmRhRequisicoesDePessoalLista::bind);
    ui.register_handler("frmRhRequisicoesDePessoalLista.form_show", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalLista::form_show));
    ui.register_handler("frmRhRequisicoesDePessoalLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalLista::btn_abrir_click));
    ui.register_handler("frmRhRequisicoesDePessoalLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalLista::edt_busca_change));
    ui.register_handler("frmRhRequisicoesDePessoalLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhRequisicoesDePessoalConsulta", FrmRhRequisicoesDePessoalConsulta::bind);
    ui.register_handler("frmRhRequisicoesDePessoalConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalConsulta::form_show));
    ui.register_handler("frmRhRequisicoesDePessoalConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalConsulta::edt_busca_change));
    ui.register_form_class("frmRhRequisicoesDePessoalHist", FrmRhRequisicoesDePessoalHist::bind);
    ui.register_handler("frmRhRequisicoesDePessoalHist.form_show", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalHist::form_show));
    ui.register_handler("frmRhRequisicoesDePessoalHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRequisicoesDePessoalHist::edt_busca_change));
    ui.register_form_class("frmRhEntregasDeEpiCad", FrmRhEntregasDeEpiCad::bind);
    ui.register_handler("frmRhEntregasDeEpiCad.form_show", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::form_show));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_novo_click));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_salvar_click));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_excluir_click));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_cancelar_click));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_novo_item_click));
    ui.register_handler("frmRhEntregasDeEpiCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::btn_fechar_click));
    ui.register_handler("frmRhEntregasDeEpiCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiCad::edt_busca_change));
    ui.register_form_class("frmRhEntregasDeEpiLista", FrmRhEntregasDeEpiLista::bind);
    ui.register_handler("frmRhEntregasDeEpiLista.form_show", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiLista::form_show));
    ui.register_handler("frmRhEntregasDeEpiLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiLista::btn_abrir_click));
    ui.register_handler("frmRhEntregasDeEpiLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiLista::edt_busca_change));
    ui.register_handler("frmRhEntregasDeEpiLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhEntregasDeEpiConsulta", FrmRhEntregasDeEpiConsulta::bind);
    ui.register_handler("frmRhEntregasDeEpiConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiConsulta::form_show));
    ui.register_handler("frmRhEntregasDeEpiConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiConsulta::edt_busca_change));
    ui.register_form_class("frmRhEntregasDeEpiHist", FrmRhEntregasDeEpiHist::bind);
    ui.register_handler("frmRhEntregasDeEpiHist.form_show", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiHist::form_show));
    ui.register_handler("frmRhEntregasDeEpiHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntregasDeEpiHist::edt_busca_change));
    ui.register_form_class("frmRhExamesRealizadosCad", FrmRhExamesRealizadosCad::bind);
    ui.register_handler("frmRhExamesRealizadosCad.form_show", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::form_show));
    ui.register_handler("frmRhExamesRealizadosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_novo_click));
    ui.register_handler("frmRhExamesRealizadosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_salvar_click));
    ui.register_handler("frmRhExamesRealizadosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_excluir_click));
    ui.register_handler("frmRhExamesRealizadosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_cancelar_click));
    ui.register_handler("frmRhExamesRealizadosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_novo_item_click));
    ui.register_handler("frmRhExamesRealizadosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::btn_fechar_click));
    ui.register_handler("frmRhExamesRealizadosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosCad::edt_busca_change));
    ui.register_form_class("frmRhExamesRealizadosLista", FrmRhExamesRealizadosLista::bind);
    ui.register_handler("frmRhExamesRealizadosLista.form_show", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosLista::form_show));
    ui.register_handler("frmRhExamesRealizadosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosLista::btn_abrir_click));
    ui.register_handler("frmRhExamesRealizadosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosLista::edt_busca_change));
    ui.register_handler("frmRhExamesRealizadosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhExamesRealizadosConsulta", FrmRhExamesRealizadosConsulta::bind);
    ui.register_handler("frmRhExamesRealizadosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosConsulta::form_show));
    ui.register_handler("frmRhExamesRealizadosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosConsulta::edt_busca_change));
    ui.register_form_class("frmRhExamesRealizadosHist", FrmRhExamesRealizadosHist::bind);
    ui.register_handler("frmRhExamesRealizadosHist.form_show", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosHist::form_show));
    ui.register_handler("frmRhExamesRealizadosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhExamesRealizadosHist::edt_busca_change));
    ui.register_form_class("frmRhLancamentosDePontoCad", FrmRhLancamentosDePontoCad::bind);
    ui.register_handler("frmRhLancamentosDePontoCad.form_show", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::form_show));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_novo_click));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_salvar_click));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_excluir_click));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_cancelar_click));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_novo_item_click));
    ui.register_handler("frmRhLancamentosDePontoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::btn_fechar_click));
    ui.register_handler("frmRhLancamentosDePontoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoCad::edt_busca_change));
    ui.register_form_class("frmRhLancamentosDePontoLista", FrmRhLancamentosDePontoLista::bind);
    ui.register_handler("frmRhLancamentosDePontoLista.form_show", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoLista::form_show));
    ui.register_handler("frmRhLancamentosDePontoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoLista::btn_abrir_click));
    ui.register_handler("frmRhLancamentosDePontoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoLista::edt_busca_change));
    ui.register_handler("frmRhLancamentosDePontoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhLancamentosDePontoConsulta", FrmRhLancamentosDePontoConsulta::bind);
    ui.register_handler("frmRhLancamentosDePontoConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoConsulta::form_show));
    ui.register_handler("frmRhLancamentosDePontoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoConsulta::edt_busca_change));
    ui.register_form_class("frmRhLancamentosDePontoHist", FrmRhLancamentosDePontoHist::bind);
    ui.register_handler("frmRhLancamentosDePontoHist.form_show", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoHist::form_show));
    ui.register_handler("frmRhLancamentosDePontoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhLancamentosDePontoHist::edt_busca_change));
    ui.register_form_class("frmRhHorasExtrasCad", FrmRhHorasExtrasCad::bind);
    ui.register_handler("frmRhHorasExtrasCad.form_show", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::form_show));
    ui.register_handler("frmRhHorasExtrasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_novo_click));
    ui.register_handler("frmRhHorasExtrasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_salvar_click));
    ui.register_handler("frmRhHorasExtrasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_excluir_click));
    ui.register_handler("frmRhHorasExtrasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_cancelar_click));
    ui.register_handler("frmRhHorasExtrasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_novo_item_click));
    ui.register_handler("frmRhHorasExtrasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::btn_fechar_click));
    ui.register_handler("frmRhHorasExtrasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhHorasExtrasCad::edt_busca_change));
    ui.register_form_class("frmRhHorasExtrasLista", FrmRhHorasExtrasLista::bind);
    ui.register_handler("frmRhHorasExtrasLista.form_show", aylis::prelude::instances::form_handler(FrmRhHorasExtrasLista::form_show));
    ui.register_handler("frmRhHorasExtrasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasLista::btn_abrir_click));
    ui.register_handler("frmRhHorasExtrasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhHorasExtrasLista::edt_busca_change));
    ui.register_handler("frmRhHorasExtrasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhHorasExtrasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhHorasExtrasConsulta", FrmRhHorasExtrasConsulta::bind);
    ui.register_handler("frmRhHorasExtrasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhHorasExtrasConsulta::form_show));
    ui.register_handler("frmRhHorasExtrasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhHorasExtrasConsulta::edt_busca_change));
    ui.register_form_class("frmRhHorasExtrasHist", FrmRhHorasExtrasHist::bind);
    ui.register_handler("frmRhHorasExtrasHist.form_show", aylis::prelude::instances::form_handler(FrmRhHorasExtrasHist::form_show));
    ui.register_handler("frmRhHorasExtrasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhHorasExtrasHist::edt_busca_change));
    ui.register_form_class("frmRhPromocoesCad", FrmRhPromocoesCad::bind);
    ui.register_handler("frmRhPromocoesCad.form_show", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::form_show));
    ui.register_handler("frmRhPromocoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_novo_click));
    ui.register_handler("frmRhPromocoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_salvar_click));
    ui.register_handler("frmRhPromocoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_excluir_click));
    ui.register_handler("frmRhPromocoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_cancelar_click));
    ui.register_handler("frmRhPromocoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_novo_item_click));
    ui.register_handler("frmRhPromocoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::btn_fechar_click));
    ui.register_handler("frmRhPromocoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPromocoesCad::edt_busca_change));
    ui.register_form_class("frmRhPromocoesLista", FrmRhPromocoesLista::bind);
    ui.register_handler("frmRhPromocoesLista.form_show", aylis::prelude::instances::form_handler(FrmRhPromocoesLista::form_show));
    ui.register_handler("frmRhPromocoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhPromocoesLista::btn_abrir_click));
    ui.register_handler("frmRhPromocoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPromocoesLista::edt_busca_change));
    ui.register_handler("frmRhPromocoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhPromocoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhPromocoesConsulta", FrmRhPromocoesConsulta::bind);
    ui.register_handler("frmRhPromocoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhPromocoesConsulta::form_show));
    ui.register_handler("frmRhPromocoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPromocoesConsulta::edt_busca_change));
    ui.register_form_class("frmRhPromocoesHist", FrmRhPromocoesHist::bind);
    ui.register_handler("frmRhPromocoesHist.form_show", aylis::prelude::instances::form_handler(FrmRhPromocoesHist::form_show));
    ui.register_handler("frmRhPromocoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhPromocoesHist::edt_busca_change));
    ui.register_form_class("frmRhTransferenciasCad", FrmRhTransferenciasCad::bind);
    ui.register_handler("frmRhTransferenciasCad.form_show", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::form_show));
    ui.register_handler("frmRhTransferenciasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_novo_click));
    ui.register_handler("frmRhTransferenciasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_salvar_click));
    ui.register_handler("frmRhTransferenciasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_excluir_click));
    ui.register_handler("frmRhTransferenciasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_cancelar_click));
    ui.register_handler("frmRhTransferenciasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_novo_item_click));
    ui.register_handler("frmRhTransferenciasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::btn_fechar_click));
    ui.register_handler("frmRhTransferenciasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTransferenciasCad::edt_busca_change));
    ui.register_form_class("frmRhTransferenciasLista", FrmRhTransferenciasLista::bind);
    ui.register_handler("frmRhTransferenciasLista.form_show", aylis::prelude::instances::form_handler(FrmRhTransferenciasLista::form_show));
    ui.register_handler("frmRhTransferenciasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasLista::btn_abrir_click));
    ui.register_handler("frmRhTransferenciasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTransferenciasLista::edt_busca_change));
    ui.register_handler("frmRhTransferenciasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhTransferenciasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhTransferenciasConsulta", FrmRhTransferenciasConsulta::bind);
    ui.register_handler("frmRhTransferenciasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhTransferenciasConsulta::form_show));
    ui.register_handler("frmRhTransferenciasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTransferenciasConsulta::edt_busca_change));
    ui.register_form_class("frmRhTransferenciasHist", FrmRhTransferenciasHist::bind);
    ui.register_handler("frmRhTransferenciasHist.form_show", aylis::prelude::instances::form_handler(FrmRhTransferenciasHist::form_show));
    ui.register_handler("frmRhTransferenciasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhTransferenciasHist::edt_busca_change));
    ui.register_form_class("frmRhAdvertenciasCad", FrmRhAdvertenciasCad::bind);
    ui.register_handler("frmRhAdvertenciasCad.form_show", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::form_show));
    ui.register_handler("frmRhAdvertenciasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_novo_click));
    ui.register_handler("frmRhAdvertenciasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_salvar_click));
    ui.register_handler("frmRhAdvertenciasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_excluir_click));
    ui.register_handler("frmRhAdvertenciasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_cancelar_click));
    ui.register_handler("frmRhAdvertenciasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_novo_item_click));
    ui.register_handler("frmRhAdvertenciasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::btn_fechar_click));
    ui.register_handler("frmRhAdvertenciasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdvertenciasCad::edt_busca_change));
    ui.register_form_class("frmRhAdvertenciasLista", FrmRhAdvertenciasLista::bind);
    ui.register_handler("frmRhAdvertenciasLista.form_show", aylis::prelude::instances::form_handler(FrmRhAdvertenciasLista::form_show));
    ui.register_handler("frmRhAdvertenciasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasLista::btn_abrir_click));
    ui.register_handler("frmRhAdvertenciasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdvertenciasLista::edt_busca_change));
    ui.register_handler("frmRhAdvertenciasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhAdvertenciasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhAdvertenciasConsulta", FrmRhAdvertenciasConsulta::bind);
    ui.register_handler("frmRhAdvertenciasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhAdvertenciasConsulta::form_show));
    ui.register_handler("frmRhAdvertenciasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdvertenciasConsulta::edt_busca_change));
    ui.register_form_class("frmRhAdvertenciasHist", FrmRhAdvertenciasHist::bind);
    ui.register_handler("frmRhAdvertenciasHist.form_show", aylis::prelude::instances::form_handler(FrmRhAdvertenciasHist::form_show));
    ui.register_handler("frmRhAdvertenciasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhAdvertenciasHist::edt_busca_change));
    ui.register_form_class("frmRhRecrutamentosCad", FrmRhRecrutamentosCad::bind);
    ui.register_handler("frmRhRecrutamentosCad.form_show", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::form_show));
    ui.register_handler("frmRhRecrutamentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_novo_click));
    ui.register_handler("frmRhRecrutamentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_salvar_click));
    ui.register_handler("frmRhRecrutamentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_excluir_click));
    ui.register_handler("frmRhRecrutamentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_cancelar_click));
    ui.register_handler("frmRhRecrutamentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_novo_item_click));
    ui.register_handler("frmRhRecrutamentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::btn_fechar_click));
    ui.register_handler("frmRhRecrutamentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRecrutamentosCad::edt_busca_change));
    ui.register_form_class("frmRhRecrutamentosLista", FrmRhRecrutamentosLista::bind);
    ui.register_handler("frmRhRecrutamentosLista.form_show", aylis::prelude::instances::form_handler(FrmRhRecrutamentosLista::form_show));
    ui.register_handler("frmRhRecrutamentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosLista::btn_abrir_click));
    ui.register_handler("frmRhRecrutamentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRecrutamentosLista::edt_busca_change));
    ui.register_handler("frmRhRecrutamentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhRecrutamentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhRecrutamentosConsulta", FrmRhRecrutamentosConsulta::bind);
    ui.register_handler("frmRhRecrutamentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhRecrutamentosConsulta::form_show));
    ui.register_handler("frmRhRecrutamentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRecrutamentosConsulta::edt_busca_change));
    ui.register_form_class("frmRhRecrutamentosHist", FrmRhRecrutamentosHist::bind);
    ui.register_handler("frmRhRecrutamentosHist.form_show", aylis::prelude::instances::form_handler(FrmRhRecrutamentosHist::form_show));
    ui.register_handler("frmRhRecrutamentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhRecrutamentosHist::edt_busca_change));
    ui.register_form_class("frmRhEntrevistasCad", FrmRhEntrevistasCad::bind);
    ui.register_handler("frmRhEntrevistasCad.form_show", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::form_show));
    ui.register_handler("frmRhEntrevistasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_novo_click));
    ui.register_handler("frmRhEntrevistasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_salvar_click));
    ui.register_handler("frmRhEntrevistasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_excluir_click));
    ui.register_handler("frmRhEntrevistasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_cancelar_click));
    ui.register_handler("frmRhEntrevistasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_novo_item_click));
    ui.register_handler("frmRhEntrevistasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::btn_fechar_click));
    ui.register_handler("frmRhEntrevistasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntrevistasCad::edt_busca_change));
    ui.register_form_class("frmRhEntrevistasLista", FrmRhEntrevistasLista::bind);
    ui.register_handler("frmRhEntrevistasLista.form_show", aylis::prelude::instances::form_handler(FrmRhEntrevistasLista::form_show));
    ui.register_handler("frmRhEntrevistasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasLista::btn_abrir_click));
    ui.register_handler("frmRhEntrevistasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntrevistasLista::edt_busca_change));
    ui.register_handler("frmRhEntrevistasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhEntrevistasLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhEntrevistasConsulta", FrmRhEntrevistasConsulta::bind);
    ui.register_handler("frmRhEntrevistasConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhEntrevistasConsulta::form_show));
    ui.register_handler("frmRhEntrevistasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntrevistasConsulta::edt_busca_change));
    ui.register_form_class("frmRhEntrevistasHist", FrmRhEntrevistasHist::bind);
    ui.register_handler("frmRhEntrevistasHist.form_show", aylis::prelude::instances::form_handler(FrmRhEntrevistasHist::form_show));
    ui.register_handler("frmRhEntrevistasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhEntrevistasHist::edt_busca_change));
    ui.register_form_class("frmRhReembolsosCad", FrmRhReembolsosCad::bind);
    ui.register_handler("frmRhReembolsosCad.form_show", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::form_show));
    ui.register_handler("frmRhReembolsosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_novo_click));
    ui.register_handler("frmRhReembolsosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_salvar_click));
    ui.register_handler("frmRhReembolsosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_excluir_click));
    ui.register_handler("frmRhReembolsosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_cancelar_click));
    ui.register_handler("frmRhReembolsosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_novo_item_click));
    ui.register_handler("frmRhReembolsosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::btn_fechar_click));
    ui.register_handler("frmRhReembolsosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhReembolsosCad::edt_busca_change));
    ui.register_form_class("frmRhReembolsosLista", FrmRhReembolsosLista::bind);
    ui.register_handler("frmRhReembolsosLista.form_show", aylis::prelude::instances::form_handler(FrmRhReembolsosLista::form_show));
    ui.register_handler("frmRhReembolsosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmRhReembolsosLista::btn_abrir_click));
    ui.register_handler("frmRhReembolsosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhReembolsosLista::edt_busca_change));
    ui.register_handler("frmRhReembolsosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmRhReembolsosLista::grd_lista_dbl_click));
    ui.register_form_class("frmRhReembolsosConsulta", FrmRhReembolsosConsulta::bind);
    ui.register_handler("frmRhReembolsosConsulta.form_show", aylis::prelude::instances::form_handler(FrmRhReembolsosConsulta::form_show));
    ui.register_handler("frmRhReembolsosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhReembolsosConsulta::edt_busca_change));
    ui.register_form_class("frmRhReembolsosHist", FrmRhReembolsosHist::bind);
    ui.register_handler("frmRhReembolsosHist.form_show", aylis::prelude::instances::form_handler(FrmRhReembolsosHist::form_show));
    ui.register_handler("frmRhReembolsosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmRhReembolsosHist::edt_busca_change));
    ui.register_form_class("frmFinBancosCad", FrmFinBancosCad::bind);
    ui.register_handler("frmFinBancosCad.form_show", aylis::prelude::instances::form_handler(FrmFinBancosCad::form_show));
    ui.register_handler("frmFinBancosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_novo_click));
    ui.register_handler("frmFinBancosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_salvar_click));
    ui.register_handler("frmFinBancosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_excluir_click));
    ui.register_handler("frmFinBancosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_cancelar_click));
    ui.register_handler("frmFinBancosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_novo_item_click));
    ui.register_handler("frmFinBancosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinBancosCad::btn_fechar_click));
    ui.register_handler("frmFinBancosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBancosCad::edt_busca_change));
    ui.register_form_class("frmFinBancosLista", FrmFinBancosLista::bind);
    ui.register_handler("frmFinBancosLista.form_show", aylis::prelude::instances::form_handler(FrmFinBancosLista::form_show));
    ui.register_handler("frmFinBancosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinBancosLista::btn_abrir_click));
    ui.register_handler("frmFinBancosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBancosLista::edt_busca_change));
    ui.register_handler("frmFinBancosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinBancosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinBancosConsulta", FrmFinBancosConsulta::bind);
    ui.register_handler("frmFinBancosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinBancosConsulta::form_show));
    ui.register_handler("frmFinBancosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBancosConsulta::edt_busca_change));
    ui.register_form_class("frmFinBancosHist", FrmFinBancosHist::bind);
    ui.register_handler("frmFinBancosHist.form_show", aylis::prelude::instances::form_handler(FrmFinBancosHist::form_show));
    ui.register_handler("frmFinBancosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBancosHist::edt_busca_change));
    ui.register_form_class("frmFinContasBancariasCad", FrmFinContasBancariasCad::bind);
    ui.register_handler("frmFinContasBancariasCad.form_show", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::form_show));
    ui.register_handler("frmFinContasBancariasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_novo_click));
    ui.register_handler("frmFinContasBancariasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_salvar_click));
    ui.register_handler("frmFinContasBancariasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_excluir_click));
    ui.register_handler("frmFinContasBancariasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_cancelar_click));
    ui.register_handler("frmFinContasBancariasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_novo_item_click));
    ui.register_handler("frmFinContasBancariasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::btn_fechar_click));
    ui.register_handler("frmFinContasBancariasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasBancariasCad::edt_busca_change));
    ui.register_form_class("frmFinContasBancariasLista", FrmFinContasBancariasLista::bind);
    ui.register_handler("frmFinContasBancariasLista.form_show", aylis::prelude::instances::form_handler(FrmFinContasBancariasLista::form_show));
    ui.register_handler("frmFinContasBancariasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasLista::btn_abrir_click));
    ui.register_handler("frmFinContasBancariasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasBancariasLista::edt_busca_change));
    ui.register_handler("frmFinContasBancariasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinContasBancariasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinContasBancariasConsulta", FrmFinContasBancariasConsulta::bind);
    ui.register_handler("frmFinContasBancariasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinContasBancariasConsulta::form_show));
    ui.register_handler("frmFinContasBancariasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasBancariasConsulta::edt_busca_change));
    ui.register_form_class("frmFinContasBancariasHist", FrmFinContasBancariasHist::bind);
    ui.register_handler("frmFinContasBancariasHist.form_show", aylis::prelude::instances::form_handler(FrmFinContasBancariasHist::form_show));
    ui.register_handler("frmFinContasBancariasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasBancariasHist::edt_busca_change));
    ui.register_form_class("frmFinFormasDePagamentoCad", FrmFinFormasDePagamentoCad::bind);
    ui.register_handler("frmFinFormasDePagamentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::form_show));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_novo_click));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_salvar_click));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_excluir_click));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_cancelar_click));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_novo_item_click));
    ui.register_handler("frmFinFormasDePagamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::btn_fechar_click));
    ui.register_handler("frmFinFormasDePagamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoCad::edt_busca_change));
    ui.register_form_class("frmFinFormasDePagamentoLista", FrmFinFormasDePagamentoLista::bind);
    ui.register_handler("frmFinFormasDePagamentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoLista::form_show));
    ui.register_handler("frmFinFormasDePagamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoLista::btn_abrir_click));
    ui.register_handler("frmFinFormasDePagamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoLista::edt_busca_change));
    ui.register_handler("frmFinFormasDePagamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinFormasDePagamentoConsulta", FrmFinFormasDePagamentoConsulta::bind);
    ui.register_handler("frmFinFormasDePagamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoConsulta::form_show));
    ui.register_handler("frmFinFormasDePagamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinFormasDePagamentoHist", FrmFinFormasDePagamentoHist::bind);
    ui.register_handler("frmFinFormasDePagamentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoHist::form_show));
    ui.register_handler("frmFinFormasDePagamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFormasDePagamentoHist::edt_busca_change));
    ui.register_form_class("frmFinCondicoesDePagamentoCad", FrmFinCondicoesDePagamentoCad::bind);
    ui.register_handler("frmFinCondicoesDePagamentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::form_show));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_novo_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_salvar_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_excluir_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_cancelar_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_novo_item_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::btn_fechar_click));
    ui.register_handler("frmFinCondicoesDePagamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoCad::edt_busca_change));
    ui.register_form_class("frmFinCondicoesDePagamentoLista", FrmFinCondicoesDePagamentoLista::bind);
    ui.register_handler("frmFinCondicoesDePagamentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoLista::form_show));
    ui.register_handler("frmFinCondicoesDePagamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoLista::btn_abrir_click));
    ui.register_handler("frmFinCondicoesDePagamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoLista::edt_busca_change));
    ui.register_handler("frmFinCondicoesDePagamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCondicoesDePagamentoConsulta", FrmFinCondicoesDePagamentoConsulta::bind);
    ui.register_handler("frmFinCondicoesDePagamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoConsulta::form_show));
    ui.register_handler("frmFinCondicoesDePagamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinCondicoesDePagamentoHist", FrmFinCondicoesDePagamentoHist::bind);
    ui.register_handler("frmFinCondicoesDePagamentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoHist::form_show));
    ui.register_handler("frmFinCondicoesDePagamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCondicoesDePagamentoHist::edt_busca_change));
    ui.register_form_class("frmFinPlanoFinanceiroCad", FrmFinPlanoFinanceiroCad::bind);
    ui.register_handler("frmFinPlanoFinanceiroCad.form_show", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::form_show));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_novo_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_salvar_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_excluir_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_cancelar_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_novo_item_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::btn_fechar_click));
    ui.register_handler("frmFinPlanoFinanceiroCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroCad::edt_busca_change));
    ui.register_form_class("frmFinPlanoFinanceiroLista", FrmFinPlanoFinanceiroLista::bind);
    ui.register_handler("frmFinPlanoFinanceiroLista.form_show", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroLista::form_show));
    ui.register_handler("frmFinPlanoFinanceiroLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroLista::btn_abrir_click));
    ui.register_handler("frmFinPlanoFinanceiroLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroLista::edt_busca_change));
    ui.register_handler("frmFinPlanoFinanceiroLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinPlanoFinanceiroConsulta", FrmFinPlanoFinanceiroConsulta::bind);
    ui.register_handler("frmFinPlanoFinanceiroConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroConsulta::form_show));
    ui.register_handler("frmFinPlanoFinanceiroConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroConsulta::edt_busca_change));
    ui.register_form_class("frmFinPlanoFinanceiroHist", FrmFinPlanoFinanceiroHist::bind);
    ui.register_handler("frmFinPlanoFinanceiroHist.form_show", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroHist::form_show));
    ui.register_handler("frmFinPlanoFinanceiroHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPlanoFinanceiroHist::edt_busca_change));
    ui.register_form_class("frmFinCentrosDeResultadoCad", FrmFinCentrosDeResultadoCad::bind);
    ui.register_handler("frmFinCentrosDeResultadoCad.form_show", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::form_show));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_novo_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_salvar_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_excluir_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_cancelar_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_novo_item_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::btn_fechar_click));
    ui.register_handler("frmFinCentrosDeResultadoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoCad::edt_busca_change));
    ui.register_form_class("frmFinCentrosDeResultadoLista", FrmFinCentrosDeResultadoLista::bind);
    ui.register_handler("frmFinCentrosDeResultadoLista.form_show", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoLista::form_show));
    ui.register_handler("frmFinCentrosDeResultadoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoLista::btn_abrir_click));
    ui.register_handler("frmFinCentrosDeResultadoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoLista::edt_busca_change));
    ui.register_handler("frmFinCentrosDeResultadoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCentrosDeResultadoConsulta", FrmFinCentrosDeResultadoConsulta::bind);
    ui.register_handler("frmFinCentrosDeResultadoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoConsulta::form_show));
    ui.register_handler("frmFinCentrosDeResultadoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoConsulta::edt_busca_change));
    ui.register_form_class("frmFinCentrosDeResultadoHist", FrmFinCentrosDeResultadoHist::bind);
    ui.register_handler("frmFinCentrosDeResultadoHist.form_show", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoHist::form_show));
    ui.register_handler("frmFinCentrosDeResultadoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCentrosDeResultadoHist::edt_busca_change));
    ui.register_form_class("frmFinCarteirasDeCobrancaCad", FrmFinCarteirasDeCobrancaCad::bind);
    ui.register_handler("frmFinCarteirasDeCobrancaCad.form_show", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::form_show));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_novo_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_salvar_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_excluir_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_cancelar_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_novo_item_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::btn_fechar_click));
    ui.register_handler("frmFinCarteirasDeCobrancaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaCad::edt_busca_change));
    ui.register_form_class("frmFinCarteirasDeCobrancaLista", FrmFinCarteirasDeCobrancaLista::bind);
    ui.register_handler("frmFinCarteirasDeCobrancaLista.form_show", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaLista::form_show));
    ui.register_handler("frmFinCarteirasDeCobrancaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaLista::btn_abrir_click));
    ui.register_handler("frmFinCarteirasDeCobrancaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaLista::edt_busca_change));
    ui.register_handler("frmFinCarteirasDeCobrancaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCarteirasDeCobrancaConsulta", FrmFinCarteirasDeCobrancaConsulta::bind);
    ui.register_handler("frmFinCarteirasDeCobrancaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaConsulta::form_show));
    ui.register_handler("frmFinCarteirasDeCobrancaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaConsulta::edt_busca_change));
    ui.register_form_class("frmFinCarteirasDeCobrancaHist", FrmFinCarteirasDeCobrancaHist::bind);
    ui.register_handler("frmFinCarteirasDeCobrancaHist.form_show", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaHist::form_show));
    ui.register_handler("frmFinCarteirasDeCobrancaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCarteirasDeCobrancaHist::edt_busca_change));
    ui.register_form_class("frmFinMoedasCad", FrmFinMoedasCad::bind);
    ui.register_handler("frmFinMoedasCad.form_show", aylis::prelude::instances::form_handler(FrmFinMoedasCad::form_show));
    ui.register_handler("frmFinMoedasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_novo_click));
    ui.register_handler("frmFinMoedasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_salvar_click));
    ui.register_handler("frmFinMoedasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_excluir_click));
    ui.register_handler("frmFinMoedasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_cancelar_click));
    ui.register_handler("frmFinMoedasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_novo_item_click));
    ui.register_handler("frmFinMoedasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinMoedasCad::btn_fechar_click));
    ui.register_handler("frmFinMoedasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMoedasCad::edt_busca_change));
    ui.register_form_class("frmFinMoedasLista", FrmFinMoedasLista::bind);
    ui.register_handler("frmFinMoedasLista.form_show", aylis::prelude::instances::form_handler(FrmFinMoedasLista::form_show));
    ui.register_handler("frmFinMoedasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinMoedasLista::btn_abrir_click));
    ui.register_handler("frmFinMoedasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMoedasLista::edt_busca_change));
    ui.register_handler("frmFinMoedasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinMoedasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinMoedasConsulta", FrmFinMoedasConsulta::bind);
    ui.register_handler("frmFinMoedasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinMoedasConsulta::form_show));
    ui.register_handler("frmFinMoedasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMoedasConsulta::edt_busca_change));
    ui.register_form_class("frmFinMoedasHist", FrmFinMoedasHist::bind);
    ui.register_handler("frmFinMoedasHist.form_show", aylis::prelude::instances::form_handler(FrmFinMoedasHist::form_show));
    ui.register_handler("frmFinMoedasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMoedasHist::edt_busca_change));
    ui.register_form_class("frmFinIndicesDeCorrecaoCad", FrmFinIndicesDeCorrecaoCad::bind);
    ui.register_handler("frmFinIndicesDeCorrecaoCad.form_show", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::form_show));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_novo_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_salvar_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_excluir_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_cancelar_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_novo_item_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::btn_fechar_click));
    ui.register_handler("frmFinIndicesDeCorrecaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoCad::edt_busca_change));
    ui.register_form_class("frmFinIndicesDeCorrecaoLista", FrmFinIndicesDeCorrecaoLista::bind);
    ui.register_handler("frmFinIndicesDeCorrecaoLista.form_show", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoLista::form_show));
    ui.register_handler("frmFinIndicesDeCorrecaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoLista::btn_abrir_click));
    ui.register_handler("frmFinIndicesDeCorrecaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoLista::edt_busca_change));
    ui.register_handler("frmFinIndicesDeCorrecaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinIndicesDeCorrecaoConsulta", FrmFinIndicesDeCorrecaoConsulta::bind);
    ui.register_handler("frmFinIndicesDeCorrecaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoConsulta::form_show));
    ui.register_handler("frmFinIndicesDeCorrecaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoConsulta::edt_busca_change));
    ui.register_form_class("frmFinIndicesDeCorrecaoHist", FrmFinIndicesDeCorrecaoHist::bind);
    ui.register_handler("frmFinIndicesDeCorrecaoHist.form_show", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoHist::form_show));
    ui.register_handler("frmFinIndicesDeCorrecaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinIndicesDeCorrecaoHist::edt_busca_change));
    ui.register_form_class("frmFinTiposDeDocumentoCad", FrmFinTiposDeDocumentoCad::bind);
    ui.register_handler("frmFinTiposDeDocumentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::form_show));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_novo_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_salvar_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_excluir_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_cancelar_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_novo_item_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::btn_fechar_click));
    ui.register_handler("frmFinTiposDeDocumentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoCad::edt_busca_change));
    ui.register_form_class("frmFinTiposDeDocumentoLista", FrmFinTiposDeDocumentoLista::bind);
    ui.register_handler("frmFinTiposDeDocumentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoLista::form_show));
    ui.register_handler("frmFinTiposDeDocumentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoLista::btn_abrir_click));
    ui.register_handler("frmFinTiposDeDocumentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoLista::edt_busca_change));
    ui.register_handler("frmFinTiposDeDocumentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinTiposDeDocumentoConsulta", FrmFinTiposDeDocumentoConsulta::bind);
    ui.register_handler("frmFinTiposDeDocumentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoConsulta::form_show));
    ui.register_handler("frmFinTiposDeDocumentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinTiposDeDocumentoHist", FrmFinTiposDeDocumentoHist::bind);
    ui.register_handler("frmFinTiposDeDocumentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoHist::form_show));
    ui.register_handler("frmFinTiposDeDocumentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTiposDeDocumentoHist::edt_busca_change));
    ui.register_form_class("frmFinPortadoresCad", FrmFinPortadoresCad::bind);
    ui.register_handler("frmFinPortadoresCad.form_show", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::form_show));
    ui.register_handler("frmFinPortadoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_novo_click));
    ui.register_handler("frmFinPortadoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_salvar_click));
    ui.register_handler("frmFinPortadoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_excluir_click));
    ui.register_handler("frmFinPortadoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_cancelar_click));
    ui.register_handler("frmFinPortadoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_novo_item_click));
    ui.register_handler("frmFinPortadoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::btn_fechar_click));
    ui.register_handler("frmFinPortadoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPortadoresCad::edt_busca_change));
    ui.register_form_class("frmFinPortadoresLista", FrmFinPortadoresLista::bind);
    ui.register_handler("frmFinPortadoresLista.form_show", aylis::prelude::instances::form_handler(FrmFinPortadoresLista::form_show));
    ui.register_handler("frmFinPortadoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinPortadoresLista::btn_abrir_click));
    ui.register_handler("frmFinPortadoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPortadoresLista::edt_busca_change));
    ui.register_handler("frmFinPortadoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinPortadoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinPortadoresConsulta", FrmFinPortadoresConsulta::bind);
    ui.register_handler("frmFinPortadoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinPortadoresConsulta::form_show));
    ui.register_handler("frmFinPortadoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPortadoresConsulta::edt_busca_change));
    ui.register_form_class("frmFinPortadoresHist", FrmFinPortadoresHist::bind);
    ui.register_handler("frmFinPortadoresHist.form_show", aylis::prelude::instances::form_handler(FrmFinPortadoresHist::form_show));
    ui.register_handler("frmFinPortadoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPortadoresHist::edt_busca_change));
    ui.register_form_class("frmFinFavorecidosCad", FrmFinFavorecidosCad::bind);
    ui.register_handler("frmFinFavorecidosCad.form_show", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::form_show));
    ui.register_handler("frmFinFavorecidosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_novo_click));
    ui.register_handler("frmFinFavorecidosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_salvar_click));
    ui.register_handler("frmFinFavorecidosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_excluir_click));
    ui.register_handler("frmFinFavorecidosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_cancelar_click));
    ui.register_handler("frmFinFavorecidosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_novo_item_click));
    ui.register_handler("frmFinFavorecidosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::btn_fechar_click));
    ui.register_handler("frmFinFavorecidosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFavorecidosCad::edt_busca_change));
    ui.register_form_class("frmFinFavorecidosLista", FrmFinFavorecidosLista::bind);
    ui.register_handler("frmFinFavorecidosLista.form_show", aylis::prelude::instances::form_handler(FrmFinFavorecidosLista::form_show));
    ui.register_handler("frmFinFavorecidosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosLista::btn_abrir_click));
    ui.register_handler("frmFinFavorecidosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFavorecidosLista::edt_busca_change));
    ui.register_handler("frmFinFavorecidosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinFavorecidosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinFavorecidosConsulta", FrmFinFavorecidosConsulta::bind);
    ui.register_handler("frmFinFavorecidosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinFavorecidosConsulta::form_show));
    ui.register_handler("frmFinFavorecidosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFavorecidosConsulta::edt_busca_change));
    ui.register_form_class("frmFinFavorecidosHist", FrmFinFavorecidosHist::bind);
    ui.register_handler("frmFinFavorecidosHist.form_show", aylis::prelude::instances::form_handler(FrmFinFavorecidosHist::form_show));
    ui.register_handler("frmFinFavorecidosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFavorecidosHist::edt_busca_change));
    ui.register_form_class("frmFinCartoesCorporativosCad", FrmFinCartoesCorporativosCad::bind);
    ui.register_handler("frmFinCartoesCorporativosCad.form_show", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::form_show));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_novo_click));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_salvar_click));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_excluir_click));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_cancelar_click));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_novo_item_click));
    ui.register_handler("frmFinCartoesCorporativosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::btn_fechar_click));
    ui.register_handler("frmFinCartoesCorporativosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosCad::edt_busca_change));
    ui.register_form_class("frmFinCartoesCorporativosLista", FrmFinCartoesCorporativosLista::bind);
    ui.register_handler("frmFinCartoesCorporativosLista.form_show", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosLista::form_show));
    ui.register_handler("frmFinCartoesCorporativosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosLista::btn_abrir_click));
    ui.register_handler("frmFinCartoesCorporativosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosLista::edt_busca_change));
    ui.register_handler("frmFinCartoesCorporativosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCartoesCorporativosConsulta", FrmFinCartoesCorporativosConsulta::bind);
    ui.register_handler("frmFinCartoesCorporativosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosConsulta::form_show));
    ui.register_handler("frmFinCartoesCorporativosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosConsulta::edt_busca_change));
    ui.register_form_class("frmFinCartoesCorporativosHist", FrmFinCartoesCorporativosHist::bind);
    ui.register_handler("frmFinCartoesCorporativosHist.form_show", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosHist::form_show));
    ui.register_handler("frmFinCartoesCorporativosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCartoesCorporativosHist::edt_busca_change));
    ui.register_form_class("frmFinOperadorasDeCartaoCad", FrmFinOperadorasDeCartaoCad::bind);
    ui.register_handler("frmFinOperadorasDeCartaoCad.form_show", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::form_show));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_novo_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_salvar_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_excluir_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_cancelar_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_novo_item_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::btn_fechar_click));
    ui.register_handler("frmFinOperadorasDeCartaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoCad::edt_busca_change));
    ui.register_form_class("frmFinOperadorasDeCartaoLista", FrmFinOperadorasDeCartaoLista::bind);
    ui.register_handler("frmFinOperadorasDeCartaoLista.form_show", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoLista::form_show));
    ui.register_handler("frmFinOperadorasDeCartaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoLista::btn_abrir_click));
    ui.register_handler("frmFinOperadorasDeCartaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoLista::edt_busca_change));
    ui.register_handler("frmFinOperadorasDeCartaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinOperadorasDeCartaoConsulta", FrmFinOperadorasDeCartaoConsulta::bind);
    ui.register_handler("frmFinOperadorasDeCartaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoConsulta::form_show));
    ui.register_handler("frmFinOperadorasDeCartaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoConsulta::edt_busca_change));
    ui.register_form_class("frmFinOperadorasDeCartaoHist", FrmFinOperadorasDeCartaoHist::bind);
    ui.register_handler("frmFinOperadorasDeCartaoHist.form_show", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoHist::form_show));
    ui.register_handler("frmFinOperadorasDeCartaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOperadorasDeCartaoHist::edt_busca_change));
    ui.register_form_class("frmFinTarifasBancariasCad", FrmFinTarifasBancariasCad::bind);
    ui.register_handler("frmFinTarifasBancariasCad.form_show", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::form_show));
    ui.register_handler("frmFinTarifasBancariasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_novo_click));
    ui.register_handler("frmFinTarifasBancariasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_salvar_click));
    ui.register_handler("frmFinTarifasBancariasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_excluir_click));
    ui.register_handler("frmFinTarifasBancariasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_cancelar_click));
    ui.register_handler("frmFinTarifasBancariasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_novo_item_click));
    ui.register_handler("frmFinTarifasBancariasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::btn_fechar_click));
    ui.register_handler("frmFinTarifasBancariasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasCad::edt_busca_change));
    ui.register_form_class("frmFinTarifasBancariasLista", FrmFinTarifasBancariasLista::bind);
    ui.register_handler("frmFinTarifasBancariasLista.form_show", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasLista::form_show));
    ui.register_handler("frmFinTarifasBancariasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasLista::btn_abrir_click));
    ui.register_handler("frmFinTarifasBancariasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasLista::edt_busca_change));
    ui.register_handler("frmFinTarifasBancariasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinTarifasBancariasConsulta", FrmFinTarifasBancariasConsulta::bind);
    ui.register_handler("frmFinTarifasBancariasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasConsulta::form_show));
    ui.register_handler("frmFinTarifasBancariasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasConsulta::edt_busca_change));
    ui.register_form_class("frmFinTarifasBancariasHist", FrmFinTarifasBancariasHist::bind);
    ui.register_handler("frmFinTarifasBancariasHist.form_show", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasHist::form_show));
    ui.register_handler("frmFinTarifasBancariasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTarifasBancariasHist::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeDespesaCad", FrmFinCategoriasDeDespesaCad::bind);
    ui.register_handler("frmFinCategoriasDeDespesaCad.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::form_show));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_novo_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_salvar_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_excluir_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_cancelar_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_novo_item_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::btn_fechar_click));
    ui.register_handler("frmFinCategoriasDeDespesaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaCad::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeDespesaLista", FrmFinCategoriasDeDespesaLista::bind);
    ui.register_handler("frmFinCategoriasDeDespesaLista.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaLista::form_show));
    ui.register_handler("frmFinCategoriasDeDespesaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaLista::btn_abrir_click));
    ui.register_handler("frmFinCategoriasDeDespesaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaLista::edt_busca_change));
    ui.register_handler("frmFinCategoriasDeDespesaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCategoriasDeDespesaConsulta", FrmFinCategoriasDeDespesaConsulta::bind);
    ui.register_handler("frmFinCategoriasDeDespesaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaConsulta::form_show));
    ui.register_handler("frmFinCategoriasDeDespesaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaConsulta::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeDespesaHist", FrmFinCategoriasDeDespesaHist::bind);
    ui.register_handler("frmFinCategoriasDeDespesaHist.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaHist::form_show));
    ui.register_handler("frmFinCategoriasDeDespesaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeDespesaHist::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeReceitaCad", FrmFinCategoriasDeReceitaCad::bind);
    ui.register_handler("frmFinCategoriasDeReceitaCad.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::form_show));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_novo_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_salvar_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_excluir_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_cancelar_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_novo_item_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::btn_fechar_click));
    ui.register_handler("frmFinCategoriasDeReceitaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaCad::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeReceitaLista", FrmFinCategoriasDeReceitaLista::bind);
    ui.register_handler("frmFinCategoriasDeReceitaLista.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaLista::form_show));
    ui.register_handler("frmFinCategoriasDeReceitaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaLista::btn_abrir_click));
    ui.register_handler("frmFinCategoriasDeReceitaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaLista::edt_busca_change));
    ui.register_handler("frmFinCategoriasDeReceitaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinCategoriasDeReceitaConsulta", FrmFinCategoriasDeReceitaConsulta::bind);
    ui.register_handler("frmFinCategoriasDeReceitaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaConsulta::form_show));
    ui.register_handler("frmFinCategoriasDeReceitaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaConsulta::edt_busca_change));
    ui.register_form_class("frmFinCategoriasDeReceitaHist", FrmFinCategoriasDeReceitaHist::bind);
    ui.register_handler("frmFinCategoriasDeReceitaHist.form_show", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaHist::form_show));
    ui.register_handler("frmFinCategoriasDeReceitaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinCategoriasDeReceitaHist::edt_busca_change));
    ui.register_form_class("frmFinProjetosCad", FrmFinProjetosCad::bind);
    ui.register_handler("frmFinProjetosCad.form_show", aylis::prelude::instances::form_handler(FrmFinProjetosCad::form_show));
    ui.register_handler("frmFinProjetosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_novo_click));
    ui.register_handler("frmFinProjetosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_salvar_click));
    ui.register_handler("frmFinProjetosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_excluir_click));
    ui.register_handler("frmFinProjetosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_cancelar_click));
    ui.register_handler("frmFinProjetosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_novo_item_click));
    ui.register_handler("frmFinProjetosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinProjetosCad::btn_fechar_click));
    ui.register_handler("frmFinProjetosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinProjetosCad::edt_busca_change));
    ui.register_form_class("frmFinProjetosLista", FrmFinProjetosLista::bind);
    ui.register_handler("frmFinProjetosLista.form_show", aylis::prelude::instances::form_handler(FrmFinProjetosLista::form_show));
    ui.register_handler("frmFinProjetosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinProjetosLista::btn_abrir_click));
    ui.register_handler("frmFinProjetosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinProjetosLista::edt_busca_change));
    ui.register_handler("frmFinProjetosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinProjetosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinProjetosConsulta", FrmFinProjetosConsulta::bind);
    ui.register_handler("frmFinProjetosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinProjetosConsulta::form_show));
    ui.register_handler("frmFinProjetosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinProjetosConsulta::edt_busca_change));
    ui.register_form_class("frmFinProjetosHist", FrmFinProjetosHist::bind);
    ui.register_handler("frmFinProjetosHist.form_show", aylis::prelude::instances::form_handler(FrmFinProjetosHist::form_show));
    ui.register_handler("frmFinProjetosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinProjetosHist::edt_busca_change));
    ui.register_form_class("frmFinContratosFinanceirosCad", FrmFinContratosFinanceirosCad::bind);
    ui.register_handler("frmFinContratosFinanceirosCad.form_show", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::form_show));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_novo_click));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_salvar_click));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_excluir_click));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_cancelar_click));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_novo_item_click));
    ui.register_handler("frmFinContratosFinanceirosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::btn_fechar_click));
    ui.register_handler("frmFinContratosFinanceirosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosCad::edt_busca_change));
    ui.register_form_class("frmFinContratosFinanceirosLista", FrmFinContratosFinanceirosLista::bind);
    ui.register_handler("frmFinContratosFinanceirosLista.form_show", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosLista::form_show));
    ui.register_handler("frmFinContratosFinanceirosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosLista::btn_abrir_click));
    ui.register_handler("frmFinContratosFinanceirosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosLista::edt_busca_change));
    ui.register_handler("frmFinContratosFinanceirosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinContratosFinanceirosConsulta", FrmFinContratosFinanceirosConsulta::bind);
    ui.register_handler("frmFinContratosFinanceirosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosConsulta::form_show));
    ui.register_handler("frmFinContratosFinanceirosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosConsulta::edt_busca_change));
    ui.register_form_class("frmFinContratosFinanceirosHist", FrmFinContratosFinanceirosHist::bind);
    ui.register_handler("frmFinContratosFinanceirosHist.form_show", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosHist::form_show));
    ui.register_handler("frmFinContratosFinanceirosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContratosFinanceirosHist::edt_busca_change));
    ui.register_form_class("frmFinAvalistasCad", FrmFinAvalistasCad::bind);
    ui.register_handler("frmFinAvalistasCad.form_show", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::form_show));
    ui.register_handler("frmFinAvalistasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_novo_click));
    ui.register_handler("frmFinAvalistasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_salvar_click));
    ui.register_handler("frmFinAvalistasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_excluir_click));
    ui.register_handler("frmFinAvalistasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_cancelar_click));
    ui.register_handler("frmFinAvalistasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_novo_item_click));
    ui.register_handler("frmFinAvalistasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::btn_fechar_click));
    ui.register_handler("frmFinAvalistasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAvalistasCad::edt_busca_change));
    ui.register_form_class("frmFinAvalistasLista", FrmFinAvalistasLista::bind);
    ui.register_handler("frmFinAvalistasLista.form_show", aylis::prelude::instances::form_handler(FrmFinAvalistasLista::form_show));
    ui.register_handler("frmFinAvalistasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinAvalistasLista::btn_abrir_click));
    ui.register_handler("frmFinAvalistasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAvalistasLista::edt_busca_change));
    ui.register_handler("frmFinAvalistasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinAvalistasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinAvalistasConsulta", FrmFinAvalistasConsulta::bind);
    ui.register_handler("frmFinAvalistasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinAvalistasConsulta::form_show));
    ui.register_handler("frmFinAvalistasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAvalistasConsulta::edt_busca_change));
    ui.register_form_class("frmFinAvalistasHist", FrmFinAvalistasHist::bind);
    ui.register_handler("frmFinAvalistasHist.form_show", aylis::prelude::instances::form_handler(FrmFinAvalistasHist::form_show));
    ui.register_handler("frmFinAvalistasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAvalistasHist::edt_busca_change));
    ui.register_form_class("frmFinAgenciasCad", FrmFinAgenciasCad::bind);
    ui.register_handler("frmFinAgenciasCad.form_show", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::form_show));
    ui.register_handler("frmFinAgenciasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_novo_click));
    ui.register_handler("frmFinAgenciasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_salvar_click));
    ui.register_handler("frmFinAgenciasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_excluir_click));
    ui.register_handler("frmFinAgenciasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_cancelar_click));
    ui.register_handler("frmFinAgenciasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_novo_item_click));
    ui.register_handler("frmFinAgenciasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::btn_fechar_click));
    ui.register_handler("frmFinAgenciasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAgenciasCad::edt_busca_change));
    ui.register_form_class("frmFinAgenciasLista", FrmFinAgenciasLista::bind);
    ui.register_handler("frmFinAgenciasLista.form_show", aylis::prelude::instances::form_handler(FrmFinAgenciasLista::form_show));
    ui.register_handler("frmFinAgenciasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinAgenciasLista::btn_abrir_click));
    ui.register_handler("frmFinAgenciasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAgenciasLista::edt_busca_change));
    ui.register_handler("frmFinAgenciasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinAgenciasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinAgenciasConsulta", FrmFinAgenciasConsulta::bind);
    ui.register_handler("frmFinAgenciasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinAgenciasConsulta::form_show));
    ui.register_handler("frmFinAgenciasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAgenciasConsulta::edt_busca_change));
    ui.register_form_class("frmFinAgenciasHist", FrmFinAgenciasHist::bind);
    ui.register_handler("frmFinAgenciasHist.form_show", aylis::prelude::instances::form_handler(FrmFinAgenciasHist::form_show));
    ui.register_handler("frmFinAgenciasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAgenciasHist::edt_busca_change));
    ui.register_form_class("frmFinContasAPagarCad", FrmFinContasAPagarCad::bind);
    ui.register_handler("frmFinContasAPagarCad.form_show", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::form_show));
    ui.register_handler("frmFinContasAPagarCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_novo_click));
    ui.register_handler("frmFinContasAPagarCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_salvar_click));
    ui.register_handler("frmFinContasAPagarCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_excluir_click));
    ui.register_handler("frmFinContasAPagarCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_cancelar_click));
    ui.register_handler("frmFinContasAPagarCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_novo_item_click));
    ui.register_handler("frmFinContasAPagarCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::btn_fechar_click));
    ui.register_handler("frmFinContasAPagarCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAPagarCad::edt_busca_change));
    ui.register_form_class("frmFinContasAPagarLista", FrmFinContasAPagarLista::bind);
    ui.register_handler("frmFinContasAPagarLista.form_show", aylis::prelude::instances::form_handler(FrmFinContasAPagarLista::form_show));
    ui.register_handler("frmFinContasAPagarLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarLista::btn_abrir_click));
    ui.register_handler("frmFinContasAPagarLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAPagarLista::edt_busca_change));
    ui.register_handler("frmFinContasAPagarLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinContasAPagarLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinContasAPagarConsulta", FrmFinContasAPagarConsulta::bind);
    ui.register_handler("frmFinContasAPagarConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinContasAPagarConsulta::form_show));
    ui.register_handler("frmFinContasAPagarConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAPagarConsulta::edt_busca_change));
    ui.register_form_class("frmFinContasAPagarHist", FrmFinContasAPagarHist::bind);
    ui.register_handler("frmFinContasAPagarHist.form_show", aylis::prelude::instances::form_handler(FrmFinContasAPagarHist::form_show));
    ui.register_handler("frmFinContasAPagarHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAPagarHist::edt_busca_change));
    ui.register_form_class("frmFinContasAReceberCad", FrmFinContasAReceberCad::bind);
    ui.register_handler("frmFinContasAReceberCad.form_show", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::form_show));
    ui.register_handler("frmFinContasAReceberCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_novo_click));
    ui.register_handler("frmFinContasAReceberCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_salvar_click));
    ui.register_handler("frmFinContasAReceberCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_excluir_click));
    ui.register_handler("frmFinContasAReceberCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_cancelar_click));
    ui.register_handler("frmFinContasAReceberCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_novo_item_click));
    ui.register_handler("frmFinContasAReceberCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::btn_fechar_click));
    ui.register_handler("frmFinContasAReceberCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAReceberCad::edt_busca_change));
    ui.register_form_class("frmFinContasAReceberLista", FrmFinContasAReceberLista::bind);
    ui.register_handler("frmFinContasAReceberLista.form_show", aylis::prelude::instances::form_handler(FrmFinContasAReceberLista::form_show));
    ui.register_handler("frmFinContasAReceberLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberLista::btn_abrir_click));
    ui.register_handler("frmFinContasAReceberLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAReceberLista::edt_busca_change));
    ui.register_handler("frmFinContasAReceberLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinContasAReceberLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinContasAReceberConsulta", FrmFinContasAReceberConsulta::bind);
    ui.register_handler("frmFinContasAReceberConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinContasAReceberConsulta::form_show));
    ui.register_handler("frmFinContasAReceberConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAReceberConsulta::edt_busca_change));
    ui.register_form_class("frmFinContasAReceberHist", FrmFinContasAReceberHist::bind);
    ui.register_handler("frmFinContasAReceberHist.form_show", aylis::prelude::instances::form_handler(FrmFinContasAReceberHist::form_show));
    ui.register_handler("frmFinContasAReceberHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinContasAReceberHist::edt_busca_change));
    ui.register_form_class("frmFinBaixasDePagamentoCad", FrmFinBaixasDePagamentoCad::bind);
    ui.register_handler("frmFinBaixasDePagamentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::form_show));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_novo_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_salvar_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_excluir_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_cancelar_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_novo_item_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::btn_fechar_click));
    ui.register_handler("frmFinBaixasDePagamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoCad::edt_busca_change));
    ui.register_form_class("frmFinBaixasDePagamentoLista", FrmFinBaixasDePagamentoLista::bind);
    ui.register_handler("frmFinBaixasDePagamentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoLista::form_show));
    ui.register_handler("frmFinBaixasDePagamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoLista::btn_abrir_click));
    ui.register_handler("frmFinBaixasDePagamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoLista::edt_busca_change));
    ui.register_handler("frmFinBaixasDePagamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinBaixasDePagamentoConsulta", FrmFinBaixasDePagamentoConsulta::bind);
    ui.register_handler("frmFinBaixasDePagamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoConsulta::form_show));
    ui.register_handler("frmFinBaixasDePagamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinBaixasDePagamentoHist", FrmFinBaixasDePagamentoHist::bind);
    ui.register_handler("frmFinBaixasDePagamentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoHist::form_show));
    ui.register_handler("frmFinBaixasDePagamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDePagamentoHist::edt_busca_change));
    ui.register_form_class("frmFinBaixasDeRecebimentoCad", FrmFinBaixasDeRecebimentoCad::bind);
    ui.register_handler("frmFinBaixasDeRecebimentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::form_show));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_novo_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_salvar_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_excluir_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_cancelar_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_novo_item_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::btn_fechar_click));
    ui.register_handler("frmFinBaixasDeRecebimentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoCad::edt_busca_change));
    ui.register_form_class("frmFinBaixasDeRecebimentoLista", FrmFinBaixasDeRecebimentoLista::bind);
    ui.register_handler("frmFinBaixasDeRecebimentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoLista::form_show));
    ui.register_handler("frmFinBaixasDeRecebimentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoLista::btn_abrir_click));
    ui.register_handler("frmFinBaixasDeRecebimentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoLista::edt_busca_change));
    ui.register_handler("frmFinBaixasDeRecebimentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinBaixasDeRecebimentoConsulta", FrmFinBaixasDeRecebimentoConsulta::bind);
    ui.register_handler("frmFinBaixasDeRecebimentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoConsulta::form_show));
    ui.register_handler("frmFinBaixasDeRecebimentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinBaixasDeRecebimentoHist", FrmFinBaixasDeRecebimentoHist::bind);
    ui.register_handler("frmFinBaixasDeRecebimentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoHist::form_show));
    ui.register_handler("frmFinBaixasDeRecebimentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBaixasDeRecebimentoHist::edt_busca_change));
    ui.register_form_class("frmFinMovimentosBancariosCad", FrmFinMovimentosBancariosCad::bind);
    ui.register_handler("frmFinMovimentosBancariosCad.form_show", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::form_show));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_novo_click));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_salvar_click));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_excluir_click));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_cancelar_click));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_novo_item_click));
    ui.register_handler("frmFinMovimentosBancariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::btn_fechar_click));
    ui.register_handler("frmFinMovimentosBancariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosCad::edt_busca_change));
    ui.register_form_class("frmFinMovimentosBancariosLista", FrmFinMovimentosBancariosLista::bind);
    ui.register_handler("frmFinMovimentosBancariosLista.form_show", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosLista::form_show));
    ui.register_handler("frmFinMovimentosBancariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosLista::btn_abrir_click));
    ui.register_handler("frmFinMovimentosBancariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosLista::edt_busca_change));
    ui.register_handler("frmFinMovimentosBancariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinMovimentosBancariosConsulta", FrmFinMovimentosBancariosConsulta::bind);
    ui.register_handler("frmFinMovimentosBancariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosConsulta::form_show));
    ui.register_handler("frmFinMovimentosBancariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosConsulta::edt_busca_change));
    ui.register_form_class("frmFinMovimentosBancariosHist", FrmFinMovimentosBancariosHist::bind);
    ui.register_handler("frmFinMovimentosBancariosHist.form_show", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosHist::form_show));
    ui.register_handler("frmFinMovimentosBancariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinMovimentosBancariosHist::edt_busca_change));
    ui.register_form_class("frmFinConciliacoesBancariasCad", FrmFinConciliacoesBancariasCad::bind);
    ui.register_handler("frmFinConciliacoesBancariasCad.form_show", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::form_show));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_novo_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_salvar_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_excluir_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_cancelar_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_novo_item_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::btn_fechar_click));
    ui.register_handler("frmFinConciliacoesBancariasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasCad::edt_busca_change));
    ui.register_form_class("frmFinConciliacoesBancariasLista", FrmFinConciliacoesBancariasLista::bind);
    ui.register_handler("frmFinConciliacoesBancariasLista.form_show", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasLista::form_show));
    ui.register_handler("frmFinConciliacoesBancariasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasLista::btn_abrir_click));
    ui.register_handler("frmFinConciliacoesBancariasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasLista::edt_busca_change));
    ui.register_handler("frmFinConciliacoesBancariasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinConciliacoesBancariasConsulta", FrmFinConciliacoesBancariasConsulta::bind);
    ui.register_handler("frmFinConciliacoesBancariasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasConsulta::form_show));
    ui.register_handler("frmFinConciliacoesBancariasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasConsulta::edt_busca_change));
    ui.register_form_class("frmFinConciliacoesBancariasHist", FrmFinConciliacoesBancariasHist::bind);
    ui.register_handler("frmFinConciliacoesBancariasHist.form_show", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasHist::form_show));
    ui.register_handler("frmFinConciliacoesBancariasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinConciliacoesBancariasHist::edt_busca_change));
    ui.register_form_class("frmFinTransferenciasEntreContasCad", FrmFinTransferenciasEntreContasCad::bind);
    ui.register_handler("frmFinTransferenciasEntreContasCad.form_show", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::form_show));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_novo_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_salvar_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_excluir_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_cancelar_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_novo_item_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::btn_fechar_click));
    ui.register_handler("frmFinTransferenciasEntreContasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasCad::edt_busca_change));
    ui.register_form_class("frmFinTransferenciasEntreContasLista", FrmFinTransferenciasEntreContasLista::bind);
    ui.register_handler("frmFinTransferenciasEntreContasLista.form_show", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasLista::form_show));
    ui.register_handler("frmFinTransferenciasEntreContasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasLista::btn_abrir_click));
    ui.register_handler("frmFinTransferenciasEntreContasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasLista::edt_busca_change));
    ui.register_handler("frmFinTransferenciasEntreContasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinTransferenciasEntreContasConsulta", FrmFinTransferenciasEntreContasConsulta::bind);
    ui.register_handler("frmFinTransferenciasEntreContasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasConsulta::form_show));
    ui.register_handler("frmFinTransferenciasEntreContasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasConsulta::edt_busca_change));
    ui.register_form_class("frmFinTransferenciasEntreContasHist", FrmFinTransferenciasEntreContasHist::bind);
    ui.register_handler("frmFinTransferenciasEntreContasHist.form_show", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasHist::form_show));
    ui.register_handler("frmFinTransferenciasEntreContasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinTransferenciasEntreContasHist::edt_busca_change));
    ui.register_form_class("frmFinBorderosDePagamentoCad", FrmFinBorderosDePagamentoCad::bind);
    ui.register_handler("frmFinBorderosDePagamentoCad.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::form_show));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_novo_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_salvar_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_excluir_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_cancelar_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_novo_item_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::btn_fechar_click));
    ui.register_handler("frmFinBorderosDePagamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoCad::edt_busca_change));
    ui.register_form_class("frmFinBorderosDePagamentoLista", FrmFinBorderosDePagamentoLista::bind);
    ui.register_handler("frmFinBorderosDePagamentoLista.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoLista::form_show));
    ui.register_handler("frmFinBorderosDePagamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoLista::btn_abrir_click));
    ui.register_handler("frmFinBorderosDePagamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoLista::edt_busca_change));
    ui.register_handler("frmFinBorderosDePagamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinBorderosDePagamentoConsulta", FrmFinBorderosDePagamentoConsulta::bind);
    ui.register_handler("frmFinBorderosDePagamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoConsulta::form_show));
    ui.register_handler("frmFinBorderosDePagamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoConsulta::edt_busca_change));
    ui.register_form_class("frmFinBorderosDePagamentoHist", FrmFinBorderosDePagamentoHist::bind);
    ui.register_handler("frmFinBorderosDePagamentoHist.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoHist::form_show));
    ui.register_handler("frmFinBorderosDePagamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDePagamentoHist::edt_busca_change));
    ui.register_form_class("frmFinBorderosDeCobrancaCad", FrmFinBorderosDeCobrancaCad::bind);
    ui.register_handler("frmFinBorderosDeCobrancaCad.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::form_show));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_novo_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_salvar_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_excluir_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_cancelar_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_novo_item_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::btn_fechar_click));
    ui.register_handler("frmFinBorderosDeCobrancaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaCad::edt_busca_change));
    ui.register_form_class("frmFinBorderosDeCobrancaLista", FrmFinBorderosDeCobrancaLista::bind);
    ui.register_handler("frmFinBorderosDeCobrancaLista.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaLista::form_show));
    ui.register_handler("frmFinBorderosDeCobrancaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaLista::btn_abrir_click));
    ui.register_handler("frmFinBorderosDeCobrancaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaLista::edt_busca_change));
    ui.register_handler("frmFinBorderosDeCobrancaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinBorderosDeCobrancaConsulta", FrmFinBorderosDeCobrancaConsulta::bind);
    ui.register_handler("frmFinBorderosDeCobrancaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaConsulta::form_show));
    ui.register_handler("frmFinBorderosDeCobrancaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaConsulta::edt_busca_change));
    ui.register_form_class("frmFinBorderosDeCobrancaHist", FrmFinBorderosDeCobrancaHist::bind);
    ui.register_handler("frmFinBorderosDeCobrancaHist.form_show", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaHist::form_show));
    ui.register_handler("frmFinBorderosDeCobrancaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinBorderosDeCobrancaHist::edt_busca_change));
    ui.register_form_class("frmFinRemessasCnabCad", FrmFinRemessasCnabCad::bind);
    ui.register_handler("frmFinRemessasCnabCad.form_show", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::form_show));
    ui.register_handler("frmFinRemessasCnabCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_novo_click));
    ui.register_handler("frmFinRemessasCnabCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_salvar_click));
    ui.register_handler("frmFinRemessasCnabCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_excluir_click));
    ui.register_handler("frmFinRemessasCnabCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_cancelar_click));
    ui.register_handler("frmFinRemessasCnabCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_novo_item_click));
    ui.register_handler("frmFinRemessasCnabCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::btn_fechar_click));
    ui.register_handler("frmFinRemessasCnabCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRemessasCnabCad::edt_busca_change));
    ui.register_form_class("frmFinRemessasCnabLista", FrmFinRemessasCnabLista::bind);
    ui.register_handler("frmFinRemessasCnabLista.form_show", aylis::prelude::instances::form_handler(FrmFinRemessasCnabLista::form_show));
    ui.register_handler("frmFinRemessasCnabLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabLista::btn_abrir_click));
    ui.register_handler("frmFinRemessasCnabLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRemessasCnabLista::edt_busca_change));
    ui.register_handler("frmFinRemessasCnabLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinRemessasCnabLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinRemessasCnabConsulta", FrmFinRemessasCnabConsulta::bind);
    ui.register_handler("frmFinRemessasCnabConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinRemessasCnabConsulta::form_show));
    ui.register_handler("frmFinRemessasCnabConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRemessasCnabConsulta::edt_busca_change));
    ui.register_form_class("frmFinRemessasCnabHist", FrmFinRemessasCnabHist::bind);
    ui.register_handler("frmFinRemessasCnabHist.form_show", aylis::prelude::instances::form_handler(FrmFinRemessasCnabHist::form_show));
    ui.register_handler("frmFinRemessasCnabHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRemessasCnabHist::edt_busca_change));
    ui.register_form_class("frmFinRetornosCnabCad", FrmFinRetornosCnabCad::bind);
    ui.register_handler("frmFinRetornosCnabCad.form_show", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::form_show));
    ui.register_handler("frmFinRetornosCnabCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_novo_click));
    ui.register_handler("frmFinRetornosCnabCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_salvar_click));
    ui.register_handler("frmFinRetornosCnabCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_excluir_click));
    ui.register_handler("frmFinRetornosCnabCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_cancelar_click));
    ui.register_handler("frmFinRetornosCnabCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_novo_item_click));
    ui.register_handler("frmFinRetornosCnabCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::btn_fechar_click));
    ui.register_handler("frmFinRetornosCnabCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRetornosCnabCad::edt_busca_change));
    ui.register_form_class("frmFinRetornosCnabLista", FrmFinRetornosCnabLista::bind);
    ui.register_handler("frmFinRetornosCnabLista.form_show", aylis::prelude::instances::form_handler(FrmFinRetornosCnabLista::form_show));
    ui.register_handler("frmFinRetornosCnabLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabLista::btn_abrir_click));
    ui.register_handler("frmFinRetornosCnabLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRetornosCnabLista::edt_busca_change));
    ui.register_handler("frmFinRetornosCnabLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinRetornosCnabLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinRetornosCnabConsulta", FrmFinRetornosCnabConsulta::bind);
    ui.register_handler("frmFinRetornosCnabConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinRetornosCnabConsulta::form_show));
    ui.register_handler("frmFinRetornosCnabConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRetornosCnabConsulta::edt_busca_change));
    ui.register_form_class("frmFinRetornosCnabHist", FrmFinRetornosCnabHist::bind);
    ui.register_handler("frmFinRetornosCnabHist.form_show", aylis::prelude::instances::form_handler(FrmFinRetornosCnabHist::form_show));
    ui.register_handler("frmFinRetornosCnabHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRetornosCnabHist::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosAFornecedoresCad", FrmFinAdiantamentosAFornecedoresCad::bind);
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::form_show));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_novo_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_salvar_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_excluir_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_cancelar_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_novo_item_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::btn_fechar_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresCad::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosAFornecedoresLista", FrmFinAdiantamentosAFornecedoresLista::bind);
    ui.register_handler("frmFinAdiantamentosAFornecedoresLista.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresLista::form_show));
    ui.register_handler("frmFinAdiantamentosAFornecedoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresLista::btn_abrir_click));
    ui.register_handler("frmFinAdiantamentosAFornecedoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresLista::edt_busca_change));
    ui.register_handler("frmFinAdiantamentosAFornecedoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinAdiantamentosAFornecedoresConsulta", FrmFinAdiantamentosAFornecedoresConsulta::bind);
    ui.register_handler("frmFinAdiantamentosAFornecedoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresConsulta::form_show));
    ui.register_handler("frmFinAdiantamentosAFornecedoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresConsulta::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosAFornecedoresHist", FrmFinAdiantamentosAFornecedoresHist::bind);
    ui.register_handler("frmFinAdiantamentosAFornecedoresHist.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresHist::form_show));
    ui.register_handler("frmFinAdiantamentosAFornecedoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosAFornecedoresHist::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosDeClientesCad", FrmFinAdiantamentosDeClientesCad::bind);
    ui.register_handler("frmFinAdiantamentosDeClientesCad.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::form_show));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_novo_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_salvar_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_excluir_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_cancelar_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_novo_item_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::btn_fechar_click));
    ui.register_handler("frmFinAdiantamentosDeClientesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesCad::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosDeClientesLista", FrmFinAdiantamentosDeClientesLista::bind);
    ui.register_handler("frmFinAdiantamentosDeClientesLista.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesLista::form_show));
    ui.register_handler("frmFinAdiantamentosDeClientesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesLista::btn_abrir_click));
    ui.register_handler("frmFinAdiantamentosDeClientesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesLista::edt_busca_change));
    ui.register_handler("frmFinAdiantamentosDeClientesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinAdiantamentosDeClientesConsulta", FrmFinAdiantamentosDeClientesConsulta::bind);
    ui.register_handler("frmFinAdiantamentosDeClientesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesConsulta::form_show));
    ui.register_handler("frmFinAdiantamentosDeClientesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesConsulta::edt_busca_change));
    ui.register_form_class("frmFinAdiantamentosDeClientesHist", FrmFinAdiantamentosDeClientesHist::bind);
    ui.register_handler("frmFinAdiantamentosDeClientesHist.form_show", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesHist::form_show));
    ui.register_handler("frmFinAdiantamentosDeClientesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAdiantamentosDeClientesHist::edt_busca_change));
    ui.register_form_class("frmFinEmprestimosCad", FrmFinEmprestimosCad::bind);
    ui.register_handler("frmFinEmprestimosCad.form_show", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::form_show));
    ui.register_handler("frmFinEmprestimosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_novo_click));
    ui.register_handler("frmFinEmprestimosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_salvar_click));
    ui.register_handler("frmFinEmprestimosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_excluir_click));
    ui.register_handler("frmFinEmprestimosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_cancelar_click));
    ui.register_handler("frmFinEmprestimosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_novo_item_click));
    ui.register_handler("frmFinEmprestimosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::btn_fechar_click));
    ui.register_handler("frmFinEmprestimosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinEmprestimosCad::edt_busca_change));
    ui.register_form_class("frmFinEmprestimosLista", FrmFinEmprestimosLista::bind);
    ui.register_handler("frmFinEmprestimosLista.form_show", aylis::prelude::instances::form_handler(FrmFinEmprestimosLista::form_show));
    ui.register_handler("frmFinEmprestimosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosLista::btn_abrir_click));
    ui.register_handler("frmFinEmprestimosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinEmprestimosLista::edt_busca_change));
    ui.register_handler("frmFinEmprestimosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinEmprestimosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinEmprestimosConsulta", FrmFinEmprestimosConsulta::bind);
    ui.register_handler("frmFinEmprestimosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinEmprestimosConsulta::form_show));
    ui.register_handler("frmFinEmprestimosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinEmprestimosConsulta::edt_busca_change));
    ui.register_form_class("frmFinEmprestimosHist", FrmFinEmprestimosHist::bind);
    ui.register_handler("frmFinEmprestimosHist.form_show", aylis::prelude::instances::form_handler(FrmFinEmprestimosHist::form_show));
    ui.register_handler("frmFinEmprestimosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinEmprestimosHist::edt_busca_change));
    ui.register_form_class("frmFinAplicacoesFinanceirasCad", FrmFinAplicacoesFinanceirasCad::bind);
    ui.register_handler("frmFinAplicacoesFinanceirasCad.form_show", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::form_show));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_novo_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_salvar_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_excluir_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_cancelar_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_novo_item_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::btn_fechar_click));
    ui.register_handler("frmFinAplicacoesFinanceirasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasCad::edt_busca_change));
    ui.register_form_class("frmFinAplicacoesFinanceirasLista", FrmFinAplicacoesFinanceirasLista::bind);
    ui.register_handler("frmFinAplicacoesFinanceirasLista.form_show", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasLista::form_show));
    ui.register_handler("frmFinAplicacoesFinanceirasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasLista::btn_abrir_click));
    ui.register_handler("frmFinAplicacoesFinanceirasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasLista::edt_busca_change));
    ui.register_handler("frmFinAplicacoesFinanceirasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinAplicacoesFinanceirasConsulta", FrmFinAplicacoesFinanceirasConsulta::bind);
    ui.register_handler("frmFinAplicacoesFinanceirasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasConsulta::form_show));
    ui.register_handler("frmFinAplicacoesFinanceirasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasConsulta::edt_busca_change));
    ui.register_form_class("frmFinAplicacoesFinanceirasHist", FrmFinAplicacoesFinanceirasHist::bind);
    ui.register_handler("frmFinAplicacoesFinanceirasHist.form_show", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasHist::form_show));
    ui.register_handler("frmFinAplicacoesFinanceirasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinAplicacoesFinanceirasHist::edt_busca_change));
    ui.register_form_class("frmFinFluxosDeCaixaCad", FrmFinFluxosDeCaixaCad::bind);
    ui.register_handler("frmFinFluxosDeCaixaCad.form_show", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::form_show));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_novo_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_salvar_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_excluir_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_cancelar_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_novo_item_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::btn_fechar_click));
    ui.register_handler("frmFinFluxosDeCaixaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaCad::edt_busca_change));
    ui.register_form_class("frmFinFluxosDeCaixaLista", FrmFinFluxosDeCaixaLista::bind);
    ui.register_handler("frmFinFluxosDeCaixaLista.form_show", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaLista::form_show));
    ui.register_handler("frmFinFluxosDeCaixaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaLista::btn_abrir_click));
    ui.register_handler("frmFinFluxosDeCaixaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaLista::edt_busca_change));
    ui.register_handler("frmFinFluxosDeCaixaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinFluxosDeCaixaConsulta", FrmFinFluxosDeCaixaConsulta::bind);
    ui.register_handler("frmFinFluxosDeCaixaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaConsulta::form_show));
    ui.register_handler("frmFinFluxosDeCaixaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaConsulta::edt_busca_change));
    ui.register_form_class("frmFinFluxosDeCaixaHist", FrmFinFluxosDeCaixaHist::bind);
    ui.register_handler("frmFinFluxosDeCaixaHist.form_show", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaHist::form_show));
    ui.register_handler("frmFinFluxosDeCaixaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinFluxosDeCaixaHist::edt_busca_change));
    ui.register_form_class("frmFinOrcamentosFinanceirosCad", FrmFinOrcamentosFinanceirosCad::bind);
    ui.register_handler("frmFinOrcamentosFinanceirosCad.form_show", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::form_show));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_novo_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_salvar_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_excluir_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_cancelar_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_novo_item_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::btn_fechar_click));
    ui.register_handler("frmFinOrcamentosFinanceirosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosCad::edt_busca_change));
    ui.register_form_class("frmFinOrcamentosFinanceirosLista", FrmFinOrcamentosFinanceirosLista::bind);
    ui.register_handler("frmFinOrcamentosFinanceirosLista.form_show", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosLista::form_show));
    ui.register_handler("frmFinOrcamentosFinanceirosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosLista::btn_abrir_click));
    ui.register_handler("frmFinOrcamentosFinanceirosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosLista::edt_busca_change));
    ui.register_handler("frmFinOrcamentosFinanceirosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinOrcamentosFinanceirosConsulta", FrmFinOrcamentosFinanceirosConsulta::bind);
    ui.register_handler("frmFinOrcamentosFinanceirosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosConsulta::form_show));
    ui.register_handler("frmFinOrcamentosFinanceirosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosConsulta::edt_busca_change));
    ui.register_form_class("frmFinOrcamentosFinanceirosHist", FrmFinOrcamentosFinanceirosHist::bind);
    ui.register_handler("frmFinOrcamentosFinanceirosHist.form_show", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosHist::form_show));
    ui.register_handler("frmFinOrcamentosFinanceirosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinOrcamentosFinanceirosHist::edt_busca_change));
    ui.register_form_class("frmFinChequesEmitidosCad", FrmFinChequesEmitidosCad::bind);
    ui.register_handler("frmFinChequesEmitidosCad.form_show", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::form_show));
    ui.register_handler("frmFinChequesEmitidosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_novo_click));
    ui.register_handler("frmFinChequesEmitidosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_salvar_click));
    ui.register_handler("frmFinChequesEmitidosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_excluir_click));
    ui.register_handler("frmFinChequesEmitidosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_cancelar_click));
    ui.register_handler("frmFinChequesEmitidosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_novo_item_click));
    ui.register_handler("frmFinChequesEmitidosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::btn_fechar_click));
    ui.register_handler("frmFinChequesEmitidosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosCad::edt_busca_change));
    ui.register_form_class("frmFinChequesEmitidosLista", FrmFinChequesEmitidosLista::bind);
    ui.register_handler("frmFinChequesEmitidosLista.form_show", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosLista::form_show));
    ui.register_handler("frmFinChequesEmitidosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosLista::btn_abrir_click));
    ui.register_handler("frmFinChequesEmitidosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosLista::edt_busca_change));
    ui.register_handler("frmFinChequesEmitidosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinChequesEmitidosConsulta", FrmFinChequesEmitidosConsulta::bind);
    ui.register_handler("frmFinChequesEmitidosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosConsulta::form_show));
    ui.register_handler("frmFinChequesEmitidosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosConsulta::edt_busca_change));
    ui.register_form_class("frmFinChequesEmitidosHist", FrmFinChequesEmitidosHist::bind);
    ui.register_handler("frmFinChequesEmitidosHist.form_show", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosHist::form_show));
    ui.register_handler("frmFinChequesEmitidosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesEmitidosHist::edt_busca_change));
    ui.register_form_class("frmFinChequesRecebidosCad", FrmFinChequesRecebidosCad::bind);
    ui.register_handler("frmFinChequesRecebidosCad.form_show", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::form_show));
    ui.register_handler("frmFinChequesRecebidosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_novo_click));
    ui.register_handler("frmFinChequesRecebidosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_salvar_click));
    ui.register_handler("frmFinChequesRecebidosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_excluir_click));
    ui.register_handler("frmFinChequesRecebidosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_cancelar_click));
    ui.register_handler("frmFinChequesRecebidosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_novo_item_click));
    ui.register_handler("frmFinChequesRecebidosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::btn_fechar_click));
    ui.register_handler("frmFinChequesRecebidosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosCad::edt_busca_change));
    ui.register_form_class("frmFinChequesRecebidosLista", FrmFinChequesRecebidosLista::bind);
    ui.register_handler("frmFinChequesRecebidosLista.form_show", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosLista::form_show));
    ui.register_handler("frmFinChequesRecebidosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosLista::btn_abrir_click));
    ui.register_handler("frmFinChequesRecebidosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosLista::edt_busca_change));
    ui.register_handler("frmFinChequesRecebidosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinChequesRecebidosConsulta", FrmFinChequesRecebidosConsulta::bind);
    ui.register_handler("frmFinChequesRecebidosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosConsulta::form_show));
    ui.register_handler("frmFinChequesRecebidosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosConsulta::edt_busca_change));
    ui.register_form_class("frmFinChequesRecebidosHist", FrmFinChequesRecebidosHist::bind);
    ui.register_handler("frmFinChequesRecebidosHist.form_show", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosHist::form_show));
    ui.register_handler("frmFinChequesRecebidosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinChequesRecebidosHist::edt_busca_change));
    ui.register_form_class("frmFinRenegociacoesCad", FrmFinRenegociacoesCad::bind);
    ui.register_handler("frmFinRenegociacoesCad.form_show", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::form_show));
    ui.register_handler("frmFinRenegociacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_novo_click));
    ui.register_handler("frmFinRenegociacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_salvar_click));
    ui.register_handler("frmFinRenegociacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_excluir_click));
    ui.register_handler("frmFinRenegociacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_cancelar_click));
    ui.register_handler("frmFinRenegociacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_novo_item_click));
    ui.register_handler("frmFinRenegociacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::btn_fechar_click));
    ui.register_handler("frmFinRenegociacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRenegociacoesCad::edt_busca_change));
    ui.register_form_class("frmFinRenegociacoesLista", FrmFinRenegociacoesLista::bind);
    ui.register_handler("frmFinRenegociacoesLista.form_show", aylis::prelude::instances::form_handler(FrmFinRenegociacoesLista::form_show));
    ui.register_handler("frmFinRenegociacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesLista::btn_abrir_click));
    ui.register_handler("frmFinRenegociacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRenegociacoesLista::edt_busca_change));
    ui.register_handler("frmFinRenegociacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinRenegociacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinRenegociacoesConsulta", FrmFinRenegociacoesConsulta::bind);
    ui.register_handler("frmFinRenegociacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinRenegociacoesConsulta::form_show));
    ui.register_handler("frmFinRenegociacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRenegociacoesConsulta::edt_busca_change));
    ui.register_form_class("frmFinRenegociacoesHist", FrmFinRenegociacoesHist::bind);
    ui.register_handler("frmFinRenegociacoesHist.form_show", aylis::prelude::instances::form_handler(FrmFinRenegociacoesHist::form_show));
    ui.register_handler("frmFinRenegociacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinRenegociacoesHist::edt_busca_change));
    ui.register_form_class("frmFinPrestacoesDeContasCad", FrmFinPrestacoesDeContasCad::bind);
    ui.register_handler("frmFinPrestacoesDeContasCad.form_show", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::form_show));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_novo_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_salvar_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_excluir_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_cancelar_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_novo_item_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::btn_fechar_click));
    ui.register_handler("frmFinPrestacoesDeContasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasCad::edt_busca_change));
    ui.register_form_class("frmFinPrestacoesDeContasLista", FrmFinPrestacoesDeContasLista::bind);
    ui.register_handler("frmFinPrestacoesDeContasLista.form_show", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasLista::form_show));
    ui.register_handler("frmFinPrestacoesDeContasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasLista::btn_abrir_click));
    ui.register_handler("frmFinPrestacoesDeContasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasLista::edt_busca_change));
    ui.register_handler("frmFinPrestacoesDeContasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFinPrestacoesDeContasConsulta", FrmFinPrestacoesDeContasConsulta::bind);
    ui.register_handler("frmFinPrestacoesDeContasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasConsulta::form_show));
    ui.register_handler("frmFinPrestacoesDeContasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasConsulta::edt_busca_change));
    ui.register_form_class("frmFinPrestacoesDeContasHist", FrmFinPrestacoesDeContasHist::bind);
    ui.register_handler("frmFinPrestacoesDeContasHist.form_show", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasHist::form_show));
    ui.register_handler("frmFinPrestacoesDeContasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFinPrestacoesDeContasHist::edt_busca_change));
    ui.register_form_class("frmCtbPlanoDeContasCad", FrmCtbPlanoDeContasCad::bind);
    ui.register_handler("frmCtbPlanoDeContasCad.form_show", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::form_show));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_novo_click));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_salvar_click));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_excluir_click));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_cancelar_click));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_novo_item_click));
    ui.register_handler("frmCtbPlanoDeContasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::btn_fechar_click));
    ui.register_handler("frmCtbPlanoDeContasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasCad::edt_busca_change));
    ui.register_form_class("frmCtbPlanoDeContasLista", FrmCtbPlanoDeContasLista::bind);
    ui.register_handler("frmCtbPlanoDeContasLista.form_show", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasLista::form_show));
    ui.register_handler("frmCtbPlanoDeContasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasLista::btn_abrir_click));
    ui.register_handler("frmCtbPlanoDeContasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasLista::edt_busca_change));
    ui.register_handler("frmCtbPlanoDeContasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbPlanoDeContasConsulta", FrmCtbPlanoDeContasConsulta::bind);
    ui.register_handler("frmCtbPlanoDeContasConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasConsulta::form_show));
    ui.register_handler("frmCtbPlanoDeContasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasConsulta::edt_busca_change));
    ui.register_form_class("frmCtbPlanoDeContasHist", FrmCtbPlanoDeContasHist::bind);
    ui.register_handler("frmCtbPlanoDeContasHist.form_show", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasHist::form_show));
    ui.register_handler("frmCtbPlanoDeContasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPlanoDeContasHist::edt_busca_change));
    ui.register_form_class("frmCtbHistoricosPadraoCad", FrmCtbHistoricosPadraoCad::bind);
    ui.register_handler("frmCtbHistoricosPadraoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::form_show));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_novo_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_salvar_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_excluir_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_cancelar_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_novo_item_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::btn_fechar_click));
    ui.register_handler("frmCtbHistoricosPadraoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoCad::edt_busca_change));
    ui.register_form_class("frmCtbHistoricosPadraoLista", FrmCtbHistoricosPadraoLista::bind);
    ui.register_handler("frmCtbHistoricosPadraoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoLista::form_show));
    ui.register_handler("frmCtbHistoricosPadraoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoLista::btn_abrir_click));
    ui.register_handler("frmCtbHistoricosPadraoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoLista::edt_busca_change));
    ui.register_handler("frmCtbHistoricosPadraoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbHistoricosPadraoConsulta", FrmCtbHistoricosPadraoConsulta::bind);
    ui.register_handler("frmCtbHistoricosPadraoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoConsulta::form_show));
    ui.register_handler("frmCtbHistoricosPadraoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbHistoricosPadraoHist", FrmCtbHistoricosPadraoHist::bind);
    ui.register_handler("frmCtbHistoricosPadraoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoHist::form_show));
    ui.register_handler("frmCtbHistoricosPadraoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbHistoricosPadraoHist::edt_busca_change));
    ui.register_form_class("frmCtbCentrosDeCustoContabeisCad", FrmCtbCentrosDeCustoContabeisCad::bind);
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::form_show));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbCentrosDeCustoContabeisLista", FrmCtbCentrosDeCustoContabeisLista::bind);
    ui.register_handler("frmCtbCentrosDeCustoContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisLista::form_show));
    ui.register_handler("frmCtbCentrosDeCustoContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbCentrosDeCustoContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbCentrosDeCustoContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbCentrosDeCustoContabeisConsulta", FrmCtbCentrosDeCustoContabeisConsulta::bind);
    ui.register_handler("frmCtbCentrosDeCustoContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisConsulta::form_show));
    ui.register_handler("frmCtbCentrosDeCustoContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbCentrosDeCustoContabeisHist", FrmCtbCentrosDeCustoContabeisHist::bind);
    ui.register_handler("frmCtbCentrosDeCustoContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisHist::form_show));
    ui.register_handler("frmCtbCentrosDeCustoContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCentrosDeCustoContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbExerciciosCad", FrmCtbExerciciosCad::bind);
    ui.register_handler("frmCtbExerciciosCad.form_show", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::form_show));
    ui.register_handler("frmCtbExerciciosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_novo_click));
    ui.register_handler("frmCtbExerciciosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_salvar_click));
    ui.register_handler("frmCtbExerciciosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_excluir_click));
    ui.register_handler("frmCtbExerciciosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_cancelar_click));
    ui.register_handler("frmCtbExerciciosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_novo_item_click));
    ui.register_handler("frmCtbExerciciosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::btn_fechar_click));
    ui.register_handler("frmCtbExerciciosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbExerciciosCad::edt_busca_change));
    ui.register_form_class("frmCtbExerciciosLista", FrmCtbExerciciosLista::bind);
    ui.register_handler("frmCtbExerciciosLista.form_show", aylis::prelude::instances::form_handler(FrmCtbExerciciosLista::form_show));
    ui.register_handler("frmCtbExerciciosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosLista::btn_abrir_click));
    ui.register_handler("frmCtbExerciciosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbExerciciosLista::edt_busca_change));
    ui.register_handler("frmCtbExerciciosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbExerciciosLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbExerciciosConsulta", FrmCtbExerciciosConsulta::bind);
    ui.register_handler("frmCtbExerciciosConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbExerciciosConsulta::form_show));
    ui.register_handler("frmCtbExerciciosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbExerciciosConsulta::edt_busca_change));
    ui.register_form_class("frmCtbExerciciosHist", FrmCtbExerciciosHist::bind);
    ui.register_handler("frmCtbExerciciosHist.form_show", aylis::prelude::instances::form_handler(FrmCtbExerciciosHist::form_show));
    ui.register_handler("frmCtbExerciciosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbExerciciosHist::edt_busca_change));
    ui.register_form_class("frmCtbPeriodosContabeisCad", FrmCtbPeriodosContabeisCad::bind);
    ui.register_handler("frmCtbPeriodosContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::form_show));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbPeriodosContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbPeriodosContabeisLista", FrmCtbPeriodosContabeisLista::bind);
    ui.register_handler("frmCtbPeriodosContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisLista::form_show));
    ui.register_handler("frmCtbPeriodosContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbPeriodosContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbPeriodosContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbPeriodosContabeisConsulta", FrmCtbPeriodosContabeisConsulta::bind);
    ui.register_handler("frmCtbPeriodosContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisConsulta::form_show));
    ui.register_handler("frmCtbPeriodosContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbPeriodosContabeisHist", FrmCtbPeriodosContabeisHist::bind);
    ui.register_handler("frmCtbPeriodosContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisHist::form_show));
    ui.register_handler("frmCtbPeriodosContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbPeriodosContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbEmpresasCad", FrmCtbEmpresasCad::bind);
    ui.register_handler("frmCtbEmpresasCad.form_show", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::form_show));
    ui.register_handler("frmCtbEmpresasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_novo_click));
    ui.register_handler("frmCtbEmpresasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_salvar_click));
    ui.register_handler("frmCtbEmpresasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_excluir_click));
    ui.register_handler("frmCtbEmpresasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_cancelar_click));
    ui.register_handler("frmCtbEmpresasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_novo_item_click));
    ui.register_handler("frmCtbEmpresasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::btn_fechar_click));
    ui.register_handler("frmCtbEmpresasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEmpresasCad::edt_busca_change));
    ui.register_form_class("frmCtbEmpresasLista", FrmCtbEmpresasLista::bind);
    ui.register_handler("frmCtbEmpresasLista.form_show", aylis::prelude::instances::form_handler(FrmCtbEmpresasLista::form_show));
    ui.register_handler("frmCtbEmpresasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasLista::btn_abrir_click));
    ui.register_handler("frmCtbEmpresasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEmpresasLista::edt_busca_change));
    ui.register_handler("frmCtbEmpresasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbEmpresasLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbEmpresasConsulta", FrmCtbEmpresasConsulta::bind);
    ui.register_handler("frmCtbEmpresasConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbEmpresasConsulta::form_show));
    ui.register_handler("frmCtbEmpresasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEmpresasConsulta::edt_busca_change));
    ui.register_form_class("frmCtbEmpresasHist", FrmCtbEmpresasHist::bind);
    ui.register_handler("frmCtbEmpresasHist.form_show", aylis::prelude::instances::form_handler(FrmCtbEmpresasHist::form_show));
    ui.register_handler("frmCtbEmpresasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEmpresasHist::edt_busca_change));
    ui.register_form_class("frmCtbFiliaisCad", FrmCtbFiliaisCad::bind);
    ui.register_handler("frmCtbFiliaisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::form_show));
    ui.register_handler("frmCtbFiliaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_novo_click));
    ui.register_handler("frmCtbFiliaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_salvar_click));
    ui.register_handler("frmCtbFiliaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_excluir_click));
    ui.register_handler("frmCtbFiliaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_cancelar_click));
    ui.register_handler("frmCtbFiliaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_novo_item_click));
    ui.register_handler("frmCtbFiliaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::btn_fechar_click));
    ui.register_handler("frmCtbFiliaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFiliaisCad::edt_busca_change));
    ui.register_form_class("frmCtbFiliaisLista", FrmCtbFiliaisLista::bind);
    ui.register_handler("frmCtbFiliaisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbFiliaisLista::form_show));
    ui.register_handler("frmCtbFiliaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisLista::btn_abrir_click));
    ui.register_handler("frmCtbFiliaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFiliaisLista::edt_busca_change));
    ui.register_handler("frmCtbFiliaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbFiliaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbFiliaisConsulta", FrmCtbFiliaisConsulta::bind);
    ui.register_handler("frmCtbFiliaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbFiliaisConsulta::form_show));
    ui.register_handler("frmCtbFiliaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFiliaisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbFiliaisHist", FrmCtbFiliaisHist::bind);
    ui.register_handler("frmCtbFiliaisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbFiliaisHist::form_show));
    ui.register_handler("frmCtbFiliaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFiliaisHist::edt_busca_change));
    ui.register_form_class("frmCtbMoedasContabeisCad", FrmCtbMoedasContabeisCad::bind);
    ui.register_handler("frmCtbMoedasContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::form_show));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbMoedasContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbMoedasContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbMoedasContabeisLista", FrmCtbMoedasContabeisLista::bind);
    ui.register_handler("frmCtbMoedasContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisLista::form_show));
    ui.register_handler("frmCtbMoedasContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbMoedasContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbMoedasContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbMoedasContabeisConsulta", FrmCtbMoedasContabeisConsulta::bind);
    ui.register_handler("frmCtbMoedasContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisConsulta::form_show));
    ui.register_handler("frmCtbMoedasContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbMoedasContabeisHist", FrmCtbMoedasContabeisHist::bind);
    ui.register_handler("frmCtbMoedasContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisHist::form_show));
    ui.register_handler("frmCtbMoedasContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbMoedasContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeContasCad", FrmCtbGruposDeContasCad::bind);
    ui.register_handler("frmCtbGruposDeContasCad.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::form_show));
    ui.register_handler("frmCtbGruposDeContasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_novo_click));
    ui.register_handler("frmCtbGruposDeContasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_salvar_click));
    ui.register_handler("frmCtbGruposDeContasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_excluir_click));
    ui.register_handler("frmCtbGruposDeContasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_cancelar_click));
    ui.register_handler("frmCtbGruposDeContasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_novo_item_click));
    ui.register_handler("frmCtbGruposDeContasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::btn_fechar_click));
    ui.register_handler("frmCtbGruposDeContasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasCad::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeContasLista", FrmCtbGruposDeContasLista::bind);
    ui.register_handler("frmCtbGruposDeContasLista.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasLista::form_show));
    ui.register_handler("frmCtbGruposDeContasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasLista::btn_abrir_click));
    ui.register_handler("frmCtbGruposDeContasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasLista::edt_busca_change));
    ui.register_handler("frmCtbGruposDeContasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbGruposDeContasConsulta", FrmCtbGruposDeContasConsulta::bind);
    ui.register_handler("frmCtbGruposDeContasConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasConsulta::form_show));
    ui.register_handler("frmCtbGruposDeContasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasConsulta::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeContasHist", FrmCtbGruposDeContasHist::bind);
    ui.register_handler("frmCtbGruposDeContasHist.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasHist::form_show));
    ui.register_handler("frmCtbGruposDeContasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeContasHist::edt_busca_change));
    ui.register_form_class("frmCtbContasReferenciaisCad", FrmCtbContasReferenciaisCad::bind);
    ui.register_handler("frmCtbContasReferenciaisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::form_show));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_novo_click));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_salvar_click));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_excluir_click));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_cancelar_click));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_novo_item_click));
    ui.register_handler("frmCtbContasReferenciaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::btn_fechar_click));
    ui.register_handler("frmCtbContasReferenciaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisCad::edt_busca_change));
    ui.register_form_class("frmCtbContasReferenciaisLista", FrmCtbContasReferenciaisLista::bind);
    ui.register_handler("frmCtbContasReferenciaisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisLista::form_show));
    ui.register_handler("frmCtbContasReferenciaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisLista::btn_abrir_click));
    ui.register_handler("frmCtbContasReferenciaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisLista::edt_busca_change));
    ui.register_handler("frmCtbContasReferenciaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbContasReferenciaisConsulta", FrmCtbContasReferenciaisConsulta::bind);
    ui.register_handler("frmCtbContasReferenciaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisConsulta::form_show));
    ui.register_handler("frmCtbContasReferenciaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbContasReferenciaisHist", FrmCtbContasReferenciaisHist::bind);
    ui.register_handler("frmCtbContasReferenciaisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisHist::form_show));
    ui.register_handler("frmCtbContasReferenciaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbContasReferenciaisHist::edt_busca_change));
    ui.register_form_class("frmCtbResponsaveisContabeisCad", FrmCtbResponsaveisContabeisCad::bind);
    ui.register_handler("frmCtbResponsaveisContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::form_show));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbResponsaveisContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbResponsaveisContabeisLista", FrmCtbResponsaveisContabeisLista::bind);
    ui.register_handler("frmCtbResponsaveisContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisLista::form_show));
    ui.register_handler("frmCtbResponsaveisContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbResponsaveisContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbResponsaveisContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbResponsaveisContabeisConsulta", FrmCtbResponsaveisContabeisConsulta::bind);
    ui.register_handler("frmCtbResponsaveisContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisConsulta::form_show));
    ui.register_handler("frmCtbResponsaveisContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbResponsaveisContabeisHist", FrmCtbResponsaveisContabeisHist::bind);
    ui.register_handler("frmCtbResponsaveisContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisHist::form_show));
    ui.register_handler("frmCtbResponsaveisContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbResponsaveisContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbLivrosContabeisCad", FrmCtbLivrosContabeisCad::bind);
    ui.register_handler("frmCtbLivrosContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::form_show));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbLivrosContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbLivrosContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbLivrosContabeisLista", FrmCtbLivrosContabeisLista::bind);
    ui.register_handler("frmCtbLivrosContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisLista::form_show));
    ui.register_handler("frmCtbLivrosContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbLivrosContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbLivrosContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbLivrosContabeisConsulta", FrmCtbLivrosContabeisConsulta::bind);
    ui.register_handler("frmCtbLivrosContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisConsulta::form_show));
    ui.register_handler("frmCtbLivrosContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbLivrosContabeisHist", FrmCtbLivrosContabeisHist::bind);
    ui.register_handler("frmCtbLivrosContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisHist::form_show));
    ui.register_handler("frmCtbLivrosContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLivrosContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbNaturezasDeSaldoCad", FrmCtbNaturezasDeSaldoCad::bind);
    ui.register_handler("frmCtbNaturezasDeSaldoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::form_show));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_novo_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_salvar_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_excluir_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_cancelar_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_novo_item_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::btn_fechar_click));
    ui.register_handler("frmCtbNaturezasDeSaldoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoCad::edt_busca_change));
    ui.register_form_class("frmCtbNaturezasDeSaldoLista", FrmCtbNaturezasDeSaldoLista::bind);
    ui.register_handler("frmCtbNaturezasDeSaldoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoLista::form_show));
    ui.register_handler("frmCtbNaturezasDeSaldoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoLista::btn_abrir_click));
    ui.register_handler("frmCtbNaturezasDeSaldoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoLista::edt_busca_change));
    ui.register_handler("frmCtbNaturezasDeSaldoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbNaturezasDeSaldoConsulta", FrmCtbNaturezasDeSaldoConsulta::bind);
    ui.register_handler("frmCtbNaturezasDeSaldoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoConsulta::form_show));
    ui.register_handler("frmCtbNaturezasDeSaldoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbNaturezasDeSaldoHist", FrmCtbNaturezasDeSaldoHist::bind);
    ui.register_handler("frmCtbNaturezasDeSaldoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoHist::form_show));
    ui.register_handler("frmCtbNaturezasDeSaldoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNaturezasDeSaldoHist::edt_busca_change));
    ui.register_form_class("frmCtbRegrasDeRateioCad", FrmCtbRegrasDeRateioCad::bind);
    ui.register_handler("frmCtbRegrasDeRateioCad.form_show", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::form_show));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_novo_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_salvar_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_excluir_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_cancelar_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_novo_item_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::btn_fechar_click));
    ui.register_handler("frmCtbRegrasDeRateioCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioCad::edt_busca_change));
    ui.register_form_class("frmCtbRegrasDeRateioLista", FrmCtbRegrasDeRateioLista::bind);
    ui.register_handler("frmCtbRegrasDeRateioLista.form_show", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioLista::form_show));
    ui.register_handler("frmCtbRegrasDeRateioLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioLista::btn_abrir_click));
    ui.register_handler("frmCtbRegrasDeRateioLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioLista::edt_busca_change));
    ui.register_handler("frmCtbRegrasDeRateioLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbRegrasDeRateioConsulta", FrmCtbRegrasDeRateioConsulta::bind);
    ui.register_handler("frmCtbRegrasDeRateioConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioConsulta::form_show));
    ui.register_handler("frmCtbRegrasDeRateioConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioConsulta::edt_busca_change));
    ui.register_form_class("frmCtbRegrasDeRateioHist", FrmCtbRegrasDeRateioHist::bind);
    ui.register_handler("frmCtbRegrasDeRateioHist.form_show", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioHist::form_show));
    ui.register_handler("frmCtbRegrasDeRateioHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRegrasDeRateioHist::edt_busca_change));
    ui.register_form_class("frmCtbCriteriosDeDepreciacaoCad", FrmCtbCriteriosDeDepreciacaoCad::bind);
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::form_show));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_novo_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_salvar_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_excluir_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_cancelar_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_novo_item_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::btn_fechar_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoCad::edt_busca_change));
    ui.register_form_class("frmCtbCriteriosDeDepreciacaoLista", FrmCtbCriteriosDeDepreciacaoLista::bind);
    ui.register_handler("frmCtbCriteriosDeDepreciacaoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoLista::form_show));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoLista::btn_abrir_click));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoLista::edt_busca_change));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbCriteriosDeDepreciacaoConsulta", FrmCtbCriteriosDeDepreciacaoConsulta::bind);
    ui.register_handler("frmCtbCriteriosDeDepreciacaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoConsulta::form_show));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbCriteriosDeDepreciacaoHist", FrmCtbCriteriosDeDepreciacaoHist::bind);
    ui.register_handler("frmCtbCriteriosDeDepreciacaoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoHist::form_show));
    ui.register_handler("frmCtbCriteriosDeDepreciacaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCriteriosDeDepreciacaoHist::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeBensCad", FrmCtbGruposDeBensCad::bind);
    ui.register_handler("frmCtbGruposDeBensCad.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::form_show));
    ui.register_handler("frmCtbGruposDeBensCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_novo_click));
    ui.register_handler("frmCtbGruposDeBensCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_salvar_click));
    ui.register_handler("frmCtbGruposDeBensCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_excluir_click));
    ui.register_handler("frmCtbGruposDeBensCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_cancelar_click));
    ui.register_handler("frmCtbGruposDeBensCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_novo_item_click));
    ui.register_handler("frmCtbGruposDeBensCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::btn_fechar_click));
    ui.register_handler("frmCtbGruposDeBensCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensCad::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeBensLista", FrmCtbGruposDeBensLista::bind);
    ui.register_handler("frmCtbGruposDeBensLista.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensLista::form_show));
    ui.register_handler("frmCtbGruposDeBensLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensLista::btn_abrir_click));
    ui.register_handler("frmCtbGruposDeBensLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensLista::edt_busca_change));
    ui.register_handler("frmCtbGruposDeBensLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbGruposDeBensConsulta", FrmCtbGruposDeBensConsulta::bind);
    ui.register_handler("frmCtbGruposDeBensConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensConsulta::form_show));
    ui.register_handler("frmCtbGruposDeBensConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensConsulta::edt_busca_change));
    ui.register_form_class("frmCtbGruposDeBensHist", FrmCtbGruposDeBensHist::bind);
    ui.register_handler("frmCtbGruposDeBensHist.form_show", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensHist::form_show));
    ui.register_handler("frmCtbGruposDeBensHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbGruposDeBensHist::edt_busca_change));
    ui.register_form_class("frmCtbLocaisDeBensCad", FrmCtbLocaisDeBensCad::bind);
    ui.register_handler("frmCtbLocaisDeBensCad.form_show", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::form_show));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_novo_click));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_salvar_click));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_excluir_click));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_cancelar_click));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_novo_item_click));
    ui.register_handler("frmCtbLocaisDeBensCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::btn_fechar_click));
    ui.register_handler("frmCtbLocaisDeBensCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensCad::edt_busca_change));
    ui.register_form_class("frmCtbLocaisDeBensLista", FrmCtbLocaisDeBensLista::bind);
    ui.register_handler("frmCtbLocaisDeBensLista.form_show", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensLista::form_show));
    ui.register_handler("frmCtbLocaisDeBensLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensLista::btn_abrir_click));
    ui.register_handler("frmCtbLocaisDeBensLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensLista::edt_busca_change));
    ui.register_handler("frmCtbLocaisDeBensLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbLocaisDeBensConsulta", FrmCtbLocaisDeBensConsulta::bind);
    ui.register_handler("frmCtbLocaisDeBensConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensConsulta::form_show));
    ui.register_handler("frmCtbLocaisDeBensConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensConsulta::edt_busca_change));
    ui.register_form_class("frmCtbLocaisDeBensHist", FrmCtbLocaisDeBensHist::bind);
    ui.register_handler("frmCtbLocaisDeBensHist.form_show", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensHist::form_show));
    ui.register_handler("frmCtbLocaisDeBensHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLocaisDeBensHist::edt_busca_change));
    ui.register_form_class("frmCtbTiposDeLancamentoCad", FrmCtbTiposDeLancamentoCad::bind);
    ui.register_handler("frmCtbTiposDeLancamentoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::form_show));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_novo_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_salvar_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_excluir_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_cancelar_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_novo_item_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::btn_fechar_click));
    ui.register_handler("frmCtbTiposDeLancamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoCad::edt_busca_change));
    ui.register_form_class("frmCtbTiposDeLancamentoLista", FrmCtbTiposDeLancamentoLista::bind);
    ui.register_handler("frmCtbTiposDeLancamentoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoLista::form_show));
    ui.register_handler("frmCtbTiposDeLancamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoLista::btn_abrir_click));
    ui.register_handler("frmCtbTiposDeLancamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoLista::edt_busca_change));
    ui.register_handler("frmCtbTiposDeLancamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbTiposDeLancamentoConsulta", FrmCtbTiposDeLancamentoConsulta::bind);
    ui.register_handler("frmCtbTiposDeLancamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoConsulta::form_show));
    ui.register_handler("frmCtbTiposDeLancamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbTiposDeLancamentoHist", FrmCtbTiposDeLancamentoHist::bind);
    ui.register_handler("frmCtbTiposDeLancamentoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoHist::form_show));
    ui.register_handler("frmCtbTiposDeLancamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTiposDeLancamentoHist::edt_busca_change));
    ui.register_form_class("frmCtbAgrupadoresDeDreCad", FrmCtbAgrupadoresDeDreCad::bind);
    ui.register_handler("frmCtbAgrupadoresDeDreCad.form_show", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::form_show));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_novo_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_salvar_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_excluir_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_cancelar_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_novo_item_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::btn_fechar_click));
    ui.register_handler("frmCtbAgrupadoresDeDreCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreCad::edt_busca_change));
    ui.register_form_class("frmCtbAgrupadoresDeDreLista", FrmCtbAgrupadoresDeDreLista::bind);
    ui.register_handler("frmCtbAgrupadoresDeDreLista.form_show", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreLista::form_show));
    ui.register_handler("frmCtbAgrupadoresDeDreLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreLista::btn_abrir_click));
    ui.register_handler("frmCtbAgrupadoresDeDreLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreLista::edt_busca_change));
    ui.register_handler("frmCtbAgrupadoresDeDreLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbAgrupadoresDeDreConsulta", FrmCtbAgrupadoresDeDreConsulta::bind);
    ui.register_handler("frmCtbAgrupadoresDeDreConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreConsulta::form_show));
    ui.register_handler("frmCtbAgrupadoresDeDreConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreConsulta::edt_busca_change));
    ui.register_form_class("frmCtbAgrupadoresDeDreHist", FrmCtbAgrupadoresDeDreHist::bind);
    ui.register_handler("frmCtbAgrupadoresDeDreHist.form_show", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreHist::form_show));
    ui.register_handler("frmCtbAgrupadoresDeDreHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAgrupadoresDeDreHist::edt_busca_change));
    ui.register_form_class("frmCtbNotasExplicativasCad", FrmCtbNotasExplicativasCad::bind);
    ui.register_handler("frmCtbNotasExplicativasCad.form_show", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::form_show));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_novo_click));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_salvar_click));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_excluir_click));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_cancelar_click));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_novo_item_click));
    ui.register_handler("frmCtbNotasExplicativasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::btn_fechar_click));
    ui.register_handler("frmCtbNotasExplicativasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasCad::edt_busca_change));
    ui.register_form_class("frmCtbNotasExplicativasLista", FrmCtbNotasExplicativasLista::bind);
    ui.register_handler("frmCtbNotasExplicativasLista.form_show", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasLista::form_show));
    ui.register_handler("frmCtbNotasExplicativasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasLista::btn_abrir_click));
    ui.register_handler("frmCtbNotasExplicativasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasLista::edt_busca_change));
    ui.register_handler("frmCtbNotasExplicativasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbNotasExplicativasConsulta", FrmCtbNotasExplicativasConsulta::bind);
    ui.register_handler("frmCtbNotasExplicativasConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasConsulta::form_show));
    ui.register_handler("frmCtbNotasExplicativasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasConsulta::edt_busca_change));
    ui.register_form_class("frmCtbNotasExplicativasHist", FrmCtbNotasExplicativasHist::bind);
    ui.register_handler("frmCtbNotasExplicativasHist.form_show", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasHist::form_show));
    ui.register_handler("frmCtbNotasExplicativasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbNotasExplicativasHist::edt_busca_change));
    ui.register_form_class("frmCtbCalendariosContabeisCad", FrmCtbCalendariosContabeisCad::bind);
    ui.register_handler("frmCtbCalendariosContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::form_show));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbCalendariosContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbCalendariosContabeisLista", FrmCtbCalendariosContabeisLista::bind);
    ui.register_handler("frmCtbCalendariosContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisLista::form_show));
    ui.register_handler("frmCtbCalendariosContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbCalendariosContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbCalendariosContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbCalendariosContabeisConsulta", FrmCtbCalendariosContabeisConsulta::bind);
    ui.register_handler("frmCtbCalendariosContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisConsulta::form_show));
    ui.register_handler("frmCtbCalendariosContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbCalendariosContabeisHist", FrmCtbCalendariosContabeisHist::bind);
    ui.register_handler("frmCtbCalendariosContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisHist::form_show));
    ui.register_handler("frmCtbCalendariosContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbCalendariosContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbLancamentosContabeisCad", FrmCtbLancamentosContabeisCad::bind);
    ui.register_handler("frmCtbLancamentosContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::form_show));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbLancamentosContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbLancamentosContabeisLista", FrmCtbLancamentosContabeisLista::bind);
    ui.register_handler("frmCtbLancamentosContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisLista::form_show));
    ui.register_handler("frmCtbLancamentosContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbLancamentosContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbLancamentosContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbLancamentosContabeisConsulta", FrmCtbLancamentosContabeisConsulta::bind);
    ui.register_handler("frmCtbLancamentosContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisConsulta::form_show));
    ui.register_handler("frmCtbLancamentosContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbLancamentosContabeisHist", FrmCtbLancamentosContabeisHist::bind);
    ui.register_handler("frmCtbLancamentosContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisHist::form_show));
    ui.register_handler("frmCtbLancamentosContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLancamentosContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbLotesDeLancamentoCad", FrmCtbLotesDeLancamentoCad::bind);
    ui.register_handler("frmCtbLotesDeLancamentoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::form_show));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_novo_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_salvar_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_excluir_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_cancelar_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_novo_item_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::btn_fechar_click));
    ui.register_handler("frmCtbLotesDeLancamentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoCad::edt_busca_change));
    ui.register_form_class("frmCtbLotesDeLancamentoLista", FrmCtbLotesDeLancamentoLista::bind);
    ui.register_handler("frmCtbLotesDeLancamentoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoLista::form_show));
    ui.register_handler("frmCtbLotesDeLancamentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoLista::btn_abrir_click));
    ui.register_handler("frmCtbLotesDeLancamentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoLista::edt_busca_change));
    ui.register_handler("frmCtbLotesDeLancamentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbLotesDeLancamentoConsulta", FrmCtbLotesDeLancamentoConsulta::bind);
    ui.register_handler("frmCtbLotesDeLancamentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoConsulta::form_show));
    ui.register_handler("frmCtbLotesDeLancamentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbLotesDeLancamentoHist", FrmCtbLotesDeLancamentoHist::bind);
    ui.register_handler("frmCtbLotesDeLancamentoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoHist::form_show));
    ui.register_handler("frmCtbLotesDeLancamentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbLotesDeLancamentoHist::edt_busca_change));
    ui.register_form_class("frmCtbRateiosCad", FrmCtbRateiosCad::bind);
    ui.register_handler("frmCtbRateiosCad.form_show", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::form_show));
    ui.register_handler("frmCtbRateiosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_novo_click));
    ui.register_handler("frmCtbRateiosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_salvar_click));
    ui.register_handler("frmCtbRateiosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_excluir_click));
    ui.register_handler("frmCtbRateiosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_cancelar_click));
    ui.register_handler("frmCtbRateiosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_novo_item_click));
    ui.register_handler("frmCtbRateiosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::btn_fechar_click));
    ui.register_handler("frmCtbRateiosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRateiosCad::edt_busca_change));
    ui.register_form_class("frmCtbRateiosLista", FrmCtbRateiosLista::bind);
    ui.register_handler("frmCtbRateiosLista.form_show", aylis::prelude::instances::form_handler(FrmCtbRateiosLista::form_show));
    ui.register_handler("frmCtbRateiosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbRateiosLista::btn_abrir_click));
    ui.register_handler("frmCtbRateiosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRateiosLista::edt_busca_change));
    ui.register_handler("frmCtbRateiosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbRateiosLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbRateiosConsulta", FrmCtbRateiosConsulta::bind);
    ui.register_handler("frmCtbRateiosConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbRateiosConsulta::form_show));
    ui.register_handler("frmCtbRateiosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRateiosConsulta::edt_busca_change));
    ui.register_form_class("frmCtbRateiosHist", FrmCtbRateiosHist::bind);
    ui.register_handler("frmCtbRateiosHist.form_show", aylis::prelude::instances::form_handler(FrmCtbRateiosHist::form_show));
    ui.register_handler("frmCtbRateiosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRateiosHist::edt_busca_change));
    ui.register_form_class("frmCtbEncerramentosCad", FrmCtbEncerramentosCad::bind);
    ui.register_handler("frmCtbEncerramentosCad.form_show", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::form_show));
    ui.register_handler("frmCtbEncerramentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_novo_click));
    ui.register_handler("frmCtbEncerramentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_salvar_click));
    ui.register_handler("frmCtbEncerramentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_excluir_click));
    ui.register_handler("frmCtbEncerramentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_cancelar_click));
    ui.register_handler("frmCtbEncerramentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_novo_item_click));
    ui.register_handler("frmCtbEncerramentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::btn_fechar_click));
    ui.register_handler("frmCtbEncerramentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEncerramentosCad::edt_busca_change));
    ui.register_form_class("frmCtbEncerramentosLista", FrmCtbEncerramentosLista::bind);
    ui.register_handler("frmCtbEncerramentosLista.form_show", aylis::prelude::instances::form_handler(FrmCtbEncerramentosLista::form_show));
    ui.register_handler("frmCtbEncerramentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosLista::btn_abrir_click));
    ui.register_handler("frmCtbEncerramentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEncerramentosLista::edt_busca_change));
    ui.register_handler("frmCtbEncerramentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbEncerramentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbEncerramentosConsulta", FrmCtbEncerramentosConsulta::bind);
    ui.register_handler("frmCtbEncerramentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbEncerramentosConsulta::form_show));
    ui.register_handler("frmCtbEncerramentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEncerramentosConsulta::edt_busca_change));
    ui.register_form_class("frmCtbEncerramentosHist", FrmCtbEncerramentosHist::bind);
    ui.register_handler("frmCtbEncerramentosHist.form_show", aylis::prelude::instances::form_handler(FrmCtbEncerramentosHist::form_show));
    ui.register_handler("frmCtbEncerramentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEncerramentosHist::edt_busca_change));
    ui.register_form_class("frmCtbApuracoesDeResultadoCad", FrmCtbApuracoesDeResultadoCad::bind);
    ui.register_handler("frmCtbApuracoesDeResultadoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::form_show));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_novo_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_salvar_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_excluir_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_cancelar_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_novo_item_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::btn_fechar_click));
    ui.register_handler("frmCtbApuracoesDeResultadoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoCad::edt_busca_change));
    ui.register_form_class("frmCtbApuracoesDeResultadoLista", FrmCtbApuracoesDeResultadoLista::bind);
    ui.register_handler("frmCtbApuracoesDeResultadoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoLista::form_show));
    ui.register_handler("frmCtbApuracoesDeResultadoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoLista::btn_abrir_click));
    ui.register_handler("frmCtbApuracoesDeResultadoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoLista::edt_busca_change));
    ui.register_handler("frmCtbApuracoesDeResultadoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbApuracoesDeResultadoConsulta", FrmCtbApuracoesDeResultadoConsulta::bind);
    ui.register_handler("frmCtbApuracoesDeResultadoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoConsulta::form_show));
    ui.register_handler("frmCtbApuracoesDeResultadoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbApuracoesDeResultadoHist", FrmCtbApuracoesDeResultadoHist::bind);
    ui.register_handler("frmCtbApuracoesDeResultadoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoHist::form_show));
    ui.register_handler("frmCtbApuracoesDeResultadoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbApuracoesDeResultadoHist::edt_busca_change));
    ui.register_form_class("frmCtbBalancetesCad", FrmCtbBalancetesCad::bind);
    ui.register_handler("frmCtbBalancetesCad.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::form_show));
    ui.register_handler("frmCtbBalancetesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_novo_click));
    ui.register_handler("frmCtbBalancetesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_salvar_click));
    ui.register_handler("frmCtbBalancetesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_excluir_click));
    ui.register_handler("frmCtbBalancetesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_cancelar_click));
    ui.register_handler("frmCtbBalancetesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_novo_item_click));
    ui.register_handler("frmCtbBalancetesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::btn_fechar_click));
    ui.register_handler("frmCtbBalancetesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancetesCad::edt_busca_change));
    ui.register_form_class("frmCtbBalancetesLista", FrmCtbBalancetesLista::bind);
    ui.register_handler("frmCtbBalancetesLista.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancetesLista::form_show));
    ui.register_handler("frmCtbBalancetesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesLista::btn_abrir_click));
    ui.register_handler("frmCtbBalancetesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancetesLista::edt_busca_change));
    ui.register_handler("frmCtbBalancetesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbBalancetesLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbBalancetesConsulta", FrmCtbBalancetesConsulta::bind);
    ui.register_handler("frmCtbBalancetesConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancetesConsulta::form_show));
    ui.register_handler("frmCtbBalancetesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancetesConsulta::edt_busca_change));
    ui.register_form_class("frmCtbBalancetesHist", FrmCtbBalancetesHist::bind);
    ui.register_handler("frmCtbBalancetesHist.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancetesHist::form_show));
    ui.register_handler("frmCtbBalancetesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancetesHist::edt_busca_change));
    ui.register_form_class("frmCtbBalancosCad", FrmCtbBalancosCad::bind);
    ui.register_handler("frmCtbBalancosCad.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::form_show));
    ui.register_handler("frmCtbBalancosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_novo_click));
    ui.register_handler("frmCtbBalancosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_salvar_click));
    ui.register_handler("frmCtbBalancosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_excluir_click));
    ui.register_handler("frmCtbBalancosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_cancelar_click));
    ui.register_handler("frmCtbBalancosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_novo_item_click));
    ui.register_handler("frmCtbBalancosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::btn_fechar_click));
    ui.register_handler("frmCtbBalancosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancosCad::edt_busca_change));
    ui.register_form_class("frmCtbBalancosLista", FrmCtbBalancosLista::bind);
    ui.register_handler("frmCtbBalancosLista.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancosLista::form_show));
    ui.register_handler("frmCtbBalancosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbBalancosLista::btn_abrir_click));
    ui.register_handler("frmCtbBalancosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancosLista::edt_busca_change));
    ui.register_handler("frmCtbBalancosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbBalancosLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbBalancosConsulta", FrmCtbBalancosConsulta::bind);
    ui.register_handler("frmCtbBalancosConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancosConsulta::form_show));
    ui.register_handler("frmCtbBalancosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancosConsulta::edt_busca_change));
    ui.register_form_class("frmCtbBalancosHist", FrmCtbBalancosHist::bind);
    ui.register_handler("frmCtbBalancosHist.form_show", aylis::prelude::instances::form_handler(FrmCtbBalancosHist::form_show));
    ui.register_handler("frmCtbBalancosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBalancosHist::edt_busca_change));
    ui.register_form_class("frmCtbDresCad", FrmCtbDresCad::bind);
    ui.register_handler("frmCtbDresCad.form_show", aylis::prelude::instances::form_handler(FrmCtbDresCad::form_show));
    ui.register_handler("frmCtbDresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_novo_click));
    ui.register_handler("frmCtbDresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_salvar_click));
    ui.register_handler("frmCtbDresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_excluir_click));
    ui.register_handler("frmCtbDresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_cancelar_click));
    ui.register_handler("frmCtbDresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_novo_item_click));
    ui.register_handler("frmCtbDresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbDresCad::btn_fechar_click));
    ui.register_handler("frmCtbDresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDresCad::edt_busca_change));
    ui.register_form_class("frmCtbDresLista", FrmCtbDresLista::bind);
    ui.register_handler("frmCtbDresLista.form_show", aylis::prelude::instances::form_handler(FrmCtbDresLista::form_show));
    ui.register_handler("frmCtbDresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbDresLista::btn_abrir_click));
    ui.register_handler("frmCtbDresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDresLista::edt_busca_change));
    ui.register_handler("frmCtbDresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbDresLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbDresConsulta", FrmCtbDresConsulta::bind);
    ui.register_handler("frmCtbDresConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbDresConsulta::form_show));
    ui.register_handler("frmCtbDresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDresConsulta::edt_busca_change));
    ui.register_form_class("frmCtbDresHist", FrmCtbDresHist::bind);
    ui.register_handler("frmCtbDresHist.form_show", aylis::prelude::instances::form_handler(FrmCtbDresHist::form_show));
    ui.register_handler("frmCtbDresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDresHist::edt_busca_change));
    ui.register_form_class("frmCtbRazoesCad", FrmCtbRazoesCad::bind);
    ui.register_handler("frmCtbRazoesCad.form_show", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::form_show));
    ui.register_handler("frmCtbRazoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_novo_click));
    ui.register_handler("frmCtbRazoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_salvar_click));
    ui.register_handler("frmCtbRazoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_excluir_click));
    ui.register_handler("frmCtbRazoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_cancelar_click));
    ui.register_handler("frmCtbRazoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_novo_item_click));
    ui.register_handler("frmCtbRazoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::btn_fechar_click));
    ui.register_handler("frmCtbRazoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRazoesCad::edt_busca_change));
    ui.register_form_class("frmCtbRazoesLista", FrmCtbRazoesLista::bind);
    ui.register_handler("frmCtbRazoesLista.form_show", aylis::prelude::instances::form_handler(FrmCtbRazoesLista::form_show));
    ui.register_handler("frmCtbRazoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbRazoesLista::btn_abrir_click));
    ui.register_handler("frmCtbRazoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRazoesLista::edt_busca_change));
    ui.register_handler("frmCtbRazoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbRazoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbRazoesConsulta", FrmCtbRazoesConsulta::bind);
    ui.register_handler("frmCtbRazoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbRazoesConsulta::form_show));
    ui.register_handler("frmCtbRazoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRazoesConsulta::edt_busca_change));
    ui.register_form_class("frmCtbRazoesHist", FrmCtbRazoesHist::bind);
    ui.register_handler("frmCtbRazoesHist.form_show", aylis::prelude::instances::form_handler(FrmCtbRazoesHist::form_show));
    ui.register_handler("frmCtbRazoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbRazoesHist::edt_busca_change));
    ui.register_form_class("frmCtbDiariosCad", FrmCtbDiariosCad::bind);
    ui.register_handler("frmCtbDiariosCad.form_show", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::form_show));
    ui.register_handler("frmCtbDiariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_novo_click));
    ui.register_handler("frmCtbDiariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_salvar_click));
    ui.register_handler("frmCtbDiariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_excluir_click));
    ui.register_handler("frmCtbDiariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_cancelar_click));
    ui.register_handler("frmCtbDiariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_novo_item_click));
    ui.register_handler("frmCtbDiariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::btn_fechar_click));
    ui.register_handler("frmCtbDiariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDiariosCad::edt_busca_change));
    ui.register_form_class("frmCtbDiariosLista", FrmCtbDiariosLista::bind);
    ui.register_handler("frmCtbDiariosLista.form_show", aylis::prelude::instances::form_handler(FrmCtbDiariosLista::form_show));
    ui.register_handler("frmCtbDiariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbDiariosLista::btn_abrir_click));
    ui.register_handler("frmCtbDiariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDiariosLista::edt_busca_change));
    ui.register_handler("frmCtbDiariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbDiariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbDiariosConsulta", FrmCtbDiariosConsulta::bind);
    ui.register_handler("frmCtbDiariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbDiariosConsulta::form_show));
    ui.register_handler("frmCtbDiariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDiariosConsulta::edt_busca_change));
    ui.register_form_class("frmCtbDiariosHist", FrmCtbDiariosHist::bind);
    ui.register_handler("frmCtbDiariosHist.form_show", aylis::prelude::instances::form_handler(FrmCtbDiariosHist::form_show));
    ui.register_handler("frmCtbDiariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDiariosHist::edt_busca_change));
    ui.register_form_class("frmCtbConciliacoesContabeisCad", FrmCtbConciliacoesContabeisCad::bind);
    ui.register_handler("frmCtbConciliacoesContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::form_show));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbConciliacoesContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbConciliacoesContabeisLista", FrmCtbConciliacoesContabeisLista::bind);
    ui.register_handler("frmCtbConciliacoesContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisLista::form_show));
    ui.register_handler("frmCtbConciliacoesContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbConciliacoesContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbConciliacoesContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbConciliacoesContabeisConsulta", FrmCtbConciliacoesContabeisConsulta::bind);
    ui.register_handler("frmCtbConciliacoesContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisConsulta::form_show));
    ui.register_handler("frmCtbConciliacoesContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbConciliacoesContabeisHist", FrmCtbConciliacoesContabeisHist::bind);
    ui.register_handler("frmCtbConciliacoesContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisHist::form_show));
    ui.register_handler("frmCtbConciliacoesContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbConciliacoesContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbBensPatrimoniaisCad", FrmCtbBensPatrimoniaisCad::bind);
    ui.register_handler("frmCtbBensPatrimoniaisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::form_show));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_novo_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_salvar_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_excluir_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_cancelar_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_novo_item_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::btn_fechar_click));
    ui.register_handler("frmCtbBensPatrimoniaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisCad::edt_busca_change));
    ui.register_form_class("frmCtbBensPatrimoniaisLista", FrmCtbBensPatrimoniaisLista::bind);
    ui.register_handler("frmCtbBensPatrimoniaisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisLista::form_show));
    ui.register_handler("frmCtbBensPatrimoniaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisLista::btn_abrir_click));
    ui.register_handler("frmCtbBensPatrimoniaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisLista::edt_busca_change));
    ui.register_handler("frmCtbBensPatrimoniaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbBensPatrimoniaisConsulta", FrmCtbBensPatrimoniaisConsulta::bind);
    ui.register_handler("frmCtbBensPatrimoniaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisConsulta::form_show));
    ui.register_handler("frmCtbBensPatrimoniaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbBensPatrimoniaisHist", FrmCtbBensPatrimoniaisHist::bind);
    ui.register_handler("frmCtbBensPatrimoniaisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisHist::form_show));
    ui.register_handler("frmCtbBensPatrimoniaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBensPatrimoniaisHist::edt_busca_change));
    ui.register_form_class("frmCtbDepreciacoesCad", FrmCtbDepreciacoesCad::bind);
    ui.register_handler("frmCtbDepreciacoesCad.form_show", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::form_show));
    ui.register_handler("frmCtbDepreciacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_novo_click));
    ui.register_handler("frmCtbDepreciacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_salvar_click));
    ui.register_handler("frmCtbDepreciacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_excluir_click));
    ui.register_handler("frmCtbDepreciacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_cancelar_click));
    ui.register_handler("frmCtbDepreciacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_novo_item_click));
    ui.register_handler("frmCtbDepreciacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::btn_fechar_click));
    ui.register_handler("frmCtbDepreciacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesCad::edt_busca_change));
    ui.register_form_class("frmCtbDepreciacoesLista", FrmCtbDepreciacoesLista::bind);
    ui.register_handler("frmCtbDepreciacoesLista.form_show", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesLista::form_show));
    ui.register_handler("frmCtbDepreciacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesLista::btn_abrir_click));
    ui.register_handler("frmCtbDepreciacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesLista::edt_busca_change));
    ui.register_handler("frmCtbDepreciacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbDepreciacoesConsulta", FrmCtbDepreciacoesConsulta::bind);
    ui.register_handler("frmCtbDepreciacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesConsulta::form_show));
    ui.register_handler("frmCtbDepreciacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesConsulta::edt_busca_change));
    ui.register_form_class("frmCtbDepreciacoesHist", FrmCtbDepreciacoesHist::bind);
    ui.register_handler("frmCtbDepreciacoesHist.form_show", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesHist::form_show));
    ui.register_handler("frmCtbDepreciacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbDepreciacoesHist::edt_busca_change));
    ui.register_form_class("frmCtbBaixasDeBensCad", FrmCtbBaixasDeBensCad::bind);
    ui.register_handler("frmCtbBaixasDeBensCad.form_show", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::form_show));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_novo_click));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_salvar_click));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_excluir_click));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_cancelar_click));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_novo_item_click));
    ui.register_handler("frmCtbBaixasDeBensCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::btn_fechar_click));
    ui.register_handler("frmCtbBaixasDeBensCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensCad::edt_busca_change));
    ui.register_form_class("frmCtbBaixasDeBensLista", FrmCtbBaixasDeBensLista::bind);
    ui.register_handler("frmCtbBaixasDeBensLista.form_show", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensLista::form_show));
    ui.register_handler("frmCtbBaixasDeBensLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensLista::btn_abrir_click));
    ui.register_handler("frmCtbBaixasDeBensLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensLista::edt_busca_change));
    ui.register_handler("frmCtbBaixasDeBensLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbBaixasDeBensConsulta", FrmCtbBaixasDeBensConsulta::bind);
    ui.register_handler("frmCtbBaixasDeBensConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensConsulta::form_show));
    ui.register_handler("frmCtbBaixasDeBensConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensConsulta::edt_busca_change));
    ui.register_form_class("frmCtbBaixasDeBensHist", FrmCtbBaixasDeBensHist::bind);
    ui.register_handler("frmCtbBaixasDeBensHist.form_show", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensHist::form_show));
    ui.register_handler("frmCtbBaixasDeBensHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbBaixasDeBensHist::edt_busca_change));
    ui.register_form_class("frmCtbTransferenciasDeBensCad", FrmCtbTransferenciasDeBensCad::bind);
    ui.register_handler("frmCtbTransferenciasDeBensCad.form_show", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::form_show));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_novo_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_salvar_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_excluir_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_cancelar_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_novo_item_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::btn_fechar_click));
    ui.register_handler("frmCtbTransferenciasDeBensCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensCad::edt_busca_change));
    ui.register_form_class("frmCtbTransferenciasDeBensLista", FrmCtbTransferenciasDeBensLista::bind);
    ui.register_handler("frmCtbTransferenciasDeBensLista.form_show", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensLista::form_show));
    ui.register_handler("frmCtbTransferenciasDeBensLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensLista::btn_abrir_click));
    ui.register_handler("frmCtbTransferenciasDeBensLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensLista::edt_busca_change));
    ui.register_handler("frmCtbTransferenciasDeBensLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbTransferenciasDeBensConsulta", FrmCtbTransferenciasDeBensConsulta::bind);
    ui.register_handler("frmCtbTransferenciasDeBensConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensConsulta::form_show));
    ui.register_handler("frmCtbTransferenciasDeBensConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensConsulta::edt_busca_change));
    ui.register_form_class("frmCtbTransferenciasDeBensHist", FrmCtbTransferenciasDeBensHist::bind);
    ui.register_handler("frmCtbTransferenciasDeBensHist.form_show", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensHist::form_show));
    ui.register_handler("frmCtbTransferenciasDeBensHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbTransferenciasDeBensHist::edt_busca_change));
    ui.register_form_class("frmCtbInventariosPatrimoniaisCad", FrmCtbInventariosPatrimoniaisCad::bind);
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::form_show));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_novo_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_salvar_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_excluir_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_cancelar_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_novo_item_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::btn_fechar_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisCad::edt_busca_change));
    ui.register_form_class("frmCtbInventariosPatrimoniaisLista", FrmCtbInventariosPatrimoniaisLista::bind);
    ui.register_handler("frmCtbInventariosPatrimoniaisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisLista::form_show));
    ui.register_handler("frmCtbInventariosPatrimoniaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisLista::btn_abrir_click));
    ui.register_handler("frmCtbInventariosPatrimoniaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisLista::edt_busca_change));
    ui.register_handler("frmCtbInventariosPatrimoniaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbInventariosPatrimoniaisConsulta", FrmCtbInventariosPatrimoniaisConsulta::bind);
    ui.register_handler("frmCtbInventariosPatrimoniaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisConsulta::form_show));
    ui.register_handler("frmCtbInventariosPatrimoniaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbInventariosPatrimoniaisHist", FrmCtbInventariosPatrimoniaisHist::bind);
    ui.register_handler("frmCtbInventariosPatrimoniaisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisHist::form_show));
    ui.register_handler("frmCtbInventariosPatrimoniaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbInventariosPatrimoniaisHist::edt_busca_change));
    ui.register_form_class("frmCtbProvisoesCad", FrmCtbProvisoesCad::bind);
    ui.register_handler("frmCtbProvisoesCad.form_show", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::form_show));
    ui.register_handler("frmCtbProvisoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_novo_click));
    ui.register_handler("frmCtbProvisoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_salvar_click));
    ui.register_handler("frmCtbProvisoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_excluir_click));
    ui.register_handler("frmCtbProvisoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_cancelar_click));
    ui.register_handler("frmCtbProvisoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_novo_item_click));
    ui.register_handler("frmCtbProvisoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::btn_fechar_click));
    ui.register_handler("frmCtbProvisoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbProvisoesCad::edt_busca_change));
    ui.register_form_class("frmCtbProvisoesLista", FrmCtbProvisoesLista::bind);
    ui.register_handler("frmCtbProvisoesLista.form_show", aylis::prelude::instances::form_handler(FrmCtbProvisoesLista::form_show));
    ui.register_handler("frmCtbProvisoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesLista::btn_abrir_click));
    ui.register_handler("frmCtbProvisoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbProvisoesLista::edt_busca_change));
    ui.register_handler("frmCtbProvisoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbProvisoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbProvisoesConsulta", FrmCtbProvisoesConsulta::bind);
    ui.register_handler("frmCtbProvisoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbProvisoesConsulta::form_show));
    ui.register_handler("frmCtbProvisoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbProvisoesConsulta::edt_busca_change));
    ui.register_form_class("frmCtbProvisoesHist", FrmCtbProvisoesHist::bind);
    ui.register_handler("frmCtbProvisoesHist.form_show", aylis::prelude::instances::form_handler(FrmCtbProvisoesHist::form_show));
    ui.register_handler("frmCtbProvisoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbProvisoesHist::edt_busca_change));
    ui.register_form_class("frmCtbAjustesDeSaldoCad", FrmCtbAjustesDeSaldoCad::bind);
    ui.register_handler("frmCtbAjustesDeSaldoCad.form_show", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::form_show));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_novo_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_salvar_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_excluir_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_cancelar_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_novo_item_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::btn_fechar_click));
    ui.register_handler("frmCtbAjustesDeSaldoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoCad::edt_busca_change));
    ui.register_form_class("frmCtbAjustesDeSaldoLista", FrmCtbAjustesDeSaldoLista::bind);
    ui.register_handler("frmCtbAjustesDeSaldoLista.form_show", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoLista::form_show));
    ui.register_handler("frmCtbAjustesDeSaldoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoLista::btn_abrir_click));
    ui.register_handler("frmCtbAjustesDeSaldoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoLista::edt_busca_change));
    ui.register_handler("frmCtbAjustesDeSaldoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbAjustesDeSaldoConsulta", FrmCtbAjustesDeSaldoConsulta::bind);
    ui.register_handler("frmCtbAjustesDeSaldoConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoConsulta::form_show));
    ui.register_handler("frmCtbAjustesDeSaldoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoConsulta::edt_busca_change));
    ui.register_form_class("frmCtbAjustesDeSaldoHist", FrmCtbAjustesDeSaldoHist::bind);
    ui.register_handler("frmCtbAjustesDeSaldoHist.form_show", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoHist::form_show));
    ui.register_handler("frmCtbAjustesDeSaldoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbAjustesDeSaldoHist::edt_busca_change));
    ui.register_form_class("frmCtbIntegracoesContabeisCad", FrmCtbIntegracoesContabeisCad::bind);
    ui.register_handler("frmCtbIntegracoesContabeisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::form_show));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_novo_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_salvar_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_excluir_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_cancelar_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_novo_item_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::btn_fechar_click));
    ui.register_handler("frmCtbIntegracoesContabeisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisCad::edt_busca_change));
    ui.register_form_class("frmCtbIntegracoesContabeisLista", FrmCtbIntegracoesContabeisLista::bind);
    ui.register_handler("frmCtbIntegracoesContabeisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisLista::form_show));
    ui.register_handler("frmCtbIntegracoesContabeisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisLista::btn_abrir_click));
    ui.register_handler("frmCtbIntegracoesContabeisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisLista::edt_busca_change));
    ui.register_handler("frmCtbIntegracoesContabeisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbIntegracoesContabeisConsulta", FrmCtbIntegracoesContabeisConsulta::bind);
    ui.register_handler("frmCtbIntegracoesContabeisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisConsulta::form_show));
    ui.register_handler("frmCtbIntegracoesContabeisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbIntegracoesContabeisHist", FrmCtbIntegracoesContabeisHist::bind);
    ui.register_handler("frmCtbIntegracoesContabeisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisHist::form_show));
    ui.register_handler("frmCtbIntegracoesContabeisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbIntegracoesContabeisHist::edt_busca_change));
    ui.register_form_class("frmCtbFechamentosMensaisCad", FrmCtbFechamentosMensaisCad::bind);
    ui.register_handler("frmCtbFechamentosMensaisCad.form_show", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::form_show));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_novo_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_salvar_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_excluir_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_cancelar_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_novo_item_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::btn_fechar_click));
    ui.register_handler("frmCtbFechamentosMensaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisCad::edt_busca_change));
    ui.register_form_class("frmCtbFechamentosMensaisLista", FrmCtbFechamentosMensaisLista::bind);
    ui.register_handler("frmCtbFechamentosMensaisLista.form_show", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisLista::form_show));
    ui.register_handler("frmCtbFechamentosMensaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisLista::btn_abrir_click));
    ui.register_handler("frmCtbFechamentosMensaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisLista::edt_busca_change));
    ui.register_handler("frmCtbFechamentosMensaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbFechamentosMensaisConsulta", FrmCtbFechamentosMensaisConsulta::bind);
    ui.register_handler("frmCtbFechamentosMensaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisConsulta::form_show));
    ui.register_handler("frmCtbFechamentosMensaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisConsulta::edt_busca_change));
    ui.register_form_class("frmCtbFechamentosMensaisHist", FrmCtbFechamentosMensaisHist::bind);
    ui.register_handler("frmCtbFechamentosMensaisHist.form_show", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisHist::form_show));
    ui.register_handler("frmCtbFechamentosMensaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbFechamentosMensaisHist::edt_busca_change));
    ui.register_form_class("frmCtbEcdsCad", FrmCtbEcdsCad::bind);
    ui.register_handler("frmCtbEcdsCad.form_show", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::form_show));
    ui.register_handler("frmCtbEcdsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_novo_click));
    ui.register_handler("frmCtbEcdsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_salvar_click));
    ui.register_handler("frmCtbEcdsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_excluir_click));
    ui.register_handler("frmCtbEcdsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_cancelar_click));
    ui.register_handler("frmCtbEcdsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_novo_item_click));
    ui.register_handler("frmCtbEcdsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::btn_fechar_click));
    ui.register_handler("frmCtbEcdsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEcdsCad::edt_busca_change));
    ui.register_form_class("frmCtbEcdsLista", FrmCtbEcdsLista::bind);
    ui.register_handler("frmCtbEcdsLista.form_show", aylis::prelude::instances::form_handler(FrmCtbEcdsLista::form_show));
    ui.register_handler("frmCtbEcdsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmCtbEcdsLista::btn_abrir_click));
    ui.register_handler("frmCtbEcdsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEcdsLista::edt_busca_change));
    ui.register_handler("frmCtbEcdsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmCtbEcdsLista::grd_lista_dbl_click));
    ui.register_form_class("frmCtbEcdsConsulta", FrmCtbEcdsConsulta::bind);
    ui.register_handler("frmCtbEcdsConsulta.form_show", aylis::prelude::instances::form_handler(FrmCtbEcdsConsulta::form_show));
    ui.register_handler("frmCtbEcdsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEcdsConsulta::edt_busca_change));
    ui.register_form_class("frmCtbEcdsHist", FrmCtbEcdsHist::bind);
    ui.register_handler("frmCtbEcdsHist.form_show", aylis::prelude::instances::form_handler(FrmCtbEcdsHist::form_show));
    ui.register_handler("frmCtbEcdsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmCtbEcdsHist::edt_busca_change));
    ui.register_form_class("frmComFornecedoresCad", FrmComFornecedoresCad::bind);
    ui.register_handler("frmComFornecedoresCad.form_show", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::form_show));
    ui.register_handler("frmComFornecedoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_novo_click));
    ui.register_handler("frmComFornecedoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_salvar_click));
    ui.register_handler("frmComFornecedoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_excluir_click));
    ui.register_handler("frmComFornecedoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_cancelar_click));
    ui.register_handler("frmComFornecedoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_novo_item_click));
    ui.register_handler("frmComFornecedoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::btn_fechar_click));
    ui.register_handler("frmComFornecedoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComFornecedoresCad::edt_busca_change));
    ui.register_form_class("frmComFornecedoresLista", FrmComFornecedoresLista::bind);
    ui.register_handler("frmComFornecedoresLista.form_show", aylis::prelude::instances::form_handler(FrmComFornecedoresLista::form_show));
    ui.register_handler("frmComFornecedoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComFornecedoresLista::btn_abrir_click));
    ui.register_handler("frmComFornecedoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComFornecedoresLista::edt_busca_change));
    ui.register_handler("frmComFornecedoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComFornecedoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmComFornecedoresConsulta", FrmComFornecedoresConsulta::bind);
    ui.register_handler("frmComFornecedoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmComFornecedoresConsulta::form_show));
    ui.register_handler("frmComFornecedoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComFornecedoresConsulta::edt_busca_change));
    ui.register_form_class("frmComFornecedoresHist", FrmComFornecedoresHist::bind);
    ui.register_handler("frmComFornecedoresHist.form_show", aylis::prelude::instances::form_handler(FrmComFornecedoresHist::form_show));
    ui.register_handler("frmComFornecedoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComFornecedoresHist::edt_busca_change));
    ui.register_form_class("frmComCompradoresCad", FrmComCompradoresCad::bind);
    ui.register_handler("frmComCompradoresCad.form_show", aylis::prelude::instances::form_handler(FrmComCompradoresCad::form_show));
    ui.register_handler("frmComCompradoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_novo_click));
    ui.register_handler("frmComCompradoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_salvar_click));
    ui.register_handler("frmComCompradoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_excluir_click));
    ui.register_handler("frmComCompradoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_cancelar_click));
    ui.register_handler("frmComCompradoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_novo_item_click));
    ui.register_handler("frmComCompradoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComCompradoresCad::btn_fechar_click));
    ui.register_handler("frmComCompradoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCompradoresCad::edt_busca_change));
    ui.register_form_class("frmComCompradoresLista", FrmComCompradoresLista::bind);
    ui.register_handler("frmComCompradoresLista.form_show", aylis::prelude::instances::form_handler(FrmComCompradoresLista::form_show));
    ui.register_handler("frmComCompradoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComCompradoresLista::btn_abrir_click));
    ui.register_handler("frmComCompradoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCompradoresLista::edt_busca_change));
    ui.register_handler("frmComCompradoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComCompradoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmComCompradoresConsulta", FrmComCompradoresConsulta::bind);
    ui.register_handler("frmComCompradoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmComCompradoresConsulta::form_show));
    ui.register_handler("frmComCompradoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCompradoresConsulta::edt_busca_change));
    ui.register_form_class("frmComCompradoresHist", FrmComCompradoresHist::bind);
    ui.register_handler("frmComCompradoresHist.form_show", aylis::prelude::instances::form_handler(FrmComCompradoresHist::form_show));
    ui.register_handler("frmComCompradoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCompradoresHist::edt_busca_change));
    ui.register_form_class("frmComProdutosCad", FrmComProdutosCad::bind);
    ui.register_handler("frmComProdutosCad.form_show", aylis::prelude::instances::form_handler(FrmComProdutosCad::form_show));
    ui.register_handler("frmComProdutosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_novo_click));
    ui.register_handler("frmComProdutosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_salvar_click));
    ui.register_handler("frmComProdutosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_excluir_click));
    ui.register_handler("frmComProdutosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_cancelar_click));
    ui.register_handler("frmComProdutosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_novo_item_click));
    ui.register_handler("frmComProdutosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComProdutosCad::btn_fechar_click));
    ui.register_handler("frmComProdutosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComProdutosCad::edt_busca_change));
    ui.register_form_class("frmComProdutosLista", FrmComProdutosLista::bind);
    ui.register_handler("frmComProdutosLista.form_show", aylis::prelude::instances::form_handler(FrmComProdutosLista::form_show));
    ui.register_handler("frmComProdutosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComProdutosLista::btn_abrir_click));
    ui.register_handler("frmComProdutosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComProdutosLista::edt_busca_change));
    ui.register_handler("frmComProdutosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComProdutosLista::grd_lista_dbl_click));
    ui.register_form_class("frmComProdutosConsulta", FrmComProdutosConsulta::bind);
    ui.register_handler("frmComProdutosConsulta.form_show", aylis::prelude::instances::form_handler(FrmComProdutosConsulta::form_show));
    ui.register_handler("frmComProdutosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComProdutosConsulta::edt_busca_change));
    ui.register_form_class("frmComProdutosHist", FrmComProdutosHist::bind);
    ui.register_handler("frmComProdutosHist.form_show", aylis::prelude::instances::form_handler(FrmComProdutosHist::form_show));
    ui.register_handler("frmComProdutosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComProdutosHist::edt_busca_change));
    ui.register_form_class("frmComGruposDeProdutosCad", FrmComGruposDeProdutosCad::bind);
    ui.register_handler("frmComGruposDeProdutosCad.form_show", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::form_show));
    ui.register_handler("frmComGruposDeProdutosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_novo_click));
    ui.register_handler("frmComGruposDeProdutosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_salvar_click));
    ui.register_handler("frmComGruposDeProdutosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_excluir_click));
    ui.register_handler("frmComGruposDeProdutosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_cancelar_click));
    ui.register_handler("frmComGruposDeProdutosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_novo_item_click));
    ui.register_handler("frmComGruposDeProdutosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::btn_fechar_click));
    ui.register_handler("frmComGruposDeProdutosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosCad::edt_busca_change));
    ui.register_form_class("frmComGruposDeProdutosLista", FrmComGruposDeProdutosLista::bind);
    ui.register_handler("frmComGruposDeProdutosLista.form_show", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosLista::form_show));
    ui.register_handler("frmComGruposDeProdutosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosLista::btn_abrir_click));
    ui.register_handler("frmComGruposDeProdutosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosLista::edt_busca_change));
    ui.register_handler("frmComGruposDeProdutosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosLista::grd_lista_dbl_click));
    ui.register_form_class("frmComGruposDeProdutosConsulta", FrmComGruposDeProdutosConsulta::bind);
    ui.register_handler("frmComGruposDeProdutosConsulta.form_show", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosConsulta::form_show));
    ui.register_handler("frmComGruposDeProdutosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosConsulta::edt_busca_change));
    ui.register_form_class("frmComGruposDeProdutosHist", FrmComGruposDeProdutosHist::bind);
    ui.register_handler("frmComGruposDeProdutosHist.form_show", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosHist::form_show));
    ui.register_handler("frmComGruposDeProdutosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComGruposDeProdutosHist::edt_busca_change));
    ui.register_form_class("frmComUnidadesDeMedidaCad", FrmComUnidadesDeMedidaCad::bind);
    ui.register_handler("frmComUnidadesDeMedidaCad.form_show", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::form_show));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_novo_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_salvar_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_excluir_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_cancelar_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_novo_item_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::btn_fechar_click));
    ui.register_handler("frmComUnidadesDeMedidaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaCad::edt_busca_change));
    ui.register_form_class("frmComUnidadesDeMedidaLista", FrmComUnidadesDeMedidaLista::bind);
    ui.register_handler("frmComUnidadesDeMedidaLista.form_show", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaLista::form_show));
    ui.register_handler("frmComUnidadesDeMedidaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaLista::btn_abrir_click));
    ui.register_handler("frmComUnidadesDeMedidaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaLista::edt_busca_change));
    ui.register_handler("frmComUnidadesDeMedidaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaLista::grd_lista_dbl_click));
    ui.register_form_class("frmComUnidadesDeMedidaConsulta", FrmComUnidadesDeMedidaConsulta::bind);
    ui.register_handler("frmComUnidadesDeMedidaConsulta.form_show", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaConsulta::form_show));
    ui.register_handler("frmComUnidadesDeMedidaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaConsulta::edt_busca_change));
    ui.register_form_class("frmComUnidadesDeMedidaHist", FrmComUnidadesDeMedidaHist::bind);
    ui.register_handler("frmComUnidadesDeMedidaHist.form_show", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaHist::form_show));
    ui.register_handler("frmComUnidadesDeMedidaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComUnidadesDeMedidaHist::edt_busca_change));
    ui.register_form_class("frmComMarcasCad", FrmComMarcasCad::bind);
    ui.register_handler("frmComMarcasCad.form_show", aylis::prelude::instances::form_handler(FrmComMarcasCad::form_show));
    ui.register_handler("frmComMarcasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_novo_click));
    ui.register_handler("frmComMarcasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_salvar_click));
    ui.register_handler("frmComMarcasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_excluir_click));
    ui.register_handler("frmComMarcasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_cancelar_click));
    ui.register_handler("frmComMarcasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_novo_item_click));
    ui.register_handler("frmComMarcasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComMarcasCad::btn_fechar_click));
    ui.register_handler("frmComMarcasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMarcasCad::edt_busca_change));
    ui.register_form_class("frmComMarcasLista", FrmComMarcasLista::bind);
    ui.register_handler("frmComMarcasLista.form_show", aylis::prelude::instances::form_handler(FrmComMarcasLista::form_show));
    ui.register_handler("frmComMarcasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComMarcasLista::btn_abrir_click));
    ui.register_handler("frmComMarcasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMarcasLista::edt_busca_change));
    ui.register_handler("frmComMarcasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComMarcasLista::grd_lista_dbl_click));
    ui.register_form_class("frmComMarcasConsulta", FrmComMarcasConsulta::bind);
    ui.register_handler("frmComMarcasConsulta.form_show", aylis::prelude::instances::form_handler(FrmComMarcasConsulta::form_show));
    ui.register_handler("frmComMarcasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMarcasConsulta::edt_busca_change));
    ui.register_form_class("frmComMarcasHist", FrmComMarcasHist::bind);
    ui.register_handler("frmComMarcasHist.form_show", aylis::prelude::instances::form_handler(FrmComMarcasHist::form_show));
    ui.register_handler("frmComMarcasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMarcasHist::edt_busca_change));
    ui.register_form_class("frmComCategoriasDeCompraCad", FrmComCategoriasDeCompraCad::bind);
    ui.register_handler("frmComCategoriasDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::form_show));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_novo_click));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComCategoriasDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComCategoriasDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComCategoriasDeCompraLista", FrmComCategoriasDeCompraLista::bind);
    ui.register_handler("frmComCategoriasDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraLista::form_show));
    ui.register_handler("frmComCategoriasDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComCategoriasDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraLista::edt_busca_change));
    ui.register_handler("frmComCategoriasDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComCategoriasDeCompraConsulta", FrmComCategoriasDeCompraConsulta::bind);
    ui.register_handler("frmComCategoriasDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraConsulta::form_show));
    ui.register_handler("frmComCategoriasDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComCategoriasDeCompraHist", FrmComCategoriasDeCompraHist::bind);
    ui.register_handler("frmComCategoriasDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraHist::form_show));
    ui.register_handler("frmComCategoriasDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCategoriasDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComLocaisDeEntregaCad", FrmComLocaisDeEntregaCad::bind);
    ui.register_handler("frmComLocaisDeEntregaCad.form_show", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::form_show));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_novo_click));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_salvar_click));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_excluir_click));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_cancelar_click));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_novo_item_click));
    ui.register_handler("frmComLocaisDeEntregaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::btn_fechar_click));
    ui.register_handler("frmComLocaisDeEntregaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaCad::edt_busca_change));
    ui.register_form_class("frmComLocaisDeEntregaLista", FrmComLocaisDeEntregaLista::bind);
    ui.register_handler("frmComLocaisDeEntregaLista.form_show", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaLista::form_show));
    ui.register_handler("frmComLocaisDeEntregaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaLista::btn_abrir_click));
    ui.register_handler("frmComLocaisDeEntregaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaLista::edt_busca_change));
    ui.register_handler("frmComLocaisDeEntregaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaLista::grd_lista_dbl_click));
    ui.register_form_class("frmComLocaisDeEntregaConsulta", FrmComLocaisDeEntregaConsulta::bind);
    ui.register_handler("frmComLocaisDeEntregaConsulta.form_show", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaConsulta::form_show));
    ui.register_handler("frmComLocaisDeEntregaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaConsulta::edt_busca_change));
    ui.register_form_class("frmComLocaisDeEntregaHist", FrmComLocaisDeEntregaHist::bind);
    ui.register_handler("frmComLocaisDeEntregaHist.form_show", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaHist::form_show));
    ui.register_handler("frmComLocaisDeEntregaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComLocaisDeEntregaHist::edt_busca_change));
    ui.register_form_class("frmComTransportadorasCad", FrmComTransportadorasCad::bind);
    ui.register_handler("frmComTransportadorasCad.form_show", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::form_show));
    ui.register_handler("frmComTransportadorasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_novo_click));
    ui.register_handler("frmComTransportadorasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_salvar_click));
    ui.register_handler("frmComTransportadorasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_excluir_click));
    ui.register_handler("frmComTransportadorasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_cancelar_click));
    ui.register_handler("frmComTransportadorasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_novo_item_click));
    ui.register_handler("frmComTransportadorasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::btn_fechar_click));
    ui.register_handler("frmComTransportadorasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransportadorasCad::edt_busca_change));
    ui.register_form_class("frmComTransportadorasLista", FrmComTransportadorasLista::bind);
    ui.register_handler("frmComTransportadorasLista.form_show", aylis::prelude::instances::form_handler(FrmComTransportadorasLista::form_show));
    ui.register_handler("frmComTransportadorasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComTransportadorasLista::btn_abrir_click));
    ui.register_handler("frmComTransportadorasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransportadorasLista::edt_busca_change));
    ui.register_handler("frmComTransportadorasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComTransportadorasLista::grd_lista_dbl_click));
    ui.register_form_class("frmComTransportadorasConsulta", FrmComTransportadorasConsulta::bind);
    ui.register_handler("frmComTransportadorasConsulta.form_show", aylis::prelude::instances::form_handler(FrmComTransportadorasConsulta::form_show));
    ui.register_handler("frmComTransportadorasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransportadorasConsulta::edt_busca_change));
    ui.register_form_class("frmComTransportadorasHist", FrmComTransportadorasHist::bind);
    ui.register_handler("frmComTransportadorasHist.form_show", aylis::prelude::instances::form_handler(FrmComTransportadorasHist::form_show));
    ui.register_handler("frmComTransportadorasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransportadorasHist::edt_busca_change));
    ui.register_form_class("frmComTabelasDePrecoDeCompraCad", FrmComTabelasDePrecoDeCompraCad::bind);
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::form_show));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_novo_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComTabelasDePrecoDeCompraLista", FrmComTabelasDePrecoDeCompraLista::bind);
    ui.register_handler("frmComTabelasDePrecoDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraLista::form_show));
    ui.register_handler("frmComTabelasDePrecoDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComTabelasDePrecoDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraLista::edt_busca_change));
    ui.register_handler("frmComTabelasDePrecoDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComTabelasDePrecoDeCompraConsulta", FrmComTabelasDePrecoDeCompraConsulta::bind);
    ui.register_handler("frmComTabelasDePrecoDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraConsulta::form_show));
    ui.register_handler("frmComTabelasDePrecoDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComTabelasDePrecoDeCompraHist", FrmComTabelasDePrecoDeCompraHist::bind);
    ui.register_handler("frmComTabelasDePrecoDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraHist::form_show));
    ui.register_handler("frmComTabelasDePrecoDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTabelasDePrecoDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComCondicoesDeCompraCad", FrmComCondicoesDeCompraCad::bind);
    ui.register_handler("frmComCondicoesDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::form_show));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_novo_click));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComCondicoesDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComCondicoesDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComCondicoesDeCompraLista", FrmComCondicoesDeCompraLista::bind);
    ui.register_handler("frmComCondicoesDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraLista::form_show));
    ui.register_handler("frmComCondicoesDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComCondicoesDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraLista::edt_busca_change));
    ui.register_handler("frmComCondicoesDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComCondicoesDeCompraConsulta", FrmComCondicoesDeCompraConsulta::bind);
    ui.register_handler("frmComCondicoesDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraConsulta::form_show));
    ui.register_handler("frmComCondicoesDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComCondicoesDeCompraHist", FrmComCondicoesDeCompraHist::bind);
    ui.register_handler("frmComCondicoesDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraHist::form_show));
    ui.register_handler("frmComCondicoesDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCondicoesDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComAprovadoresCad", FrmComAprovadoresCad::bind);
    ui.register_handler("frmComAprovadoresCad.form_show", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::form_show));
    ui.register_handler("frmComAprovadoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_novo_click));
    ui.register_handler("frmComAprovadoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_salvar_click));
    ui.register_handler("frmComAprovadoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_excluir_click));
    ui.register_handler("frmComAprovadoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_cancelar_click));
    ui.register_handler("frmComAprovadoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_novo_item_click));
    ui.register_handler("frmComAprovadoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::btn_fechar_click));
    ui.register_handler("frmComAprovadoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovadoresCad::edt_busca_change));
    ui.register_form_class("frmComAprovadoresLista", FrmComAprovadoresLista::bind);
    ui.register_handler("frmComAprovadoresLista.form_show", aylis::prelude::instances::form_handler(FrmComAprovadoresLista::form_show));
    ui.register_handler("frmComAprovadoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAprovadoresLista::btn_abrir_click));
    ui.register_handler("frmComAprovadoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovadoresLista::edt_busca_change));
    ui.register_handler("frmComAprovadoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAprovadoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAprovadoresConsulta", FrmComAprovadoresConsulta::bind);
    ui.register_handler("frmComAprovadoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAprovadoresConsulta::form_show));
    ui.register_handler("frmComAprovadoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovadoresConsulta::edt_busca_change));
    ui.register_form_class("frmComAprovadoresHist", FrmComAprovadoresHist::bind);
    ui.register_handler("frmComAprovadoresHist.form_show", aylis::prelude::instances::form_handler(FrmComAprovadoresHist::form_show));
    ui.register_handler("frmComAprovadoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovadoresHist::edt_busca_change));
    ui.register_form_class("frmComAlcadasDeAprovacaoCad", FrmComAlcadasDeAprovacaoCad::bind);
    ui.register_handler("frmComAlcadasDeAprovacaoCad.form_show", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::form_show));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_novo_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_salvar_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_excluir_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_cancelar_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_novo_item_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::btn_fechar_click));
    ui.register_handler("frmComAlcadasDeAprovacaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoCad::edt_busca_change));
    ui.register_form_class("frmComAlcadasDeAprovacaoLista", FrmComAlcadasDeAprovacaoLista::bind);
    ui.register_handler("frmComAlcadasDeAprovacaoLista.form_show", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoLista::form_show));
    ui.register_handler("frmComAlcadasDeAprovacaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoLista::btn_abrir_click));
    ui.register_handler("frmComAlcadasDeAprovacaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoLista::edt_busca_change));
    ui.register_handler("frmComAlcadasDeAprovacaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAlcadasDeAprovacaoConsulta", FrmComAlcadasDeAprovacaoConsulta::bind);
    ui.register_handler("frmComAlcadasDeAprovacaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoConsulta::form_show));
    ui.register_handler("frmComAlcadasDeAprovacaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoConsulta::edt_busca_change));
    ui.register_form_class("frmComAlcadasDeAprovacaoHist", FrmComAlcadasDeAprovacaoHist::bind);
    ui.register_handler("frmComAlcadasDeAprovacaoHist.form_show", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoHist::form_show));
    ui.register_handler("frmComAlcadasDeAprovacaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlcadasDeAprovacaoHist::edt_busca_change));
    ui.register_form_class("frmComContratosDeFornecimentoCad", FrmComContratosDeFornecimentoCad::bind);
    ui.register_handler("frmComContratosDeFornecimentoCad.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::form_show));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_novo_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_salvar_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_excluir_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_cancelar_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_novo_item_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::btn_fechar_click));
    ui.register_handler("frmComContratosDeFornecimentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoCad::edt_busca_change));
    ui.register_form_class("frmComContratosDeFornecimentoLista", FrmComContratosDeFornecimentoLista::bind);
    ui.register_handler("frmComContratosDeFornecimentoLista.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoLista::form_show));
    ui.register_handler("frmComContratosDeFornecimentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoLista::btn_abrir_click));
    ui.register_handler("frmComContratosDeFornecimentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoLista::edt_busca_change));
    ui.register_handler("frmComContratosDeFornecimentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComContratosDeFornecimentoConsulta", FrmComContratosDeFornecimentoConsulta::bind);
    ui.register_handler("frmComContratosDeFornecimentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoConsulta::form_show));
    ui.register_handler("frmComContratosDeFornecimentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoConsulta::edt_busca_change));
    ui.register_form_class("frmComContratosDeFornecimentoHist", FrmComContratosDeFornecimentoHist::bind);
    ui.register_handler("frmComContratosDeFornecimentoHist.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoHist::form_show));
    ui.register_handler("frmComContratosDeFornecimentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeFornecimentoHist::edt_busca_change));
    ui.register_form_class("frmComHomologacoesCad", FrmComHomologacoesCad::bind);
    ui.register_handler("frmComHomologacoesCad.form_show", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::form_show));
    ui.register_handler("frmComHomologacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_novo_click));
    ui.register_handler("frmComHomologacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_salvar_click));
    ui.register_handler("frmComHomologacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_excluir_click));
    ui.register_handler("frmComHomologacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_cancelar_click));
    ui.register_handler("frmComHomologacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_novo_item_click));
    ui.register_handler("frmComHomologacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::btn_fechar_click));
    ui.register_handler("frmComHomologacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComHomologacoesCad::edt_busca_change));
    ui.register_form_class("frmComHomologacoesLista", FrmComHomologacoesLista::bind);
    ui.register_handler("frmComHomologacoesLista.form_show", aylis::prelude::instances::form_handler(FrmComHomologacoesLista::form_show));
    ui.register_handler("frmComHomologacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComHomologacoesLista::btn_abrir_click));
    ui.register_handler("frmComHomologacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComHomologacoesLista::edt_busca_change));
    ui.register_handler("frmComHomologacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComHomologacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmComHomologacoesConsulta", FrmComHomologacoesConsulta::bind);
    ui.register_handler("frmComHomologacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmComHomologacoesConsulta::form_show));
    ui.register_handler("frmComHomologacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComHomologacoesConsulta::edt_busca_change));
    ui.register_form_class("frmComHomologacoesHist", FrmComHomologacoesHist::bind);
    ui.register_handler("frmComHomologacoesHist.form_show", aylis::prelude::instances::form_handler(FrmComHomologacoesHist::form_show));
    ui.register_handler("frmComHomologacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComHomologacoesHist::edt_busca_change));
    ui.register_form_class("frmComQualificacoesDeFornecedorCad", FrmComQualificacoesDeFornecedorCad::bind);
    ui.register_handler("frmComQualificacoesDeFornecedorCad.form_show", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::form_show));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_novo_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_salvar_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_excluir_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_cancelar_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_novo_item_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::btn_fechar_click));
    ui.register_handler("frmComQualificacoesDeFornecedorCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorCad::edt_busca_change));
    ui.register_form_class("frmComQualificacoesDeFornecedorLista", FrmComQualificacoesDeFornecedorLista::bind);
    ui.register_handler("frmComQualificacoesDeFornecedorLista.form_show", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorLista::form_show));
    ui.register_handler("frmComQualificacoesDeFornecedorLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorLista::btn_abrir_click));
    ui.register_handler("frmComQualificacoesDeFornecedorLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorLista::edt_busca_change));
    ui.register_handler("frmComQualificacoesDeFornecedorLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorLista::grd_lista_dbl_click));
    ui.register_form_class("frmComQualificacoesDeFornecedorConsulta", FrmComQualificacoesDeFornecedorConsulta::bind);
    ui.register_handler("frmComQualificacoesDeFornecedorConsulta.form_show", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorConsulta::form_show));
    ui.register_handler("frmComQualificacoesDeFornecedorConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorConsulta::edt_busca_change));
    ui.register_form_class("frmComQualificacoesDeFornecedorHist", FrmComQualificacoesDeFornecedorHist::bind);
    ui.register_handler("frmComQualificacoesDeFornecedorHist.form_show", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorHist::form_show));
    ui.register_handler("frmComQualificacoesDeFornecedorHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComQualificacoesDeFornecedorHist::edt_busca_change));
    ui.register_form_class("frmComTiposDeFreteCad", FrmComTiposDeFreteCad::bind);
    ui.register_handler("frmComTiposDeFreteCad.form_show", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::form_show));
    ui.register_handler("frmComTiposDeFreteCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_novo_click));
    ui.register_handler("frmComTiposDeFreteCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_salvar_click));
    ui.register_handler("frmComTiposDeFreteCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_excluir_click));
    ui.register_handler("frmComTiposDeFreteCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_cancelar_click));
    ui.register_handler("frmComTiposDeFreteCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_novo_item_click));
    ui.register_handler("frmComTiposDeFreteCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::btn_fechar_click));
    ui.register_handler("frmComTiposDeFreteCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTiposDeFreteCad::edt_busca_change));
    ui.register_form_class("frmComTiposDeFreteLista", FrmComTiposDeFreteLista::bind);
    ui.register_handler("frmComTiposDeFreteLista.form_show", aylis::prelude::instances::form_handler(FrmComTiposDeFreteLista::form_show));
    ui.register_handler("frmComTiposDeFreteLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteLista::btn_abrir_click));
    ui.register_handler("frmComTiposDeFreteLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTiposDeFreteLista::edt_busca_change));
    ui.register_handler("frmComTiposDeFreteLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComTiposDeFreteLista::grd_lista_dbl_click));
    ui.register_form_class("frmComTiposDeFreteConsulta", FrmComTiposDeFreteConsulta::bind);
    ui.register_handler("frmComTiposDeFreteConsulta.form_show", aylis::prelude::instances::form_handler(FrmComTiposDeFreteConsulta::form_show));
    ui.register_handler("frmComTiposDeFreteConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTiposDeFreteConsulta::edt_busca_change));
    ui.register_form_class("frmComTiposDeFreteHist", FrmComTiposDeFreteHist::bind);
    ui.register_handler("frmComTiposDeFreteHist.form_show", aylis::prelude::instances::form_handler(FrmComTiposDeFreteHist::form_show));
    ui.register_handler("frmComTiposDeFreteHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTiposDeFreteHist::edt_busca_change));
    ui.register_form_class("frmComEmbalagensCad", FrmComEmbalagensCad::bind);
    ui.register_handler("frmComEmbalagensCad.form_show", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::form_show));
    ui.register_handler("frmComEmbalagensCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_novo_click));
    ui.register_handler("frmComEmbalagensCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_salvar_click));
    ui.register_handler("frmComEmbalagensCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_excluir_click));
    ui.register_handler("frmComEmbalagensCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_cancelar_click));
    ui.register_handler("frmComEmbalagensCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_novo_item_click));
    ui.register_handler("frmComEmbalagensCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::btn_fechar_click));
    ui.register_handler("frmComEmbalagensCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEmbalagensCad::edt_busca_change));
    ui.register_form_class("frmComEmbalagensLista", FrmComEmbalagensLista::bind);
    ui.register_handler("frmComEmbalagensLista.form_show", aylis::prelude::instances::form_handler(FrmComEmbalagensLista::form_show));
    ui.register_handler("frmComEmbalagensLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComEmbalagensLista::btn_abrir_click));
    ui.register_handler("frmComEmbalagensLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEmbalagensLista::edt_busca_change));
    ui.register_handler("frmComEmbalagensLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComEmbalagensLista::grd_lista_dbl_click));
    ui.register_form_class("frmComEmbalagensConsulta", FrmComEmbalagensConsulta::bind);
    ui.register_handler("frmComEmbalagensConsulta.form_show", aylis::prelude::instances::form_handler(FrmComEmbalagensConsulta::form_show));
    ui.register_handler("frmComEmbalagensConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEmbalagensConsulta::edt_busca_change));
    ui.register_form_class("frmComEmbalagensHist", FrmComEmbalagensHist::bind);
    ui.register_handler("frmComEmbalagensHist.form_show", aylis::prelude::instances::form_handler(FrmComEmbalagensHist::form_show));
    ui.register_handler("frmComEmbalagensHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEmbalagensHist::edt_busca_change));
    ui.register_form_class("frmComAlmoxarifadosCad", FrmComAlmoxarifadosCad::bind);
    ui.register_handler("frmComAlmoxarifadosCad.form_show", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::form_show));
    ui.register_handler("frmComAlmoxarifadosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_novo_click));
    ui.register_handler("frmComAlmoxarifadosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_salvar_click));
    ui.register_handler("frmComAlmoxarifadosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_excluir_click));
    ui.register_handler("frmComAlmoxarifadosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_cancelar_click));
    ui.register_handler("frmComAlmoxarifadosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_novo_item_click));
    ui.register_handler("frmComAlmoxarifadosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::btn_fechar_click));
    ui.register_handler("frmComAlmoxarifadosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosCad::edt_busca_change));
    ui.register_form_class("frmComAlmoxarifadosLista", FrmComAlmoxarifadosLista::bind);
    ui.register_handler("frmComAlmoxarifadosLista.form_show", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosLista::form_show));
    ui.register_handler("frmComAlmoxarifadosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosLista::btn_abrir_click));
    ui.register_handler("frmComAlmoxarifadosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosLista::edt_busca_change));
    ui.register_handler("frmComAlmoxarifadosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAlmoxarifadosConsulta", FrmComAlmoxarifadosConsulta::bind);
    ui.register_handler("frmComAlmoxarifadosConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosConsulta::form_show));
    ui.register_handler("frmComAlmoxarifadosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosConsulta::edt_busca_change));
    ui.register_form_class("frmComAlmoxarifadosHist", FrmComAlmoxarifadosHist::bind);
    ui.register_handler("frmComAlmoxarifadosHist.form_show", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosHist::form_show));
    ui.register_handler("frmComAlmoxarifadosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAlmoxarifadosHist::edt_busca_change));
    ui.register_form_class("frmComClassesDeMaterialCad", FrmComClassesDeMaterialCad::bind);
    ui.register_handler("frmComClassesDeMaterialCad.form_show", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::form_show));
    ui.register_handler("frmComClassesDeMaterialCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_novo_click));
    ui.register_handler("frmComClassesDeMaterialCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_salvar_click));
    ui.register_handler("frmComClassesDeMaterialCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_excluir_click));
    ui.register_handler("frmComClassesDeMaterialCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_cancelar_click));
    ui.register_handler("frmComClassesDeMaterialCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_novo_item_click));
    ui.register_handler("frmComClassesDeMaterialCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::btn_fechar_click));
    ui.register_handler("frmComClassesDeMaterialCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialCad::edt_busca_change));
    ui.register_form_class("frmComClassesDeMaterialLista", FrmComClassesDeMaterialLista::bind);
    ui.register_handler("frmComClassesDeMaterialLista.form_show", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialLista::form_show));
    ui.register_handler("frmComClassesDeMaterialLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialLista::btn_abrir_click));
    ui.register_handler("frmComClassesDeMaterialLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialLista::edt_busca_change));
    ui.register_handler("frmComClassesDeMaterialLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialLista::grd_lista_dbl_click));
    ui.register_form_class("frmComClassesDeMaterialConsulta", FrmComClassesDeMaterialConsulta::bind);
    ui.register_handler("frmComClassesDeMaterialConsulta.form_show", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialConsulta::form_show));
    ui.register_handler("frmComClassesDeMaterialConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialConsulta::edt_busca_change));
    ui.register_form_class("frmComClassesDeMaterialHist", FrmComClassesDeMaterialHist::bind);
    ui.register_handler("frmComClassesDeMaterialHist.form_show", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialHist::form_show));
    ui.register_handler("frmComClassesDeMaterialHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComClassesDeMaterialHist::edt_busca_change));
    ui.register_form_class("frmComCatalogosDeFornecedorCad", FrmComCatalogosDeFornecedorCad::bind);
    ui.register_handler("frmComCatalogosDeFornecedorCad.form_show", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::form_show));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_novo_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_salvar_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_excluir_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_cancelar_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_novo_item_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::btn_fechar_click));
    ui.register_handler("frmComCatalogosDeFornecedorCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorCad::edt_busca_change));
    ui.register_form_class("frmComCatalogosDeFornecedorLista", FrmComCatalogosDeFornecedorLista::bind);
    ui.register_handler("frmComCatalogosDeFornecedorLista.form_show", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorLista::form_show));
    ui.register_handler("frmComCatalogosDeFornecedorLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorLista::btn_abrir_click));
    ui.register_handler("frmComCatalogosDeFornecedorLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorLista::edt_busca_change));
    ui.register_handler("frmComCatalogosDeFornecedorLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorLista::grd_lista_dbl_click));
    ui.register_form_class("frmComCatalogosDeFornecedorConsulta", FrmComCatalogosDeFornecedorConsulta::bind);
    ui.register_handler("frmComCatalogosDeFornecedorConsulta.form_show", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorConsulta::form_show));
    ui.register_handler("frmComCatalogosDeFornecedorConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorConsulta::edt_busca_change));
    ui.register_form_class("frmComCatalogosDeFornecedorHist", FrmComCatalogosDeFornecedorHist::bind);
    ui.register_handler("frmComCatalogosDeFornecedorHist.form_show", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorHist::form_show));
    ui.register_handler("frmComCatalogosDeFornecedorHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCatalogosDeFornecedorHist::edt_busca_change));
    ui.register_form_class("frmComSolicitacoesDeCompraCad", FrmComSolicitacoesDeCompraCad::bind);
    ui.register_handler("frmComSolicitacoesDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::form_show));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_novo_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComSolicitacoesDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComSolicitacoesDeCompraLista", FrmComSolicitacoesDeCompraLista::bind);
    ui.register_handler("frmComSolicitacoesDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraLista::form_show));
    ui.register_handler("frmComSolicitacoesDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComSolicitacoesDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraLista::edt_busca_change));
    ui.register_handler("frmComSolicitacoesDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComSolicitacoesDeCompraConsulta", FrmComSolicitacoesDeCompraConsulta::bind);
    ui.register_handler("frmComSolicitacoesDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraConsulta::form_show));
    ui.register_handler("frmComSolicitacoesDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComSolicitacoesDeCompraHist", FrmComSolicitacoesDeCompraHist::bind);
    ui.register_handler("frmComSolicitacoesDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraHist::form_show));
    ui.register_handler("frmComSolicitacoesDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSolicitacoesDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComCotacoesCad", FrmComCotacoesCad::bind);
    ui.register_handler("frmComCotacoesCad.form_show", aylis::prelude::instances::form_handler(FrmComCotacoesCad::form_show));
    ui.register_handler("frmComCotacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_novo_click));
    ui.register_handler("frmComCotacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_salvar_click));
    ui.register_handler("frmComCotacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_excluir_click));
    ui.register_handler("frmComCotacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_cancelar_click));
    ui.register_handler("frmComCotacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_novo_item_click));
    ui.register_handler("frmComCotacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComCotacoesCad::btn_fechar_click));
    ui.register_handler("frmComCotacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCotacoesCad::edt_busca_change));
    ui.register_form_class("frmComCotacoesLista", FrmComCotacoesLista::bind);
    ui.register_handler("frmComCotacoesLista.form_show", aylis::prelude::instances::form_handler(FrmComCotacoesLista::form_show));
    ui.register_handler("frmComCotacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComCotacoesLista::btn_abrir_click));
    ui.register_handler("frmComCotacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCotacoesLista::edt_busca_change));
    ui.register_handler("frmComCotacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComCotacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmComCotacoesConsulta", FrmComCotacoesConsulta::bind);
    ui.register_handler("frmComCotacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmComCotacoesConsulta::form_show));
    ui.register_handler("frmComCotacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCotacoesConsulta::edt_busca_change));
    ui.register_form_class("frmComCotacoesHist", FrmComCotacoesHist::bind);
    ui.register_handler("frmComCotacoesHist.form_show", aylis::prelude::instances::form_handler(FrmComCotacoesHist::form_show));
    ui.register_handler("frmComCotacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComCotacoesHist::edt_busca_change));
    ui.register_form_class("frmComMapasDeCotacaoCad", FrmComMapasDeCotacaoCad::bind);
    ui.register_handler("frmComMapasDeCotacaoCad.form_show", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::form_show));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_novo_click));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_salvar_click));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_excluir_click));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_cancelar_click));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_novo_item_click));
    ui.register_handler("frmComMapasDeCotacaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::btn_fechar_click));
    ui.register_handler("frmComMapasDeCotacaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoCad::edt_busca_change));
    ui.register_form_class("frmComMapasDeCotacaoLista", FrmComMapasDeCotacaoLista::bind);
    ui.register_handler("frmComMapasDeCotacaoLista.form_show", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoLista::form_show));
    ui.register_handler("frmComMapasDeCotacaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoLista::btn_abrir_click));
    ui.register_handler("frmComMapasDeCotacaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoLista::edt_busca_change));
    ui.register_handler("frmComMapasDeCotacaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComMapasDeCotacaoConsulta", FrmComMapasDeCotacaoConsulta::bind);
    ui.register_handler("frmComMapasDeCotacaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoConsulta::form_show));
    ui.register_handler("frmComMapasDeCotacaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoConsulta::edt_busca_change));
    ui.register_form_class("frmComMapasDeCotacaoHist", FrmComMapasDeCotacaoHist::bind);
    ui.register_handler("frmComMapasDeCotacaoHist.form_show", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoHist::form_show));
    ui.register_handler("frmComMapasDeCotacaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComMapasDeCotacaoHist::edt_busca_change));
    ui.register_form_class("frmComPedidosDeCompraCad", FrmComPedidosDeCompraCad::bind);
    ui.register_handler("frmComPedidosDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::form_show));
    ui.register_handler("frmComPedidosDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_novo_click));
    ui.register_handler("frmComPedidosDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComPedidosDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComPedidosDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComPedidosDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComPedidosDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComPedidosDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComPedidosDeCompraLista", FrmComPedidosDeCompraLista::bind);
    ui.register_handler("frmComPedidosDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraLista::form_show));
    ui.register_handler("frmComPedidosDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComPedidosDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraLista::edt_busca_change));
    ui.register_handler("frmComPedidosDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComPedidosDeCompraConsulta", FrmComPedidosDeCompraConsulta::bind);
    ui.register_handler("frmComPedidosDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraConsulta::form_show));
    ui.register_handler("frmComPedidosDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComPedidosDeCompraHist", FrmComPedidosDeCompraHist::bind);
    ui.register_handler("frmComPedidosDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraHist::form_show));
    ui.register_handler("frmComPedidosDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPedidosDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComRecebimentosCad", FrmComRecebimentosCad::bind);
    ui.register_handler("frmComRecebimentosCad.form_show", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::form_show));
    ui.register_handler("frmComRecebimentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_novo_click));
    ui.register_handler("frmComRecebimentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_salvar_click));
    ui.register_handler("frmComRecebimentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_excluir_click));
    ui.register_handler("frmComRecebimentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_cancelar_click));
    ui.register_handler("frmComRecebimentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_novo_item_click));
    ui.register_handler("frmComRecebimentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::btn_fechar_click));
    ui.register_handler("frmComRecebimentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRecebimentosCad::edt_busca_change));
    ui.register_form_class("frmComRecebimentosLista", FrmComRecebimentosLista::bind);
    ui.register_handler("frmComRecebimentosLista.form_show", aylis::prelude::instances::form_handler(FrmComRecebimentosLista::form_show));
    ui.register_handler("frmComRecebimentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComRecebimentosLista::btn_abrir_click));
    ui.register_handler("frmComRecebimentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRecebimentosLista::edt_busca_change));
    ui.register_handler("frmComRecebimentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComRecebimentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmComRecebimentosConsulta", FrmComRecebimentosConsulta::bind);
    ui.register_handler("frmComRecebimentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmComRecebimentosConsulta::form_show));
    ui.register_handler("frmComRecebimentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRecebimentosConsulta::edt_busca_change));
    ui.register_form_class("frmComRecebimentosHist", FrmComRecebimentosHist::bind);
    ui.register_handler("frmComRecebimentosHist.form_show", aylis::prelude::instances::form_handler(FrmComRecebimentosHist::form_show));
    ui.register_handler("frmComRecebimentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRecebimentosHist::edt_busca_change));
    ui.register_form_class("frmComNotasDeEntradaCad", FrmComNotasDeEntradaCad::bind);
    ui.register_handler("frmComNotasDeEntradaCad.form_show", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::form_show));
    ui.register_handler("frmComNotasDeEntradaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_novo_click));
    ui.register_handler("frmComNotasDeEntradaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_salvar_click));
    ui.register_handler("frmComNotasDeEntradaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_excluir_click));
    ui.register_handler("frmComNotasDeEntradaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_cancelar_click));
    ui.register_handler("frmComNotasDeEntradaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_novo_item_click));
    ui.register_handler("frmComNotasDeEntradaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::btn_fechar_click));
    ui.register_handler("frmComNotasDeEntradaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaCad::edt_busca_change));
    ui.register_form_class("frmComNotasDeEntradaLista", FrmComNotasDeEntradaLista::bind);
    ui.register_handler("frmComNotasDeEntradaLista.form_show", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaLista::form_show));
    ui.register_handler("frmComNotasDeEntradaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaLista::btn_abrir_click));
    ui.register_handler("frmComNotasDeEntradaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaLista::edt_busca_change));
    ui.register_handler("frmComNotasDeEntradaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaLista::grd_lista_dbl_click));
    ui.register_form_class("frmComNotasDeEntradaConsulta", FrmComNotasDeEntradaConsulta::bind);
    ui.register_handler("frmComNotasDeEntradaConsulta.form_show", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaConsulta::form_show));
    ui.register_handler("frmComNotasDeEntradaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaConsulta::edt_busca_change));
    ui.register_form_class("frmComNotasDeEntradaHist", FrmComNotasDeEntradaHist::bind);
    ui.register_handler("frmComNotasDeEntradaHist.form_show", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaHist::form_show));
    ui.register_handler("frmComNotasDeEntradaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComNotasDeEntradaHist::edt_busca_change));
    ui.register_form_class("frmComDevolucoesAFornecedorCad", FrmComDevolucoesAFornecedorCad::bind);
    ui.register_handler("frmComDevolucoesAFornecedorCad.form_show", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::form_show));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_novo_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_salvar_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_excluir_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_cancelar_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_novo_item_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::btn_fechar_click));
    ui.register_handler("frmComDevolucoesAFornecedorCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorCad::edt_busca_change));
    ui.register_form_class("frmComDevolucoesAFornecedorLista", FrmComDevolucoesAFornecedorLista::bind);
    ui.register_handler("frmComDevolucoesAFornecedorLista.form_show", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorLista::form_show));
    ui.register_handler("frmComDevolucoesAFornecedorLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorLista::btn_abrir_click));
    ui.register_handler("frmComDevolucoesAFornecedorLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorLista::edt_busca_change));
    ui.register_handler("frmComDevolucoesAFornecedorLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorLista::grd_lista_dbl_click));
    ui.register_form_class("frmComDevolucoesAFornecedorConsulta", FrmComDevolucoesAFornecedorConsulta::bind);
    ui.register_handler("frmComDevolucoesAFornecedorConsulta.form_show", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorConsulta::form_show));
    ui.register_handler("frmComDevolucoesAFornecedorConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorConsulta::edt_busca_change));
    ui.register_form_class("frmComDevolucoesAFornecedorHist", FrmComDevolucoesAFornecedorHist::bind);
    ui.register_handler("frmComDevolucoesAFornecedorHist.form_show", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorHist::form_show));
    ui.register_handler("frmComDevolucoesAFornecedorHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDevolucoesAFornecedorHist::edt_busca_change));
    ui.register_form_class("frmComAvaliacoesDeFornecedorCad", FrmComAvaliacoesDeFornecedorCad::bind);
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.form_show", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::form_show));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_novo_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_salvar_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_excluir_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_cancelar_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_novo_item_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::btn_fechar_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorCad::edt_busca_change));
    ui.register_form_class("frmComAvaliacoesDeFornecedorLista", FrmComAvaliacoesDeFornecedorLista::bind);
    ui.register_handler("frmComAvaliacoesDeFornecedorLista.form_show", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorLista::form_show));
    ui.register_handler("frmComAvaliacoesDeFornecedorLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorLista::btn_abrir_click));
    ui.register_handler("frmComAvaliacoesDeFornecedorLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorLista::edt_busca_change));
    ui.register_handler("frmComAvaliacoesDeFornecedorLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAvaliacoesDeFornecedorConsulta", FrmComAvaliacoesDeFornecedorConsulta::bind);
    ui.register_handler("frmComAvaliacoesDeFornecedorConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorConsulta::form_show));
    ui.register_handler("frmComAvaliacoesDeFornecedorConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorConsulta::edt_busca_change));
    ui.register_form_class("frmComAvaliacoesDeFornecedorHist", FrmComAvaliacoesDeFornecedorHist::bind);
    ui.register_handler("frmComAvaliacoesDeFornecedorHist.form_show", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorHist::form_show));
    ui.register_handler("frmComAvaliacoesDeFornecedorHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAvaliacoesDeFornecedorHist::edt_busca_change));
    ui.register_form_class("frmComRequisicoesAoAlmoxarifadoCad", FrmComRequisicoesAoAlmoxarifadoCad::bind);
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.form_show", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::form_show));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_novo_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_salvar_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_excluir_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_cancelar_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_novo_item_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::btn_fechar_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoCad::edt_busca_change));
    ui.register_form_class("frmComRequisicoesAoAlmoxarifadoLista", FrmComRequisicoesAoAlmoxarifadoLista::bind);
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoLista.form_show", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoLista::form_show));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoLista::btn_abrir_click));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoLista::edt_busca_change));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComRequisicoesAoAlmoxarifadoConsulta", FrmComRequisicoesAoAlmoxarifadoConsulta::bind);
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoConsulta::form_show));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoConsulta::edt_busca_change));
    ui.register_form_class("frmComRequisicoesAoAlmoxarifadoHist", FrmComRequisicoesAoAlmoxarifadoHist::bind);
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoHist.form_show", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoHist::form_show));
    ui.register_handler("frmComRequisicoesAoAlmoxarifadoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComRequisicoesAoAlmoxarifadoHist::edt_busca_change));
    ui.register_form_class("frmComEntradasDeEstoqueCad", FrmComEntradasDeEstoqueCad::bind);
    ui.register_handler("frmComEntradasDeEstoqueCad.form_show", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::form_show));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_novo_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_salvar_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_excluir_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_cancelar_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_novo_item_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::btn_fechar_click));
    ui.register_handler("frmComEntradasDeEstoqueCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueCad::edt_busca_change));
    ui.register_form_class("frmComEntradasDeEstoqueLista", FrmComEntradasDeEstoqueLista::bind);
    ui.register_handler("frmComEntradasDeEstoqueLista.form_show", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueLista::form_show));
    ui.register_handler("frmComEntradasDeEstoqueLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueLista::btn_abrir_click));
    ui.register_handler("frmComEntradasDeEstoqueLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueLista::edt_busca_change));
    ui.register_handler("frmComEntradasDeEstoqueLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueLista::grd_lista_dbl_click));
    ui.register_form_class("frmComEntradasDeEstoqueConsulta", FrmComEntradasDeEstoqueConsulta::bind);
    ui.register_handler("frmComEntradasDeEstoqueConsulta.form_show", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueConsulta::form_show));
    ui.register_handler("frmComEntradasDeEstoqueConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueConsulta::edt_busca_change));
    ui.register_form_class("frmComEntradasDeEstoqueHist", FrmComEntradasDeEstoqueHist::bind);
    ui.register_handler("frmComEntradasDeEstoqueHist.form_show", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueHist::form_show));
    ui.register_handler("frmComEntradasDeEstoqueHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComEntradasDeEstoqueHist::edt_busca_change));
    ui.register_form_class("frmComSaidasDeEstoqueCad", FrmComSaidasDeEstoqueCad::bind);
    ui.register_handler("frmComSaidasDeEstoqueCad.form_show", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::form_show));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_novo_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_salvar_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_excluir_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_cancelar_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_novo_item_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::btn_fechar_click));
    ui.register_handler("frmComSaidasDeEstoqueCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueCad::edt_busca_change));
    ui.register_form_class("frmComSaidasDeEstoqueLista", FrmComSaidasDeEstoqueLista::bind);
    ui.register_handler("frmComSaidasDeEstoqueLista.form_show", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueLista::form_show));
    ui.register_handler("frmComSaidasDeEstoqueLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueLista::btn_abrir_click));
    ui.register_handler("frmComSaidasDeEstoqueLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueLista::edt_busca_change));
    ui.register_handler("frmComSaidasDeEstoqueLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueLista::grd_lista_dbl_click));
    ui.register_form_class("frmComSaidasDeEstoqueConsulta", FrmComSaidasDeEstoqueConsulta::bind);
    ui.register_handler("frmComSaidasDeEstoqueConsulta.form_show", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueConsulta::form_show));
    ui.register_handler("frmComSaidasDeEstoqueConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueConsulta::edt_busca_change));
    ui.register_form_class("frmComSaidasDeEstoqueHist", FrmComSaidasDeEstoqueHist::bind);
    ui.register_handler("frmComSaidasDeEstoqueHist.form_show", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueHist::form_show));
    ui.register_handler("frmComSaidasDeEstoqueHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComSaidasDeEstoqueHist::edt_busca_change));
    ui.register_form_class("frmComTransferenciasDeEstoqueCad", FrmComTransferenciasDeEstoqueCad::bind);
    ui.register_handler("frmComTransferenciasDeEstoqueCad.form_show", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::form_show));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_novo_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_salvar_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_excluir_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_cancelar_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_novo_item_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::btn_fechar_click));
    ui.register_handler("frmComTransferenciasDeEstoqueCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueCad::edt_busca_change));
    ui.register_form_class("frmComTransferenciasDeEstoqueLista", FrmComTransferenciasDeEstoqueLista::bind);
    ui.register_handler("frmComTransferenciasDeEstoqueLista.form_show", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueLista::form_show));
    ui.register_handler("frmComTransferenciasDeEstoqueLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueLista::btn_abrir_click));
    ui.register_handler("frmComTransferenciasDeEstoqueLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueLista::edt_busca_change));
    ui.register_handler("frmComTransferenciasDeEstoqueLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueLista::grd_lista_dbl_click));
    ui.register_form_class("frmComTransferenciasDeEstoqueConsulta", FrmComTransferenciasDeEstoqueConsulta::bind);
    ui.register_handler("frmComTransferenciasDeEstoqueConsulta.form_show", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueConsulta::form_show));
    ui.register_handler("frmComTransferenciasDeEstoqueConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueConsulta::edt_busca_change));
    ui.register_form_class("frmComTransferenciasDeEstoqueHist", FrmComTransferenciasDeEstoqueHist::bind);
    ui.register_handler("frmComTransferenciasDeEstoqueHist.form_show", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueHist::form_show));
    ui.register_handler("frmComTransferenciasDeEstoqueHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComTransferenciasDeEstoqueHist::edt_busca_change));
    ui.register_form_class("frmComInventariosCad", FrmComInventariosCad::bind);
    ui.register_handler("frmComInventariosCad.form_show", aylis::prelude::instances::form_handler(FrmComInventariosCad::form_show));
    ui.register_handler("frmComInventariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_novo_click));
    ui.register_handler("frmComInventariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_salvar_click));
    ui.register_handler("frmComInventariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_excluir_click));
    ui.register_handler("frmComInventariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_cancelar_click));
    ui.register_handler("frmComInventariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_novo_item_click));
    ui.register_handler("frmComInventariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComInventariosCad::btn_fechar_click));
    ui.register_handler("frmComInventariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComInventariosCad::edt_busca_change));
    ui.register_form_class("frmComInventariosLista", FrmComInventariosLista::bind);
    ui.register_handler("frmComInventariosLista.form_show", aylis::prelude::instances::form_handler(FrmComInventariosLista::form_show));
    ui.register_handler("frmComInventariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComInventariosLista::btn_abrir_click));
    ui.register_handler("frmComInventariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComInventariosLista::edt_busca_change));
    ui.register_handler("frmComInventariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComInventariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmComInventariosConsulta", FrmComInventariosConsulta::bind);
    ui.register_handler("frmComInventariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmComInventariosConsulta::form_show));
    ui.register_handler("frmComInventariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComInventariosConsulta::edt_busca_change));
    ui.register_form_class("frmComInventariosHist", FrmComInventariosHist::bind);
    ui.register_handler("frmComInventariosHist.form_show", aylis::prelude::instances::form_handler(FrmComInventariosHist::form_show));
    ui.register_handler("frmComInventariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComInventariosHist::edt_busca_change));
    ui.register_form_class("frmComAjustesDeEstoqueCad", FrmComAjustesDeEstoqueCad::bind);
    ui.register_handler("frmComAjustesDeEstoqueCad.form_show", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::form_show));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_novo_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_salvar_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_excluir_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_cancelar_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_novo_item_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::btn_fechar_click));
    ui.register_handler("frmComAjustesDeEstoqueCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueCad::edt_busca_change));
    ui.register_form_class("frmComAjustesDeEstoqueLista", FrmComAjustesDeEstoqueLista::bind);
    ui.register_handler("frmComAjustesDeEstoqueLista.form_show", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueLista::form_show));
    ui.register_handler("frmComAjustesDeEstoqueLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueLista::btn_abrir_click));
    ui.register_handler("frmComAjustesDeEstoqueLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueLista::edt_busca_change));
    ui.register_handler("frmComAjustesDeEstoqueLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAjustesDeEstoqueConsulta", FrmComAjustesDeEstoqueConsulta::bind);
    ui.register_handler("frmComAjustesDeEstoqueConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueConsulta::form_show));
    ui.register_handler("frmComAjustesDeEstoqueConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueConsulta::edt_busca_change));
    ui.register_form_class("frmComAjustesDeEstoqueHist", FrmComAjustesDeEstoqueHist::bind);
    ui.register_handler("frmComAjustesDeEstoqueHist.form_show", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueHist::form_show));
    ui.register_handler("frmComAjustesDeEstoqueHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAjustesDeEstoqueHist::edt_busca_change));
    ui.register_form_class("frmComAprovacoesDeCompraCad", FrmComAprovacoesDeCompraCad::bind);
    ui.register_handler("frmComAprovacoesDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::form_show));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_novo_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComAprovacoesDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComAprovacoesDeCompraLista", FrmComAprovacoesDeCompraLista::bind);
    ui.register_handler("frmComAprovacoesDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraLista::form_show));
    ui.register_handler("frmComAprovacoesDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComAprovacoesDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraLista::edt_busca_change));
    ui.register_handler("frmComAprovacoesDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComAprovacoesDeCompraConsulta", FrmComAprovacoesDeCompraConsulta::bind);
    ui.register_handler("frmComAprovacoesDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraConsulta::form_show));
    ui.register_handler("frmComAprovacoesDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComAprovacoesDeCompraHist", FrmComAprovacoesDeCompraHist::bind);
    ui.register_handler("frmComAprovacoesDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraHist::form_show));
    ui.register_handler("frmComAprovacoesDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComAprovacoesDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComOrdensDeServicoExternasCad", FrmComOrdensDeServicoExternasCad::bind);
    ui.register_handler("frmComOrdensDeServicoExternasCad.form_show", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::form_show));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_novo_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_salvar_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_excluir_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_cancelar_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_novo_item_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::btn_fechar_click));
    ui.register_handler("frmComOrdensDeServicoExternasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasCad::edt_busca_change));
    ui.register_form_class("frmComOrdensDeServicoExternasLista", FrmComOrdensDeServicoExternasLista::bind);
    ui.register_handler("frmComOrdensDeServicoExternasLista.form_show", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasLista::form_show));
    ui.register_handler("frmComOrdensDeServicoExternasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasLista::btn_abrir_click));
    ui.register_handler("frmComOrdensDeServicoExternasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasLista::edt_busca_change));
    ui.register_handler("frmComOrdensDeServicoExternasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasLista::grd_lista_dbl_click));
    ui.register_form_class("frmComOrdensDeServicoExternasConsulta", FrmComOrdensDeServicoExternasConsulta::bind);
    ui.register_handler("frmComOrdensDeServicoExternasConsulta.form_show", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasConsulta::form_show));
    ui.register_handler("frmComOrdensDeServicoExternasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasConsulta::edt_busca_change));
    ui.register_form_class("frmComOrdensDeServicoExternasHist", FrmComOrdensDeServicoExternasHist::bind);
    ui.register_handler("frmComOrdensDeServicoExternasHist.form_show", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasHist::form_show));
    ui.register_handler("frmComOrdensDeServicoExternasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComOrdensDeServicoExternasHist::edt_busca_change));
    ui.register_form_class("frmComContratosDeCompraCad", FrmComContratosDeCompraCad::bind);
    ui.register_handler("frmComContratosDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::form_show));
    ui.register_handler("frmComContratosDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_novo_click));
    ui.register_handler("frmComContratosDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComContratosDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComContratosDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComContratosDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComContratosDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComContratosDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComContratosDeCompraLista", FrmComContratosDeCompraLista::bind);
    ui.register_handler("frmComContratosDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeCompraLista::form_show));
    ui.register_handler("frmComContratosDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComContratosDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeCompraLista::edt_busca_change));
    ui.register_handler("frmComContratosDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComContratosDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComContratosDeCompraConsulta", FrmComContratosDeCompraConsulta::bind);
    ui.register_handler("frmComContratosDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeCompraConsulta::form_show));
    ui.register_handler("frmComContratosDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComContratosDeCompraHist", FrmComContratosDeCompraHist::bind);
    ui.register_handler("frmComContratosDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComContratosDeCompraHist::form_show));
    ui.register_handler("frmComContratosDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComContratosDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComPrevisoesDeCompraCad", FrmComPrevisoesDeCompraCad::bind);
    ui.register_handler("frmComPrevisoesDeCompraCad.form_show", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::form_show));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_novo_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_salvar_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_excluir_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_cancelar_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_novo_item_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::btn_fechar_click));
    ui.register_handler("frmComPrevisoesDeCompraCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraCad::edt_busca_change));
    ui.register_form_class("frmComPrevisoesDeCompraLista", FrmComPrevisoesDeCompraLista::bind);
    ui.register_handler("frmComPrevisoesDeCompraLista.form_show", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraLista::form_show));
    ui.register_handler("frmComPrevisoesDeCompraLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraLista::btn_abrir_click));
    ui.register_handler("frmComPrevisoesDeCompraLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraLista::edt_busca_change));
    ui.register_handler("frmComPrevisoesDeCompraLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraLista::grd_lista_dbl_click));
    ui.register_form_class("frmComPrevisoesDeCompraConsulta", FrmComPrevisoesDeCompraConsulta::bind);
    ui.register_handler("frmComPrevisoesDeCompraConsulta.form_show", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraConsulta::form_show));
    ui.register_handler("frmComPrevisoesDeCompraConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraConsulta::edt_busca_change));
    ui.register_form_class("frmComPrevisoesDeCompraHist", FrmComPrevisoesDeCompraHist::bind);
    ui.register_handler("frmComPrevisoesDeCompraHist.form_show", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraHist::form_show));
    ui.register_handler("frmComPrevisoesDeCompraHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComPrevisoesDeCompraHist::edt_busca_change));
    ui.register_form_class("frmComImportacoesCad", FrmComImportacoesCad::bind);
    ui.register_handler("frmComImportacoesCad.form_show", aylis::prelude::instances::form_handler(FrmComImportacoesCad::form_show));
    ui.register_handler("frmComImportacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_novo_click));
    ui.register_handler("frmComImportacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_salvar_click));
    ui.register_handler("frmComImportacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_excluir_click));
    ui.register_handler("frmComImportacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_cancelar_click));
    ui.register_handler("frmComImportacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_novo_item_click));
    ui.register_handler("frmComImportacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComImportacoesCad::btn_fechar_click));
    ui.register_handler("frmComImportacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComImportacoesCad::edt_busca_change));
    ui.register_form_class("frmComImportacoesLista", FrmComImportacoesLista::bind);
    ui.register_handler("frmComImportacoesLista.form_show", aylis::prelude::instances::form_handler(FrmComImportacoesLista::form_show));
    ui.register_handler("frmComImportacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComImportacoesLista::btn_abrir_click));
    ui.register_handler("frmComImportacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComImportacoesLista::edt_busca_change));
    ui.register_handler("frmComImportacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComImportacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmComImportacoesConsulta", FrmComImportacoesConsulta::bind);
    ui.register_handler("frmComImportacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmComImportacoesConsulta::form_show));
    ui.register_handler("frmComImportacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComImportacoesConsulta::edt_busca_change));
    ui.register_form_class("frmComImportacoesHist", FrmComImportacoesHist::bind);
    ui.register_handler("frmComImportacoesHist.form_show", aylis::prelude::instances::form_handler(FrmComImportacoesHist::form_show));
    ui.register_handler("frmComImportacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComImportacoesHist::edt_busca_change));
    ui.register_form_class("frmComDespesasDeImportacaoCad", FrmComDespesasDeImportacaoCad::bind);
    ui.register_handler("frmComDespesasDeImportacaoCad.form_show", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::form_show));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_novo_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_salvar_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_excluir_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_cancelar_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_novo_item_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::btn_fechar_click));
    ui.register_handler("frmComDespesasDeImportacaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoCad::edt_busca_change));
    ui.register_form_class("frmComDespesasDeImportacaoLista", FrmComDespesasDeImportacaoLista::bind);
    ui.register_handler("frmComDespesasDeImportacaoLista.form_show", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoLista::form_show));
    ui.register_handler("frmComDespesasDeImportacaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoLista::btn_abrir_click));
    ui.register_handler("frmComDespesasDeImportacaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoLista::edt_busca_change));
    ui.register_handler("frmComDespesasDeImportacaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComDespesasDeImportacaoConsulta", FrmComDespesasDeImportacaoConsulta::bind);
    ui.register_handler("frmComDespesasDeImportacaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoConsulta::form_show));
    ui.register_handler("frmComDespesasDeImportacaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoConsulta::edt_busca_change));
    ui.register_form_class("frmComDespesasDeImportacaoHist", FrmComDespesasDeImportacaoHist::bind);
    ui.register_handler("frmComDespesasDeImportacaoHist.form_show", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoHist::form_show));
    ui.register_handler("frmComDespesasDeImportacaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComDespesasDeImportacaoHist::edt_busca_change));
    ui.register_form_class("frmComConferenciasDeRecebimentoCad", FrmComConferenciasDeRecebimentoCad::bind);
    ui.register_handler("frmComConferenciasDeRecebimentoCad.form_show", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::form_show));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_novo_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_salvar_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_excluir_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_cancelar_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_novo_item_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::btn_fechar_click));
    ui.register_handler("frmComConferenciasDeRecebimentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoCad::edt_busca_change));
    ui.register_form_class("frmComConferenciasDeRecebimentoLista", FrmComConferenciasDeRecebimentoLista::bind);
    ui.register_handler("frmComConferenciasDeRecebimentoLista.form_show", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoLista::form_show));
    ui.register_handler("frmComConferenciasDeRecebimentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoLista::btn_abrir_click));
    ui.register_handler("frmComConferenciasDeRecebimentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoLista::edt_busca_change));
    ui.register_handler("frmComConferenciasDeRecebimentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmComConferenciasDeRecebimentoConsulta", FrmComConferenciasDeRecebimentoConsulta::bind);
    ui.register_handler("frmComConferenciasDeRecebimentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoConsulta::form_show));
    ui.register_handler("frmComConferenciasDeRecebimentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoConsulta::edt_busca_change));
    ui.register_form_class("frmComConferenciasDeRecebimentoHist", FrmComConferenciasDeRecebimentoHist::bind);
    ui.register_handler("frmComConferenciasDeRecebimentoHist.form_show", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoHist::form_show));
    ui.register_handler("frmComConferenciasDeRecebimentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmComConferenciasDeRecebimentoHist::edt_busca_change));
    ui.register_form_class("frmVenClientesCad", FrmVenClientesCad::bind);
    ui.register_handler("frmVenClientesCad.form_show", aylis::prelude::instances::form_handler(FrmVenClientesCad::form_show));
    ui.register_handler("frmVenClientesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_novo_click));
    ui.register_handler("frmVenClientesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_salvar_click));
    ui.register_handler("frmVenClientesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_excluir_click));
    ui.register_handler("frmVenClientesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_cancelar_click));
    ui.register_handler("frmVenClientesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_novo_item_click));
    ui.register_handler("frmVenClientesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenClientesCad::btn_fechar_click));
    ui.register_handler("frmVenClientesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenClientesCad::edt_busca_change));
    ui.register_form_class("frmVenClientesLista", FrmVenClientesLista::bind);
    ui.register_handler("frmVenClientesLista.form_show", aylis::prelude::instances::form_handler(FrmVenClientesLista::form_show));
    ui.register_handler("frmVenClientesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenClientesLista::btn_abrir_click));
    ui.register_handler("frmVenClientesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenClientesLista::edt_busca_change));
    ui.register_handler("frmVenClientesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenClientesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenClientesConsulta", FrmVenClientesConsulta::bind);
    ui.register_handler("frmVenClientesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenClientesConsulta::form_show));
    ui.register_handler("frmVenClientesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenClientesConsulta::edt_busca_change));
    ui.register_form_class("frmVenClientesHist", FrmVenClientesHist::bind);
    ui.register_handler("frmVenClientesHist.form_show", aylis::prelude::instances::form_handler(FrmVenClientesHist::form_show));
    ui.register_handler("frmVenClientesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenClientesHist::edt_busca_change));
    ui.register_form_class("frmVenVendedoresCad", FrmVenVendedoresCad::bind);
    ui.register_handler("frmVenVendedoresCad.form_show", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::form_show));
    ui.register_handler("frmVenVendedoresCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_novo_click));
    ui.register_handler("frmVenVendedoresCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_salvar_click));
    ui.register_handler("frmVenVendedoresCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_excluir_click));
    ui.register_handler("frmVenVendedoresCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_cancelar_click));
    ui.register_handler("frmVenVendedoresCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_novo_item_click));
    ui.register_handler("frmVenVendedoresCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::btn_fechar_click));
    ui.register_handler("frmVenVendedoresCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVendedoresCad::edt_busca_change));
    ui.register_form_class("frmVenVendedoresLista", FrmVenVendedoresLista::bind);
    ui.register_handler("frmVenVendedoresLista.form_show", aylis::prelude::instances::form_handler(FrmVenVendedoresLista::form_show));
    ui.register_handler("frmVenVendedoresLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenVendedoresLista::btn_abrir_click));
    ui.register_handler("frmVenVendedoresLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVendedoresLista::edt_busca_change));
    ui.register_handler("frmVenVendedoresLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenVendedoresLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenVendedoresConsulta", FrmVenVendedoresConsulta::bind);
    ui.register_handler("frmVenVendedoresConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenVendedoresConsulta::form_show));
    ui.register_handler("frmVenVendedoresConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVendedoresConsulta::edt_busca_change));
    ui.register_form_class("frmVenVendedoresHist", FrmVenVendedoresHist::bind);
    ui.register_handler("frmVenVendedoresHist.form_show", aylis::prelude::instances::form_handler(FrmVenVendedoresHist::form_show));
    ui.register_handler("frmVenVendedoresHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVendedoresHist::edt_busca_change));
    ui.register_form_class("frmVenRepresentantesCad", FrmVenRepresentantesCad::bind);
    ui.register_handler("frmVenRepresentantesCad.form_show", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::form_show));
    ui.register_handler("frmVenRepresentantesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_novo_click));
    ui.register_handler("frmVenRepresentantesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_salvar_click));
    ui.register_handler("frmVenRepresentantesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_excluir_click));
    ui.register_handler("frmVenRepresentantesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_cancelar_click));
    ui.register_handler("frmVenRepresentantesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_novo_item_click));
    ui.register_handler("frmVenRepresentantesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::btn_fechar_click));
    ui.register_handler("frmVenRepresentantesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRepresentantesCad::edt_busca_change));
    ui.register_form_class("frmVenRepresentantesLista", FrmVenRepresentantesLista::bind);
    ui.register_handler("frmVenRepresentantesLista.form_show", aylis::prelude::instances::form_handler(FrmVenRepresentantesLista::form_show));
    ui.register_handler("frmVenRepresentantesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesLista::btn_abrir_click));
    ui.register_handler("frmVenRepresentantesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRepresentantesLista::edt_busca_change));
    ui.register_handler("frmVenRepresentantesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenRepresentantesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenRepresentantesConsulta", FrmVenRepresentantesConsulta::bind);
    ui.register_handler("frmVenRepresentantesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenRepresentantesConsulta::form_show));
    ui.register_handler("frmVenRepresentantesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRepresentantesConsulta::edt_busca_change));
    ui.register_form_class("frmVenRepresentantesHist", FrmVenRepresentantesHist::bind);
    ui.register_handler("frmVenRepresentantesHist.form_show", aylis::prelude::instances::form_handler(FrmVenRepresentantesHist::form_show));
    ui.register_handler("frmVenRepresentantesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRepresentantesHist::edt_busca_change));
    ui.register_form_class("frmVenRegioesDeVendaCad", FrmVenRegioesDeVendaCad::bind);
    ui.register_handler("frmVenRegioesDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::form_show));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenRegioesDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenRegioesDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenRegioesDeVendaLista", FrmVenRegioesDeVendaLista::bind);
    ui.register_handler("frmVenRegioesDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaLista::form_show));
    ui.register_handler("frmVenRegioesDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenRegioesDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenRegioesDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenRegioesDeVendaConsulta", FrmVenRegioesDeVendaConsulta::bind);
    ui.register_handler("frmVenRegioesDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaConsulta::form_show));
    ui.register_handler("frmVenRegioesDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenRegioesDeVendaHist", FrmVenRegioesDeVendaHist::bind);
    ui.register_handler("frmVenRegioesDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaHist::form_show));
    ui.register_handler("frmVenRegioesDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRegioesDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenTabelasDePrecoCad", FrmVenTabelasDePrecoCad::bind);
    ui.register_handler("frmVenTabelasDePrecoCad.form_show", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::form_show));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_novo_click));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_salvar_click));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_excluir_click));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_cancelar_click));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_novo_item_click));
    ui.register_handler("frmVenTabelasDePrecoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::btn_fechar_click));
    ui.register_handler("frmVenTabelasDePrecoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoCad::edt_busca_change));
    ui.register_form_class("frmVenTabelasDePrecoLista", FrmVenTabelasDePrecoLista::bind);
    ui.register_handler("frmVenTabelasDePrecoLista.form_show", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoLista::form_show));
    ui.register_handler("frmVenTabelasDePrecoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoLista::btn_abrir_click));
    ui.register_handler("frmVenTabelasDePrecoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoLista::edt_busca_change));
    ui.register_handler("frmVenTabelasDePrecoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenTabelasDePrecoConsulta", FrmVenTabelasDePrecoConsulta::bind);
    ui.register_handler("frmVenTabelasDePrecoConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoConsulta::form_show));
    ui.register_handler("frmVenTabelasDePrecoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoConsulta::edt_busca_change));
    ui.register_form_class("frmVenTabelasDePrecoHist", FrmVenTabelasDePrecoHist::bind);
    ui.register_handler("frmVenTabelasDePrecoHist.form_show", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoHist::form_show));
    ui.register_handler("frmVenTabelasDePrecoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTabelasDePrecoHist::edt_busca_change));
    ui.register_form_class("frmVenCampanhasCad", FrmVenCampanhasCad::bind);
    ui.register_handler("frmVenCampanhasCad.form_show", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::form_show));
    ui.register_handler("frmVenCampanhasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_novo_click));
    ui.register_handler("frmVenCampanhasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_salvar_click));
    ui.register_handler("frmVenCampanhasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_excluir_click));
    ui.register_handler("frmVenCampanhasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_cancelar_click));
    ui.register_handler("frmVenCampanhasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_novo_item_click));
    ui.register_handler("frmVenCampanhasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::btn_fechar_click));
    ui.register_handler("frmVenCampanhasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCampanhasCad::edt_busca_change));
    ui.register_form_class("frmVenCampanhasLista", FrmVenCampanhasLista::bind);
    ui.register_handler("frmVenCampanhasLista.form_show", aylis::prelude::instances::form_handler(FrmVenCampanhasLista::form_show));
    ui.register_handler("frmVenCampanhasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenCampanhasLista::btn_abrir_click));
    ui.register_handler("frmVenCampanhasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCampanhasLista::edt_busca_change));
    ui.register_handler("frmVenCampanhasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenCampanhasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenCampanhasConsulta", FrmVenCampanhasConsulta::bind);
    ui.register_handler("frmVenCampanhasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenCampanhasConsulta::form_show));
    ui.register_handler("frmVenCampanhasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCampanhasConsulta::edt_busca_change));
    ui.register_form_class("frmVenCampanhasHist", FrmVenCampanhasHist::bind);
    ui.register_handler("frmVenCampanhasHist.form_show", aylis::prelude::instances::form_handler(FrmVenCampanhasHist::form_show));
    ui.register_handler("frmVenCampanhasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCampanhasHist::edt_busca_change));
    ui.register_form_class("frmVenPromocoesCad", FrmVenPromocoesCad::bind);
    ui.register_handler("frmVenPromocoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::form_show));
    ui.register_handler("frmVenPromocoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_novo_click));
    ui.register_handler("frmVenPromocoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_salvar_click));
    ui.register_handler("frmVenPromocoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_excluir_click));
    ui.register_handler("frmVenPromocoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_cancelar_click));
    ui.register_handler("frmVenPromocoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_novo_item_click));
    ui.register_handler("frmVenPromocoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::btn_fechar_click));
    ui.register_handler("frmVenPromocoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPromocoesCad::edt_busca_change));
    ui.register_form_class("frmVenPromocoesLista", FrmVenPromocoesLista::bind);
    ui.register_handler("frmVenPromocoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenPromocoesLista::form_show));
    ui.register_handler("frmVenPromocoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPromocoesLista::btn_abrir_click));
    ui.register_handler("frmVenPromocoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPromocoesLista::edt_busca_change));
    ui.register_handler("frmVenPromocoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPromocoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPromocoesConsulta", FrmVenPromocoesConsulta::bind);
    ui.register_handler("frmVenPromocoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPromocoesConsulta::form_show));
    ui.register_handler("frmVenPromocoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPromocoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenPromocoesHist", FrmVenPromocoesHist::bind);
    ui.register_handler("frmVenPromocoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenPromocoesHist::form_show));
    ui.register_handler("frmVenPromocoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPromocoesHist::edt_busca_change));
    ui.register_form_class("frmVenMetasDeVendaCad", FrmVenMetasDeVendaCad::bind);
    ui.register_handler("frmVenMetasDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::form_show));
    ui.register_handler("frmVenMetasDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenMetasDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenMetasDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenMetasDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenMetasDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenMetasDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenMetasDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenMetasDeVendaLista", FrmVenMetasDeVendaLista::bind);
    ui.register_handler("frmVenMetasDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaLista::form_show));
    ui.register_handler("frmVenMetasDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenMetasDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenMetasDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenMetasDeVendaConsulta", FrmVenMetasDeVendaConsulta::bind);
    ui.register_handler("frmVenMetasDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaConsulta::form_show));
    ui.register_handler("frmVenMetasDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenMetasDeVendaHist", FrmVenMetasDeVendaHist::bind);
    ui.register_handler("frmVenMetasDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaHist::form_show));
    ui.register_handler("frmVenMetasDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMetasDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenCanaisDeVendaCad", FrmVenCanaisDeVendaCad::bind);
    ui.register_handler("frmVenCanaisDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::form_show));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenCanaisDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenCanaisDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenCanaisDeVendaLista", FrmVenCanaisDeVendaLista::bind);
    ui.register_handler("frmVenCanaisDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaLista::form_show));
    ui.register_handler("frmVenCanaisDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenCanaisDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenCanaisDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenCanaisDeVendaConsulta", FrmVenCanaisDeVendaConsulta::bind);
    ui.register_handler("frmVenCanaisDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaConsulta::form_show));
    ui.register_handler("frmVenCanaisDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenCanaisDeVendaHist", FrmVenCanaisDeVendaHist::bind);
    ui.register_handler("frmVenCanaisDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaHist::form_show));
    ui.register_handler("frmVenCanaisDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenCanaisDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenSegmentosDeClienteCad", FrmVenSegmentosDeClienteCad::bind);
    ui.register_handler("frmVenSegmentosDeClienteCad.form_show", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::form_show));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_novo_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_salvar_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_excluir_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_cancelar_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_novo_item_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::btn_fechar_click));
    ui.register_handler("frmVenSegmentosDeClienteCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteCad::edt_busca_change));
    ui.register_form_class("frmVenSegmentosDeClienteLista", FrmVenSegmentosDeClienteLista::bind);
    ui.register_handler("frmVenSegmentosDeClienteLista.form_show", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteLista::form_show));
    ui.register_handler("frmVenSegmentosDeClienteLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteLista::btn_abrir_click));
    ui.register_handler("frmVenSegmentosDeClienteLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteLista::edt_busca_change));
    ui.register_handler("frmVenSegmentosDeClienteLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenSegmentosDeClienteConsulta", FrmVenSegmentosDeClienteConsulta::bind);
    ui.register_handler("frmVenSegmentosDeClienteConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteConsulta::form_show));
    ui.register_handler("frmVenSegmentosDeClienteConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteConsulta::edt_busca_change));
    ui.register_form_class("frmVenSegmentosDeClienteHist", FrmVenSegmentosDeClienteHist::bind);
    ui.register_handler("frmVenSegmentosDeClienteHist.form_show", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteHist::form_show));
    ui.register_handler("frmVenSegmentosDeClienteHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenSegmentosDeClienteHist::edt_busca_change));
    ui.register_form_class("frmVenComissoesCad", FrmVenComissoesCad::bind);
    ui.register_handler("frmVenComissoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenComissoesCad::form_show));
    ui.register_handler("frmVenComissoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_novo_click));
    ui.register_handler("frmVenComissoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_salvar_click));
    ui.register_handler("frmVenComissoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_excluir_click));
    ui.register_handler("frmVenComissoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_cancelar_click));
    ui.register_handler("frmVenComissoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_novo_item_click));
    ui.register_handler("frmVenComissoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenComissoesCad::btn_fechar_click));
    ui.register_handler("frmVenComissoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenComissoesCad::edt_busca_change));
    ui.register_form_class("frmVenComissoesLista", FrmVenComissoesLista::bind);
    ui.register_handler("frmVenComissoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenComissoesLista::form_show));
    ui.register_handler("frmVenComissoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenComissoesLista::btn_abrir_click));
    ui.register_handler("frmVenComissoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenComissoesLista::edt_busca_change));
    ui.register_handler("frmVenComissoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenComissoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenComissoesConsulta", FrmVenComissoesConsulta::bind);
    ui.register_handler("frmVenComissoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenComissoesConsulta::form_show));
    ui.register_handler("frmVenComissoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenComissoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenComissoesHist", FrmVenComissoesHist::bind);
    ui.register_handler("frmVenComissoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenComissoesHist::form_show));
    ui.register_handler("frmVenComissoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenComissoesHist::edt_busca_change));
    ui.register_form_class("frmVenRotasDeEntregaCad", FrmVenRotasDeEntregaCad::bind);
    ui.register_handler("frmVenRotasDeEntregaCad.form_show", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::form_show));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_novo_click));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_salvar_click));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_excluir_click));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_cancelar_click));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_novo_item_click));
    ui.register_handler("frmVenRotasDeEntregaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::btn_fechar_click));
    ui.register_handler("frmVenRotasDeEntregaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaCad::edt_busca_change));
    ui.register_form_class("frmVenRotasDeEntregaLista", FrmVenRotasDeEntregaLista::bind);
    ui.register_handler("frmVenRotasDeEntregaLista.form_show", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaLista::form_show));
    ui.register_handler("frmVenRotasDeEntregaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaLista::btn_abrir_click));
    ui.register_handler("frmVenRotasDeEntregaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaLista::edt_busca_change));
    ui.register_handler("frmVenRotasDeEntregaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenRotasDeEntregaConsulta", FrmVenRotasDeEntregaConsulta::bind);
    ui.register_handler("frmVenRotasDeEntregaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaConsulta::form_show));
    ui.register_handler("frmVenRotasDeEntregaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaConsulta::edt_busca_change));
    ui.register_form_class("frmVenRotasDeEntregaHist", FrmVenRotasDeEntregaHist::bind);
    ui.register_handler("frmVenRotasDeEntregaHist.form_show", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaHist::form_show));
    ui.register_handler("frmVenRotasDeEntregaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRotasDeEntregaHist::edt_busca_change));
    ui.register_form_class("frmVenMotivosDePerdaCad", FrmVenMotivosDePerdaCad::bind);
    ui.register_handler("frmVenMotivosDePerdaCad.form_show", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::form_show));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_novo_click));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_salvar_click));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_excluir_click));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_cancelar_click));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_novo_item_click));
    ui.register_handler("frmVenMotivosDePerdaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::btn_fechar_click));
    ui.register_handler("frmVenMotivosDePerdaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaCad::edt_busca_change));
    ui.register_form_class("frmVenMotivosDePerdaLista", FrmVenMotivosDePerdaLista::bind);
    ui.register_handler("frmVenMotivosDePerdaLista.form_show", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaLista::form_show));
    ui.register_handler("frmVenMotivosDePerdaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaLista::btn_abrir_click));
    ui.register_handler("frmVenMotivosDePerdaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaLista::edt_busca_change));
    ui.register_handler("frmVenMotivosDePerdaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenMotivosDePerdaConsulta", FrmVenMotivosDePerdaConsulta::bind);
    ui.register_handler("frmVenMotivosDePerdaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaConsulta::form_show));
    ui.register_handler("frmVenMotivosDePerdaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaConsulta::edt_busca_change));
    ui.register_form_class("frmVenMotivosDePerdaHist", FrmVenMotivosDePerdaHist::bind);
    ui.register_handler("frmVenMotivosDePerdaHist.form_show", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaHist::form_show));
    ui.register_handler("frmVenMotivosDePerdaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenMotivosDePerdaHist::edt_busca_change));
    ui.register_form_class("frmVenTiposDePedidoCad", FrmVenTiposDePedidoCad::bind);
    ui.register_handler("frmVenTiposDePedidoCad.form_show", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::form_show));
    ui.register_handler("frmVenTiposDePedidoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_novo_click));
    ui.register_handler("frmVenTiposDePedidoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_salvar_click));
    ui.register_handler("frmVenTiposDePedidoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_excluir_click));
    ui.register_handler("frmVenTiposDePedidoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_cancelar_click));
    ui.register_handler("frmVenTiposDePedidoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_novo_item_click));
    ui.register_handler("frmVenTiposDePedidoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::btn_fechar_click));
    ui.register_handler("frmVenTiposDePedidoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoCad::edt_busca_change));
    ui.register_form_class("frmVenTiposDePedidoLista", FrmVenTiposDePedidoLista::bind);
    ui.register_handler("frmVenTiposDePedidoLista.form_show", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoLista::form_show));
    ui.register_handler("frmVenTiposDePedidoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoLista::btn_abrir_click));
    ui.register_handler("frmVenTiposDePedidoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoLista::edt_busca_change));
    ui.register_handler("frmVenTiposDePedidoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenTiposDePedidoConsulta", FrmVenTiposDePedidoConsulta::bind);
    ui.register_handler("frmVenTiposDePedidoConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoConsulta::form_show));
    ui.register_handler("frmVenTiposDePedidoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoConsulta::edt_busca_change));
    ui.register_form_class("frmVenTiposDePedidoHist", FrmVenTiposDePedidoHist::bind);
    ui.register_handler("frmVenTiposDePedidoHist.form_show", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoHist::form_show));
    ui.register_handler("frmVenTiposDePedidoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTiposDePedidoHist::edt_busca_change));
    ui.register_form_class("frmVenPrazosDeEntregaCad", FrmVenPrazosDeEntregaCad::bind);
    ui.register_handler("frmVenPrazosDeEntregaCad.form_show", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::form_show));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_novo_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_salvar_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_excluir_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_cancelar_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_novo_item_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::btn_fechar_click));
    ui.register_handler("frmVenPrazosDeEntregaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaCad::edt_busca_change));
    ui.register_form_class("frmVenPrazosDeEntregaLista", FrmVenPrazosDeEntregaLista::bind);
    ui.register_handler("frmVenPrazosDeEntregaLista.form_show", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaLista::form_show));
    ui.register_handler("frmVenPrazosDeEntregaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaLista::btn_abrir_click));
    ui.register_handler("frmVenPrazosDeEntregaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaLista::edt_busca_change));
    ui.register_handler("frmVenPrazosDeEntregaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPrazosDeEntregaConsulta", FrmVenPrazosDeEntregaConsulta::bind);
    ui.register_handler("frmVenPrazosDeEntregaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaConsulta::form_show));
    ui.register_handler("frmVenPrazosDeEntregaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaConsulta::edt_busca_change));
    ui.register_form_class("frmVenPrazosDeEntregaHist", FrmVenPrazosDeEntregaHist::bind);
    ui.register_handler("frmVenPrazosDeEntregaHist.form_show", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaHist::form_show));
    ui.register_handler("frmVenPrazosDeEntregaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrazosDeEntregaHist::edt_busca_change));
    ui.register_form_class("frmVenContatosCad", FrmVenContatosCad::bind);
    ui.register_handler("frmVenContatosCad.form_show", aylis::prelude::instances::form_handler(FrmVenContatosCad::form_show));
    ui.register_handler("frmVenContatosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_novo_click));
    ui.register_handler("frmVenContatosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_salvar_click));
    ui.register_handler("frmVenContatosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_excluir_click));
    ui.register_handler("frmVenContatosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_cancelar_click));
    ui.register_handler("frmVenContatosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_novo_item_click));
    ui.register_handler("frmVenContatosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenContatosCad::btn_fechar_click));
    ui.register_handler("frmVenContatosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContatosCad::edt_busca_change));
    ui.register_form_class("frmVenContatosLista", FrmVenContatosLista::bind);
    ui.register_handler("frmVenContatosLista.form_show", aylis::prelude::instances::form_handler(FrmVenContatosLista::form_show));
    ui.register_handler("frmVenContatosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenContatosLista::btn_abrir_click));
    ui.register_handler("frmVenContatosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContatosLista::edt_busca_change));
    ui.register_handler("frmVenContatosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenContatosLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenContatosConsulta", FrmVenContatosConsulta::bind);
    ui.register_handler("frmVenContatosConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenContatosConsulta::form_show));
    ui.register_handler("frmVenContatosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContatosConsulta::edt_busca_change));
    ui.register_form_class("frmVenContatosHist", FrmVenContatosHist::bind);
    ui.register_handler("frmVenContatosHist.form_show", aylis::prelude::instances::form_handler(FrmVenContatosHist::form_show));
    ui.register_handler("frmVenContatosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContatosHist::edt_busca_change));
    ui.register_form_class("frmVenConcorrentesCad", FrmVenConcorrentesCad::bind);
    ui.register_handler("frmVenConcorrentesCad.form_show", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::form_show));
    ui.register_handler("frmVenConcorrentesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_novo_click));
    ui.register_handler("frmVenConcorrentesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_salvar_click));
    ui.register_handler("frmVenConcorrentesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_excluir_click));
    ui.register_handler("frmVenConcorrentesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_cancelar_click));
    ui.register_handler("frmVenConcorrentesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_novo_item_click));
    ui.register_handler("frmVenConcorrentesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::btn_fechar_click));
    ui.register_handler("frmVenConcorrentesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConcorrentesCad::edt_busca_change));
    ui.register_form_class("frmVenConcorrentesLista", FrmVenConcorrentesLista::bind);
    ui.register_handler("frmVenConcorrentesLista.form_show", aylis::prelude::instances::form_handler(FrmVenConcorrentesLista::form_show));
    ui.register_handler("frmVenConcorrentesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesLista::btn_abrir_click));
    ui.register_handler("frmVenConcorrentesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConcorrentesLista::edt_busca_change));
    ui.register_handler("frmVenConcorrentesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenConcorrentesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenConcorrentesConsulta", FrmVenConcorrentesConsulta::bind);
    ui.register_handler("frmVenConcorrentesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenConcorrentesConsulta::form_show));
    ui.register_handler("frmVenConcorrentesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConcorrentesConsulta::edt_busca_change));
    ui.register_form_class("frmVenConcorrentesHist", FrmVenConcorrentesHist::bind);
    ui.register_handler("frmVenConcorrentesHist.form_show", aylis::prelude::instances::form_handler(FrmVenConcorrentesHist::form_show));
    ui.register_handler("frmVenConcorrentesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConcorrentesHist::edt_busca_change));
    ui.register_form_class("frmVenGruposDeClientesCad", FrmVenGruposDeClientesCad::bind);
    ui.register_handler("frmVenGruposDeClientesCad.form_show", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::form_show));
    ui.register_handler("frmVenGruposDeClientesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_novo_click));
    ui.register_handler("frmVenGruposDeClientesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_salvar_click));
    ui.register_handler("frmVenGruposDeClientesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_excluir_click));
    ui.register_handler("frmVenGruposDeClientesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_cancelar_click));
    ui.register_handler("frmVenGruposDeClientesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_novo_item_click));
    ui.register_handler("frmVenGruposDeClientesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::btn_fechar_click));
    ui.register_handler("frmVenGruposDeClientesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesCad::edt_busca_change));
    ui.register_form_class("frmVenGruposDeClientesLista", FrmVenGruposDeClientesLista::bind);
    ui.register_handler("frmVenGruposDeClientesLista.form_show", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesLista::form_show));
    ui.register_handler("frmVenGruposDeClientesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesLista::btn_abrir_click));
    ui.register_handler("frmVenGruposDeClientesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesLista::edt_busca_change));
    ui.register_handler("frmVenGruposDeClientesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenGruposDeClientesConsulta", FrmVenGruposDeClientesConsulta::bind);
    ui.register_handler("frmVenGruposDeClientesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesConsulta::form_show));
    ui.register_handler("frmVenGruposDeClientesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesConsulta::edt_busca_change));
    ui.register_form_class("frmVenGruposDeClientesHist", FrmVenGruposDeClientesHist::bind);
    ui.register_handler("frmVenGruposDeClientesHist.form_show", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesHist::form_show));
    ui.register_handler("frmVenGruposDeClientesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenGruposDeClientesHist::edt_busca_change));
    ui.register_form_class("frmVenDescontosCad", FrmVenDescontosCad::bind);
    ui.register_handler("frmVenDescontosCad.form_show", aylis::prelude::instances::form_handler(FrmVenDescontosCad::form_show));
    ui.register_handler("frmVenDescontosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_novo_click));
    ui.register_handler("frmVenDescontosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_salvar_click));
    ui.register_handler("frmVenDescontosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_excluir_click));
    ui.register_handler("frmVenDescontosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_cancelar_click));
    ui.register_handler("frmVenDescontosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_novo_item_click));
    ui.register_handler("frmVenDescontosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenDescontosCad::btn_fechar_click));
    ui.register_handler("frmVenDescontosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDescontosCad::edt_busca_change));
    ui.register_form_class("frmVenDescontosLista", FrmVenDescontosLista::bind);
    ui.register_handler("frmVenDescontosLista.form_show", aylis::prelude::instances::form_handler(FrmVenDescontosLista::form_show));
    ui.register_handler("frmVenDescontosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenDescontosLista::btn_abrir_click));
    ui.register_handler("frmVenDescontosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDescontosLista::edt_busca_change));
    ui.register_handler("frmVenDescontosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenDescontosLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenDescontosConsulta", FrmVenDescontosConsulta::bind);
    ui.register_handler("frmVenDescontosConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenDescontosConsulta::form_show));
    ui.register_handler("frmVenDescontosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDescontosConsulta::edt_busca_change));
    ui.register_form_class("frmVenDescontosHist", FrmVenDescontosHist::bind);
    ui.register_handler("frmVenDescontosHist.form_show", aylis::prelude::instances::form_handler(FrmVenDescontosHist::form_show));
    ui.register_handler("frmVenDescontosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDescontosHist::edt_busca_change));
    ui.register_form_class("frmVenKitsDeProdutosCad", FrmVenKitsDeProdutosCad::bind);
    ui.register_handler("frmVenKitsDeProdutosCad.form_show", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::form_show));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_novo_click));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_salvar_click));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_excluir_click));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_cancelar_click));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_novo_item_click));
    ui.register_handler("frmVenKitsDeProdutosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::btn_fechar_click));
    ui.register_handler("frmVenKitsDeProdutosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosCad::edt_busca_change));
    ui.register_form_class("frmVenKitsDeProdutosLista", FrmVenKitsDeProdutosLista::bind);
    ui.register_handler("frmVenKitsDeProdutosLista.form_show", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosLista::form_show));
    ui.register_handler("frmVenKitsDeProdutosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosLista::btn_abrir_click));
    ui.register_handler("frmVenKitsDeProdutosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosLista::edt_busca_change));
    ui.register_handler("frmVenKitsDeProdutosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenKitsDeProdutosConsulta", FrmVenKitsDeProdutosConsulta::bind);
    ui.register_handler("frmVenKitsDeProdutosConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosConsulta::form_show));
    ui.register_handler("frmVenKitsDeProdutosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosConsulta::edt_busca_change));
    ui.register_form_class("frmVenKitsDeProdutosHist", FrmVenKitsDeProdutosHist::bind);
    ui.register_handler("frmVenKitsDeProdutosHist.form_show", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosHist::form_show));
    ui.register_handler("frmVenKitsDeProdutosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenKitsDeProdutosHist::edt_busca_change));
    ui.register_form_class("frmVenPontosDeVendaCad", FrmVenPontosDeVendaCad::bind);
    ui.register_handler("frmVenPontosDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::form_show));
    ui.register_handler("frmVenPontosDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenPontosDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenPontosDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenPontosDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenPontosDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenPontosDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenPontosDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenPontosDeVendaLista", FrmVenPontosDeVendaLista::bind);
    ui.register_handler("frmVenPontosDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaLista::form_show));
    ui.register_handler("frmVenPontosDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenPontosDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenPontosDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPontosDeVendaConsulta", FrmVenPontosDeVendaConsulta::bind);
    ui.register_handler("frmVenPontosDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaConsulta::form_show));
    ui.register_handler("frmVenPontosDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenPontosDeVendaHist", FrmVenPontosDeVendaHist::bind);
    ui.register_handler("frmVenPontosDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaHist::form_show));
    ui.register_handler("frmVenPontosDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPontosDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenOrcamentosDeVendaCad", FrmVenOrcamentosDeVendaCad::bind);
    ui.register_handler("frmVenOrcamentosDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::form_show));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenOrcamentosDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenOrcamentosDeVendaLista", FrmVenOrcamentosDeVendaLista::bind);
    ui.register_handler("frmVenOrcamentosDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaLista::form_show));
    ui.register_handler("frmVenOrcamentosDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenOrcamentosDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenOrcamentosDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenOrcamentosDeVendaConsulta", FrmVenOrcamentosDeVendaConsulta::bind);
    ui.register_handler("frmVenOrcamentosDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaConsulta::form_show));
    ui.register_handler("frmVenOrcamentosDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenOrcamentosDeVendaHist", FrmVenOrcamentosDeVendaHist::bind);
    ui.register_handler("frmVenOrcamentosDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaHist::form_show));
    ui.register_handler("frmVenOrcamentosDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOrcamentosDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeVendaCad", FrmVenPedidosDeVendaCad::bind);
    ui.register_handler("frmVenPedidosDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::form_show));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenPedidosDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenPedidosDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeVendaLista", FrmVenPedidosDeVendaLista::bind);
    ui.register_handler("frmVenPedidosDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaLista::form_show));
    ui.register_handler("frmVenPedidosDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenPedidosDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenPedidosDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPedidosDeVendaConsulta", FrmVenPedidosDeVendaConsulta::bind);
    ui.register_handler("frmVenPedidosDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaConsulta::form_show));
    ui.register_handler("frmVenPedidosDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeVendaHist", FrmVenPedidosDeVendaHist::bind);
    ui.register_handler("frmVenPedidosDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaHist::form_show));
    ui.register_handler("frmVenPedidosDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenFaturamentosCad", FrmVenFaturamentosCad::bind);
    ui.register_handler("frmVenFaturamentosCad.form_show", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::form_show));
    ui.register_handler("frmVenFaturamentosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_novo_click));
    ui.register_handler("frmVenFaturamentosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_salvar_click));
    ui.register_handler("frmVenFaturamentosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_excluir_click));
    ui.register_handler("frmVenFaturamentosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_cancelar_click));
    ui.register_handler("frmVenFaturamentosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_novo_item_click));
    ui.register_handler("frmVenFaturamentosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::btn_fechar_click));
    ui.register_handler("frmVenFaturamentosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenFaturamentosCad::edt_busca_change));
    ui.register_form_class("frmVenFaturamentosLista", FrmVenFaturamentosLista::bind);
    ui.register_handler("frmVenFaturamentosLista.form_show", aylis::prelude::instances::form_handler(FrmVenFaturamentosLista::form_show));
    ui.register_handler("frmVenFaturamentosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosLista::btn_abrir_click));
    ui.register_handler("frmVenFaturamentosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenFaturamentosLista::edt_busca_change));
    ui.register_handler("frmVenFaturamentosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenFaturamentosLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenFaturamentosConsulta", FrmVenFaturamentosConsulta::bind);
    ui.register_handler("frmVenFaturamentosConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenFaturamentosConsulta::form_show));
    ui.register_handler("frmVenFaturamentosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenFaturamentosConsulta::edt_busca_change));
    ui.register_form_class("frmVenFaturamentosHist", FrmVenFaturamentosHist::bind);
    ui.register_handler("frmVenFaturamentosHist.form_show", aylis::prelude::instances::form_handler(FrmVenFaturamentosHist::form_show));
    ui.register_handler("frmVenFaturamentosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenFaturamentosHist::edt_busca_change));
    ui.register_form_class("frmVenNotasDeSaidaCad", FrmVenNotasDeSaidaCad::bind);
    ui.register_handler("frmVenNotasDeSaidaCad.form_show", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::form_show));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_novo_click));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_salvar_click));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_excluir_click));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_cancelar_click));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_novo_item_click));
    ui.register_handler("frmVenNotasDeSaidaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::btn_fechar_click));
    ui.register_handler("frmVenNotasDeSaidaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaCad::edt_busca_change));
    ui.register_form_class("frmVenNotasDeSaidaLista", FrmVenNotasDeSaidaLista::bind);
    ui.register_handler("frmVenNotasDeSaidaLista.form_show", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaLista::form_show));
    ui.register_handler("frmVenNotasDeSaidaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaLista::btn_abrir_click));
    ui.register_handler("frmVenNotasDeSaidaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaLista::edt_busca_change));
    ui.register_handler("frmVenNotasDeSaidaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenNotasDeSaidaConsulta", FrmVenNotasDeSaidaConsulta::bind);
    ui.register_handler("frmVenNotasDeSaidaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaConsulta::form_show));
    ui.register_handler("frmVenNotasDeSaidaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaConsulta::edt_busca_change));
    ui.register_form_class("frmVenNotasDeSaidaHist", FrmVenNotasDeSaidaHist::bind);
    ui.register_handler("frmVenNotasDeSaidaHist.form_show", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaHist::form_show));
    ui.register_handler("frmVenNotasDeSaidaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenNotasDeSaidaHist::edt_busca_change));
    ui.register_form_class("frmVenDevolucoesDeVendaCad", FrmVenDevolucoesDeVendaCad::bind);
    ui.register_handler("frmVenDevolucoesDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::form_show));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenDevolucoesDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenDevolucoesDeVendaLista", FrmVenDevolucoesDeVendaLista::bind);
    ui.register_handler("frmVenDevolucoesDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaLista::form_show));
    ui.register_handler("frmVenDevolucoesDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenDevolucoesDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenDevolucoesDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenDevolucoesDeVendaConsulta", FrmVenDevolucoesDeVendaConsulta::bind);
    ui.register_handler("frmVenDevolucoesDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaConsulta::form_show));
    ui.register_handler("frmVenDevolucoesDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenDevolucoesDeVendaHist", FrmVenDevolucoesDeVendaHist::bind);
    ui.register_handler("frmVenDevolucoesDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaHist::form_show));
    ui.register_handler("frmVenDevolucoesDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenDevolucoesDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenExpedicoesCad", FrmVenExpedicoesCad::bind);
    ui.register_handler("frmVenExpedicoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::form_show));
    ui.register_handler("frmVenExpedicoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_novo_click));
    ui.register_handler("frmVenExpedicoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_salvar_click));
    ui.register_handler("frmVenExpedicoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_excluir_click));
    ui.register_handler("frmVenExpedicoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_cancelar_click));
    ui.register_handler("frmVenExpedicoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_novo_item_click));
    ui.register_handler("frmVenExpedicoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::btn_fechar_click));
    ui.register_handler("frmVenExpedicoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenExpedicoesCad::edt_busca_change));
    ui.register_form_class("frmVenExpedicoesLista", FrmVenExpedicoesLista::bind);
    ui.register_handler("frmVenExpedicoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenExpedicoesLista::form_show));
    ui.register_handler("frmVenExpedicoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesLista::btn_abrir_click));
    ui.register_handler("frmVenExpedicoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenExpedicoesLista::edt_busca_change));
    ui.register_handler("frmVenExpedicoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenExpedicoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenExpedicoesConsulta", FrmVenExpedicoesConsulta::bind);
    ui.register_handler("frmVenExpedicoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenExpedicoesConsulta::form_show));
    ui.register_handler("frmVenExpedicoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenExpedicoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenExpedicoesHist", FrmVenExpedicoesHist::bind);
    ui.register_handler("frmVenExpedicoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenExpedicoesHist::form_show));
    ui.register_handler("frmVenExpedicoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenExpedicoesHist::edt_busca_change));
    ui.register_form_class("frmVenRomaneiosCad", FrmVenRomaneiosCad::bind);
    ui.register_handler("frmVenRomaneiosCad.form_show", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::form_show));
    ui.register_handler("frmVenRomaneiosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_novo_click));
    ui.register_handler("frmVenRomaneiosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_salvar_click));
    ui.register_handler("frmVenRomaneiosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_excluir_click));
    ui.register_handler("frmVenRomaneiosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_cancelar_click));
    ui.register_handler("frmVenRomaneiosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_novo_item_click));
    ui.register_handler("frmVenRomaneiosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::btn_fechar_click));
    ui.register_handler("frmVenRomaneiosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRomaneiosCad::edt_busca_change));
    ui.register_form_class("frmVenRomaneiosLista", FrmVenRomaneiosLista::bind);
    ui.register_handler("frmVenRomaneiosLista.form_show", aylis::prelude::instances::form_handler(FrmVenRomaneiosLista::form_show));
    ui.register_handler("frmVenRomaneiosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosLista::btn_abrir_click));
    ui.register_handler("frmVenRomaneiosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRomaneiosLista::edt_busca_change));
    ui.register_handler("frmVenRomaneiosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenRomaneiosLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenRomaneiosConsulta", FrmVenRomaneiosConsulta::bind);
    ui.register_handler("frmVenRomaneiosConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenRomaneiosConsulta::form_show));
    ui.register_handler("frmVenRomaneiosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRomaneiosConsulta::edt_busca_change));
    ui.register_form_class("frmVenRomaneiosHist", FrmVenRomaneiosHist::bind);
    ui.register_handler("frmVenRomaneiosHist.form_show", aylis::prelude::instances::form_handler(FrmVenRomaneiosHist::form_show));
    ui.register_handler("frmVenRomaneiosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenRomaneiosHist::edt_busca_change));
    ui.register_form_class("frmVenEntregasCad", FrmVenEntregasCad::bind);
    ui.register_handler("frmVenEntregasCad.form_show", aylis::prelude::instances::form_handler(FrmVenEntregasCad::form_show));
    ui.register_handler("frmVenEntregasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_novo_click));
    ui.register_handler("frmVenEntregasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_salvar_click));
    ui.register_handler("frmVenEntregasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_excluir_click));
    ui.register_handler("frmVenEntregasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_cancelar_click));
    ui.register_handler("frmVenEntregasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_novo_item_click));
    ui.register_handler("frmVenEntregasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenEntregasCad::btn_fechar_click));
    ui.register_handler("frmVenEntregasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenEntregasCad::edt_busca_change));
    ui.register_form_class("frmVenEntregasLista", FrmVenEntregasLista::bind);
    ui.register_handler("frmVenEntregasLista.form_show", aylis::prelude::instances::form_handler(FrmVenEntregasLista::form_show));
    ui.register_handler("frmVenEntregasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenEntregasLista::btn_abrir_click));
    ui.register_handler("frmVenEntregasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenEntregasLista::edt_busca_change));
    ui.register_handler("frmVenEntregasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenEntregasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenEntregasConsulta", FrmVenEntregasConsulta::bind);
    ui.register_handler("frmVenEntregasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenEntregasConsulta::form_show));
    ui.register_handler("frmVenEntregasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenEntregasConsulta::edt_busca_change));
    ui.register_form_class("frmVenEntregasHist", FrmVenEntregasHist::bind);
    ui.register_handler("frmVenEntregasHist.form_show", aylis::prelude::instances::form_handler(FrmVenEntregasHist::form_show));
    ui.register_handler("frmVenEntregasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenEntregasHist::edt_busca_change));
    ui.register_form_class("frmVenOportunidadesCad", FrmVenOportunidadesCad::bind);
    ui.register_handler("frmVenOportunidadesCad.form_show", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::form_show));
    ui.register_handler("frmVenOportunidadesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_novo_click));
    ui.register_handler("frmVenOportunidadesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_salvar_click));
    ui.register_handler("frmVenOportunidadesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_excluir_click));
    ui.register_handler("frmVenOportunidadesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_cancelar_click));
    ui.register_handler("frmVenOportunidadesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_novo_item_click));
    ui.register_handler("frmVenOportunidadesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::btn_fechar_click));
    ui.register_handler("frmVenOportunidadesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOportunidadesCad::edt_busca_change));
    ui.register_form_class("frmVenOportunidadesLista", FrmVenOportunidadesLista::bind);
    ui.register_handler("frmVenOportunidadesLista.form_show", aylis::prelude::instances::form_handler(FrmVenOportunidadesLista::form_show));
    ui.register_handler("frmVenOportunidadesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesLista::btn_abrir_click));
    ui.register_handler("frmVenOportunidadesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOportunidadesLista::edt_busca_change));
    ui.register_handler("frmVenOportunidadesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenOportunidadesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenOportunidadesConsulta", FrmVenOportunidadesConsulta::bind);
    ui.register_handler("frmVenOportunidadesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenOportunidadesConsulta::form_show));
    ui.register_handler("frmVenOportunidadesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOportunidadesConsulta::edt_busca_change));
    ui.register_form_class("frmVenOportunidadesHist", FrmVenOportunidadesHist::bind);
    ui.register_handler("frmVenOportunidadesHist.form_show", aylis::prelude::instances::form_handler(FrmVenOportunidadesHist::form_show));
    ui.register_handler("frmVenOportunidadesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenOportunidadesHist::edt_busca_change));
    ui.register_form_class("frmVenPropostasComerciaisCad", FrmVenPropostasComerciaisCad::bind);
    ui.register_handler("frmVenPropostasComerciaisCad.form_show", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::form_show));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_novo_click));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_salvar_click));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_excluir_click));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_cancelar_click));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_novo_item_click));
    ui.register_handler("frmVenPropostasComerciaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::btn_fechar_click));
    ui.register_handler("frmVenPropostasComerciaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisCad::edt_busca_change));
    ui.register_form_class("frmVenPropostasComerciaisLista", FrmVenPropostasComerciaisLista::bind);
    ui.register_handler("frmVenPropostasComerciaisLista.form_show", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisLista::form_show));
    ui.register_handler("frmVenPropostasComerciaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisLista::btn_abrir_click));
    ui.register_handler("frmVenPropostasComerciaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisLista::edt_busca_change));
    ui.register_handler("frmVenPropostasComerciaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPropostasComerciaisConsulta", FrmVenPropostasComerciaisConsulta::bind);
    ui.register_handler("frmVenPropostasComerciaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisConsulta::form_show));
    ui.register_handler("frmVenPropostasComerciaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisConsulta::edt_busca_change));
    ui.register_form_class("frmVenPropostasComerciaisHist", FrmVenPropostasComerciaisHist::bind);
    ui.register_handler("frmVenPropostasComerciaisHist.form_show", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisHist::form_show));
    ui.register_handler("frmVenPropostasComerciaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPropostasComerciaisHist::edt_busca_change));
    ui.register_form_class("frmVenVisitasCad", FrmVenVisitasCad::bind);
    ui.register_handler("frmVenVisitasCad.form_show", aylis::prelude::instances::form_handler(FrmVenVisitasCad::form_show));
    ui.register_handler("frmVenVisitasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_novo_click));
    ui.register_handler("frmVenVisitasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_salvar_click));
    ui.register_handler("frmVenVisitasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_excluir_click));
    ui.register_handler("frmVenVisitasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_cancelar_click));
    ui.register_handler("frmVenVisitasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_novo_item_click));
    ui.register_handler("frmVenVisitasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenVisitasCad::btn_fechar_click));
    ui.register_handler("frmVenVisitasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVisitasCad::edt_busca_change));
    ui.register_form_class("frmVenVisitasLista", FrmVenVisitasLista::bind);
    ui.register_handler("frmVenVisitasLista.form_show", aylis::prelude::instances::form_handler(FrmVenVisitasLista::form_show));
    ui.register_handler("frmVenVisitasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenVisitasLista::btn_abrir_click));
    ui.register_handler("frmVenVisitasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVisitasLista::edt_busca_change));
    ui.register_handler("frmVenVisitasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenVisitasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenVisitasConsulta", FrmVenVisitasConsulta::bind);
    ui.register_handler("frmVenVisitasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenVisitasConsulta::form_show));
    ui.register_handler("frmVenVisitasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVisitasConsulta::edt_busca_change));
    ui.register_form_class("frmVenVisitasHist", FrmVenVisitasHist::bind);
    ui.register_handler("frmVenVisitasHist.form_show", aylis::prelude::instances::form_handler(FrmVenVisitasHist::form_show));
    ui.register_handler("frmVenVisitasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenVisitasHist::edt_busca_change));
    ui.register_form_class("frmVenPosVendasCad", FrmVenPosVendasCad::bind);
    ui.register_handler("frmVenPosVendasCad.form_show", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::form_show));
    ui.register_handler("frmVenPosVendasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_novo_click));
    ui.register_handler("frmVenPosVendasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_salvar_click));
    ui.register_handler("frmVenPosVendasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_excluir_click));
    ui.register_handler("frmVenPosVendasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_cancelar_click));
    ui.register_handler("frmVenPosVendasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_novo_item_click));
    ui.register_handler("frmVenPosVendasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::btn_fechar_click));
    ui.register_handler("frmVenPosVendasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPosVendasCad::edt_busca_change));
    ui.register_form_class("frmVenPosVendasLista", FrmVenPosVendasLista::bind);
    ui.register_handler("frmVenPosVendasLista.form_show", aylis::prelude::instances::form_handler(FrmVenPosVendasLista::form_show));
    ui.register_handler("frmVenPosVendasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPosVendasLista::btn_abrir_click));
    ui.register_handler("frmVenPosVendasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPosVendasLista::edt_busca_change));
    ui.register_handler("frmVenPosVendasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPosVendasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPosVendasConsulta", FrmVenPosVendasConsulta::bind);
    ui.register_handler("frmVenPosVendasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPosVendasConsulta::form_show));
    ui.register_handler("frmVenPosVendasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPosVendasConsulta::edt_busca_change));
    ui.register_form_class("frmVenPosVendasHist", FrmVenPosVendasHist::bind);
    ui.register_handler("frmVenPosVendasHist.form_show", aylis::prelude::instances::form_handler(FrmVenPosVendasHist::form_show));
    ui.register_handler("frmVenPosVendasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPosVendasHist::edt_busca_change));
    ui.register_form_class("frmVenReclamacoesCad", FrmVenReclamacoesCad::bind);
    ui.register_handler("frmVenReclamacoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::form_show));
    ui.register_handler("frmVenReclamacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_novo_click));
    ui.register_handler("frmVenReclamacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_salvar_click));
    ui.register_handler("frmVenReclamacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_excluir_click));
    ui.register_handler("frmVenReclamacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_cancelar_click));
    ui.register_handler("frmVenReclamacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_novo_item_click));
    ui.register_handler("frmVenReclamacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::btn_fechar_click));
    ui.register_handler("frmVenReclamacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenReclamacoesCad::edt_busca_change));
    ui.register_form_class("frmVenReclamacoesLista", FrmVenReclamacoesLista::bind);
    ui.register_handler("frmVenReclamacoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenReclamacoesLista::form_show));
    ui.register_handler("frmVenReclamacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesLista::btn_abrir_click));
    ui.register_handler("frmVenReclamacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenReclamacoesLista::edt_busca_change));
    ui.register_handler("frmVenReclamacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenReclamacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenReclamacoesConsulta", FrmVenReclamacoesConsulta::bind);
    ui.register_handler("frmVenReclamacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenReclamacoesConsulta::form_show));
    ui.register_handler("frmVenReclamacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenReclamacoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenReclamacoesHist", FrmVenReclamacoesHist::bind);
    ui.register_handler("frmVenReclamacoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenReclamacoesHist::form_show));
    ui.register_handler("frmVenReclamacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenReclamacoesHist::edt_busca_change));
    ui.register_form_class("frmVenTrocasCad", FrmVenTrocasCad::bind);
    ui.register_handler("frmVenTrocasCad.form_show", aylis::prelude::instances::form_handler(FrmVenTrocasCad::form_show));
    ui.register_handler("frmVenTrocasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_novo_click));
    ui.register_handler("frmVenTrocasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_salvar_click));
    ui.register_handler("frmVenTrocasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_excluir_click));
    ui.register_handler("frmVenTrocasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_cancelar_click));
    ui.register_handler("frmVenTrocasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_novo_item_click));
    ui.register_handler("frmVenTrocasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenTrocasCad::btn_fechar_click));
    ui.register_handler("frmVenTrocasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTrocasCad::edt_busca_change));
    ui.register_form_class("frmVenTrocasLista", FrmVenTrocasLista::bind);
    ui.register_handler("frmVenTrocasLista.form_show", aylis::prelude::instances::form_handler(FrmVenTrocasLista::form_show));
    ui.register_handler("frmVenTrocasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenTrocasLista::btn_abrir_click));
    ui.register_handler("frmVenTrocasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTrocasLista::edt_busca_change));
    ui.register_handler("frmVenTrocasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenTrocasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenTrocasConsulta", FrmVenTrocasConsulta::bind);
    ui.register_handler("frmVenTrocasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenTrocasConsulta::form_show));
    ui.register_handler("frmVenTrocasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTrocasConsulta::edt_busca_change));
    ui.register_form_class("frmVenTrocasHist", FrmVenTrocasHist::bind);
    ui.register_handler("frmVenTrocasHist.form_show", aylis::prelude::instances::form_handler(FrmVenTrocasHist::form_show));
    ui.register_handler("frmVenTrocasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenTrocasHist::edt_busca_change));
    ui.register_form_class("frmVenBonificacoesCad", FrmVenBonificacoesCad::bind);
    ui.register_handler("frmVenBonificacoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::form_show));
    ui.register_handler("frmVenBonificacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_novo_click));
    ui.register_handler("frmVenBonificacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_salvar_click));
    ui.register_handler("frmVenBonificacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_excluir_click));
    ui.register_handler("frmVenBonificacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_cancelar_click));
    ui.register_handler("frmVenBonificacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_novo_item_click));
    ui.register_handler("frmVenBonificacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::btn_fechar_click));
    ui.register_handler("frmVenBonificacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenBonificacoesCad::edt_busca_change));
    ui.register_form_class("frmVenBonificacoesLista", FrmVenBonificacoesLista::bind);
    ui.register_handler("frmVenBonificacoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenBonificacoesLista::form_show));
    ui.register_handler("frmVenBonificacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesLista::btn_abrir_click));
    ui.register_handler("frmVenBonificacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenBonificacoesLista::edt_busca_change));
    ui.register_handler("frmVenBonificacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenBonificacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenBonificacoesConsulta", FrmVenBonificacoesConsulta::bind);
    ui.register_handler("frmVenBonificacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenBonificacoesConsulta::form_show));
    ui.register_handler("frmVenBonificacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenBonificacoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenBonificacoesHist", FrmVenBonificacoesHist::bind);
    ui.register_handler("frmVenBonificacoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenBonificacoesHist::form_show));
    ui.register_handler("frmVenBonificacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenBonificacoesHist::edt_busca_change));
    ui.register_form_class("frmVenConsignacoesCad", FrmVenConsignacoesCad::bind);
    ui.register_handler("frmVenConsignacoesCad.form_show", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::form_show));
    ui.register_handler("frmVenConsignacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_novo_click));
    ui.register_handler("frmVenConsignacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_salvar_click));
    ui.register_handler("frmVenConsignacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_excluir_click));
    ui.register_handler("frmVenConsignacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_cancelar_click));
    ui.register_handler("frmVenConsignacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_novo_item_click));
    ui.register_handler("frmVenConsignacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::btn_fechar_click));
    ui.register_handler("frmVenConsignacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConsignacoesCad::edt_busca_change));
    ui.register_form_class("frmVenConsignacoesLista", FrmVenConsignacoesLista::bind);
    ui.register_handler("frmVenConsignacoesLista.form_show", aylis::prelude::instances::form_handler(FrmVenConsignacoesLista::form_show));
    ui.register_handler("frmVenConsignacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesLista::btn_abrir_click));
    ui.register_handler("frmVenConsignacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConsignacoesLista::edt_busca_change));
    ui.register_handler("frmVenConsignacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenConsignacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenConsignacoesConsulta", FrmVenConsignacoesConsulta::bind);
    ui.register_handler("frmVenConsignacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenConsignacoesConsulta::form_show));
    ui.register_handler("frmVenConsignacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConsignacoesConsulta::edt_busca_change));
    ui.register_form_class("frmVenConsignacoesHist", FrmVenConsignacoesHist::bind);
    ui.register_handler("frmVenConsignacoesHist.form_show", aylis::prelude::instances::form_handler(FrmVenConsignacoesHist::form_show));
    ui.register_handler("frmVenConsignacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenConsignacoesHist::edt_busca_change));
    ui.register_form_class("frmVenContratosDeVendaCad", FrmVenContratosDeVendaCad::bind);
    ui.register_handler("frmVenContratosDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::form_show));
    ui.register_handler("frmVenContratosDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenContratosDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenContratosDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenContratosDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenContratosDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenContratosDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenContratosDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenContratosDeVendaLista", FrmVenContratosDeVendaLista::bind);
    ui.register_handler("frmVenContratosDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaLista::form_show));
    ui.register_handler("frmVenContratosDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenContratosDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenContratosDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenContratosDeVendaConsulta", FrmVenContratosDeVendaConsulta::bind);
    ui.register_handler("frmVenContratosDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaConsulta::form_show));
    ui.register_handler("frmVenContratosDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenContratosDeVendaHist", FrmVenContratosDeVendaHist::bind);
    ui.register_handler("frmVenContratosDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaHist::form_show));
    ui.register_handler("frmVenContratosDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenContratosDeVendaHist::edt_busca_change));
    ui.register_form_class("frmVenAssinaturasCad", FrmVenAssinaturasCad::bind);
    ui.register_handler("frmVenAssinaturasCad.form_show", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::form_show));
    ui.register_handler("frmVenAssinaturasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_novo_click));
    ui.register_handler("frmVenAssinaturasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_salvar_click));
    ui.register_handler("frmVenAssinaturasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_excluir_click));
    ui.register_handler("frmVenAssinaturasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_cancelar_click));
    ui.register_handler("frmVenAssinaturasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_novo_item_click));
    ui.register_handler("frmVenAssinaturasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::btn_fechar_click));
    ui.register_handler("frmVenAssinaturasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenAssinaturasCad::edt_busca_change));
    ui.register_form_class("frmVenAssinaturasLista", FrmVenAssinaturasLista::bind);
    ui.register_handler("frmVenAssinaturasLista.form_show", aylis::prelude::instances::form_handler(FrmVenAssinaturasLista::form_show));
    ui.register_handler("frmVenAssinaturasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasLista::btn_abrir_click));
    ui.register_handler("frmVenAssinaturasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenAssinaturasLista::edt_busca_change));
    ui.register_handler("frmVenAssinaturasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenAssinaturasLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenAssinaturasConsulta", FrmVenAssinaturasConsulta::bind);
    ui.register_handler("frmVenAssinaturasConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenAssinaturasConsulta::form_show));
    ui.register_handler("frmVenAssinaturasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenAssinaturasConsulta::edt_busca_change));
    ui.register_form_class("frmVenAssinaturasHist", FrmVenAssinaturasHist::bind);
    ui.register_handler("frmVenAssinaturasHist.form_show", aylis::prelude::instances::form_handler(FrmVenAssinaturasHist::form_show));
    ui.register_handler("frmVenAssinaturasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenAssinaturasHist::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeECommerceCad", FrmVenPedidosDeECommerceCad::bind);
    ui.register_handler("frmVenPedidosDeECommerceCad.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::form_show));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_novo_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_salvar_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_excluir_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_cancelar_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_novo_item_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::btn_fechar_click));
    ui.register_handler("frmVenPedidosDeECommerceCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceCad::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeECommerceLista", FrmVenPedidosDeECommerceLista::bind);
    ui.register_handler("frmVenPedidosDeECommerceLista.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceLista::form_show));
    ui.register_handler("frmVenPedidosDeECommerceLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceLista::btn_abrir_click));
    ui.register_handler("frmVenPedidosDeECommerceLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceLista::edt_busca_change));
    ui.register_handler("frmVenPedidosDeECommerceLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPedidosDeECommerceConsulta", FrmVenPedidosDeECommerceConsulta::bind);
    ui.register_handler("frmVenPedidosDeECommerceConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceConsulta::form_show));
    ui.register_handler("frmVenPedidosDeECommerceConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceConsulta::edt_busca_change));
    ui.register_form_class("frmVenPedidosDeECommerceHist", FrmVenPedidosDeECommerceHist::bind);
    ui.register_handler("frmVenPedidosDeECommerceHist.form_show", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceHist::form_show));
    ui.register_handler("frmVenPedidosDeECommerceHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPedidosDeECommerceHist::edt_busca_change));
    ui.register_form_class("frmVenApuracoesDeComissaoCad", FrmVenApuracoesDeComissaoCad::bind);
    ui.register_handler("frmVenApuracoesDeComissaoCad.form_show", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::form_show));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_novo_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_salvar_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_excluir_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_cancelar_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_novo_item_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::btn_fechar_click));
    ui.register_handler("frmVenApuracoesDeComissaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoCad::edt_busca_change));
    ui.register_form_class("frmVenApuracoesDeComissaoLista", FrmVenApuracoesDeComissaoLista::bind);
    ui.register_handler("frmVenApuracoesDeComissaoLista.form_show", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoLista::form_show));
    ui.register_handler("frmVenApuracoesDeComissaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoLista::btn_abrir_click));
    ui.register_handler("frmVenApuracoesDeComissaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoLista::edt_busca_change));
    ui.register_handler("frmVenApuracoesDeComissaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenApuracoesDeComissaoConsulta", FrmVenApuracoesDeComissaoConsulta::bind);
    ui.register_handler("frmVenApuracoesDeComissaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoConsulta::form_show));
    ui.register_handler("frmVenApuracoesDeComissaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoConsulta::edt_busca_change));
    ui.register_form_class("frmVenApuracoesDeComissaoHist", FrmVenApuracoesDeComissaoHist::bind);
    ui.register_handler("frmVenApuracoesDeComissaoHist.form_show", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoHist::form_show));
    ui.register_handler("frmVenApuracoesDeComissaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenApuracoesDeComissaoHist::edt_busca_change));
    ui.register_form_class("frmVenPrevisoesDeVendaCad", FrmVenPrevisoesDeVendaCad::bind);
    ui.register_handler("frmVenPrevisoesDeVendaCad.form_show", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::form_show));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_novo_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_salvar_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_excluir_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_cancelar_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_novo_item_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::btn_fechar_click));
    ui.register_handler("frmVenPrevisoesDeVendaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaCad::edt_busca_change));
    ui.register_form_class("frmVenPrevisoesDeVendaLista", FrmVenPrevisoesDeVendaLista::bind);
    ui.register_handler("frmVenPrevisoesDeVendaLista.form_show", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaLista::form_show));
    ui.register_handler("frmVenPrevisoesDeVendaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaLista::btn_abrir_click));
    ui.register_handler("frmVenPrevisoesDeVendaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaLista::edt_busca_change));
    ui.register_handler("frmVenPrevisoesDeVendaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaLista::grd_lista_dbl_click));
    ui.register_form_class("frmVenPrevisoesDeVendaConsulta", FrmVenPrevisoesDeVendaConsulta::bind);
    ui.register_handler("frmVenPrevisoesDeVendaConsulta.form_show", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaConsulta::form_show));
    ui.register_handler("frmVenPrevisoesDeVendaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaConsulta::edt_busca_change));
    ui.register_form_class("frmVenPrevisoesDeVendaHist", FrmVenPrevisoesDeVendaHist::bind);
    ui.register_handler("frmVenPrevisoesDeVendaHist.form_show", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaHist::form_show));
    ui.register_handler("frmVenPrevisoesDeVendaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmVenPrevisoesDeVendaHist::edt_busca_change));
    ui.register_form_class("frmFisCfopsCad", FrmFisCfopsCad::bind);
    ui.register_handler("frmFisCfopsCad.form_show", aylis::prelude::instances::form_handler(FrmFisCfopsCad::form_show));
    ui.register_handler("frmFisCfopsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_novo_click));
    ui.register_handler("frmFisCfopsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_salvar_click));
    ui.register_handler("frmFisCfopsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_excluir_click));
    ui.register_handler("frmFisCfopsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_cancelar_click));
    ui.register_handler("frmFisCfopsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_novo_item_click));
    ui.register_handler("frmFisCfopsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCfopsCad::btn_fechar_click));
    ui.register_handler("frmFisCfopsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCfopsCad::edt_busca_change));
    ui.register_form_class("frmFisCfopsLista", FrmFisCfopsLista::bind);
    ui.register_handler("frmFisCfopsLista.form_show", aylis::prelude::instances::form_handler(FrmFisCfopsLista::form_show));
    ui.register_handler("frmFisCfopsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCfopsLista::btn_abrir_click));
    ui.register_handler("frmFisCfopsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCfopsLista::edt_busca_change));
    ui.register_handler("frmFisCfopsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCfopsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCfopsConsulta", FrmFisCfopsConsulta::bind);
    ui.register_handler("frmFisCfopsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCfopsConsulta::form_show));
    ui.register_handler("frmFisCfopsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCfopsConsulta::edt_busca_change));
    ui.register_form_class("frmFisCfopsHist", FrmFisCfopsHist::bind);
    ui.register_handler("frmFisCfopsHist.form_show", aylis::prelude::instances::form_handler(FrmFisCfopsHist::form_show));
    ui.register_handler("frmFisCfopsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCfopsHist::edt_busca_change));
    ui.register_form_class("frmFisNcmsCad", FrmFisNcmsCad::bind);
    ui.register_handler("frmFisNcmsCad.form_show", aylis::prelude::instances::form_handler(FrmFisNcmsCad::form_show));
    ui.register_handler("frmFisNcmsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_novo_click));
    ui.register_handler("frmFisNcmsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_salvar_click));
    ui.register_handler("frmFisNcmsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_excluir_click));
    ui.register_handler("frmFisNcmsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_cancelar_click));
    ui.register_handler("frmFisNcmsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_novo_item_click));
    ui.register_handler("frmFisNcmsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisNcmsCad::btn_fechar_click));
    ui.register_handler("frmFisNcmsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNcmsCad::edt_busca_change));
    ui.register_form_class("frmFisNcmsLista", FrmFisNcmsLista::bind);
    ui.register_handler("frmFisNcmsLista.form_show", aylis::prelude::instances::form_handler(FrmFisNcmsLista::form_show));
    ui.register_handler("frmFisNcmsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisNcmsLista::btn_abrir_click));
    ui.register_handler("frmFisNcmsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNcmsLista::edt_busca_change));
    ui.register_handler("frmFisNcmsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisNcmsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisNcmsConsulta", FrmFisNcmsConsulta::bind);
    ui.register_handler("frmFisNcmsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisNcmsConsulta::form_show));
    ui.register_handler("frmFisNcmsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNcmsConsulta::edt_busca_change));
    ui.register_form_class("frmFisNcmsHist", FrmFisNcmsHist::bind);
    ui.register_handler("frmFisNcmsHist.form_show", aylis::prelude::instances::form_handler(FrmFisNcmsHist::form_show));
    ui.register_handler("frmFisNcmsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNcmsHist::edt_busca_change));
    ui.register_form_class("frmFisCestsCad", FrmFisCestsCad::bind);
    ui.register_handler("frmFisCestsCad.form_show", aylis::prelude::instances::form_handler(FrmFisCestsCad::form_show));
    ui.register_handler("frmFisCestsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_novo_click));
    ui.register_handler("frmFisCestsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_salvar_click));
    ui.register_handler("frmFisCestsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_excluir_click));
    ui.register_handler("frmFisCestsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_cancelar_click));
    ui.register_handler("frmFisCestsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_novo_item_click));
    ui.register_handler("frmFisCestsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCestsCad::btn_fechar_click));
    ui.register_handler("frmFisCestsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCestsCad::edt_busca_change));
    ui.register_form_class("frmFisCestsLista", FrmFisCestsLista::bind);
    ui.register_handler("frmFisCestsLista.form_show", aylis::prelude::instances::form_handler(FrmFisCestsLista::form_show));
    ui.register_handler("frmFisCestsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCestsLista::btn_abrir_click));
    ui.register_handler("frmFisCestsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCestsLista::edt_busca_change));
    ui.register_handler("frmFisCestsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCestsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCestsConsulta", FrmFisCestsConsulta::bind);
    ui.register_handler("frmFisCestsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCestsConsulta::form_show));
    ui.register_handler("frmFisCestsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCestsConsulta::edt_busca_change));
    ui.register_form_class("frmFisCestsHist", FrmFisCestsHist::bind);
    ui.register_handler("frmFisCestsHist.form_show", aylis::prelude::instances::form_handler(FrmFisCestsHist::form_show));
    ui.register_handler("frmFisCestsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCestsHist::edt_busca_change));
    ui.register_form_class("frmFisNaturezasDeOperacaoCad", FrmFisNaturezasDeOperacaoCad::bind);
    ui.register_handler("frmFisNaturezasDeOperacaoCad.form_show", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::form_show));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_novo_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_salvar_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_excluir_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_cancelar_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_novo_item_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::btn_fechar_click));
    ui.register_handler("frmFisNaturezasDeOperacaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoCad::edt_busca_change));
    ui.register_form_class("frmFisNaturezasDeOperacaoLista", FrmFisNaturezasDeOperacaoLista::bind);
    ui.register_handler("frmFisNaturezasDeOperacaoLista.form_show", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoLista::form_show));
    ui.register_handler("frmFisNaturezasDeOperacaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoLista::btn_abrir_click));
    ui.register_handler("frmFisNaturezasDeOperacaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoLista::edt_busca_change));
    ui.register_handler("frmFisNaturezasDeOperacaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisNaturezasDeOperacaoConsulta", FrmFisNaturezasDeOperacaoConsulta::bind);
    ui.register_handler("frmFisNaturezasDeOperacaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoConsulta::form_show));
    ui.register_handler("frmFisNaturezasDeOperacaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoConsulta::edt_busca_change));
    ui.register_form_class("frmFisNaturezasDeOperacaoHist", FrmFisNaturezasDeOperacaoHist::bind);
    ui.register_handler("frmFisNaturezasDeOperacaoHist.form_show", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoHist::form_show));
    ui.register_handler("frmFisNaturezasDeOperacaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNaturezasDeOperacaoHist::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIcmsCad", FrmFisRegrasDeIcmsCad::bind);
    ui.register_handler("frmFisRegrasDeIcmsCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::form_show));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_novo_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_salvar_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_excluir_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_cancelar_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_novo_item_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::btn_fechar_click));
    ui.register_handler("frmFisRegrasDeIcmsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsCad::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIcmsLista", FrmFisRegrasDeIcmsLista::bind);
    ui.register_handler("frmFisRegrasDeIcmsLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsLista::form_show));
    ui.register_handler("frmFisRegrasDeIcmsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsLista::btn_abrir_click));
    ui.register_handler("frmFisRegrasDeIcmsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsLista::edt_busca_change));
    ui.register_handler("frmFisRegrasDeIcmsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegrasDeIcmsConsulta", FrmFisRegrasDeIcmsConsulta::bind);
    ui.register_handler("frmFisRegrasDeIcmsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsConsulta::form_show));
    ui.register_handler("frmFisRegrasDeIcmsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIcmsHist", FrmFisRegrasDeIcmsHist::bind);
    ui.register_handler("frmFisRegrasDeIcmsHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsHist::form_show));
    ui.register_handler("frmFisRegrasDeIcmsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIcmsHist::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIpiCad", FrmFisRegrasDeIpiCad::bind);
    ui.register_handler("frmFisRegrasDeIpiCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::form_show));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_novo_click));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_salvar_click));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_excluir_click));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_cancelar_click));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_novo_item_click));
    ui.register_handler("frmFisRegrasDeIpiCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::btn_fechar_click));
    ui.register_handler("frmFisRegrasDeIpiCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiCad::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIpiLista", FrmFisRegrasDeIpiLista::bind);
    ui.register_handler("frmFisRegrasDeIpiLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiLista::form_show));
    ui.register_handler("frmFisRegrasDeIpiLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiLista::btn_abrir_click));
    ui.register_handler("frmFisRegrasDeIpiLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiLista::edt_busca_change));
    ui.register_handler("frmFisRegrasDeIpiLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegrasDeIpiConsulta", FrmFisRegrasDeIpiConsulta::bind);
    ui.register_handler("frmFisRegrasDeIpiConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiConsulta::form_show));
    ui.register_handler("frmFisRegrasDeIpiConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIpiHist", FrmFisRegrasDeIpiHist::bind);
    ui.register_handler("frmFisRegrasDeIpiHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiHist::form_show));
    ui.register_handler("frmFisRegrasDeIpiHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIpiHist::edt_busca_change));
    ui.register_form_class("frmFisRegrasDePisCad", FrmFisRegrasDePisCad::bind);
    ui.register_handler("frmFisRegrasDePisCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::form_show));
    ui.register_handler("frmFisRegrasDePisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_novo_click));
    ui.register_handler("frmFisRegrasDePisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_salvar_click));
    ui.register_handler("frmFisRegrasDePisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_excluir_click));
    ui.register_handler("frmFisRegrasDePisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_cancelar_click));
    ui.register_handler("frmFisRegrasDePisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_novo_item_click));
    ui.register_handler("frmFisRegrasDePisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::btn_fechar_click));
    ui.register_handler("frmFisRegrasDePisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDePisCad::edt_busca_change));
    ui.register_form_class("frmFisRegrasDePisLista", FrmFisRegrasDePisLista::bind);
    ui.register_handler("frmFisRegrasDePisLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDePisLista::form_show));
    ui.register_handler("frmFisRegrasDePisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisLista::btn_abrir_click));
    ui.register_handler("frmFisRegrasDePisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDePisLista::edt_busca_change));
    ui.register_handler("frmFisRegrasDePisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegrasDePisLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegrasDePisConsulta", FrmFisRegrasDePisConsulta::bind);
    ui.register_handler("frmFisRegrasDePisConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDePisConsulta::form_show));
    ui.register_handler("frmFisRegrasDePisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDePisConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegrasDePisHist", FrmFisRegrasDePisHist::bind);
    ui.register_handler("frmFisRegrasDePisHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDePisHist::form_show));
    ui.register_handler("frmFisRegrasDePisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDePisHist::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeCofinsCad", FrmFisRegrasDeCofinsCad::bind);
    ui.register_handler("frmFisRegrasDeCofinsCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::form_show));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_novo_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_salvar_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_excluir_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_cancelar_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_novo_item_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::btn_fechar_click));
    ui.register_handler("frmFisRegrasDeCofinsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsCad::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeCofinsLista", FrmFisRegrasDeCofinsLista::bind);
    ui.register_handler("frmFisRegrasDeCofinsLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsLista::form_show));
    ui.register_handler("frmFisRegrasDeCofinsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsLista::btn_abrir_click));
    ui.register_handler("frmFisRegrasDeCofinsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsLista::edt_busca_change));
    ui.register_handler("frmFisRegrasDeCofinsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegrasDeCofinsConsulta", FrmFisRegrasDeCofinsConsulta::bind);
    ui.register_handler("frmFisRegrasDeCofinsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsConsulta::form_show));
    ui.register_handler("frmFisRegrasDeCofinsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeCofinsHist", FrmFisRegrasDeCofinsHist::bind);
    ui.register_handler("frmFisRegrasDeCofinsHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsHist::form_show));
    ui.register_handler("frmFisRegrasDeCofinsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeCofinsHist::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIssCad", FrmFisRegrasDeIssCad::bind);
    ui.register_handler("frmFisRegrasDeIssCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::form_show));
    ui.register_handler("frmFisRegrasDeIssCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_novo_click));
    ui.register_handler("frmFisRegrasDeIssCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_salvar_click));
    ui.register_handler("frmFisRegrasDeIssCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_excluir_click));
    ui.register_handler("frmFisRegrasDeIssCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_cancelar_click));
    ui.register_handler("frmFisRegrasDeIssCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_novo_item_click));
    ui.register_handler("frmFisRegrasDeIssCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::btn_fechar_click));
    ui.register_handler("frmFisRegrasDeIssCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssCad::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIssLista", FrmFisRegrasDeIssLista::bind);
    ui.register_handler("frmFisRegrasDeIssLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssLista::form_show));
    ui.register_handler("frmFisRegrasDeIssLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssLista::btn_abrir_click));
    ui.register_handler("frmFisRegrasDeIssLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssLista::edt_busca_change));
    ui.register_handler("frmFisRegrasDeIssLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegrasDeIssConsulta", FrmFisRegrasDeIssConsulta::bind);
    ui.register_handler("frmFisRegrasDeIssConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssConsulta::form_show));
    ui.register_handler("frmFisRegrasDeIssConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegrasDeIssHist", FrmFisRegrasDeIssHist::bind);
    ui.register_handler("frmFisRegrasDeIssHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssHist::form_show));
    ui.register_handler("frmFisRegrasDeIssHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegrasDeIssHist::edt_busca_change));
    ui.register_form_class("frmFisAliquotasInterestaduaisCad", FrmFisAliquotasInterestaduaisCad::bind);
    ui.register_handler("frmFisAliquotasInterestaduaisCad.form_show", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::form_show));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_novo_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_salvar_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_excluir_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_cancelar_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_novo_item_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::btn_fechar_click));
    ui.register_handler("frmFisAliquotasInterestaduaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisCad::edt_busca_change));
    ui.register_form_class("frmFisAliquotasInterestaduaisLista", FrmFisAliquotasInterestaduaisLista::bind);
    ui.register_handler("frmFisAliquotasInterestaduaisLista.form_show", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisLista::form_show));
    ui.register_handler("frmFisAliquotasInterestaduaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisLista::btn_abrir_click));
    ui.register_handler("frmFisAliquotasInterestaduaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisLista::edt_busca_change));
    ui.register_handler("frmFisAliquotasInterestaduaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisAliquotasInterestaduaisConsulta", FrmFisAliquotasInterestaduaisConsulta::bind);
    ui.register_handler("frmFisAliquotasInterestaduaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisConsulta::form_show));
    ui.register_handler("frmFisAliquotasInterestaduaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisConsulta::edt_busca_change));
    ui.register_form_class("frmFisAliquotasInterestaduaisHist", FrmFisAliquotasInterestaduaisHist::bind);
    ui.register_handler("frmFisAliquotasInterestaduaisHist.form_show", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisHist::form_show));
    ui.register_handler("frmFisAliquotasInterestaduaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAliquotasInterestaduaisHist::edt_busca_change));
    ui.register_form_class("frmFisBeneficiosFiscaisCad", FrmFisBeneficiosFiscaisCad::bind);
    ui.register_handler("frmFisBeneficiosFiscaisCad.form_show", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::form_show));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_novo_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_salvar_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_excluir_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_cancelar_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_novo_item_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::btn_fechar_click));
    ui.register_handler("frmFisBeneficiosFiscaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisCad::edt_busca_change));
    ui.register_form_class("frmFisBeneficiosFiscaisLista", FrmFisBeneficiosFiscaisLista::bind);
    ui.register_handler("frmFisBeneficiosFiscaisLista.form_show", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisLista::form_show));
    ui.register_handler("frmFisBeneficiosFiscaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisLista::btn_abrir_click));
    ui.register_handler("frmFisBeneficiosFiscaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisLista::edt_busca_change));
    ui.register_handler("frmFisBeneficiosFiscaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisBeneficiosFiscaisConsulta", FrmFisBeneficiosFiscaisConsulta::bind);
    ui.register_handler("frmFisBeneficiosFiscaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisConsulta::form_show));
    ui.register_handler("frmFisBeneficiosFiscaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisConsulta::edt_busca_change));
    ui.register_form_class("frmFisBeneficiosFiscaisHist", FrmFisBeneficiosFiscaisHist::bind);
    ui.register_handler("frmFisBeneficiosFiscaisHist.form_show", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisHist::form_show));
    ui.register_handler("frmFisBeneficiosFiscaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisBeneficiosFiscaisHist::edt_busca_change));
    ui.register_form_class("frmFisRegimesTributariosCad", FrmFisRegimesTributariosCad::bind);
    ui.register_handler("frmFisRegimesTributariosCad.form_show", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::form_show));
    ui.register_handler("frmFisRegimesTributariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_novo_click));
    ui.register_handler("frmFisRegimesTributariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_salvar_click));
    ui.register_handler("frmFisRegimesTributariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_excluir_click));
    ui.register_handler("frmFisRegimesTributariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_cancelar_click));
    ui.register_handler("frmFisRegimesTributariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_novo_item_click));
    ui.register_handler("frmFisRegimesTributariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::btn_fechar_click));
    ui.register_handler("frmFisRegimesTributariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosCad::edt_busca_change));
    ui.register_form_class("frmFisRegimesTributariosLista", FrmFisRegimesTributariosLista::bind);
    ui.register_handler("frmFisRegimesTributariosLista.form_show", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosLista::form_show));
    ui.register_handler("frmFisRegimesTributariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosLista::btn_abrir_click));
    ui.register_handler("frmFisRegimesTributariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosLista::edt_busca_change));
    ui.register_handler("frmFisRegimesTributariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRegimesTributariosConsulta", FrmFisRegimesTributariosConsulta::bind);
    ui.register_handler("frmFisRegimesTributariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosConsulta::form_show));
    ui.register_handler("frmFisRegimesTributariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosConsulta::edt_busca_change));
    ui.register_form_class("frmFisRegimesTributariosHist", FrmFisRegimesTributariosHist::bind);
    ui.register_handler("frmFisRegimesTributariosHist.form_show", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosHist::form_show));
    ui.register_handler("frmFisRegimesTributariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRegimesTributariosHist::edt_busca_change));
    ui.register_form_class("frmFisMunicipiosCad", FrmFisMunicipiosCad::bind);
    ui.register_handler("frmFisMunicipiosCad.form_show", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::form_show));
    ui.register_handler("frmFisMunicipiosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_novo_click));
    ui.register_handler("frmFisMunicipiosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_salvar_click));
    ui.register_handler("frmFisMunicipiosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_excluir_click));
    ui.register_handler("frmFisMunicipiosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_cancelar_click));
    ui.register_handler("frmFisMunicipiosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_novo_item_click));
    ui.register_handler("frmFisMunicipiosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::btn_fechar_click));
    ui.register_handler("frmFisMunicipiosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMunicipiosCad::edt_busca_change));
    ui.register_form_class("frmFisMunicipiosLista", FrmFisMunicipiosLista::bind);
    ui.register_handler("frmFisMunicipiosLista.form_show", aylis::prelude::instances::form_handler(FrmFisMunicipiosLista::form_show));
    ui.register_handler("frmFisMunicipiosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosLista::btn_abrir_click));
    ui.register_handler("frmFisMunicipiosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMunicipiosLista::edt_busca_change));
    ui.register_handler("frmFisMunicipiosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisMunicipiosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisMunicipiosConsulta", FrmFisMunicipiosConsulta::bind);
    ui.register_handler("frmFisMunicipiosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisMunicipiosConsulta::form_show));
    ui.register_handler("frmFisMunicipiosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMunicipiosConsulta::edt_busca_change));
    ui.register_form_class("frmFisMunicipiosHist", FrmFisMunicipiosHist::bind);
    ui.register_handler("frmFisMunicipiosHist.form_show", aylis::prelude::instances::form_handler(FrmFisMunicipiosHist::form_show));
    ui.register_handler("frmFisMunicipiosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMunicipiosHist::edt_busca_change));
    ui.register_form_class("frmFisEstadosCad", FrmFisEstadosCad::bind);
    ui.register_handler("frmFisEstadosCad.form_show", aylis::prelude::instances::form_handler(FrmFisEstadosCad::form_show));
    ui.register_handler("frmFisEstadosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_novo_click));
    ui.register_handler("frmFisEstadosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_salvar_click));
    ui.register_handler("frmFisEstadosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_excluir_click));
    ui.register_handler("frmFisEstadosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_cancelar_click));
    ui.register_handler("frmFisEstadosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_novo_item_click));
    ui.register_handler("frmFisEstadosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisEstadosCad::btn_fechar_click));
    ui.register_handler("frmFisEstadosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEstadosCad::edt_busca_change));
    ui.register_form_class("frmFisEstadosLista", FrmFisEstadosLista::bind);
    ui.register_handler("frmFisEstadosLista.form_show", aylis::prelude::instances::form_handler(FrmFisEstadosLista::form_show));
    ui.register_handler("frmFisEstadosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisEstadosLista::btn_abrir_click));
    ui.register_handler("frmFisEstadosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEstadosLista::edt_busca_change));
    ui.register_handler("frmFisEstadosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisEstadosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisEstadosConsulta", FrmFisEstadosConsulta::bind);
    ui.register_handler("frmFisEstadosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisEstadosConsulta::form_show));
    ui.register_handler("frmFisEstadosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEstadosConsulta::edt_busca_change));
    ui.register_form_class("frmFisEstadosHist", FrmFisEstadosHist::bind);
    ui.register_handler("frmFisEstadosHist.form_show", aylis::prelude::instances::form_handler(FrmFisEstadosHist::form_show));
    ui.register_handler("frmFisEstadosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEstadosHist::edt_busca_change));
    ui.register_form_class("frmFisPaisesCad", FrmFisPaisesCad::bind);
    ui.register_handler("frmFisPaisesCad.form_show", aylis::prelude::instances::form_handler(FrmFisPaisesCad::form_show));
    ui.register_handler("frmFisPaisesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_novo_click));
    ui.register_handler("frmFisPaisesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_salvar_click));
    ui.register_handler("frmFisPaisesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_excluir_click));
    ui.register_handler("frmFisPaisesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_cancelar_click));
    ui.register_handler("frmFisPaisesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_novo_item_click));
    ui.register_handler("frmFisPaisesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisPaisesCad::btn_fechar_click));
    ui.register_handler("frmFisPaisesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisPaisesCad::edt_busca_change));
    ui.register_form_class("frmFisPaisesLista", FrmFisPaisesLista::bind);
    ui.register_handler("frmFisPaisesLista.form_show", aylis::prelude::instances::form_handler(FrmFisPaisesLista::form_show));
    ui.register_handler("frmFisPaisesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisPaisesLista::btn_abrir_click));
    ui.register_handler("frmFisPaisesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisPaisesLista::edt_busca_change));
    ui.register_handler("frmFisPaisesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisPaisesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisPaisesConsulta", FrmFisPaisesConsulta::bind);
    ui.register_handler("frmFisPaisesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisPaisesConsulta::form_show));
    ui.register_handler("frmFisPaisesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisPaisesConsulta::edt_busca_change));
    ui.register_form_class("frmFisPaisesHist", FrmFisPaisesHist::bind);
    ui.register_handler("frmFisPaisesHist.form_show", aylis::prelude::instances::form_handler(FrmFisPaisesHist::form_show));
    ui.register_handler("frmFisPaisesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisPaisesHist::edt_busca_change));
    ui.register_form_class("frmFisServicosDaLc116Cad", FrmFisServicosDaLc116Cad::bind);
    ui.register_handler("frmFisServicosDaLc116Cad.form_show", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::form_show));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_novo_click));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_salvar_click));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_excluir_click));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_cancelar_click));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_novo_item_click));
    ui.register_handler("frmFisServicosDaLc116Cad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::btn_fechar_click));
    ui.register_handler("frmFisServicosDaLc116Cad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Cad::edt_busca_change));
    ui.register_form_class("frmFisServicosDaLc116Lista", FrmFisServicosDaLc116Lista::bind);
    ui.register_handler("frmFisServicosDaLc116Lista.form_show", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Lista::form_show));
    ui.register_handler("frmFisServicosDaLc116Lista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Lista::btn_abrir_click));
    ui.register_handler("frmFisServicosDaLc116Lista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Lista::edt_busca_change));
    ui.register_handler("frmFisServicosDaLc116Lista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Lista::grd_lista_dbl_click));
    ui.register_form_class("frmFisServicosDaLc116Consulta", FrmFisServicosDaLc116Consulta::bind);
    ui.register_handler("frmFisServicosDaLc116Consulta.form_show", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Consulta::form_show));
    ui.register_handler("frmFisServicosDaLc116Consulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Consulta::edt_busca_change));
    ui.register_form_class("frmFisServicosDaLc116Hist", FrmFisServicosDaLc116Hist::bind);
    ui.register_handler("frmFisServicosDaLc116Hist.form_show", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Hist::form_show));
    ui.register_handler("frmFisServicosDaLc116Hist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisServicosDaLc116Hist::edt_busca_change));
    ui.register_form_class("frmFisObrigacoesAcessoriasCad", FrmFisObrigacoesAcessoriasCad::bind);
    ui.register_handler("frmFisObrigacoesAcessoriasCad.form_show", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::form_show));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_novo_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_salvar_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_excluir_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_cancelar_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_novo_item_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::btn_fechar_click));
    ui.register_handler("frmFisObrigacoesAcessoriasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasCad::edt_busca_change));
    ui.register_form_class("frmFisObrigacoesAcessoriasLista", FrmFisObrigacoesAcessoriasLista::bind);
    ui.register_handler("frmFisObrigacoesAcessoriasLista.form_show", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasLista::form_show));
    ui.register_handler("frmFisObrigacoesAcessoriasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasLista::btn_abrir_click));
    ui.register_handler("frmFisObrigacoesAcessoriasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasLista::edt_busca_change));
    ui.register_handler("frmFisObrigacoesAcessoriasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisObrigacoesAcessoriasConsulta", FrmFisObrigacoesAcessoriasConsulta::bind);
    ui.register_handler("frmFisObrigacoesAcessoriasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasConsulta::form_show));
    ui.register_handler("frmFisObrigacoesAcessoriasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasConsulta::edt_busca_change));
    ui.register_form_class("frmFisObrigacoesAcessoriasHist", FrmFisObrigacoesAcessoriasHist::bind);
    ui.register_handler("frmFisObrigacoesAcessoriasHist.form_show", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasHist::form_show));
    ui.register_handler("frmFisObrigacoesAcessoriasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisObrigacoesAcessoriasHist::edt_busca_change));
    ui.register_form_class("frmFisContabilistasCad", FrmFisContabilistasCad::bind);
    ui.register_handler("frmFisContabilistasCad.form_show", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::form_show));
    ui.register_handler("frmFisContabilistasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_novo_click));
    ui.register_handler("frmFisContabilistasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_salvar_click));
    ui.register_handler("frmFisContabilistasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_excluir_click));
    ui.register_handler("frmFisContabilistasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_cancelar_click));
    ui.register_handler("frmFisContabilistasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_novo_item_click));
    ui.register_handler("frmFisContabilistasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::btn_fechar_click));
    ui.register_handler("frmFisContabilistasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisContabilistasCad::edt_busca_change));
    ui.register_form_class("frmFisContabilistasLista", FrmFisContabilistasLista::bind);
    ui.register_handler("frmFisContabilistasLista.form_show", aylis::prelude::instances::form_handler(FrmFisContabilistasLista::form_show));
    ui.register_handler("frmFisContabilistasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisContabilistasLista::btn_abrir_click));
    ui.register_handler("frmFisContabilistasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisContabilistasLista::edt_busca_change));
    ui.register_handler("frmFisContabilistasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisContabilistasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisContabilistasConsulta", FrmFisContabilistasConsulta::bind);
    ui.register_handler("frmFisContabilistasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisContabilistasConsulta::form_show));
    ui.register_handler("frmFisContabilistasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisContabilistasConsulta::edt_busca_change));
    ui.register_form_class("frmFisContabilistasHist", FrmFisContabilistasHist::bind);
    ui.register_handler("frmFisContabilistasHist.form_show", aylis::prelude::instances::form_handler(FrmFisContabilistasHist::form_show));
    ui.register_handler("frmFisContabilistasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisContabilistasHist::edt_busca_change));
    ui.register_form_class("frmFisCertificadosDigitaisCad", FrmFisCertificadosDigitaisCad::bind);
    ui.register_handler("frmFisCertificadosDigitaisCad.form_show", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::form_show));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_novo_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_salvar_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_excluir_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_cancelar_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_novo_item_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::btn_fechar_click));
    ui.register_handler("frmFisCertificadosDigitaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisCad::edt_busca_change));
    ui.register_form_class("frmFisCertificadosDigitaisLista", FrmFisCertificadosDigitaisLista::bind);
    ui.register_handler("frmFisCertificadosDigitaisLista.form_show", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisLista::form_show));
    ui.register_handler("frmFisCertificadosDigitaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisLista::btn_abrir_click));
    ui.register_handler("frmFisCertificadosDigitaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisLista::edt_busca_change));
    ui.register_handler("frmFisCertificadosDigitaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCertificadosDigitaisConsulta", FrmFisCertificadosDigitaisConsulta::bind);
    ui.register_handler("frmFisCertificadosDigitaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisConsulta::form_show));
    ui.register_handler("frmFisCertificadosDigitaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisConsulta::edt_busca_change));
    ui.register_form_class("frmFisCertificadosDigitaisHist", FrmFisCertificadosDigitaisHist::bind);
    ui.register_handler("frmFisCertificadosDigitaisHist.form_show", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisHist::form_show));
    ui.register_handler("frmFisCertificadosDigitaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCertificadosDigitaisHist::edt_busca_change));
    ui.register_form_class("frmFisSeriesDeNotasCad", FrmFisSeriesDeNotasCad::bind);
    ui.register_handler("frmFisSeriesDeNotasCad.form_show", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::form_show));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_novo_click));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_salvar_click));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_excluir_click));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_cancelar_click));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_novo_item_click));
    ui.register_handler("frmFisSeriesDeNotasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::btn_fechar_click));
    ui.register_handler("frmFisSeriesDeNotasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasCad::edt_busca_change));
    ui.register_form_class("frmFisSeriesDeNotasLista", FrmFisSeriesDeNotasLista::bind);
    ui.register_handler("frmFisSeriesDeNotasLista.form_show", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasLista::form_show));
    ui.register_handler("frmFisSeriesDeNotasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasLista::btn_abrir_click));
    ui.register_handler("frmFisSeriesDeNotasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasLista::edt_busca_change));
    ui.register_handler("frmFisSeriesDeNotasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisSeriesDeNotasConsulta", FrmFisSeriesDeNotasConsulta::bind);
    ui.register_handler("frmFisSeriesDeNotasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasConsulta::form_show));
    ui.register_handler("frmFisSeriesDeNotasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasConsulta::edt_busca_change));
    ui.register_form_class("frmFisSeriesDeNotasHist", FrmFisSeriesDeNotasHist::bind);
    ui.register_handler("frmFisSeriesDeNotasHist.form_show", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasHist::form_show));
    ui.register_handler("frmFisSeriesDeNotasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSeriesDeNotasHist::edt_busca_change));
    ui.register_form_class("frmFisMensagensFiscaisCad", FrmFisMensagensFiscaisCad::bind);
    ui.register_handler("frmFisMensagensFiscaisCad.form_show", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::form_show));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_novo_click));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_salvar_click));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_excluir_click));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_cancelar_click));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_novo_item_click));
    ui.register_handler("frmFisMensagensFiscaisCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::btn_fechar_click));
    ui.register_handler("frmFisMensagensFiscaisCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisCad::edt_busca_change));
    ui.register_form_class("frmFisMensagensFiscaisLista", FrmFisMensagensFiscaisLista::bind);
    ui.register_handler("frmFisMensagensFiscaisLista.form_show", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisLista::form_show));
    ui.register_handler("frmFisMensagensFiscaisLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisLista::btn_abrir_click));
    ui.register_handler("frmFisMensagensFiscaisLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisLista::edt_busca_change));
    ui.register_handler("frmFisMensagensFiscaisLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisMensagensFiscaisConsulta", FrmFisMensagensFiscaisConsulta::bind);
    ui.register_handler("frmFisMensagensFiscaisConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisConsulta::form_show));
    ui.register_handler("frmFisMensagensFiscaisConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisConsulta::edt_busca_change));
    ui.register_form_class("frmFisMensagensFiscaisHist", FrmFisMensagensFiscaisHist::bind);
    ui.register_handler("frmFisMensagensFiscaisHist.form_show", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisHist::form_show));
    ui.register_handler("frmFisMensagensFiscaisHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisMensagensFiscaisHist::edt_busca_change));
    ui.register_form_class("frmFisNotasFiscaisEletronicasCad", FrmFisNotasFiscaisEletronicasCad::bind);
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.form_show", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::form_show));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_novo_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_salvar_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_excluir_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_cancelar_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_novo_item_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::btn_fechar_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasCad::edt_busca_change));
    ui.register_form_class("frmFisNotasFiscaisEletronicasLista", FrmFisNotasFiscaisEletronicasLista::bind);
    ui.register_handler("frmFisNotasFiscaisEletronicasLista.form_show", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasLista::form_show));
    ui.register_handler("frmFisNotasFiscaisEletronicasLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasLista::btn_abrir_click));
    ui.register_handler("frmFisNotasFiscaisEletronicasLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasLista::edt_busca_change));
    ui.register_handler("frmFisNotasFiscaisEletronicasLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisNotasFiscaisEletronicasConsulta", FrmFisNotasFiscaisEletronicasConsulta::bind);
    ui.register_handler("frmFisNotasFiscaisEletronicasConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasConsulta::form_show));
    ui.register_handler("frmFisNotasFiscaisEletronicasConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasConsulta::edt_busca_change));
    ui.register_form_class("frmFisNotasFiscaisEletronicasHist", FrmFisNotasFiscaisEletronicasHist::bind);
    ui.register_handler("frmFisNotasFiscaisEletronicasHist.form_show", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasHist::form_show));
    ui.register_handler("frmFisNotasFiscaisEletronicasHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasFiscaisEletronicasHist::edt_busca_change));
    ui.register_form_class("frmFisNotasDeServicoCad", FrmFisNotasDeServicoCad::bind);
    ui.register_handler("frmFisNotasDeServicoCad.form_show", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::form_show));
    ui.register_handler("frmFisNotasDeServicoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_novo_click));
    ui.register_handler("frmFisNotasDeServicoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_salvar_click));
    ui.register_handler("frmFisNotasDeServicoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_excluir_click));
    ui.register_handler("frmFisNotasDeServicoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_cancelar_click));
    ui.register_handler("frmFisNotasDeServicoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_novo_item_click));
    ui.register_handler("frmFisNotasDeServicoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::btn_fechar_click));
    ui.register_handler("frmFisNotasDeServicoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoCad::edt_busca_change));
    ui.register_form_class("frmFisNotasDeServicoLista", FrmFisNotasDeServicoLista::bind);
    ui.register_handler("frmFisNotasDeServicoLista.form_show", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoLista::form_show));
    ui.register_handler("frmFisNotasDeServicoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoLista::btn_abrir_click));
    ui.register_handler("frmFisNotasDeServicoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoLista::edt_busca_change));
    ui.register_handler("frmFisNotasDeServicoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisNotasDeServicoConsulta", FrmFisNotasDeServicoConsulta::bind);
    ui.register_handler("frmFisNotasDeServicoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoConsulta::form_show));
    ui.register_handler("frmFisNotasDeServicoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoConsulta::edt_busca_change));
    ui.register_form_class("frmFisNotasDeServicoHist", FrmFisNotasDeServicoHist::bind);
    ui.register_handler("frmFisNotasDeServicoHist.form_show", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoHist::form_show));
    ui.register_handler("frmFisNotasDeServicoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisNotasDeServicoHist::edt_busca_change));
    ui.register_form_class("frmFisConhecimentosDeTransporteCad", FrmFisConhecimentosDeTransporteCad::bind);
    ui.register_handler("frmFisConhecimentosDeTransporteCad.form_show", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::form_show));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_novo_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_salvar_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_excluir_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_cancelar_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_novo_item_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::btn_fechar_click));
    ui.register_handler("frmFisConhecimentosDeTransporteCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteCad::edt_busca_change));
    ui.register_form_class("frmFisConhecimentosDeTransporteLista", FrmFisConhecimentosDeTransporteLista::bind);
    ui.register_handler("frmFisConhecimentosDeTransporteLista.form_show", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteLista::form_show));
    ui.register_handler("frmFisConhecimentosDeTransporteLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteLista::btn_abrir_click));
    ui.register_handler("frmFisConhecimentosDeTransporteLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteLista::edt_busca_change));
    ui.register_handler("frmFisConhecimentosDeTransporteLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisConhecimentosDeTransporteConsulta", FrmFisConhecimentosDeTransporteConsulta::bind);
    ui.register_handler("frmFisConhecimentosDeTransporteConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteConsulta::form_show));
    ui.register_handler("frmFisConhecimentosDeTransporteConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteConsulta::edt_busca_change));
    ui.register_form_class("frmFisConhecimentosDeTransporteHist", FrmFisConhecimentosDeTransporteHist::bind);
    ui.register_handler("frmFisConhecimentosDeTransporteHist.form_show", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteHist::form_show));
    ui.register_handler("frmFisConhecimentosDeTransporteHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisConhecimentosDeTransporteHist::edt_busca_change));
    ui.register_form_class("frmFisManifestosDeCargaCad", FrmFisManifestosDeCargaCad::bind);
    ui.register_handler("frmFisManifestosDeCargaCad.form_show", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::form_show));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_novo_click));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_salvar_click));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_excluir_click));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_cancelar_click));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_novo_item_click));
    ui.register_handler("frmFisManifestosDeCargaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::btn_fechar_click));
    ui.register_handler("frmFisManifestosDeCargaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaCad::edt_busca_change));
    ui.register_form_class("frmFisManifestosDeCargaLista", FrmFisManifestosDeCargaLista::bind);
    ui.register_handler("frmFisManifestosDeCargaLista.form_show", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaLista::form_show));
    ui.register_handler("frmFisManifestosDeCargaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaLista::btn_abrir_click));
    ui.register_handler("frmFisManifestosDeCargaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaLista::edt_busca_change));
    ui.register_handler("frmFisManifestosDeCargaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisManifestosDeCargaConsulta", FrmFisManifestosDeCargaConsulta::bind);
    ui.register_handler("frmFisManifestosDeCargaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaConsulta::form_show));
    ui.register_handler("frmFisManifestosDeCargaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaConsulta::edt_busca_change));
    ui.register_form_class("frmFisManifestosDeCargaHist", FrmFisManifestosDeCargaHist::bind);
    ui.register_handler("frmFisManifestosDeCargaHist.form_show", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaHist::form_show));
    ui.register_handler("frmFisManifestosDeCargaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisManifestosDeCargaHist::edt_busca_change));
    ui.register_form_class("frmFisCartasDeCorrecaoCad", FrmFisCartasDeCorrecaoCad::bind);
    ui.register_handler("frmFisCartasDeCorrecaoCad.form_show", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::form_show));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_novo_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_salvar_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_excluir_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_cancelar_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_novo_item_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::btn_fechar_click));
    ui.register_handler("frmFisCartasDeCorrecaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoCad::edt_busca_change));
    ui.register_form_class("frmFisCartasDeCorrecaoLista", FrmFisCartasDeCorrecaoLista::bind);
    ui.register_handler("frmFisCartasDeCorrecaoLista.form_show", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoLista::form_show));
    ui.register_handler("frmFisCartasDeCorrecaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoLista::btn_abrir_click));
    ui.register_handler("frmFisCartasDeCorrecaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoLista::edt_busca_change));
    ui.register_handler("frmFisCartasDeCorrecaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCartasDeCorrecaoConsulta", FrmFisCartasDeCorrecaoConsulta::bind);
    ui.register_handler("frmFisCartasDeCorrecaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoConsulta::form_show));
    ui.register_handler("frmFisCartasDeCorrecaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoConsulta::edt_busca_change));
    ui.register_form_class("frmFisCartasDeCorrecaoHist", FrmFisCartasDeCorrecaoHist::bind);
    ui.register_handler("frmFisCartasDeCorrecaoHist.form_show", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoHist::form_show));
    ui.register_handler("frmFisCartasDeCorrecaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCartasDeCorrecaoHist::edt_busca_change));
    ui.register_form_class("frmFisCancelamentosDeNotaCad", FrmFisCancelamentosDeNotaCad::bind);
    ui.register_handler("frmFisCancelamentosDeNotaCad.form_show", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::form_show));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_novo_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_salvar_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_excluir_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_cancelar_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_novo_item_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::btn_fechar_click));
    ui.register_handler("frmFisCancelamentosDeNotaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaCad::edt_busca_change));
    ui.register_form_class("frmFisCancelamentosDeNotaLista", FrmFisCancelamentosDeNotaLista::bind);
    ui.register_handler("frmFisCancelamentosDeNotaLista.form_show", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaLista::form_show));
    ui.register_handler("frmFisCancelamentosDeNotaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaLista::btn_abrir_click));
    ui.register_handler("frmFisCancelamentosDeNotaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaLista::edt_busca_change));
    ui.register_handler("frmFisCancelamentosDeNotaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCancelamentosDeNotaConsulta", FrmFisCancelamentosDeNotaConsulta::bind);
    ui.register_handler("frmFisCancelamentosDeNotaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaConsulta::form_show));
    ui.register_handler("frmFisCancelamentosDeNotaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaConsulta::edt_busca_change));
    ui.register_form_class("frmFisCancelamentosDeNotaHist", FrmFisCancelamentosDeNotaHist::bind);
    ui.register_handler("frmFisCancelamentosDeNotaHist.form_show", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaHist::form_show));
    ui.register_handler("frmFisCancelamentosDeNotaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCancelamentosDeNotaHist::edt_busca_change));
    ui.register_form_class("frmFisInutilizacoesCad", FrmFisInutilizacoesCad::bind);
    ui.register_handler("frmFisInutilizacoesCad.form_show", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::form_show));
    ui.register_handler("frmFisInutilizacoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_novo_click));
    ui.register_handler("frmFisInutilizacoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_salvar_click));
    ui.register_handler("frmFisInutilizacoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_excluir_click));
    ui.register_handler("frmFisInutilizacoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_cancelar_click));
    ui.register_handler("frmFisInutilizacoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_novo_item_click));
    ui.register_handler("frmFisInutilizacoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::btn_fechar_click));
    ui.register_handler("frmFisInutilizacoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisInutilizacoesCad::edt_busca_change));
    ui.register_form_class("frmFisInutilizacoesLista", FrmFisInutilizacoesLista::bind);
    ui.register_handler("frmFisInutilizacoesLista.form_show", aylis::prelude::instances::form_handler(FrmFisInutilizacoesLista::form_show));
    ui.register_handler("frmFisInutilizacoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesLista::btn_abrir_click));
    ui.register_handler("frmFisInutilizacoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisInutilizacoesLista::edt_busca_change));
    ui.register_handler("frmFisInutilizacoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisInutilizacoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisInutilizacoesConsulta", FrmFisInutilizacoesConsulta::bind);
    ui.register_handler("frmFisInutilizacoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisInutilizacoesConsulta::form_show));
    ui.register_handler("frmFisInutilizacoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisInutilizacoesConsulta::edt_busca_change));
    ui.register_form_class("frmFisInutilizacoesHist", FrmFisInutilizacoesHist::bind);
    ui.register_handler("frmFisInutilizacoesHist.form_show", aylis::prelude::instances::form_handler(FrmFisInutilizacoesHist::form_show));
    ui.register_handler("frmFisInutilizacoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisInutilizacoesHist::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIcmsCad", FrmFisApuracoesDeIcmsCad::bind);
    ui.register_handler("frmFisApuracoesDeIcmsCad.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::form_show));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_novo_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_salvar_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_excluir_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_cancelar_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_novo_item_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::btn_fechar_click));
    ui.register_handler("frmFisApuracoesDeIcmsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsCad::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIcmsLista", FrmFisApuracoesDeIcmsLista::bind);
    ui.register_handler("frmFisApuracoesDeIcmsLista.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsLista::form_show));
    ui.register_handler("frmFisApuracoesDeIcmsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsLista::btn_abrir_click));
    ui.register_handler("frmFisApuracoesDeIcmsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsLista::edt_busca_change));
    ui.register_handler("frmFisApuracoesDeIcmsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisApuracoesDeIcmsConsulta", FrmFisApuracoesDeIcmsConsulta::bind);
    ui.register_handler("frmFisApuracoesDeIcmsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsConsulta::form_show));
    ui.register_handler("frmFisApuracoesDeIcmsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsConsulta::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIcmsHist", FrmFisApuracoesDeIcmsHist::bind);
    ui.register_handler("frmFisApuracoesDeIcmsHist.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsHist::form_show));
    ui.register_handler("frmFisApuracoesDeIcmsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIcmsHist::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIpiCad", FrmFisApuracoesDeIpiCad::bind);
    ui.register_handler("frmFisApuracoesDeIpiCad.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::form_show));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_novo_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_salvar_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_excluir_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_cancelar_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_novo_item_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::btn_fechar_click));
    ui.register_handler("frmFisApuracoesDeIpiCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiCad::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIpiLista", FrmFisApuracoesDeIpiLista::bind);
    ui.register_handler("frmFisApuracoesDeIpiLista.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiLista::form_show));
    ui.register_handler("frmFisApuracoesDeIpiLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiLista::btn_abrir_click));
    ui.register_handler("frmFisApuracoesDeIpiLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiLista::edt_busca_change));
    ui.register_handler("frmFisApuracoesDeIpiLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisApuracoesDeIpiConsulta", FrmFisApuracoesDeIpiConsulta::bind);
    ui.register_handler("frmFisApuracoesDeIpiConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiConsulta::form_show));
    ui.register_handler("frmFisApuracoesDeIpiConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiConsulta::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDeIpiHist", FrmFisApuracoesDeIpiHist::bind);
    ui.register_handler("frmFisApuracoesDeIpiHist.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiHist::form_show));
    ui.register_handler("frmFisApuracoesDeIpiHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDeIpiHist::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDePisECofinsCad", FrmFisApuracoesDePisECofinsCad::bind);
    ui.register_handler("frmFisApuracoesDePisECofinsCad.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::form_show));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_novo_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_salvar_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_excluir_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_cancelar_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_novo_item_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::btn_fechar_click));
    ui.register_handler("frmFisApuracoesDePisECofinsCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsCad::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDePisECofinsLista", FrmFisApuracoesDePisECofinsLista::bind);
    ui.register_handler("frmFisApuracoesDePisECofinsLista.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsLista::form_show));
    ui.register_handler("frmFisApuracoesDePisECofinsLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsLista::btn_abrir_click));
    ui.register_handler("frmFisApuracoesDePisECofinsLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsLista::edt_busca_change));
    ui.register_handler("frmFisApuracoesDePisECofinsLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisApuracoesDePisECofinsConsulta", FrmFisApuracoesDePisECofinsConsulta::bind);
    ui.register_handler("frmFisApuracoesDePisECofinsConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsConsulta::form_show));
    ui.register_handler("frmFisApuracoesDePisECofinsConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsConsulta::edt_busca_change));
    ui.register_form_class("frmFisApuracoesDePisECofinsHist", FrmFisApuracoesDePisECofinsHist::bind);
    ui.register_handler("frmFisApuracoesDePisECofinsHist.form_show", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsHist::form_show));
    ui.register_handler("frmFisApuracoesDePisECofinsHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisApuracoesDePisECofinsHist::edt_busca_change));
    ui.register_form_class("frmFisGuiasDeRecolhimentoCad", FrmFisGuiasDeRecolhimentoCad::bind);
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.form_show", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::form_show));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_novo_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_salvar_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_excluir_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_cancelar_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_novo_item_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::btn_fechar_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoCad::edt_busca_change));
    ui.register_form_class("frmFisGuiasDeRecolhimentoLista", FrmFisGuiasDeRecolhimentoLista::bind);
    ui.register_handler("frmFisGuiasDeRecolhimentoLista.form_show", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoLista::form_show));
    ui.register_handler("frmFisGuiasDeRecolhimentoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoLista::btn_abrir_click));
    ui.register_handler("frmFisGuiasDeRecolhimentoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoLista::edt_busca_change));
    ui.register_handler("frmFisGuiasDeRecolhimentoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisGuiasDeRecolhimentoConsulta", FrmFisGuiasDeRecolhimentoConsulta::bind);
    ui.register_handler("frmFisGuiasDeRecolhimentoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoConsulta::form_show));
    ui.register_handler("frmFisGuiasDeRecolhimentoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoConsulta::edt_busca_change));
    ui.register_form_class("frmFisGuiasDeRecolhimentoHist", FrmFisGuiasDeRecolhimentoHist::bind);
    ui.register_handler("frmFisGuiasDeRecolhimentoHist.form_show", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoHist::form_show));
    ui.register_handler("frmFisGuiasDeRecolhimentoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisGuiasDeRecolhimentoHist::edt_busca_change));
    ui.register_form_class("frmFisSpedFiscalCad", FrmFisSpedFiscalCad::bind);
    ui.register_handler("frmFisSpedFiscalCad.form_show", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::form_show));
    ui.register_handler("frmFisSpedFiscalCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_novo_click));
    ui.register_handler("frmFisSpedFiscalCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_salvar_click));
    ui.register_handler("frmFisSpedFiscalCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_excluir_click));
    ui.register_handler("frmFisSpedFiscalCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_cancelar_click));
    ui.register_handler("frmFisSpedFiscalCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_novo_item_click));
    ui.register_handler("frmFisSpedFiscalCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::btn_fechar_click));
    ui.register_handler("frmFisSpedFiscalCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedFiscalCad::edt_busca_change));
    ui.register_form_class("frmFisSpedFiscalLista", FrmFisSpedFiscalLista::bind);
    ui.register_handler("frmFisSpedFiscalLista.form_show", aylis::prelude::instances::form_handler(FrmFisSpedFiscalLista::form_show));
    ui.register_handler("frmFisSpedFiscalLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalLista::btn_abrir_click));
    ui.register_handler("frmFisSpedFiscalLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedFiscalLista::edt_busca_change));
    ui.register_handler("frmFisSpedFiscalLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisSpedFiscalLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisSpedFiscalConsulta", FrmFisSpedFiscalConsulta::bind);
    ui.register_handler("frmFisSpedFiscalConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisSpedFiscalConsulta::form_show));
    ui.register_handler("frmFisSpedFiscalConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedFiscalConsulta::edt_busca_change));
    ui.register_form_class("frmFisSpedFiscalHist", FrmFisSpedFiscalHist::bind);
    ui.register_handler("frmFisSpedFiscalHist.form_show", aylis::prelude::instances::form_handler(FrmFisSpedFiscalHist::form_show));
    ui.register_handler("frmFisSpedFiscalHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedFiscalHist::edt_busca_change));
    ui.register_form_class("frmFisSpedContribuicoesCad", FrmFisSpedContribuicoesCad::bind);
    ui.register_handler("frmFisSpedContribuicoesCad.form_show", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::form_show));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_novo_click));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_salvar_click));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_excluir_click));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_cancelar_click));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_novo_item_click));
    ui.register_handler("frmFisSpedContribuicoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::btn_fechar_click));
    ui.register_handler("frmFisSpedContribuicoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesCad::edt_busca_change));
    ui.register_form_class("frmFisSpedContribuicoesLista", FrmFisSpedContribuicoesLista::bind);
    ui.register_handler("frmFisSpedContribuicoesLista.form_show", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesLista::form_show));
    ui.register_handler("frmFisSpedContribuicoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesLista::btn_abrir_click));
    ui.register_handler("frmFisSpedContribuicoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesLista::edt_busca_change));
    ui.register_handler("frmFisSpedContribuicoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisSpedContribuicoesConsulta", FrmFisSpedContribuicoesConsulta::bind);
    ui.register_handler("frmFisSpedContribuicoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesConsulta::form_show));
    ui.register_handler("frmFisSpedContribuicoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesConsulta::edt_busca_change));
    ui.register_form_class("frmFisSpedContribuicoesHist", FrmFisSpedContribuicoesHist::bind);
    ui.register_handler("frmFisSpedContribuicoesHist.form_show", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesHist::form_show));
    ui.register_handler("frmFisSpedContribuicoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisSpedContribuicoesHist::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeEntradaCad", FrmFisLivrosDeEntradaCad::bind);
    ui.register_handler("frmFisLivrosDeEntradaCad.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::form_show));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_novo_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_salvar_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_excluir_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_cancelar_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_novo_item_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::btn_fechar_click));
    ui.register_handler("frmFisLivrosDeEntradaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaCad::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeEntradaLista", FrmFisLivrosDeEntradaLista::bind);
    ui.register_handler("frmFisLivrosDeEntradaLista.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaLista::form_show));
    ui.register_handler("frmFisLivrosDeEntradaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaLista::btn_abrir_click));
    ui.register_handler("frmFisLivrosDeEntradaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaLista::edt_busca_change));
    ui.register_handler("frmFisLivrosDeEntradaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisLivrosDeEntradaConsulta", FrmFisLivrosDeEntradaConsulta::bind);
    ui.register_handler("frmFisLivrosDeEntradaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaConsulta::form_show));
    ui.register_handler("frmFisLivrosDeEntradaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaConsulta::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeEntradaHist", FrmFisLivrosDeEntradaHist::bind);
    ui.register_handler("frmFisLivrosDeEntradaHist.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaHist::form_show));
    ui.register_handler("frmFisLivrosDeEntradaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeEntradaHist::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeSaidaCad", FrmFisLivrosDeSaidaCad::bind);
    ui.register_handler("frmFisLivrosDeSaidaCad.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::form_show));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_novo_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_salvar_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_excluir_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_cancelar_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_novo_item_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::btn_fechar_click));
    ui.register_handler("frmFisLivrosDeSaidaCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaCad::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeSaidaLista", FrmFisLivrosDeSaidaLista::bind);
    ui.register_handler("frmFisLivrosDeSaidaLista.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaLista::form_show));
    ui.register_handler("frmFisLivrosDeSaidaLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaLista::btn_abrir_click));
    ui.register_handler("frmFisLivrosDeSaidaLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaLista::edt_busca_change));
    ui.register_handler("frmFisLivrosDeSaidaLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisLivrosDeSaidaConsulta", FrmFisLivrosDeSaidaConsulta::bind);
    ui.register_handler("frmFisLivrosDeSaidaConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaConsulta::form_show));
    ui.register_handler("frmFisLivrosDeSaidaConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaConsulta::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeSaidaHist", FrmFisLivrosDeSaidaHist::bind);
    ui.register_handler("frmFisLivrosDeSaidaHist.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaHist::form_show));
    ui.register_handler("frmFisLivrosDeSaidaHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeSaidaHist::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeServicoCad", FrmFisLivrosDeServicoCad::bind);
    ui.register_handler("frmFisLivrosDeServicoCad.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::form_show));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_novo_click));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_salvar_click));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_excluir_click));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_cancelar_click));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_novo_item_click));
    ui.register_handler("frmFisLivrosDeServicoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::btn_fechar_click));
    ui.register_handler("frmFisLivrosDeServicoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoCad::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeServicoLista", FrmFisLivrosDeServicoLista::bind);
    ui.register_handler("frmFisLivrosDeServicoLista.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoLista::form_show));
    ui.register_handler("frmFisLivrosDeServicoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoLista::btn_abrir_click));
    ui.register_handler("frmFisLivrosDeServicoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoLista::edt_busca_change));
    ui.register_handler("frmFisLivrosDeServicoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisLivrosDeServicoConsulta", FrmFisLivrosDeServicoConsulta::bind);
    ui.register_handler("frmFisLivrosDeServicoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoConsulta::form_show));
    ui.register_handler("frmFisLivrosDeServicoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoConsulta::edt_busca_change));
    ui.register_form_class("frmFisLivrosDeServicoHist", FrmFisLivrosDeServicoHist::bind);
    ui.register_handler("frmFisLivrosDeServicoHist.form_show", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoHist::form_show));
    ui.register_handler("frmFisLivrosDeServicoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisLivrosDeServicoHist::edt_busca_change));
    ui.register_form_class("frmFisRetencoesNaFonteCad", FrmFisRetencoesNaFonteCad::bind);
    ui.register_handler("frmFisRetencoesNaFonteCad.form_show", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::form_show));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_novo_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_salvar_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_excluir_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_cancelar_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_novo_item_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::btn_fechar_click));
    ui.register_handler("frmFisRetencoesNaFonteCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteCad::edt_busca_change));
    ui.register_form_class("frmFisRetencoesNaFonteLista", FrmFisRetencoesNaFonteLista::bind);
    ui.register_handler("frmFisRetencoesNaFonteLista.form_show", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteLista::form_show));
    ui.register_handler("frmFisRetencoesNaFonteLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteLista::btn_abrir_click));
    ui.register_handler("frmFisRetencoesNaFonteLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteLista::edt_busca_change));
    ui.register_handler("frmFisRetencoesNaFonteLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisRetencoesNaFonteConsulta", FrmFisRetencoesNaFonteConsulta::bind);
    ui.register_handler("frmFisRetencoesNaFonteConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteConsulta::form_show));
    ui.register_handler("frmFisRetencoesNaFonteConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteConsulta::edt_busca_change));
    ui.register_form_class("frmFisRetencoesNaFonteHist", FrmFisRetencoesNaFonteHist::bind);
    ui.register_handler("frmFisRetencoesNaFonteHist.form_show", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteHist::form_show));
    ui.register_handler("frmFisRetencoesNaFonteHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisRetencoesNaFonteHist::edt_busca_change));
    ui.register_form_class("frmFisCreditosTributariosCad", FrmFisCreditosTributariosCad::bind);
    ui.register_handler("frmFisCreditosTributariosCad.form_show", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::form_show));
    ui.register_handler("frmFisCreditosTributariosCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_novo_click));
    ui.register_handler("frmFisCreditosTributariosCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_salvar_click));
    ui.register_handler("frmFisCreditosTributariosCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_excluir_click));
    ui.register_handler("frmFisCreditosTributariosCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_cancelar_click));
    ui.register_handler("frmFisCreditosTributariosCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_novo_item_click));
    ui.register_handler("frmFisCreditosTributariosCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::btn_fechar_click));
    ui.register_handler("frmFisCreditosTributariosCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosCad::edt_busca_change));
    ui.register_form_class("frmFisCreditosTributariosLista", FrmFisCreditosTributariosLista::bind);
    ui.register_handler("frmFisCreditosTributariosLista.form_show", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosLista::form_show));
    ui.register_handler("frmFisCreditosTributariosLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosLista::btn_abrir_click));
    ui.register_handler("frmFisCreditosTributariosLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosLista::edt_busca_change));
    ui.register_handler("frmFisCreditosTributariosLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisCreditosTributariosConsulta", FrmFisCreditosTributariosConsulta::bind);
    ui.register_handler("frmFisCreditosTributariosConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosConsulta::form_show));
    ui.register_handler("frmFisCreditosTributariosConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosConsulta::edt_busca_change));
    ui.register_form_class("frmFisCreditosTributariosHist", FrmFisCreditosTributariosHist::bind);
    ui.register_handler("frmFisCreditosTributariosHist.form_show", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosHist::form_show));
    ui.register_handler("frmFisCreditosTributariosHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisCreditosTributariosHist::edt_busca_change));
    ui.register_form_class("frmFisAjustesDeApuracaoCad", FrmFisAjustesDeApuracaoCad::bind);
    ui.register_handler("frmFisAjustesDeApuracaoCad.form_show", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::form_show));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_novo_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_salvar_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_excluir_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_cancelar_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_novo_item_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::btn_fechar_click));
    ui.register_handler("frmFisAjustesDeApuracaoCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoCad::edt_busca_change));
    ui.register_form_class("frmFisAjustesDeApuracaoLista", FrmFisAjustesDeApuracaoLista::bind);
    ui.register_handler("frmFisAjustesDeApuracaoLista.form_show", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoLista::form_show));
    ui.register_handler("frmFisAjustesDeApuracaoLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoLista::btn_abrir_click));
    ui.register_handler("frmFisAjustesDeApuracaoLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoLista::edt_busca_change));
    ui.register_handler("frmFisAjustesDeApuracaoLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisAjustesDeApuracaoConsulta", FrmFisAjustesDeApuracaoConsulta::bind);
    ui.register_handler("frmFisAjustesDeApuracaoConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoConsulta::form_show));
    ui.register_handler("frmFisAjustesDeApuracaoConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoConsulta::edt_busca_change));
    ui.register_form_class("frmFisAjustesDeApuracaoHist", FrmFisAjustesDeApuracaoHist::bind);
    ui.register_handler("frmFisAjustesDeApuracaoHist.form_show", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoHist::form_show));
    ui.register_handler("frmFisAjustesDeApuracaoHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisAjustesDeApuracaoHist::edt_busca_change));
    ui.register_form_class("frmFisEscrituracoesCad", FrmFisEscrituracoesCad::bind);
    ui.register_handler("frmFisEscrituracoesCad.form_show", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::form_show));
    ui.register_handler("frmFisEscrituracoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_novo_click));
    ui.register_handler("frmFisEscrituracoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_salvar_click));
    ui.register_handler("frmFisEscrituracoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_excluir_click));
    ui.register_handler("frmFisEscrituracoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_cancelar_click));
    ui.register_handler("frmFisEscrituracoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_novo_item_click));
    ui.register_handler("frmFisEscrituracoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::btn_fechar_click));
    ui.register_handler("frmFisEscrituracoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEscrituracoesCad::edt_busca_change));
    ui.register_form_class("frmFisEscrituracoesLista", FrmFisEscrituracoesLista::bind);
    ui.register_handler("frmFisEscrituracoesLista.form_show", aylis::prelude::instances::form_handler(FrmFisEscrituracoesLista::form_show));
    ui.register_handler("frmFisEscrituracoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesLista::btn_abrir_click));
    ui.register_handler("frmFisEscrituracoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEscrituracoesLista::edt_busca_change));
    ui.register_handler("frmFisEscrituracoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisEscrituracoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisEscrituracoesConsulta", FrmFisEscrituracoesConsulta::bind);
    ui.register_handler("frmFisEscrituracoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisEscrituracoesConsulta::form_show));
    ui.register_handler("frmFisEscrituracoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEscrituracoesConsulta::edt_busca_change));
    ui.register_form_class("frmFisEscrituracoesHist", FrmFisEscrituracoesHist::bind);
    ui.register_handler("frmFisEscrituracoesHist.form_show", aylis::prelude::instances::form_handler(FrmFisEscrituracoesHist::form_show));
    ui.register_handler("frmFisEscrituracoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisEscrituracoesHist::edt_busca_change));
    ui.register_form_class("frmFisDeclaracoesCad", FrmFisDeclaracoesCad::bind);
    ui.register_handler("frmFisDeclaracoesCad.form_show", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::form_show));
    ui.register_handler("frmFisDeclaracoesCad.btn_novo_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_novo_click));
    ui.register_handler("frmFisDeclaracoesCad.btn_salvar_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_salvar_click));
    ui.register_handler("frmFisDeclaracoesCad.btn_excluir_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_excluir_click));
    ui.register_handler("frmFisDeclaracoesCad.btn_cancelar_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_cancelar_click));
    ui.register_handler("frmFisDeclaracoesCad.btn_novo_item_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_novo_item_click));
    ui.register_handler("frmFisDeclaracoesCad.btn_fechar_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::btn_fechar_click));
    ui.register_handler("frmFisDeclaracoesCad.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisDeclaracoesCad::edt_busca_change));
    ui.register_form_class("frmFisDeclaracoesLista", FrmFisDeclaracoesLista::bind);
    ui.register_handler("frmFisDeclaracoesLista.form_show", aylis::prelude::instances::form_handler(FrmFisDeclaracoesLista::form_show));
    ui.register_handler("frmFisDeclaracoesLista.btn_abrir_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesLista::btn_abrir_click));
    ui.register_handler("frmFisDeclaracoesLista.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisDeclaracoesLista::edt_busca_change));
    ui.register_handler("frmFisDeclaracoesLista.grd_lista_dbl_click", aylis::prelude::instances::form_handler(FrmFisDeclaracoesLista::grd_lista_dbl_click));
    ui.register_form_class("frmFisDeclaracoesConsulta", FrmFisDeclaracoesConsulta::bind);
    ui.register_handler("frmFisDeclaracoesConsulta.form_show", aylis::prelude::instances::form_handler(FrmFisDeclaracoesConsulta::form_show));
    ui.register_handler("frmFisDeclaracoesConsulta.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisDeclaracoesConsulta::edt_busca_change));
    ui.register_form_class("frmFisDeclaracoesHist", FrmFisDeclaracoesHist::bind);
    ui.register_handler("frmFisDeclaracoesHist.form_show", aylis::prelude::instances::form_handler(FrmFisDeclaracoesHist::form_show));
    ui.register_handler("frmFisDeclaracoesHist.edt_busca_change", aylis::prelude::instances::form_handler(FrmFisDeclaracoesHist::edt_busca_change));
}
