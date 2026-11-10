// Eventos do form frmIBGE: o código é seu (o registro dos handlers fica no Forms.java
// gerado). Dentro de um handler, o form é a janela do evento.
//
// Os dados vêm da pasta assets/dados (a mesma para as 8 linguagens; `aylis-tool ibge` baixa
// e gera). Os estados, os municípios e os distritos vão para os ClientDataSets, que fazem
// o mestre-detalhe sozinhos (master_source); aqui fica o que muda a cada estado ou
// município escolhido: as bandeiras, os mapas e os dados dos gráficos.

package forms;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;

import dev.aylis.Aylis;
import static forms.Components.frmIBGE;

public final class FrmIBGE {
    /** A pasta dos dados no projeto (para os componentes). */
    private static final String DADOS = "assets/dados/";

    /** A mesma pasta no disco (o backend roda na pasta da linguagem). */
    private static final Path PASTA = Stream.of("../assets/dados", "assets/dados").map(Path::of).filter(Files::isDirectory).findFirst().orElse(Path.of("../assets/dados"));

    /** Os municípios (para os 10 maiores de cada estado). */
    private static List<Object> municipios = List.of();

    /** Código do estado ou do município → [[ano, habitantes], ...]. */
    private static Map<String, Object> populacao = Map.of();

    /** Um arquivo JSON da pasta dos dados (null se faltar). */
    private static Object ler(String arquivo) {
        try {
            return Aylis.Json.parse(Files.readString(PASTA.resolve(arquivo), StandardCharsets.UTF_8));
        } catch (IOException e) {
            return null;
        }
    }

    /** Põe os registros (uma lista de objetos) num ClientDataSet do form. */
    private static void carregar(Aylis aylis, Aylis.AylisComponent dataset, Object linhas) {
        aylis.call("data", Aylis.obj("target", dataset.id, "op", "load", "rows", linhas == null ? List.of() : linhas));
    }

    /** O registro atual de um ClientDataSet (campo → valor). */
    @SuppressWarnings("unchecked")
    private static Map<String, Object> registro(Aylis aylis, Aylis.AylisComponent dataset) {
        Object r = aylis.call("data", Aylis.obj("target", dataset.id, "op", "record"));
        return r instanceof Map<?, ?> m ? (Map<String, Object>) m : Map.of();
    }

    /** O valor de um campo como texto (vazio se for nulo; números sem casas). */
    private static String texto(Object v) {
        if (v == null) return "";
        if (v instanceof Double d && d == Math.rint(d)) return Long.toString(d.longValue());
        return v.toString();
    }

    private static long numero(Object v) {
        return v instanceof Number n ? n.longValue() : 0;
    }

    /** O caminho de uma imagem da pasta dos dados (vazio: não tem). */
    private static String imagem(String caminho) {
        return caminho.isEmpty() ? "" : DADOS + caminho;
    }

    @SuppressWarnings("unchecked")
    public static void form_show(Aylis aylis, Aylis.Event ev) {
        Object estados = ler("estados.json");
        if (estados == null) {
            frmIBGE.lblFonte.setText("Faltam os dados: rode  aylis-tool ibge examples/IBGE/assets/dados");
            return;
        }
        municipios = ler("municipios.json") instanceof List<?> l ? (List<Object>) l : List.of();
        populacao = ler("populacao.json") instanceof Map<?, ?> m ? (Map<String, Object>) m : Map.of();
        frmIBGE.svgBrasil.setSource(DADOS + "mapas/BR.svg");
        // Os detalhes primeiro: quando o mestre abre, eles já filtram pelo estado.
        carregar(aylis, frmIBGE.cdsDistritos, ler("distritos.json"));
        carregar(aylis, frmIBGE.cdsMunicipios, municipios);
        carregar(aylis, frmIBGE.cdsEstados, estados);
        estado_mudou(aylis, ev);
    }

    /** Outro estado na grade: a bandeira, os mapas e os 10 maiores municípios. */
    @SuppressWarnings("unchecked")
    public static void estado_mudou(Aylis aylis, Aylis.Event ev) {
        Map<String, Object> e = registro(aylis, frmIBGE.cdsEstados);
        if (e.get("id") == null) return;
        long uf = numero(e.get("id"));
        frmIBGE.lblEstado.setText(texto(e.get("nome")));
        frmIBGE.lblRegiao.setText(texto(e.get("sigla")) + " · Região " + texto(e.get("regiao")));
        frmIBGE.imgBandeiraEstado.setImage(imagem(texto(e.get("bandeira"))));
        frmIBGE.lblEstadoEstimativa.setText("Estimativa de " + texto(e.get("estimativa_ano")));
        frmIBGE.svgBrasil.setHighlight("e" + uf);
        String mapa = DADOS + "mapas/" + uf + ".svg";
        frmIBGE.svgEstado.setSource(mapa);
        frmIBGE.svgMunicipio.setSource(mapa);

        List<Map<String, Object>> doEstado = new ArrayList<>();
        for (Object o : municipios) {
            Map<String, Object> m = (Map<String, Object>) o;
            if (numero(m.get("uf_id")) == uf) doEstado.add(m);
        }
        doEstado.sort(Comparator.<Map<String, Object>>comparingLong(m -> -numero(m.get("populacao"))).thenComparing(m -> (String) m.get("nome")));
        List<Object> maiores = new ArrayList<>();
        for (Map<String, Object> m : doEstado.subList(0, Math.min(10, doEstado.size()))) {
            maiores.add(Aylis.obj("nome", m.get("nome"), "habitantes", m.get("populacao")));
        }
        carregar(aylis, frmIBGE.cdsMaiores, maiores);
        municipio_mudou(aylis, ev);
    }

    /** Outro município: a bandeira, o destaque nos mapas e a população ano a ano. */
    public static void municipio_mudou(Aylis aylis, Aylis.Event ev) {
        Map<String, Object> m = registro(aylis, frmIBGE.cdsMunicipios);
        if (m.get("id") == null) return;
        long codigo = numero(m.get("id"));
        frmIBGE.lblMunicipio.setText(texto(m.get("nome")));
        frmIBGE.lblCodigo.setText("Código IBGE " + codigo);
        String bandeira = texto(m.get("bandeira"));
        frmIBGE.imgBandeiraMunicipio.setImage(imagem(bandeira));
        frmIBGE.lblSemBandeira.setVisible(bandeira.isEmpty());
        frmIBGE.lblEstimativa.setText("Estimativa de " + texto(m.get("estimativa_ano")));
        String alvo = "m" + codigo;
        frmIBGE.svgEstado.setHighlight(alvo);
        frmIBGE.svgMunicipio.setHighlight(alvo);
        frmIBGE.svgMunicipio.setZoomTo(alvo);

        List<Object> linhas = new ArrayList<>();
        if (populacao.get(Long.toString(codigo)) instanceof List<?> serie) {
            for (Object p : serie) {
                List<?> par = (List<?>) p;
                linhas.add(Aylis.obj("ano", texto(par.get(0)), "habitantes", par.get(1)));
            }
        }
        carregar(aylis, frmIBGE.cdsPopulacao, linhas);
    }
}
