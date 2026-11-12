// Os componentes dos forms com as propriedades tipadas (frmX.componente.Propriedade).
// Gerado pela IDE do Aylis a cada "Gerar código": não edite.

import type { ComponentInfo, AylisEvent } from "../aylis.ts";
import { AylisComponent, AylisForm, registerFormClass } from "../aylis.ts";

/** Um Button de um form. */
export class Button extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get icon(): Promise<string> {
        return this.getProp("icon");
    }
    set icon(value: string) {
        this.setProp("icon", value);
    }
    get checkedIcon(): Promise<string> {
        return this.getProp("checked_icon");
    }
    set checkedIcon(value: string) {
        this.setProp("checked_icon", value);
    }
    get iconPosition(): Promise<string> {
        return this.getProp("icon_position");
    }
    set iconPosition(value: string) {
        this.setProp("icon_position", value);
    }
    get iconSize(): Promise<number> {
        return this.getProp("icon_size");
    }
    set iconSize(value: number) {
        this.setProp("icon_size", value);
    }
    get checkable(): Promise<boolean> {
        return this.getProp("checkable");
    }
    set checkable(value: boolean) {
        this.setProp("checkable", value);
    }
    get checked(): Promise<boolean> {
        return this.getProp("checked");
    }
    set checked(value: boolean) {
        this.setProp("checked", value);
    }
    get modalResult(): Promise<string> {
        return this.getProp("modal_result");
    }
    set modalResult(value: string) {
        this.setProp("modal_result", value);
    }
    get repeatEnter(): Promise<boolean> {
        return this.getProp("repeat_enter");
    }
    set repeatEnter(value: boolean) {
        this.setProp("repeat_enter", value);
    }
    get defaultButton(): Promise<boolean> {
        return this.getProp("default_button");
    }
    set defaultButton(value: boolean) {
        this.setProp("default_button", value);
    }
    get cancelButton(): Promise<boolean> {
        return this.getProp("cancel_button");
    }
    set cancelButton(value: boolean) {
        this.setProp("cancel_button", value);
    }
    get hAlign(): Promise<string> {
        return this.getProp("h_align");
    }
    set hAlign(value: string) {
        this.setProp("h_align", value);
    }
    get groupIndex(): Promise<number> {
        return this.getProp("group_index");
    }
    set groupIndex(value: number) {
        this.setProp("group_index", value);
    }
    get radius(): Promise<string> {
        return this.getProp("radius");
    }
    set radius(value: string) {
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
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

/** Um CheckBox de um form. */
export class CheckBox extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get checked(): Promise<boolean> {
        return this.getProp("checked");
    }
    set checked(value: boolean) {
        this.setProp("checked", value);
    }
    get boxSize(): Promise<number> {
        return this.getProp("box_size");
    }
    set boxSize(value: number) {
        this.setProp("box_size", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
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

/** Um Circle de um form. */
export class Circle extends AylisComponent {
    get fill(): Promise<string> {
        return this.getProp("fill");
    }
    set fill(value: string) {
        this.setProp("fill", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um ComboBox de um form. */
export class ComboBox extends AylisComponent {
    get items(): Promise<string> {
        return this.getProp("items");
    }
    set items(value: string) {
        this.setProp("items", value);
    }
    get values(): Promise<string> {
        return this.getProp("values");
    }
    set values(value: string) {
        this.setProp("values", value);
    }
    get itemIndex(): Promise<number> {
        return this.getProp("item_index");
    }
    set itemIndex(value: number) {
        this.setProp("item_index", value);
    }
    get hAlign(): Promise<string> {
        return this.getProp("h_align");
    }
    set hAlign(value: string) {
        this.setProp("h_align", value);
    }
    get trimming(): Promise<string> {
        return this.getProp("trimming");
    }
    set trimming(value: string) {
        this.setProp("trimming", value);
    }
    get arrow(): Promise<string> {
        return this.getProp("arrow");
    }
    set arrow(value: string) {
        this.setProp("arrow", value);
    }
    get sensitive(): Promise<boolean> {
        return this.getProp("sensitive");
    }
    set sensitive(value: boolean) {
        this.setProp("sensitive", value);
    }
    get groups(): Promise<string> {
        return this.getProp("groups");
    }
    set groups(value: string) {
        this.setProp("groups", value);
    }
    get disabledItems(): Promise<string> {
        return this.getProp("disabled_items");
    }
    set disabledItems(value: string) {
        this.setProp("disabled_items", value);
    }
    get value(): Promise<string> {
        return this.getProp("value");
    }
    set value(value: string) {
        this.setProp("value", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
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

/** Um Divider de um form. */
export class Divider extends AylisComponent {
    get vertical(): Promise<boolean> {
        return this.getProp("vertical");
    }
    set vertical(value: boolean) {
        this.setProp("vertical", value);
    }
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get thickness(): Promise<number> {
        return this.getProp("thickness");
    }
    set thickness(value: number) {
        this.setProp("thickness", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
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
    get gpu(): Promise<string> {
        return this.getProp("gpu");
    }
    set gpu(value: string) {
        this.setProp("gpu", value);
    }
    get activateOnShow(): Promise<boolean> {
        return this.getProp("activate_on_show");
    }
    set activateOnShow(value: boolean) {
        this.setProp("activate_on_show", value);
    }
    get closeAction(): Promise<string> {
        return this.getProp("close_action");
    }
    set closeAction(value: string) {
        this.setProp("close_action", value);
    }
    get frameRate(): Promise<number> {
        return this.getProp("frame_rate");
    }
    set frameRate(value: number) {
        this.setProp("frame_rate", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    moveToMonitor(monitor: number) {
        this.aylis.send("invoke", { target: this.id, method: "move_to_monitor", args: [monitor] });
    }
    gpuAdapter(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapter", args: [] }) as Promise<any>;
    }
    gpuAdapters(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "gpu_adapters", args: [] }) as Promise<any>;
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

/** Um GridLayout de um form. */
export class GridLayout extends AylisComponent {
    get columns(): Promise<number> {
        return this.getProp("columns");
    }
    set columns(value: number) {
        this.setProp("columns", value);
    }
    get minColumnWidth(): Promise<number> {
        return this.getProp("min_column_width");
    }
    set minColumnWidth(value: number) {
        this.setProp("min_column_width", value);
    }
    get gap(): Promise<number> {
        return this.getProp("gap");
    }
    set gap(value: number) {
        this.setProp("gap", value);
    }
    get rowHeight(): Promise<number> {
        return this.getProp("row_height");
    }
    set rowHeight(value: number) {
        this.setProp("row_height", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
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
    get lineHeight(): Promise<number> {
        return this.getProp("line_height");
    }
    set lineHeight(value: number) {
        this.setProp("line_height", value);
    }
    get textShadow(): Promise<string> {
        return this.getProp("text_shadow");
    }
    set textShadow(value: string) {
        this.setProp("text_shadow", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    minContentWidth(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "min_content_width", args: [] }) as Promise<any>;
    }
    maxContentWidth(): Promise<number> {
        return this.aylis.call("invoke", { target: this.id, method: "max_content_width", args: [] }) as Promise<any>;
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
    get gap(): Promise<number> {
        return this.getProp("gap");
    }
    set gap(value: number) {
        this.setProp("gap", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um MainMenu de um form. */
export class MainMenu extends AylisComponent {
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um MenuItem de um form. */
export class MenuItem extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
    }
    get shortcut(): Promise<string> {
        return this.getProp("shortcut");
    }
    set shortcut(value: string) {
        this.setProp("shortcut", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get checked(): Promise<boolean> {
        return this.getProp("checked");
    }
    set checked(value: boolean) {
        this.setProp("checked", value);
    }
    get autoCheck(): Promise<boolean> {
        return this.getProp("auto_check");
    }
    set autoCheck(value: boolean) {
        this.setProp("auto_check", value);
    }
    get icon(): Promise<string> {
        return this.getProp("icon");
    }
    set icon(value: string) {
        this.setProp("icon", value);
    }
    get visible(): Promise<boolean> {
        return this.getProp("visible");
    }
    set visible(value: boolean) {
        this.setProp("visible", value);
    }
    get danger(): Promise<boolean> {
        return this.getProp("danger");
    }
    set danger(value: boolean) {
        this.setProp("danger", value);
    }
    get header(): Promise<boolean> {
        return this.getProp("header");
    }
    set header(value: boolean) {
        this.setProp("header", value);
    }
}

/** Um OpenDialog de um form. */
export class OpenDialog extends AylisComponent {
    get title(): Promise<string> {
        return this.getProp("title");
    }
    set title(value: string) {
        this.setProp("title", value);
    }
    get filter(): Promise<string> {
        return this.getProp("filter");
    }
    set filter(value: string) {
        this.setProp("filter", value);
    }
    get initialDir(): Promise<string> {
        return this.getProp("initial_dir");
    }
    set initialDir(value: string) {
        this.setProp("initial_dir", value);
    }
    get fileName(): Promise<string> {
        return this.getProp("file_name");
    }
    set fileName(value: string) {
        this.setProp("file_name", value);
    }
    get multiple(): Promise<boolean> {
        return this.getProp("multiple");
    }
    set multiple(value: boolean) {
        this.setProp("multiple", value);
    }
    execute() {
        this.aylis.send("execute", { target: this.id });
    }
}

/** Um PaintCanvas de um form. */
export class PaintCanvas extends AylisComponent {
    get tool(): Promise<string> {
        return this.getProp("tool");
    }
    set tool(value: string) {
        this.setProp("tool", value);
    }
    get color(): Promise<string> {
        return this.getProp("color");
    }
    set color(value: string) {
        this.setProp("color", value);
    }
    get backColor(): Promise<string> {
        return this.getProp("back_color");
    }
    set backColor(value: string) {
        this.setProp("back_color", value);
    }
    get brushSize(): Promise<number> {
        return this.getProp("brush_size");
    }
    set brushSize(value: number) {
        this.setProp("brush_size", value);
    }
    get filled(): Promise<boolean> {
        return this.getProp("filled");
    }
    set filled(value: boolean) {
        this.setProp("filled", value);
    }
    get zoom(): Promise<number> {
        return this.getProp("zoom");
    }
    set zoom(value: number) {
        this.setProp("zoom", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    undo(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "undo", args: [] }) as Promise<any>;
    }
    redo(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "redo", args: [] }) as Promise<any>;
    }
    canUndo(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "can_undo", args: [] }) as Promise<any>;
    }
    canRedo(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "can_redo", args: [] }) as Promise<any>;
    }
    size(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "size", args: [] }) as Promise<any>;
    }
    checksum(): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "checksum", args: [] }) as Promise<any>;
    }
    clear(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "clear", args: [] }) as Promise<any>;
    }
    resize(width: number, height: number): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "resize", args: [width, height] }) as Promise<any>;
    }
    new(width: number, height: number): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "new", args: [width, height] }) as Promise<any>;
    }
    open(path: string): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "open", args: [path] }) as Promise<any>;
    }
    save(path: string): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "save", args: [path] }) as Promise<any>;
    }
    selectAll(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "select_all", args: [] }) as Promise<any>;
    }
    hasSelection(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "has_selection", args: [] }) as Promise<any>;
    }
    copy(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "copy", args: [] }) as Promise<any>;
    }
    cut(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "cut", args: [] }) as Promise<any>;
    }
    paste(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "paste", args: [] }) as Promise<any>;
    }
    delete(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "delete", args: [] }) as Promise<any>;
    }
    deselect(): Promise<boolean> {
        return this.aylis.call("invoke", { target: this.id, method: "deselect", args: [] }) as Promise<any>;
    }
    setFocus() {
        this.aylis.send("focus", { target: this.id });
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
    get gap(): Promise<number> {
        return this.getProp("gap");
    }
    set gap(value: number) {
        this.setProp("gap", value);
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
    get radius(): Promise<string> {
        return this.getProp("radius");
    }
    set radius(value: string) {
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** Um SaveDialog de um form. */
export class SaveDialog extends AylisComponent {
    get title(): Promise<string> {
        return this.getProp("title");
    }
    set title(value: string) {
        this.setProp("title", value);
    }
    get filter(): Promise<string> {
        return this.getProp("filter");
    }
    set filter(value: string) {
        this.setProp("filter", value);
    }
    get initialDir(): Promise<string> {
        return this.getProp("initial_dir");
    }
    set initialDir(value: string) {
        this.setProp("initial_dir", value);
    }
    get fileName(): Promise<string> {
        return this.getProp("file_name");
    }
    set fileName(value: string) {
        this.setProp("file_name", value);
    }
    get multiple(): Promise<boolean> {
        return this.getProp("multiple");
    }
    set multiple(value: boolean) {
        this.setProp("multiple", value);
    }
    execute() {
        this.aylis.send("execute", { target: this.id });
    }
}

/** Um ScrollBox de um form. */
export class ScrollBox extends AylisComponent {
    get horizontal(): Promise<boolean> {
        return this.getProp("horizontal");
    }
    set horizontal(value: boolean) {
        this.setProp("horizontal", value);
    }
    get pullToRefresh(): Promise<boolean> {
        return this.getProp("pull_to_refresh");
    }
    set pullToRefresh(value: boolean) {
        this.setProp("pull_to_refresh", value);
    }
    get refreshing(): Promise<boolean> {
        return this.getProp("refreshing");
    }
    set refreshing(value: boolean) {
        this.setProp("refreshing", value);
    }
    get wheelStep(): Promise<number> {
        return this.getProp("wheel_step");
    }
    set wheelStep(value: number) {
        this.setProp("wheel_step", value);
    }
    get smooth(): Promise<string> {
        return this.getProp("smooth");
    }
    set smooth(value: string) {
        this.setProp("smooth", value);
    }
    get gutter(): Promise<boolean> {
        return this.getProp("gutter");
    }
    set gutter(value: boolean) {
        this.setProp("gutter", value);
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
    get scrollPosition(): Promise<number> {
        return this.getProp("scroll_position");
    }
    set scrollPosition(value: number) {
        this.setProp("scroll_position", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
    scrollTo(component: AylisComponent) {
        this.aylis.send("invoke", { target: this.id, method: "scroll_to", args: [component.id] });
    }
    visibleComponents(list: AylisComponent, fully: boolean): Promise<string> {
        return this.aylis.call("invoke", { target: this.id, method: "visible_components", args: [list.id, fully] }) as Promise<any>;
    }
}

/** Um StatusBar de um form. */
export class StatusBar extends AylisComponent {
    get text(): Promise<string> {
        return this.getProp("text");
    }
    set text(value: string) {
        this.setProp("text", value);
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
    get anchors(): Promise<string> {
        return this.getProp("anchors");
    }
    set anchors(value: string) {
        this.setProp("anchors", value);
    }
    get margins(): Promise<string> {
        return this.getProp("margins");
    }
    set margins(value: string) {
        this.setProp("margins", value);
    }
    get padding(): Promise<string> {
        return this.getProp("padding");
    }
    set padding(value: string) {
        this.setProp("padding", value);
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
    get inheritState(): Promise<boolean> {
        return this.getProp("inherit_state");
    }
    set inheritState(value: boolean) {
        this.setProp("inherit_state", value);
    }
    get translate(): Promise<boolean> {
        return this.getProp("translate");
    }
    set translate(value: boolean) {
        this.setProp("translate", value);
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
    get gridRow(): Promise<number> {
        return this.getProp("grid_row");
    }
    set gridRow(value: number) {
        this.setProp("grid_row", value);
    }
    get gridColumn(): Promise<number> {
        return this.getProp("grid_column");
    }
    set gridColumn(value: number) {
        this.setProp("grid_column", value);
    }
    get rowSpan(): Promise<number> {
        return this.getProp("row_span");
    }
    set rowSpan(value: number) {
        this.setProp("row_span", value);
    }
    get grow(): Promise<number> {
        return this.getProp("grow");
    }
    set grow(value: number) {
        this.setProp("grow", value);
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
    get tagString(): Promise<string> {
        return this.getProp("tag_string");
    }
    set tagString(value: string) {
        this.setProp("tag_string", value);
    }
    get accessibleName(): Promise<string> {
        return this.getProp("accessible_name");
    }
    set accessibleName(value: string) {
        this.setProp("accessible_name", value);
    }
    get popupMenu(): Promise<string> {
        return this.getProp("popup_menu");
    }
    set popupMenu(value: string) {
        this.setProp("popup_menu", value);
    }
}

/** O form frmPaint e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmPaintComponents extends Form {
    lytTopo!: Layout;
    mnuPrincipal!: MainMenu;
    mnuArquivo!: MenuItem;
    mnuNovo!: MenuItem;
    mnuAbrir!: MenuItem;
    mnuSalvar!: MenuItem;
    mnuSalvarComo!: MenuItem;
    mnuSep1!: MenuItem;
    mnuSair!: MenuItem;
    mnuEditar!: MenuItem;
    mnuDesfazer!: MenuItem;
    mnuRefazer!: MenuItem;
    mnuSep2!: MenuItem;
    mnuRecortar!: MenuItem;
    mnuCopiar!: MenuItem;
    mnuColar!: MenuItem;
    mnuExcluir!: MenuItem;
    mnuSelecionarTudo!: MenuItem;
    mnuSep3!: MenuItem;
    mnuLimpar!: MenuItem;
    mnuExibir!: MenuItem;
    mnuAmpliar!: MenuItem;
    mnuReduzir!: MenuItem;
    mnuTamanhoReal!: MenuItem;
    divTopo!: Divider;
    btnSalvar!: Button;
    btnDesfazer!: Button;
    btnRefazer!: Button;
    pnlFaixa!: Panel;
    lytSelecao!: Layout;
    lblSelecaoGrupo!: Label;
    btnSelecionar!: Button;
    divSelecao!: Divider;
    lytFerramentas!: Layout;
    lblFerramentasGrupo!: Label;
    grdFerramentas!: GridLayout;
    btnLapis!: Button;
    btnBalde!: Button;
    btnBorracha!: Button;
    btnContaGotas!: Button;
    divFerramentas!: Divider;
    lytPinceis!: Layout;
    lblPinceis!: Label;
    btnPincel!: Button;
    divPinceis!: Divider;
    lytFormas!: Layout;
    lblFormas!: Label;
    chkPreencher!: CheckBox;
    grdFormas!: GridLayout;
    btnLinha!: Button;
    btnRetangulo!: Button;
    btnElipse!: Button;
    divFormas!: Divider;
    lytTamanho!: Layout;
    lblTamanhoGrupo!: Label;
    cbEspessura!: ComboBox;
    divTamanho!: Divider;
    lytCores!: Layout;
    lblCores!: Label;
    btnCor1!: Circle;
    btnCor2!: Circle;
    grdPaleta!: GridLayout;
    btnPaleta01!: Circle;
    btnPaleta02!: Circle;
    btnPaleta03!: Circle;
    btnPaleta04!: Circle;
    btnPaleta05!: Circle;
    btnPaleta06!: Circle;
    btnPaleta07!: Circle;
    btnPaleta08!: Circle;
    btnPaleta09!: Circle;
    btnPaleta10!: Circle;
    btnPaleta11!: Circle;
    btnPaleta12!: Circle;
    btnPaleta13!: Circle;
    btnPaleta14!: Circle;
    btnPaleta15!: Circle;
    btnPaleta16!: Circle;
    btnPaleta17!: Circle;
    btnPaleta18!: Circle;
    btnPaleta19!: Circle;
    btnPaleta20!: Circle;
    stbStatus!: StatusBar;
    lblPosicao!: Label;
    lblTamanho!: Label;
    lblZoom!: Label;
    sbxArea!: ScrollBox;
    pcDesenho!: PaintCanvas;
    dlgAbrir!: OpenDialog;
    dlgSalvar!: SaveDialog;
    bind(list: ComponentInfo[]) {
        this.lytTopo = this.resolve(list, "lytTopo", "Layout", Layout);
        this.mnuPrincipal = this.resolve(list, "mnuPrincipal", "MainMenu", MainMenu);
        this.mnuArquivo = this.resolve(list, "mnuArquivo", "MenuItem", MenuItem);
        this.mnuNovo = this.resolve(list, "mnuNovo", "MenuItem", MenuItem);
        this.mnuAbrir = this.resolve(list, "mnuAbrir", "MenuItem", MenuItem);
        this.mnuSalvar = this.resolve(list, "mnuSalvar", "MenuItem", MenuItem);
        this.mnuSalvarComo = this.resolve(list, "mnuSalvarComo", "MenuItem", MenuItem);
        this.mnuSep1 = this.resolve(list, "mnuSep1", "MenuItem", MenuItem);
        this.mnuSair = this.resolve(list, "mnuSair", "MenuItem", MenuItem);
        this.mnuEditar = this.resolve(list, "mnuEditar", "MenuItem", MenuItem);
        this.mnuDesfazer = this.resolve(list, "mnuDesfazer", "MenuItem", MenuItem);
        this.mnuRefazer = this.resolve(list, "mnuRefazer", "MenuItem", MenuItem);
        this.mnuSep2 = this.resolve(list, "mnuSep2", "MenuItem", MenuItem);
        this.mnuRecortar = this.resolve(list, "mnuRecortar", "MenuItem", MenuItem);
        this.mnuCopiar = this.resolve(list, "mnuCopiar", "MenuItem", MenuItem);
        this.mnuColar = this.resolve(list, "mnuColar", "MenuItem", MenuItem);
        this.mnuExcluir = this.resolve(list, "mnuExcluir", "MenuItem", MenuItem);
        this.mnuSelecionarTudo = this.resolve(list, "mnuSelecionarTudo", "MenuItem", MenuItem);
        this.mnuSep3 = this.resolve(list, "mnuSep3", "MenuItem", MenuItem);
        this.mnuLimpar = this.resolve(list, "mnuLimpar", "MenuItem", MenuItem);
        this.mnuExibir = this.resolve(list, "mnuExibir", "MenuItem", MenuItem);
        this.mnuAmpliar = this.resolve(list, "mnuAmpliar", "MenuItem", MenuItem);
        this.mnuReduzir = this.resolve(list, "mnuReduzir", "MenuItem", MenuItem);
        this.mnuTamanhoReal = this.resolve(list, "mnuTamanhoReal", "MenuItem", MenuItem);
        this.divTopo = this.resolve(list, "divTopo", "Divider", Divider);
        this.btnSalvar = this.resolve(list, "btnSalvar", "Button", Button);
        this.btnDesfazer = this.resolve(list, "btnDesfazer", "Button", Button);
        this.btnRefazer = this.resolve(list, "btnRefazer", "Button", Button);
        this.pnlFaixa = this.resolve(list, "pnlFaixa", "Panel", Panel);
        this.lytSelecao = this.resolve(list, "lytSelecao", "Layout", Layout);
        this.lblSelecaoGrupo = this.resolve(list, "lblSelecaoGrupo", "Label", Label);
        this.btnSelecionar = this.resolve(list, "btnSelecionar", "Button", Button);
        this.divSelecao = this.resolve(list, "divSelecao", "Divider", Divider);
        this.lytFerramentas = this.resolve(list, "lytFerramentas", "Layout", Layout);
        this.lblFerramentasGrupo = this.resolve(list, "lblFerramentasGrupo", "Label", Label);
        this.grdFerramentas = this.resolve(list, "grdFerramentas", "GridLayout", GridLayout);
        this.btnLapis = this.resolve(list, "btnLapis", "Button", Button);
        this.btnBalde = this.resolve(list, "btnBalde", "Button", Button);
        this.btnBorracha = this.resolve(list, "btnBorracha", "Button", Button);
        this.btnContaGotas = this.resolve(list, "btnContaGotas", "Button", Button);
        this.divFerramentas = this.resolve(list, "divFerramentas", "Divider", Divider);
        this.lytPinceis = this.resolve(list, "lytPinceis", "Layout", Layout);
        this.lblPinceis = this.resolve(list, "lblPinceis", "Label", Label);
        this.btnPincel = this.resolve(list, "btnPincel", "Button", Button);
        this.divPinceis = this.resolve(list, "divPinceis", "Divider", Divider);
        this.lytFormas = this.resolve(list, "lytFormas", "Layout", Layout);
        this.lblFormas = this.resolve(list, "lblFormas", "Label", Label);
        this.chkPreencher = this.resolve(list, "chkPreencher", "CheckBox", CheckBox);
        this.grdFormas = this.resolve(list, "grdFormas", "GridLayout", GridLayout);
        this.btnLinha = this.resolve(list, "btnLinha", "Button", Button);
        this.btnRetangulo = this.resolve(list, "btnRetangulo", "Button", Button);
        this.btnElipse = this.resolve(list, "btnElipse", "Button", Button);
        this.divFormas = this.resolve(list, "divFormas", "Divider", Divider);
        this.lytTamanho = this.resolve(list, "lytTamanho", "Layout", Layout);
        this.lblTamanhoGrupo = this.resolve(list, "lblTamanhoGrupo", "Label", Label);
        this.cbEspessura = this.resolve(list, "cbEspessura", "ComboBox", ComboBox);
        this.divTamanho = this.resolve(list, "divTamanho", "Divider", Divider);
        this.lytCores = this.resolve(list, "lytCores", "Layout", Layout);
        this.lblCores = this.resolve(list, "lblCores", "Label", Label);
        this.btnCor1 = this.resolve(list, "btnCor1", "Circle", Circle);
        this.btnCor2 = this.resolve(list, "btnCor2", "Circle", Circle);
        this.grdPaleta = this.resolve(list, "grdPaleta", "GridLayout", GridLayout);
        this.btnPaleta01 = this.resolve(list, "btnPaleta01", "Circle", Circle);
        this.btnPaleta02 = this.resolve(list, "btnPaleta02", "Circle", Circle);
        this.btnPaleta03 = this.resolve(list, "btnPaleta03", "Circle", Circle);
        this.btnPaleta04 = this.resolve(list, "btnPaleta04", "Circle", Circle);
        this.btnPaleta05 = this.resolve(list, "btnPaleta05", "Circle", Circle);
        this.btnPaleta06 = this.resolve(list, "btnPaleta06", "Circle", Circle);
        this.btnPaleta07 = this.resolve(list, "btnPaleta07", "Circle", Circle);
        this.btnPaleta08 = this.resolve(list, "btnPaleta08", "Circle", Circle);
        this.btnPaleta09 = this.resolve(list, "btnPaleta09", "Circle", Circle);
        this.btnPaleta10 = this.resolve(list, "btnPaleta10", "Circle", Circle);
        this.btnPaleta11 = this.resolve(list, "btnPaleta11", "Circle", Circle);
        this.btnPaleta12 = this.resolve(list, "btnPaleta12", "Circle", Circle);
        this.btnPaleta13 = this.resolve(list, "btnPaleta13", "Circle", Circle);
        this.btnPaleta14 = this.resolve(list, "btnPaleta14", "Circle", Circle);
        this.btnPaleta15 = this.resolve(list, "btnPaleta15", "Circle", Circle);
        this.btnPaleta16 = this.resolve(list, "btnPaleta16", "Circle", Circle);
        this.btnPaleta17 = this.resolve(list, "btnPaleta17", "Circle", Circle);
        this.btnPaleta18 = this.resolve(list, "btnPaleta18", "Circle", Circle);
        this.btnPaleta19 = this.resolve(list, "btnPaleta19", "Circle", Circle);
        this.btnPaleta20 = this.resolve(list, "btnPaleta20", "Circle", Circle);
        this.stbStatus = this.resolve(list, "stbStatus", "StatusBar", StatusBar);
        this.lblPosicao = this.resolve(list, "lblPosicao", "Label", Label);
        this.lblTamanho = this.resolve(list, "lblTamanho", "Label", Label);
        this.lblZoom = this.resolve(list, "lblZoom", "Label", Label);
        this.sbxArea = this.resolve(list, "sbxArea", "ScrollBox", ScrollBox);
        this.pcDesenho = this.resolve(list, "pcDesenho", "PaintCanvas", PaintCanvas);
        this.dlgAbrir = this.resolve(list, "dlgAbrir", "OpenDialog", OpenDialog);
        this.dlgSalvar = this.resolve(list, "dlgSalvar", "SaveDialog", SaveDialog);
    }
    /** O Button deste form com o nome (como o FindComponent). */
    findButton(name: string): Button | undefined {
        switch (name) {
            case "btnSalvar": return this.btnSalvar;
            case "btnDesfazer": return this.btnDesfazer;
            case "btnRefazer": return this.btnRefazer;
            case "btnSelecionar": return this.btnSelecionar;
            case "btnLapis": return this.btnLapis;
            case "btnBalde": return this.btnBalde;
            case "btnBorracha": return this.btnBorracha;
            case "btnContaGotas": return this.btnContaGotas;
            case "btnPincel": return this.btnPincel;
            case "btnLinha": return this.btnLinha;
            case "btnRetangulo": return this.btnRetangulo;
            case "btnElipse": return this.btnElipse;
        }
        return undefined;
    }
    /** O Button que disparou o evento. */
    senderAsButton(ev: AylisEvent): Button | undefined {
        return this.findButton(ev.sender.name);
    }
    /** O CheckBox deste form com o nome (como o FindComponent). */
    findCheckBox(name: string): CheckBox | undefined {
        switch (name) {
            case "chkPreencher": return this.chkPreencher;
        }
        return undefined;
    }
    /** O CheckBox que disparou o evento. */
    senderAsCheckBox(ev: AylisEvent): CheckBox | undefined {
        return this.findCheckBox(ev.sender.name);
    }
    /** O Circle deste form com o nome (como o FindComponent). */
    findCircle(name: string): Circle | undefined {
        switch (name) {
            case "btnCor1": return this.btnCor1;
            case "btnCor2": return this.btnCor2;
            case "btnPaleta01": return this.btnPaleta01;
            case "btnPaleta02": return this.btnPaleta02;
            case "btnPaleta03": return this.btnPaleta03;
            case "btnPaleta04": return this.btnPaleta04;
            case "btnPaleta05": return this.btnPaleta05;
            case "btnPaleta06": return this.btnPaleta06;
            case "btnPaleta07": return this.btnPaleta07;
            case "btnPaleta08": return this.btnPaleta08;
            case "btnPaleta09": return this.btnPaleta09;
            case "btnPaleta10": return this.btnPaleta10;
            case "btnPaleta11": return this.btnPaleta11;
            case "btnPaleta12": return this.btnPaleta12;
            case "btnPaleta13": return this.btnPaleta13;
            case "btnPaleta14": return this.btnPaleta14;
            case "btnPaleta15": return this.btnPaleta15;
            case "btnPaleta16": return this.btnPaleta16;
            case "btnPaleta17": return this.btnPaleta17;
            case "btnPaleta18": return this.btnPaleta18;
            case "btnPaleta19": return this.btnPaleta19;
            case "btnPaleta20": return this.btnPaleta20;
        }
        return undefined;
    }
    /** O Circle que disparou o evento. */
    senderAsCircle(ev: AylisEvent): Circle | undefined {
        return this.findCircle(ev.sender.name);
    }
    /** O ComboBox deste form com o nome (como o FindComponent). */
    findComboBox(name: string): ComboBox | undefined {
        switch (name) {
            case "cbEspessura": return this.cbEspessura;
        }
        return undefined;
    }
    /** O ComboBox que disparou o evento. */
    senderAsComboBox(ev: AylisEvent): ComboBox | undefined {
        return this.findComboBox(ev.sender.name);
    }
    /** O Divider deste form com o nome (como o FindComponent). */
    findDivider(name: string): Divider | undefined {
        switch (name) {
            case "divTopo": return this.divTopo;
            case "divSelecao": return this.divSelecao;
            case "divFerramentas": return this.divFerramentas;
            case "divPinceis": return this.divPinceis;
            case "divFormas": return this.divFormas;
            case "divTamanho": return this.divTamanho;
        }
        return undefined;
    }
    /** O Divider que disparou o evento. */
    senderAsDivider(ev: AylisEvent): Divider | undefined {
        return this.findDivider(ev.sender.name);
    }
    /** O GridLayout deste form com o nome (como o FindComponent). */
    findGridLayout(name: string): GridLayout | undefined {
        switch (name) {
            case "grdFerramentas": return this.grdFerramentas;
            case "grdFormas": return this.grdFormas;
            case "grdPaleta": return this.grdPaleta;
        }
        return undefined;
    }
    /** O GridLayout que disparou o evento. */
    senderAsGridLayout(ev: AylisEvent): GridLayout | undefined {
        return this.findGridLayout(ev.sender.name);
    }
    /** O Label deste form com o nome (como o FindComponent). */
    findLabel(name: string): Label | undefined {
        switch (name) {
            case "lblSelecaoGrupo": return this.lblSelecaoGrupo;
            case "lblFerramentasGrupo": return this.lblFerramentasGrupo;
            case "lblPinceis": return this.lblPinceis;
            case "lblFormas": return this.lblFormas;
            case "lblTamanhoGrupo": return this.lblTamanhoGrupo;
            case "lblCores": return this.lblCores;
            case "lblPosicao": return this.lblPosicao;
            case "lblTamanho": return this.lblTamanho;
            case "lblZoom": return this.lblZoom;
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
            case "lytTopo": return this.lytTopo;
            case "lytSelecao": return this.lytSelecao;
            case "lytFerramentas": return this.lytFerramentas;
            case "lytPinceis": return this.lytPinceis;
            case "lytFormas": return this.lytFormas;
            case "lytTamanho": return this.lytTamanho;
            case "lytCores": return this.lytCores;
        }
        return undefined;
    }
    /** O Layout que disparou o evento. */
    senderAsLayout(ev: AylisEvent): Layout | undefined {
        return this.findLayout(ev.sender.name);
    }
    /** O MainMenu deste form com o nome (como o FindComponent). */
    findMainMenu(name: string): MainMenu | undefined {
        switch (name) {
            case "mnuPrincipal": return this.mnuPrincipal;
        }
        return undefined;
    }
    /** O MainMenu que disparou o evento. */
    senderAsMainMenu(ev: AylisEvent): MainMenu | undefined {
        return this.findMainMenu(ev.sender.name);
    }
    /** O MenuItem deste form com o nome (como o FindComponent). */
    findMenuItem(name: string): MenuItem | undefined {
        switch (name) {
            case "mnuArquivo": return this.mnuArquivo;
            case "mnuNovo": return this.mnuNovo;
            case "mnuAbrir": return this.mnuAbrir;
            case "mnuSalvar": return this.mnuSalvar;
            case "mnuSalvarComo": return this.mnuSalvarComo;
            case "mnuSep1": return this.mnuSep1;
            case "mnuSair": return this.mnuSair;
            case "mnuEditar": return this.mnuEditar;
            case "mnuDesfazer": return this.mnuDesfazer;
            case "mnuRefazer": return this.mnuRefazer;
            case "mnuSep2": return this.mnuSep2;
            case "mnuRecortar": return this.mnuRecortar;
            case "mnuCopiar": return this.mnuCopiar;
            case "mnuColar": return this.mnuColar;
            case "mnuExcluir": return this.mnuExcluir;
            case "mnuSelecionarTudo": return this.mnuSelecionarTudo;
            case "mnuSep3": return this.mnuSep3;
            case "mnuLimpar": return this.mnuLimpar;
            case "mnuExibir": return this.mnuExibir;
            case "mnuAmpliar": return this.mnuAmpliar;
            case "mnuReduzir": return this.mnuReduzir;
            case "mnuTamanhoReal": return this.mnuTamanhoReal;
        }
        return undefined;
    }
    /** O MenuItem que disparou o evento. */
    senderAsMenuItem(ev: AylisEvent): MenuItem | undefined {
        return this.findMenuItem(ev.sender.name);
    }
    /** O OpenDialog deste form com o nome (como o FindComponent). */
    findOpenDialog(name: string): OpenDialog | undefined {
        switch (name) {
            case "dlgAbrir": return this.dlgAbrir;
        }
        return undefined;
    }
    /** O OpenDialog que disparou o evento. */
    senderAsOpenDialog(ev: AylisEvent): OpenDialog | undefined {
        return this.findOpenDialog(ev.sender.name);
    }
    /** O PaintCanvas deste form com o nome (como o FindComponent). */
    findPaintCanvas(name: string): PaintCanvas | undefined {
        switch (name) {
            case "pcDesenho": return this.pcDesenho;
        }
        return undefined;
    }
    /** O PaintCanvas que disparou o evento. */
    senderAsPaintCanvas(ev: AylisEvent): PaintCanvas | undefined {
        return this.findPaintCanvas(ev.sender.name);
    }
    /** O Panel deste form com o nome (como o FindComponent). */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlFaixa": return this.pnlFaixa;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O SaveDialog deste form com o nome (como o FindComponent). */
    findSaveDialog(name: string): SaveDialog | undefined {
        switch (name) {
            case "dlgSalvar": return this.dlgSalvar;
        }
        return undefined;
    }
    /** O SaveDialog que disparou o evento. */
    senderAsSaveDialog(ev: AylisEvent): SaveDialog | undefined {
        return this.findSaveDialog(ev.sender.name);
    }
    /** O ScrollBox deste form com o nome (como o FindComponent). */
    findScrollBox(name: string): ScrollBox | undefined {
        switch (name) {
            case "sbxArea": return this.sbxArea;
        }
        return undefined;
    }
    /** O ScrollBox que disparou o evento. */
    senderAsScrollBox(ev: AylisEvent): ScrollBox | undefined {
        return this.findScrollBox(ev.sender.name);
    }
    /** O StatusBar deste form com o nome (como o FindComponent). */
    findStatusBar(name: string): StatusBar | undefined {
        switch (name) {
            case "stbStatus": return this.stbStatus;
        }
        return undefined;
    }
    /** O StatusBar que disparou o evento. */
    senderAsStatusBar(ev: AylisEvent): StatusBar | undefined {
        return this.findStatusBar(ev.sender.name);
    }
}

/** A janela de frmPaint cujo evento está rodando (a primeira, fora dos handlers). */
export let frmPaint: FrmPaintComponents;
registerFormClass("frmPaint", FrmPaintComponents, (form) => {
    frmPaint = form as FrmPaintComponents;
});
