// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { ComponentInfo, AylisEvent } from "../aylis.ts";
import { AylisComponent, AylisForm, registerFormClass } from "../aylis.ts";

/** Um ClientDataSet de um form. */
export class ClientDataSet extends AylisComponent {
    get keyFields(): Promise<string> {
        return this.getProp("key_fields");
    }
    set keyFields(value: string) {
        this.setProp("key_fields", value);
    }
    get applyMode(): Promise<string> {
        return this.getProp("apply_mode");
    }
    set applyMode(value: string) {
        this.setProp("apply_mode", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only");
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get active(): Promise<boolean> {
        return this.getProp("active");
    }
    set active(value: boolean) {
        this.setProp("active", value);
    }
    get filter(): Promise<string> {
        return this.getProp("filter");
    }
    set filter(value: string) {
        this.setProp("filter", value);
    }
    get filtered(): Promise<boolean> {
        return this.getProp("filtered");
    }
    set filtered(value: boolean) {
        this.setProp("filtered", value);
    }
    get indexFieldNames(): Promise<string> {
        return this.getProp("index_field_names");
    }
    set indexFieldNames(value: string) {
        this.setProp("index_field_names", value);
    }
    get indexes(): Promise<string> {
        return this.getProp("indexes");
    }
    set indexes(value: string) {
        this.setProp("indexes", value);
    }
    get indexName(): Promise<string> {
        return this.getProp("index_name");
    }
    set indexName(value: string) {
        this.setProp("index_name", value);
    }
    get params(): Promise<string> {
        return this.getProp("params");
    }
    set params(value: string) {
        this.setProp("params", value);
    }
    get statusFilter(): Promise<string> {
        return this.getProp("status_filter");
    }
    set statusFilter(value: string) {
        this.setProp("status_filter", value);
    }
    get masterSource(): Promise<string> {
        return this.getProp("master_source");
    }
    set masterSource(value: string) {
        this.setProp("master_source", value);
    }
    get masterFields(): Promise<string> {
        return this.getProp("master_fields");
    }
    set masterFields(value: string) {
        this.setProp("master_fields", value);
    }
    get applySeparately(): Promise<boolean> {
        return this.getProp("apply_separately");
    }
    set applySeparately(value: boolean) {
        this.setProp("apply_separately", value);
    }
    get recordTotal(): Promise<number> {
        return this.getProp("record_total");
    }
    set recordTotal(value: number) {
        this.setProp("record_total", value);
    }
    get maxRecords(): Promise<number> {
        return this.getProp("max_records");
    }
    set maxRecords(value: number) {
        this.setProp("max_records", value);
    }
    get partialNotice(): Promise<string> {
        return this.getProp("partial_notice");
    }
}

/** Um DBChart de um form. */
export class DBChart extends AylisComponent {
    get dataSource(): Promise<string> {
        return this.getProp("data_source");
    }
    set dataSource(value: string) {
        this.setProp("data_source", value);
    }
    get labelField(): Promise<string> {
        return this.getProp("label_field");
    }
    set labelField(value: string) {
        this.setProp("label_field", value);
    }
    get valueFields(): Promise<string> {
        return this.getProp("value_fields");
    }
    set valueFields(value: string) {
        this.setProp("value_fields", value);
    }
    get kind(): Promise<string> {
        return this.getProp("kind");
    }
    set kind(value: string) {
        this.setProp("kind", value);
    }
    get title(): Promise<string> {
        return this.getProp("title");
    }
    set title(value: string) {
        this.setProp("title", value);
    }
    get labels(): Promise<string> {
        return this.getProp("labels");
    }
    set labels(value: string) {
        this.setProp("labels", value);
    }
    get series(): Promise<string> {
        return this.getProp("series");
    }
    set series(value: string) {
        this.setProp("series", value);
    }
    get json(): Promise<string> {
        return this.getProp("json");
    }
    set json(value: string) {
        this.setProp("json", value);
    }
    get legend(): Promise<boolean> {
        return this.getProp("legend");
    }
    set legend(value: boolean) {
        this.setProp("legend", value);
    }
    get stepped(): Promise<boolean> {
        return this.getProp("stepped");
    }
    set stepped(value: boolean) {
        this.setProp("stepped", value);
    }
    get tension(): Promise<number> {
        return this.getProp("tension");
    }
    set tension(value: number) {
        this.setProp("tension", value);
    }
    get animate(): Promise<boolean> {
        return this.getProp("animate");
    }
    set animate(value: boolean) {
        this.setProp("animate", value);
    }
    get zoom(): Promise<boolean> {
        return this.getProp("zoom");
    }
    set zoom(value: boolean) {
        this.setProp("zoom", value);
    }
    get autoScale(): Promise<boolean> {
        return this.getProp("auto_scale");
    }
    set autoScale(value: boolean) {
        this.setProp("auto_scale", value);
    }
    get yMin(): Promise<number> {
        return this.getProp("y_min");
    }
    set yMin(value: number) {
        this.setProp("y_min", value);
    }
    get yMax(): Promise<number> {
        return this.getProp("y_max");
    }
    set yMax(value: number) {
        this.setProp("y_max", value);
    }
    get logScale(): Promise<boolean> {
        return this.getProp("log_scale");
    }
    set logScale(value: boolean) {
        this.setProp("log_scale", value);
    }
    get reverse(): Promise<boolean> {
        return this.getProp("reverse");
    }
    set reverse(value: boolean) {
        this.setProp("reverse", value);
    }
    get timeAxis(): Promise<boolean> {
        return this.getProp("time_axis");
    }
    set timeAxis(value: boolean) {
        this.setProp("time_axis", value);
    }
    get clickedSeries(): Promise<number> {
        return this.getProp("clicked_series");
    }
    set clickedSeries(value: number) {
        this.setProp("clicked_series", value);
    }
    get clickedIndex(): Promise<number> {
        return this.getProp("clicked_index");
    }
    set clickedIndex(value: number) {
        this.setProp("clicked_index", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um DataGrid de um form. */
export class DataGrid extends AylisComponent {
    get dataSource(): Promise<string> {
        return this.getProp("data_source");
    }
    set dataSource(value: string) {
        this.setProp("data_source", value);
    }
    get columns(): Promise<string> {
        return this.getProp("columns");
    }
    set columns(value: string) {
        this.setProp("columns", value);
    }
    get rowHeight(): Promise<number> {
        return this.getProp("row_height");
    }
    set rowHeight(value: number) {
        this.setProp("row_height", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only");
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get sortable(): Promise<boolean> {
        return this.getProp("sortable");
    }
    set sortable(value: boolean) {
        this.setProp("sortable", value);
    }
    get columnMove(): Promise<boolean> {
        return this.getProp("column_move");
    }
    set columnMove(value: boolean) {
        this.setProp("column_move", value);
    }
    get fixedColumns(): Promise<number> {
        return this.getProp("fixed_columns");
    }
    set fixedColumns(value: number) {
        this.setProp("fixed_columns", value);
    }
    get filterRow(): Promise<boolean> {
        return this.getProp("filter_row");
    }
    set filterRow(value: boolean) {
        this.setProp("filter_row", value);
    }
    get multiSelect(): Promise<boolean> {
        return this.getProp("multi_select");
    }
    set multiSelect(value: boolean) {
        this.setProp("multi_select", value);
    }
    get footer(): Promise<string> {
        return this.getProp("footer");
    }
    set footer(value: string) {
        this.setProp("footer", value);
    }
    get incrementalSearch(): Promise<boolean> {
        return this.getProp("incremental_search");
    }
    set incrementalSearch(value: boolean) {
        this.setProp("incremental_search", value);
    }
    get cursorColor(): Promise<string> {
        return this.getProp("cursor_color");
    }
    set cursorColor(value: string) {
        this.setProp("cursor_color", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    setFocus() {
        this.aylis.send("focus", { target: this.id });
    }
}

/** Um DataSource de um form. */
export class DataSource extends AylisComponent {
    get dataSet(): Promise<string> {
        return this.getProp("data_set");
    }
    set dataSet(value: string) {
        this.setProp("data_set", value);
    }
    get autoEdit(): Promise<boolean> {
        return this.getProp("auto_edit");
    }
    set autoEdit(value: boolean) {
        this.setProp("auto_edit", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
}

/** Um Field de um form. */
export class Field extends AylisComponent {
    get fieldName(): Promise<string> {
        return this.getProp("field_name");
    }
    set fieldName(value: string) {
        this.setProp("field_name", value);
    }
    get type(): Promise<string> {
        return this.getProp("type");
    }
    set type(value: string) {
        this.setProp("type", value);
    }
    get size(): Promise<number> {
        return this.getProp("size");
    }
    set size(value: number) {
        this.setProp("size", value);
    }
    get scale(): Promise<number> {
        return this.getProp("scale");
    }
    set scale(value: number) {
        this.setProp("scale", value);
    }
    get rounding(): Promise<string> {
        return this.getProp("rounding");
    }
    set rounding(value: string) {
        this.setProp("rounding", value);
    }
    get displayLabel(): Promise<string> {
        return this.getProp("display_label");
    }
    set displayLabel(value: string) {
        this.setProp("display_label", value);
    }
    get displayWidth(): Promise<number> {
        return this.getProp("display_width");
    }
    set displayWidth(value: number) {
        this.setProp("display_width", value);
    }
    get displayFormat(): Promise<string> {
        return this.getProp("display_format");
    }
    set displayFormat(value: string) {
        this.setProp("display_format", value);
    }
    get editFormat(): Promise<string> {
        return this.getProp("edit_format");
    }
    set editFormat(value: string) {
        this.setProp("edit_format", value);
    }
    get alignment(): Promise<string> {
        return this.getProp("alignment");
    }
    set alignment(value: string) {
        this.setProp("alignment", value);
    }
    get required(): Promise<boolean> {
        return this.getProp("required");
    }
    set required(value: boolean) {
        this.setProp("required", value);
    }
    get readOnly(): Promise<boolean> {
        return this.getProp("read_only");
    }
    set readOnly(value: boolean) {
        this.setProp("read_only", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get min(): Promise<number> {
        return this.getProp("min");
    }
    set min(value: number) {
        this.setProp("min", value);
    }
    get max(): Promise<number> {
        return this.getProp("max");
    }
    set max(value: number) {
        this.setProp("max", value);
    }
    get defaultValue(): Promise<string> {
        return this.getProp("default_value");
    }
    set defaultValue(value: string) {
        this.setProp("default_value", value);
    }
    get displayValues(): Promise<string> {
        return this.getProp("display_values");
    }
    set displayValues(value: string) {
        this.setProp("display_values", value);
    }
    get charCase(): Promise<string> {
        return this.getProp("char_case");
    }
    set charCase(value: string) {
        this.setProp("char_case", value);
    }
    get editMask(): Promise<string> {
        return this.getProp("edit_mask");
    }
    set editMask(value: string) {
        this.setProp("edit_mask", value);
    }
    get sensitive(): Promise<boolean> {
        return this.getProp("sensitive");
    }
    set sensitive(value: boolean) {
        this.setProp("sensitive", value);
    }
    get kind(): Promise<string> {
        return this.getProp("kind");
    }
    set kind(value: string) {
        this.setProp("kind", value);
    }
    get expression(): Promise<string> {
        return this.getProp("expression");
    }
    set expression(value: string) {
        this.setProp("expression", value);
    }
    get lookupDataSet(): Promise<string> {
        return this.getProp("lookup_data_set");
    }
    set lookupDataSet(value: string) {
        this.setProp("lookup_data_set", value);
    }
    get keyFields(): Promise<string> {
        return this.getProp("key_fields");
    }
    set keyFields(value: string) {
        this.setProp("key_fields", value);
    }
    get lookupKeyFields(): Promise<string> {
        return this.getProp("lookup_key_fields");
    }
    set lookupKeyFields(value: string) {
        this.setProp("lookup_key_fields", value);
    }
    get lookupResultField(): Promise<string> {
        return this.getProp("lookup_result_field");
    }
    set lookupResultField(value: string) {
        this.setProp("lookup_result_field", value);
    }
}

/** Um Form de um form. */
export class Form extends AylisForm {
    get caption(): Promise<string> {
        return this.getProp("caption");
    }
    set caption(value: string) {
        this.setProp("caption", value);
    }
    get position(): Promise<string> {
        return this.getProp("position");
    }
    set position(value: string) {
        this.setProp("position", value);
    }
    get borderStyle(): Promise<string> {
        return this.getProp("border_style");
    }
    set borderStyle(value: string) {
        this.setProp("border_style", value);
    }
    get windowState(): Promise<string> {
        return this.getProp("window_state");
    }
    set windowState(value: string) {
        this.setProp("window_state", value);
    }
    get transition(): Promise<string> {
        return this.getProp("transition");
    }
    set transition(value: string) {
        this.setProp("transition", value);
    }
    get topmost(): Promise<boolean> {
        return this.getProp("topmost");
    }
    set topmost(value: boolean) {
        this.setProp("topmost", value);
    }
    get showInTaskbar(): Promise<boolean> {
        return this.getProp("show_in_taskbar");
    }
    set showInTaskbar(value: boolean) {
        this.setProp("show_in_taskbar", value);
    }
    get activeControl(): Promise<string> {
        return this.getProp("active_control");
    }
    set activeControl(value: string) {
        this.setProp("active_control", value);
    }
    get tabOrder(): Promise<string> {
        return this.getProp("tab_order");
    }
    set tabOrder(value: string) {
        this.setProp("tab_order", value);
    }
    get tabKeys(): Promise<string> {
        return this.getProp("tab_keys");
    }
    set tabKeys(value: string) {
        this.setProp("tab_keys", value);
    }
    get backTabKeys(): Promise<string> {
        return this.getProp("back_tab_keys");
    }
    set backTabKeys(value: string) {
        this.setProp("back_tab_keys", value);
    }
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars");
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars");
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed");
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override");
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map");
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get keyPreview(): Promise<boolean> {
        return this.getProp("key_preview");
    }
    set keyPreview(value: boolean) {
        this.setProp("key_preview", value);
    }
    get zoom(): Promise<number> {
        return this.getProp("zoom");
    }
    set zoom(value: number) {
        this.setProp("zoom", value);
    }
    get monitor(): Promise<number> {
        return this.getProp("monitor");
    }
    set monitor(value: number) {
        this.setProp("monitor", value);
    }
    get rememberPlacement(): Promise<boolean> {
        return this.getProp("remember_placement");
    }
    set rememberPlacement(value: boolean) {
        this.setProp("remember_placement", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    moveToMonitor(a1: number) {
        this.aylis.send("invoke", { target: this.id, method: "move_to_monitor", args: [a1] });
    }
    focused(): Promise<string> {
        return this.aylis.call("focused", { target: this.id }) as Promise<any>;
    }
    focusNext() {
        this.aylis.send("focus_next", { target: this.id });
    }
    focusPrev() {
        this.aylis.send("focus_prev", { target: this.id });
    }
}

/** Um GroupBox de um form. */
export class GroupBox extends AylisComponent {
    get caption(): Promise<string> {
        return this.getProp("caption");
    }
    set caption(value: string) {
        this.setProp("caption", value);
    }
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars");
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars");
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed");
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override");
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map");
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Image de um form. */
export class Image extends AylisComponent {
    get image(): Promise<string> {
        return this.getProp("image");
    }
    set image(value: string) {
        this.setProp("image", value);
    }
    get fit(): Promise<string> {
        return this.getProp("fit");
    }
    set fit(value: string) {
        this.setProp("fit", value);
    }
    get radius(): Promise<number> {
        return this.getProp("radius");
    }
    set radius(value: number) {
        this.setProp("radius", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Label de um form. */
export class Label extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get hAlign(): Promise<string> {
        return this.getProp("h_align");
    }
    set hAlign(value: string) {
        this.setProp("h_align", value);
    }
    get vAlign(): Promise<string> {
        return this.getProp("v_align");
    }
    set vAlign(value: string) {
        this.setProp("v_align", value);
    }
    get wrap(): Promise<boolean> {
        return this.getProp("wrap");
    }
    set wrap(value: boolean) {
        this.setProp("wrap", value);
    }
    get trimming(): Promise<string> {
        return this.getProp("trimming");
    }
    set trimming(value: string) {
        this.setProp("trimming", value);
    }
    get bold(): Promise<boolean> {
        return this.getProp("bold");
    }
    set bold(value: boolean) {
        this.setProp("bold", value);
    }
    get color(): Promise<string> {
        return this.getProp("color");
    }
    set color(value: string) {
        this.setProp("color", value);
    }
    get highlight(): Promise<string> {
        return this.getProp("highlight");
    }
    set highlight(value: string) {
        this.setProp("highlight", value);
    }
    get letterSpacing(): Promise<number> {
        return this.getProp("letter_spacing");
    }
    set letterSpacing(value: number) {
        this.setProp("letter_spacing", value);
    }
    get textCase(): Promise<string> {
        return this.getProp("text_case");
    }
    set textCase(value: string) {
        this.setProp("text_case", value);
    }
    get markup(): Promise<boolean> {
        return this.getProp("markup");
    }
    set markup(value: boolean) {
        this.setProp("markup", value);
    }
    get outline(): Promise<string> {
        return this.getProp("outline");
    }
    set outline(value: string) {
        this.setProp("outline", value);
    }
    get outlineWidth(): Promise<number> {
        return this.getProp("outline_width");
    }
    set outlineWidth(value: number) {
        this.setProp("outline_width", value);
    }
    get focusControl(): Promise<string> {
        return this.getProp("focus_control");
    }
    set focusControl(value: string) {
        this.setProp("focus_control", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Layout de um form. */
export class Layout extends AylisComponent {
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars");
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars");
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed");
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override");
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map");
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Panel de um form. */
export class Panel extends AylisComponent {
    get allowedChars(): Promise<string> {
        return this.getProp("allowed_chars");
    }
    set allowedChars(value: string) {
        this.setProp("allowed_chars", value);
    }
    get blockedChars(): Promise<string> {
        return this.getProp("blocked_chars");
    }
    set blockedChars(value: string) {
        this.setProp("blocked_chars", value);
    }
    get onlyAllowed(): Promise<boolean> {
        return this.getProp("only_allowed");
    }
    set onlyAllowed(value: boolean) {
        this.setProp("only_allowed", value);
    }
    get charsOverride(): Promise<boolean> {
        return this.getProp("chars_override");
    }
    set charsOverride(value: boolean) {
        this.setProp("chars_override", value);
    }
    get charMap(): Promise<string> {
        return this.getProp("char_map");
    }
    set charMap(value: string) {
        this.setProp("char_map", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get fill2(): Promise<string> {
        return this.getProp("fill2");
    }
    set fill2(value: string) {
        this.setProp("fill2", value);
    }
    get fillMid(): Promise<string> {
        return this.getProp("fill_mid");
    }
    set fillMid(value: string) {
        this.setProp("fill_mid", value);
    }
    get fillMidAt(): Promise<number> {
        return this.getProp("fill_mid_at");
    }
    set fillMidAt(value: number) {
        this.setProp("fill_mid_at", value);
    }
    get gradientAngle(): Promise<number> {
        return this.getProp("gradient_angle");
    }
    set gradientAngle(value: number) {
        this.setProp("gradient_angle", value);
    }
    get gradientRadial(): Promise<boolean> {
        return this.getProp("gradient_radial");
    }
    set gradientRadial(value: boolean) {
        this.setProp("gradient_radial", value);
    }
    get stroke(): Promise<string> {
        return this.getProp("stroke");
    }
    set stroke(value: string) {
        this.setProp("stroke", value);
    }
    get strokeWidth(): Promise<number> {
        return this.getProp("stroke_width");
    }
    set strokeWidth(value: number) {
        this.setProp("stroke_width", value);
    }
    get radius(): Promise<number> {
        return this.getProp("radius");
    }
    set radius(value: number) {
        this.setProp("radius", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Splitter de um form. */
export class Splitter extends AylisComponent {
    get minSize(): Promise<number> {
        return this.getProp("min_size");
    }
    set minSize(value: number) {
        this.setProp("min_size", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um Svg de um form. */
export class Svg extends AylisComponent {
    get source(): Promise<string> {
        return this.getProp("source");
    }
    set source(value: string) {
        this.setProp("source", value);
    }
    get color(): Promise<string> {
        return this.getProp("color");
    }
    set color(value: string) {
        this.setProp("color", value);
    }
    get fit(): Promise<string> {
        return this.getProp("fit");
    }
    set fit(value: string) {
        this.setProp("fit", value);
    }
    get highlight(): Promise<string> {
        return this.getProp("highlight");
    }
    set highlight(value: string) {
        this.setProp("highlight", value);
    }
    get highlightColor(): Promise<string> {
        return this.getProp("highlight_color");
    }
    set highlightColor(value: string) {
        this.setProp("highlight_color", value);
    }
    get zoomTo(): Promise<string> {
        return this.getProp("zoom_to");
    }
    set zoomTo(value: string) {
        this.setProp("zoom_to", value);
    }
    get left(): Promise<number> {
        return this.getProp("left");
    }
    set left(value: number) {
        this.setProp("left", value);
    }
    get top(): Promise<number> {
        return this.getProp("top");
    }
    set top(value: number) {
        this.setProp("top", value);
    }
    get width(): Promise<number> {
        return this.getProp("width");
    }
    set width(value: number) {
        this.setProp("width", value);
    }
    get height(): Promise<number> {
        return this.getProp("height");
    }
    set height(value: number) {
        this.setProp("height", value);
    }
    get align(): Promise<string> {
        return this.getProp("align");
    }
    set align(value: string) {
        this.setProp("align", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get hitTest(): Promise<boolean> {
        return this.getProp("hit_test");
    }
    set hitTest(value: boolean) {
        this.setProp("hit_test", value);
    }
    get clipChildren(): Promise<boolean> {
        return this.getProp("clip_children");
    }
    set clipChildren(value: boolean) {
        this.setProp("clip_children", value);
    }
    get capturesWheel(): Promise<boolean> {
        return this.getProp("captures_wheel");
    }
    set capturesWheel(value: boolean) {
        this.setProp("captures_wheel", value);
    }
    get canFocus(): Promise<boolean> {
        return this.getProp("can_focus");
    }
    set canFocus(value: boolean) {
        this.setProp("can_focus", value);
    }
    get draggable(): Promise<boolean> {
        return this.getProp("draggable");
    }
    set draggable(value: boolean) {
        this.setProp("draggable", value);
    }
    get acceptsDrops(): Promise<boolean> {
        return this.getProp("accepts_drops");
    }
    set acceptsDrops(value: boolean) {
        this.setProp("accepts_drops", value);
    }
    get dragData(): Promise<string> {
        return this.getProp("drag_data");
    }
    set dragData(value: string) {
        this.setProp("drag_data", value);
    }
    get dragLabel(): Promise<string> {
        return this.getProp("drag_label");
    }
    set dragLabel(value: string) {
        this.setProp("drag_label", value);
    }
    get dragImage(): Promise<string> {
        return this.getProp("drag_image");
    }
    set dragImage(value: string) {
        this.setProp("drag_image", value);
    }
    get dropTypes(): Promise<string> {
        return this.getProp("drop_types");
    }
    set dropTypes(value: string) {
        this.setProp("drop_types", value);
    }
    get tabStop(): Promise<boolean> {
        return this.getProp("tab_stop");
    }
    set tabStop(value: boolean) {
        this.setProp("tab_stop", value);
    }
    get cursor(): Promise<string> {
        return this.getProp("cursor");
    }
    set cursor(value: string) {
        this.setProp("cursor", value);
    }
    get opacity(): Promise<number> {
        return this.getProp("opacity");
    }
    set opacity(value: number) {
        this.setProp("opacity", value);
    }
    get cacheLayer(): Promise<boolean> {
        return this.getProp("cache_layer");
    }
    set cacheLayer(value: boolean) {
        this.setProp("cache_layer", value);
    }
    get rotation(): Promise<number> {
        return this.getProp("rotation");
    }
    set rotation(value: number) {
        this.setProp("rotation", value);
    }
    get translateX(): Promise<number> {
        return this.getProp("translate_x");
    }
    set translateX(value: number) {
        this.setProp("translate_x", value);
    }
    get translateY(): Promise<number> {
        return this.getProp("translate_y");
    }
    set translateY(value: number) {
        this.setProp("translate_y", value);
    }
    get aspectRatio(): Promise<number> {
        return this.getProp("aspect_ratio");
    }
    set aspectRatio(value: number) {
        this.setProp("aspect_ratio", value);
    }
    get maxWidth(): Promise<number> {
        return this.getProp("max_width");
    }
    set maxWidth(value: number) {
        this.setProp("max_width", value);
    }
    get minWidth(): Promise<number> {
        return this.getProp("min_width");
    }
    set minWidth(value: number) {
        this.setProp("min_width", value);
    }
    get minHeight(): Promise<number> {
        return this.getProp("min_height");
    }
    set minHeight(value: number) {
        this.setProp("min_height", value);
    }
    get maxHeight(): Promise<number> {
        return this.getProp("max_height");
    }
    set maxHeight(value: number) {
        this.setProp("max_height", value);
    }
    get selfAlign(): Promise<string> {
        return this.getProp("self_align");
    }
    set selfAlign(value: string) {
        this.setProp("self_align", value);
    }
    get autoSize(): Promise<boolean> {
        return this.getProp("auto_size");
    }
    set autoSize(value: boolean) {
        this.setProp("auto_size", value);
    }
    get columnSpan(): Promise<number> {
        return this.getProp("column_span");
    }
    set columnSpan(value: number) {
        this.setProp("column_span", value);
    }
    get shadow(): Promise<string> {
        return this.getProp("shadow");
    }
    set shadow(value: string) {
        this.setProp("shadow", value);
    }
    get style(): Promise<string> {
        return this.getProp("style");
    }
    set style(value: string) {
        this.setProp("style", value);
    }
    get theme(): Promise<string> {
        return this.getProp("theme");
    }
    set theme(value: string) {
        this.setProp("theme", value);
    }
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
    }
    get borderColor(): Promise<string> {
        return this.getProp("border_color");
    }
    set borderColor(value: string) {
        this.setProp("border_color", value);
    }
    get textColor(): Promise<string> {
        return this.getProp("text_color");
    }
    set textColor(value: string) {
        this.setProp("text_color", value);
    }
    get accentColor(): Promise<string> {
        return this.getProp("accent_color");
    }
    set accentColor(value: string) {
        this.setProp("accent_color", value);
    }
    get successColor(): Promise<string> {
        return this.getProp("success_color");
    }
    set successColor(value: string) {
        this.setProp("success_color", value);
    }
    get warningColor(): Promise<string> {
        return this.getProp("warning_color");
    }
    set warningColor(value: string) {
        this.setProp("warning_color", value);
    }
    get dangerColor(): Promise<string> {
        return this.getProp("danger_color");
    }
    set dangerColor(value: string) {
        this.setProp("danger_color", value);
    }
    get fontFamily(): Promise<string> {
        return this.getProp("font_family");
    }
    set fontFamily(value: string) {
        this.setProp("font_family", value);
    }
    get fontSize(): Promise<number> {
        return this.getProp("font_size");
    }
    set fontSize(value: number) {
        this.setProp("font_size", value);
    }
    get fontWeight(): Promise<string> {
        return this.getProp("font_weight");
    }
    set fontWeight(value: string) {
        this.setProp("font_weight", value);
    }
    get fontItalic(): Promise<boolean> {
        return this.getProp("font_italic");
    }
    set fontItalic(value: boolean) {
        this.setProp("font_italic", value);
    }
    get fontUnderline(): Promise<boolean> {
        return this.getProp("font_underline");
    }
    set fontUnderline(value: boolean) {
        this.setProp("font_underline", value);
    }
    get fontStrikeout(): Promise<boolean> {
        return this.getProp("font_strikeout");
    }
    set fontStrikeout(value: boolean) {
        this.setProp("font_strikeout", value);
    }
    get hint(): Promise<string> {
        return this.getProp("hint");
    }
    set hint(value: string) {
        this.setProp("hint", value);
    }
    get tag(): Promise<number> {
        return this.getProp("tag");
    }
    set tag(value: number) {
        this.setProp("tag", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** O form frmIBGE e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmIBGEComponents extends Form {
    cdsEstados!: ClientDataSet;
    cdsEstadosId!: Field;
    cdsEstadosSigla!: Field;
    cdsEstadosNome!: Field;
    cdsEstadosRegiao!: Field;
    cdsEstadosPopulacao!: Field;
    cdsEstadosArea!: Field;
    cdsEstadosDensidade!: Field;
    cdsEstadosPib!: Field;
    cdsEstadosPibPerCapita!: Field;
    cdsEstadosEstimativa!: Field;
    cdsEstadosEstimativaAno!: Field;
    cdsEstadosMunicipios!: Field;
    cdsEstadosBandeira!: Field;
    dsEstados!: DataSource;
    cdsMunicipios!: ClientDataSet;
    cdsMunicipiosId!: Field;
    cdsMunicipiosUfId!: Field;
    cdsMunicipiosUf!: Field;
    cdsMunicipiosNome!: Field;
    cdsMunicipiosMesorregiao!: Field;
    cdsMunicipiosMicrorregiao!: Field;
    cdsMunicipiosRegiaoIntermediaria!: Field;
    cdsMunicipiosRegiaoImediata!: Field;
    cdsMunicipiosPopulacao!: Field;
    cdsMunicipiosArea!: Field;
    cdsMunicipiosDensidade!: Field;
    cdsMunicipiosPib!: Field;
    cdsMunicipiosPibParticipacao!: Field;
    cdsMunicipiosPibPerCapita!: Field;
    cdsMunicipiosEstimativa!: Field;
    cdsMunicipiosEstimativaAno!: Field;
    cdsMunicipiosDistritos!: Field;
    cdsMunicipiosBandeira!: Field;
    dsMunicipios!: DataSource;
    cdsDistritos!: ClientDataSet;
    cdsDistritosId!: Field;
    cdsDistritosMunicipioId!: Field;
    cdsDistritosNome!: Field;
    dsDistritos!: DataSource;
    cdsPopulacao!: ClientDataSet;
    cdsPopulacaoAno!: Field;
    cdsPopulacaoHabitantes!: Field;
    dsPopulacao!: DataSource;
    cdsMaiores!: ClientDataSet;
    cdsMaioresNome!: Field;
    cdsMaioresHabitantes!: Field;
    dsMaiores!: DataSource;
    lblFonte!: Label;
    lytListas!: Layout;
    gbEstados!: GroupBox;
    grdEstados!: DataGrid;
    gbDistritos!: GroupBox;
    grdDistritos!: DataGrid;
    gbMunicipios!: GroupBox;
    grdMunicipios!: DataGrid;
    splListas!: Splitter;
    lytPainel!: Layout;
    pnlEstado!: Panel;
    svgBrasil!: Svg;
    lytBandeiraEstado!: Layout;
    lblEstado!: Label;
    lblRegiao!: Label;
    imgBandeiraEstado!: Image;
    lytNumerosEstado!: Layout;
    dbtEstadoPopulacao!: Label;
    lblEstadoEstimativa!: Label;
    dbtEstadoEstimativa!: Label;
    dbtEstadoArea!: Label;
    dbtEstadoDensidade!: Label;
    dbtEstadoPib!: Label;
    dbtEstadoPibPerCapita!: Label;
    dbtEstadoMunicipios!: Label;
    pnlMunicipio!: Panel;
    svgMunicipio!: Svg;
    svgEstado!: Svg;
    lytBandeiraMunicipio!: Layout;
    lblMunicipio!: Label;
    lblCodigo!: Label;
    imgBandeiraMunicipio!: Image;
    lblSemBandeira!: Label;
    lytNumerosMunicipio!: Layout;
    dbtMesorregiao!: Label;
    dbtMicrorregiao!: Label;
    dbtRegiaoImediata!: Label;
    dbtPopulacao!: Label;
    lblEstimativa!: Label;
    dbtEstimativa!: Label;
    dbtArea!: Label;
    dbtDensidade!: Label;
    dbtPib!: Label;
    dbtPibPerCapita!: Label;
    dbtPibParticipacao!: Label;
    lytGraficos!: Layout;
    dbcPopulacao!: DBChart;
    dbcMaiores!: DBChart;
    dbcEstados!: DBChart;
    bind(list: ComponentInfo[]) {
        this.cdsEstados = this.resolve(list, "cdsEstados", "ClientDataSet", ClientDataSet);
        this.cdsEstadosId = this.resolve(list, "cdsEstadosId", "Field", Field);
        this.cdsEstadosSigla = this.resolve(list, "cdsEstadosSigla", "Field", Field);
        this.cdsEstadosNome = this.resolve(list, "cdsEstadosNome", "Field", Field);
        this.cdsEstadosRegiao = this.resolve(list, "cdsEstadosRegiao", "Field", Field);
        this.cdsEstadosPopulacao = this.resolve(list, "cdsEstadosPopulacao", "Field", Field);
        this.cdsEstadosArea = this.resolve(list, "cdsEstadosArea", "Field", Field);
        this.cdsEstadosDensidade = this.resolve(list, "cdsEstadosDensidade", "Field", Field);
        this.cdsEstadosPib = this.resolve(list, "cdsEstadosPib", "Field", Field);
        this.cdsEstadosPibPerCapita = this.resolve(list, "cdsEstadosPibPerCapita", "Field", Field);
        this.cdsEstadosEstimativa = this.resolve(list, "cdsEstadosEstimativa", "Field", Field);
        this.cdsEstadosEstimativaAno = this.resolve(list, "cdsEstadosEstimativaAno", "Field", Field);
        this.cdsEstadosMunicipios = this.resolve(list, "cdsEstadosMunicipios", "Field", Field);
        this.cdsEstadosBandeira = this.resolve(list, "cdsEstadosBandeira", "Field", Field);
        this.dsEstados = this.resolve(list, "dsEstados", "DataSource", DataSource);
        this.cdsMunicipios = this.resolve(list, "cdsMunicipios", "ClientDataSet", ClientDataSet);
        this.cdsMunicipiosId = this.resolve(list, "cdsMunicipiosId", "Field", Field);
        this.cdsMunicipiosUfId = this.resolve(list, "cdsMunicipiosUfId", "Field", Field);
        this.cdsMunicipiosUf = this.resolve(list, "cdsMunicipiosUf", "Field", Field);
        this.cdsMunicipiosNome = this.resolve(list, "cdsMunicipiosNome", "Field", Field);
        this.cdsMunicipiosMesorregiao = this.resolve(list, "cdsMunicipiosMesorregiao", "Field", Field);
        this.cdsMunicipiosMicrorregiao = this.resolve(list, "cdsMunicipiosMicrorregiao", "Field", Field);
        this.cdsMunicipiosRegiaoIntermediaria = this.resolve(list, "cdsMunicipiosRegiaoIntermediaria", "Field", Field);
        this.cdsMunicipiosRegiaoImediata = this.resolve(list, "cdsMunicipiosRegiaoImediata", "Field", Field);
        this.cdsMunicipiosPopulacao = this.resolve(list, "cdsMunicipiosPopulacao", "Field", Field);
        this.cdsMunicipiosArea = this.resolve(list, "cdsMunicipiosArea", "Field", Field);
        this.cdsMunicipiosDensidade = this.resolve(list, "cdsMunicipiosDensidade", "Field", Field);
        this.cdsMunicipiosPib = this.resolve(list, "cdsMunicipiosPib", "Field", Field);
        this.cdsMunicipiosPibParticipacao = this.resolve(list, "cdsMunicipiosPibParticipacao", "Field", Field);
        this.cdsMunicipiosPibPerCapita = this.resolve(list, "cdsMunicipiosPibPerCapita", "Field", Field);
        this.cdsMunicipiosEstimativa = this.resolve(list, "cdsMunicipiosEstimativa", "Field", Field);
        this.cdsMunicipiosEstimativaAno = this.resolve(list, "cdsMunicipiosEstimativaAno", "Field", Field);
        this.cdsMunicipiosDistritos = this.resolve(list, "cdsMunicipiosDistritos", "Field", Field);
        this.cdsMunicipiosBandeira = this.resolve(list, "cdsMunicipiosBandeira", "Field", Field);
        this.dsMunicipios = this.resolve(list, "dsMunicipios", "DataSource", DataSource);
        this.cdsDistritos = this.resolve(list, "cdsDistritos", "ClientDataSet", ClientDataSet);
        this.cdsDistritosId = this.resolve(list, "cdsDistritosId", "Field", Field);
        this.cdsDistritosMunicipioId = this.resolve(list, "cdsDistritosMunicipioId", "Field", Field);
        this.cdsDistritosNome = this.resolve(list, "cdsDistritosNome", "Field", Field);
        this.dsDistritos = this.resolve(list, "dsDistritos", "DataSource", DataSource);
        this.cdsPopulacao = this.resolve(list, "cdsPopulacao", "ClientDataSet", ClientDataSet);
        this.cdsPopulacaoAno = this.resolve(list, "cdsPopulacaoAno", "Field", Field);
        this.cdsPopulacaoHabitantes = this.resolve(list, "cdsPopulacaoHabitantes", "Field", Field);
        this.dsPopulacao = this.resolve(list, "dsPopulacao", "DataSource", DataSource);
        this.cdsMaiores = this.resolve(list, "cdsMaiores", "ClientDataSet", ClientDataSet);
        this.cdsMaioresNome = this.resolve(list, "cdsMaioresNome", "Field", Field);
        this.cdsMaioresHabitantes = this.resolve(list, "cdsMaioresHabitantes", "Field", Field);
        this.dsMaiores = this.resolve(list, "dsMaiores", "DataSource", DataSource);
        this.lblFonte = this.resolve(list, "lblFonte", "Label", Label);
        this.lytListas = this.resolve(list, "lytListas", "Layout", Layout);
        this.gbEstados = this.resolve(list, "gbEstados", "GroupBox", GroupBox);
        this.grdEstados = this.resolve(list, "grdEstados", "DataGrid", DataGrid);
        this.gbDistritos = this.resolve(list, "gbDistritos", "GroupBox", GroupBox);
        this.grdDistritos = this.resolve(list, "grdDistritos", "DataGrid", DataGrid);
        this.gbMunicipios = this.resolve(list, "gbMunicipios", "GroupBox", GroupBox);
        this.grdMunicipios = this.resolve(list, "grdMunicipios", "DataGrid", DataGrid);
        this.splListas = this.resolve(list, "splListas", "Splitter", Splitter);
        this.lytPainel = this.resolve(list, "lytPainel", "Layout", Layout);
        this.pnlEstado = this.resolve(list, "pnlEstado", "Panel", Panel);
        this.svgBrasil = this.resolve(list, "svgBrasil", "Svg", Svg);
        this.lytBandeiraEstado = this.resolve(list, "lytBandeiraEstado", "Layout", Layout);
        this.lblEstado = this.resolve(list, "lblEstado", "Label", Label);
        this.lblRegiao = this.resolve(list, "lblRegiao", "Label", Label);
        this.imgBandeiraEstado = this.resolve(list, "imgBandeiraEstado", "Image", Image);
        this.lytNumerosEstado = this.resolve(list, "lytNumerosEstado", "Layout", Layout);
        this.dbtEstadoPopulacao = this.resolve(list, "dbtEstadoPopulacao", "Label", Label);
        this.lblEstadoEstimativa = this.resolve(list, "lblEstadoEstimativa", "Label", Label);
        this.dbtEstadoEstimativa = this.resolve(list, "dbtEstadoEstimativa", "Label", Label);
        this.dbtEstadoArea = this.resolve(list, "dbtEstadoArea", "Label", Label);
        this.dbtEstadoDensidade = this.resolve(list, "dbtEstadoDensidade", "Label", Label);
        this.dbtEstadoPib = this.resolve(list, "dbtEstadoPib", "Label", Label);
        this.dbtEstadoPibPerCapita = this.resolve(list, "dbtEstadoPibPerCapita", "Label", Label);
        this.dbtEstadoMunicipios = this.resolve(list, "dbtEstadoMunicipios", "Label", Label);
        this.pnlMunicipio = this.resolve(list, "pnlMunicipio", "Panel", Panel);
        this.svgMunicipio = this.resolve(list, "svgMunicipio", "Svg", Svg);
        this.svgEstado = this.resolve(list, "svgEstado", "Svg", Svg);
        this.lytBandeiraMunicipio = this.resolve(list, "lytBandeiraMunicipio", "Layout", Layout);
        this.lblMunicipio = this.resolve(list, "lblMunicipio", "Label", Label);
        this.lblCodigo = this.resolve(list, "lblCodigo", "Label", Label);
        this.imgBandeiraMunicipio = this.resolve(list, "imgBandeiraMunicipio", "Image", Image);
        this.lblSemBandeira = this.resolve(list, "lblSemBandeira", "Label", Label);
        this.lytNumerosMunicipio = this.resolve(list, "lytNumerosMunicipio", "Layout", Layout);
        this.dbtMesorregiao = this.resolve(list, "dbtMesorregiao", "Label", Label);
        this.dbtMicrorregiao = this.resolve(list, "dbtMicrorregiao", "Label", Label);
        this.dbtRegiaoImediata = this.resolve(list, "dbtRegiaoImediata", "Label", Label);
        this.dbtPopulacao = this.resolve(list, "dbtPopulacao", "Label", Label);
        this.lblEstimativa = this.resolve(list, "lblEstimativa", "Label", Label);
        this.dbtEstimativa = this.resolve(list, "dbtEstimativa", "Label", Label);
        this.dbtArea = this.resolve(list, "dbtArea", "Label", Label);
        this.dbtDensidade = this.resolve(list, "dbtDensidade", "Label", Label);
        this.dbtPib = this.resolve(list, "dbtPib", "Label", Label);
        this.dbtPibPerCapita = this.resolve(list, "dbtPibPerCapita", "Label", Label);
        this.dbtPibParticipacao = this.resolve(list, "dbtPibParticipacao", "Label", Label);
        this.lytGraficos = this.resolve(list, "lytGraficos", "Layout", Layout);
        this.dbcPopulacao = this.resolve(list, "dbcPopulacao", "DBChart", DBChart);
        this.dbcMaiores = this.resolve(list, "dbcMaiores", "DBChart", DBChart);
        this.dbcEstados = this.resolve(list, "dbcEstados", "DBChart", DBChart);
    }
    /** O ClientDataSet deste form com o nome (como o FindComponent). */
    findClientDataSet(name: string): ClientDataSet | undefined {
        switch (name) {
            case "cdsEstados": return this.cdsEstados;
            case "cdsMunicipios": return this.cdsMunicipios;
            case "cdsDistritos": return this.cdsDistritos;
            case "cdsPopulacao": return this.cdsPopulacao;
            case "cdsMaiores": return this.cdsMaiores;
        }
        return undefined;
    }
    /** O ClientDataSet que disparou o evento. */
    senderAsClientDataSet(ev: AylisEvent): ClientDataSet | undefined {
        return this.findClientDataSet(ev.sender.name);
    }
    /** O DBChart deste form com o nome (como o FindComponent). */
    findDBChart(name: string): DBChart | undefined {
        switch (name) {
            case "dbcPopulacao": return this.dbcPopulacao;
            case "dbcMaiores": return this.dbcMaiores;
            case "dbcEstados": return this.dbcEstados;
        }
        return undefined;
    }
    /** O DBChart que disparou o evento. */
    senderAsDBChart(ev: AylisEvent): DBChart | undefined {
        return this.findDBChart(ev.sender.name);
    }
    /** O DataGrid deste form com o nome (como o FindComponent). */
    findDataGrid(name: string): DataGrid | undefined {
        switch (name) {
            case "grdEstados": return this.grdEstados;
            case "grdDistritos": return this.grdDistritos;
            case "grdMunicipios": return this.grdMunicipios;
        }
        return undefined;
    }
    /** O DataGrid que disparou o evento. */
    senderAsDataGrid(ev: AylisEvent): DataGrid | undefined {
        return this.findDataGrid(ev.sender.name);
    }
    /** O DataSource deste form com o nome (como o FindComponent). */
    findDataSource(name: string): DataSource | undefined {
        switch (name) {
            case "dsEstados": return this.dsEstados;
            case "dsMunicipios": return this.dsMunicipios;
            case "dsDistritos": return this.dsDistritos;
            case "dsPopulacao": return this.dsPopulacao;
            case "dsMaiores": return this.dsMaiores;
        }
        return undefined;
    }
    /** O DataSource que disparou o evento. */
    senderAsDataSource(ev: AylisEvent): DataSource | undefined {
        return this.findDataSource(ev.sender.name);
    }
    /** O Field deste form com o nome (como o FindComponent). */
    findField(name: string): Field | undefined {
        switch (name) {
            case "cdsEstadosId": return this.cdsEstadosId;
            case "cdsEstadosSigla": return this.cdsEstadosSigla;
            case "cdsEstadosNome": return this.cdsEstadosNome;
            case "cdsEstadosRegiao": return this.cdsEstadosRegiao;
            case "cdsEstadosPopulacao": return this.cdsEstadosPopulacao;
            case "cdsEstadosArea": return this.cdsEstadosArea;
            case "cdsEstadosDensidade": return this.cdsEstadosDensidade;
            case "cdsEstadosPib": return this.cdsEstadosPib;
            case "cdsEstadosPibPerCapita": return this.cdsEstadosPibPerCapita;
            case "cdsEstadosEstimativa": return this.cdsEstadosEstimativa;
            case "cdsEstadosEstimativaAno": return this.cdsEstadosEstimativaAno;
            case "cdsEstadosMunicipios": return this.cdsEstadosMunicipios;
            case "cdsEstadosBandeira": return this.cdsEstadosBandeira;
            case "cdsMunicipiosId": return this.cdsMunicipiosId;
            case "cdsMunicipiosUfId": return this.cdsMunicipiosUfId;
            case "cdsMunicipiosUf": return this.cdsMunicipiosUf;
            case "cdsMunicipiosNome": return this.cdsMunicipiosNome;
            case "cdsMunicipiosMesorregiao": return this.cdsMunicipiosMesorregiao;
            case "cdsMunicipiosMicrorregiao": return this.cdsMunicipiosMicrorregiao;
            case "cdsMunicipiosRegiaoIntermediaria": return this.cdsMunicipiosRegiaoIntermediaria;
            case "cdsMunicipiosRegiaoImediata": return this.cdsMunicipiosRegiaoImediata;
            case "cdsMunicipiosPopulacao": return this.cdsMunicipiosPopulacao;
            case "cdsMunicipiosArea": return this.cdsMunicipiosArea;
            case "cdsMunicipiosDensidade": return this.cdsMunicipiosDensidade;
            case "cdsMunicipiosPib": return this.cdsMunicipiosPib;
            case "cdsMunicipiosPibParticipacao": return this.cdsMunicipiosPibParticipacao;
            case "cdsMunicipiosPibPerCapita": return this.cdsMunicipiosPibPerCapita;
            case "cdsMunicipiosEstimativa": return this.cdsMunicipiosEstimativa;
            case "cdsMunicipiosEstimativaAno": return this.cdsMunicipiosEstimativaAno;
            case "cdsMunicipiosDistritos": return this.cdsMunicipiosDistritos;
            case "cdsMunicipiosBandeira": return this.cdsMunicipiosBandeira;
            case "cdsDistritosId": return this.cdsDistritosId;
            case "cdsDistritosMunicipioId": return this.cdsDistritosMunicipioId;
            case "cdsDistritosNome": return this.cdsDistritosNome;
            case "cdsPopulacaoAno": return this.cdsPopulacaoAno;
            case "cdsPopulacaoHabitantes": return this.cdsPopulacaoHabitantes;
            case "cdsMaioresNome": return this.cdsMaioresNome;
            case "cdsMaioresHabitantes": return this.cdsMaioresHabitantes;
        }
        return undefined;
    }
    /** O Field que disparou o evento. */
    senderAsField(ev: AylisEvent): Field | undefined {
        return this.findField(ev.sender.name);
    }
    /** O GroupBox deste form com o nome (como o FindComponent). */
    findGroupBox(name: string): GroupBox | undefined {
        switch (name) {
            case "gbEstados": return this.gbEstados;
            case "gbDistritos": return this.gbDistritos;
            case "gbMunicipios": return this.gbMunicipios;
        }
        return undefined;
    }
    /** O GroupBox que disparou o evento. */
    senderAsGroupBox(ev: AylisEvent): GroupBox | undefined {
        return this.findGroupBox(ev.sender.name);
    }
    /** O Image deste form com o nome (como o FindComponent). */
    findImage(name: string): Image | undefined {
        switch (name) {
            case "imgBandeiraEstado": return this.imgBandeiraEstado;
            case "imgBandeiraMunicipio": return this.imgBandeiraMunicipio;
        }
        return undefined;
    }
    /** O Image que disparou o evento. */
    senderAsImage(ev: AylisEvent): Image | undefined {
        return this.findImage(ev.sender.name);
    }
    /** O Label deste form com o nome (como o FindComponent). */
    findLabel(name: string): Label | undefined {
        switch (name) {
            case "lblFonte": return this.lblFonte;
            case "lblEstado": return this.lblEstado;
            case "lblRegiao": return this.lblRegiao;
            case "dbtEstadoPopulacao": return this.dbtEstadoPopulacao;
            case "lblEstadoEstimativa": return this.lblEstadoEstimativa;
            case "dbtEstadoEstimativa": return this.dbtEstadoEstimativa;
            case "dbtEstadoArea": return this.dbtEstadoArea;
            case "dbtEstadoDensidade": return this.dbtEstadoDensidade;
            case "dbtEstadoPib": return this.dbtEstadoPib;
            case "dbtEstadoPibPerCapita": return this.dbtEstadoPibPerCapita;
            case "dbtEstadoMunicipios": return this.dbtEstadoMunicipios;
            case "lblMunicipio": return this.lblMunicipio;
            case "lblCodigo": return this.lblCodigo;
            case "lblSemBandeira": return this.lblSemBandeira;
            case "dbtMesorregiao": return this.dbtMesorregiao;
            case "dbtMicrorregiao": return this.dbtMicrorregiao;
            case "dbtRegiaoImediata": return this.dbtRegiaoImediata;
            case "dbtPopulacao": return this.dbtPopulacao;
            case "lblEstimativa": return this.lblEstimativa;
            case "dbtEstimativa": return this.dbtEstimativa;
            case "dbtArea": return this.dbtArea;
            case "dbtDensidade": return this.dbtDensidade;
            case "dbtPib": return this.dbtPib;
            case "dbtPibPerCapita": return this.dbtPibPerCapita;
            case "dbtPibParticipacao": return this.dbtPibParticipacao;
        }
        return undefined;
    }
    /** O Label que disparou o evento. */
    senderAsLabel(ev: AylisEvent): Label | undefined {
        return this.findLabel(ev.sender.name);
    }
    /** O Layout deste form com o nome (como o FindComponent). */
    findLayout(name: string): Layout | undefined {
        switch (name) {
            case "lytListas": return this.lytListas;
            case "lytPainel": return this.lytPainel;
            case "lytBandeiraEstado": return this.lytBandeiraEstado;
            case "lytNumerosEstado": return this.lytNumerosEstado;
            case "lytBandeiraMunicipio": return this.lytBandeiraMunicipio;
            case "lytNumerosMunicipio": return this.lytNumerosMunicipio;
            case "lytGraficos": return this.lytGraficos;
        }
        return undefined;
    }
    /** O Layout que disparou o evento. */
    senderAsLayout(ev: AylisEvent): Layout | undefined {
        return this.findLayout(ev.sender.name);
    }
    /** O Panel deste form com o nome (como o FindComponent). */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlEstado": return this.pnlEstado;
            case "pnlMunicipio": return this.pnlMunicipio;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O Splitter deste form com o nome (como o FindComponent). */
    findSplitter(name: string): Splitter | undefined {
        switch (name) {
            case "splListas": return this.splListas;
        }
        return undefined;
    }
    /** O Splitter que disparou o evento. */
    senderAsSplitter(ev: AylisEvent): Splitter | undefined {
        return this.findSplitter(ev.sender.name);
    }
    /** O Svg deste form com o nome (como o FindComponent). */
    findSvg(name: string): Svg | undefined {
        switch (name) {
            case "svgBrasil": return this.svgBrasil;
            case "svgMunicipio": return this.svgMunicipio;
            case "svgEstado": return this.svgEstado;
        }
        return undefined;
    }
    /** O Svg que disparou o evento. */
    senderAsSvg(ev: AylisEvent): Svg | undefined {
        return this.findSvg(ev.sender.name);
    }
}

/** A janela de frmIBGE cujo evento está rodando (a primeira, fora dos handlers). */
export let frmIBGE: FrmIBGEComponents;
registerFormClass("frmIBGE", FrmIBGEComponents, (form) => {
    frmIBGE = form as FrmIBGEComponents;
});
