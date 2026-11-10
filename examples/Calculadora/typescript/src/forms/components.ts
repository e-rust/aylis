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

/** Um ListView de um form. */
export class ListView extends AylisComponent {
    get items(): Promise<string> {
        return this.getProp("items");
    }
    set items(value: string) {
        this.setProp("items", value);
    }
    get itemIndex(): Promise<number> {
        return this.getProp("item_index");
    }
    set itemIndex(value: number) {
        this.setProp("item_index", value);
    }
    get itemHeight(): Promise<number> {
        return this.getProp("item_height");
    }
    set itemHeight(value: number) {
        this.setProp("item_height", value);
    }
    get accessory(): Promise<boolean> {
        return this.getProp("accessory");
    }
    set accessory(value: boolean) {
        this.setProp("accessory", value);
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

/** Um TabControl de um form. */
export class TabControl extends AylisComponent {
    get tabIndex(): Promise<number> {
        return this.getProp("tab_index");
    }
    set tabIndex(value: number) {
        this.setProp("tab_index", value);
    }
    get showTabs(): Promise<boolean> {
        return this.getProp("show_tabs");
    }
    set showTabs(value: boolean) {
        this.setProp("show_tabs", value);
    }
    get transition(): Promise<string> {
        return this.getProp("transition");
    }
    set transition(value: string) {
        this.setProp("transition", value);
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

/** Um TabItem de um form. */
export class TabItem extends AylisComponent {
    get caption(): Promise<string> {
        return this.getProp("caption");
    }
    set caption(value: string) {
        this.setProp("caption", value);
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

/** O form frmCalculadora e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmCalculadoraComponents extends Form {
    pnlLateral!: Panel;
    tabLateral!: TabControl;
    tabHistorico!: TabItem;
    btnLimparHistorico!: Button;
    lvHistorico!: ListView;
    tabMemoria!: TabItem;
    lvMemoria!: ListView;
    lytCorpo!: Layout;
    lytTopo!: Layout;
    btnHistorico!: Button;
    lblTitulo!: Label;
    lblExpressao!: Label;
    lblVisor!: Label;
    grdMemoria!: GridLayout;
    btnMC!: Button;
    btnMR!: Button;
    btnMMais!: Button;
    btnMMenos!: Button;
    btnMS!: Button;
    grdTeclado!: GridLayout;
    btnPorcento!: Button;
    btnCE!: Button;
    btnC!: Button;
    btnApagar!: Button;
    btnInverso!: Button;
    btnQuadrado!: Button;
    btnRaiz!: Button;
    btnDividir!: Button;
    btn7!: Button;
    btn8!: Button;
    btn9!: Button;
    btnMultiplicar!: Button;
    btn4!: Button;
    btn5!: Button;
    btn6!: Button;
    btnSubtrair!: Button;
    btn1!: Button;
    btn2!: Button;
    btn3!: Button;
    btnSomar!: Button;
    btnSinal!: Button;
    btn0!: Button;
    btnVirgula!: Button;
    btnIgual!: Button;
    bind(list: ComponentInfo[]) {
        this.pnlLateral = this.resolve(list, "pnlLateral", "Panel", Panel);
        this.tabLateral = this.resolve(list, "tabLateral", "TabControl", TabControl);
        this.tabHistorico = this.resolve(list, "tabHistorico", "TabItem", TabItem);
        this.btnLimparHistorico = this.resolve(list, "btnLimparHistorico", "Button", Button);
        this.lvHistorico = this.resolve(list, "lvHistorico", "ListView", ListView);
        this.tabMemoria = this.resolve(list, "tabMemoria", "TabItem", TabItem);
        this.lvMemoria = this.resolve(list, "lvMemoria", "ListView", ListView);
        this.lytCorpo = this.resolve(list, "lytCorpo", "Layout", Layout);
        this.lytTopo = this.resolve(list, "lytTopo", "Layout", Layout);
        this.btnHistorico = this.resolve(list, "btnHistorico", "Button", Button);
        this.lblTitulo = this.resolve(list, "lblTitulo", "Label", Label);
        this.lblExpressao = this.resolve(list, "lblExpressao", "Label", Label);
        this.lblVisor = this.resolve(list, "lblVisor", "Label", Label);
        this.grdMemoria = this.resolve(list, "grdMemoria", "GridLayout", GridLayout);
        this.btnMC = this.resolve(list, "btnMC", "Button", Button);
        this.btnMR = this.resolve(list, "btnMR", "Button", Button);
        this.btnMMais = this.resolve(list, "btnMMais", "Button", Button);
        this.btnMMenos = this.resolve(list, "btnMMenos", "Button", Button);
        this.btnMS = this.resolve(list, "btnMS", "Button", Button);
        this.grdTeclado = this.resolve(list, "grdTeclado", "GridLayout", GridLayout);
        this.btnPorcento = this.resolve(list, "btnPorcento", "Button", Button);
        this.btnCE = this.resolve(list, "btnCE", "Button", Button);
        this.btnC = this.resolve(list, "btnC", "Button", Button);
        this.btnApagar = this.resolve(list, "btnApagar", "Button", Button);
        this.btnInverso = this.resolve(list, "btnInverso", "Button", Button);
        this.btnQuadrado = this.resolve(list, "btnQuadrado", "Button", Button);
        this.btnRaiz = this.resolve(list, "btnRaiz", "Button", Button);
        this.btnDividir = this.resolve(list, "btnDividir", "Button", Button);
        this.btn7 = this.resolve(list, "btn7", "Button", Button);
        this.btn8 = this.resolve(list, "btn8", "Button", Button);
        this.btn9 = this.resolve(list, "btn9", "Button", Button);
        this.btnMultiplicar = this.resolve(list, "btnMultiplicar", "Button", Button);
        this.btn4 = this.resolve(list, "btn4", "Button", Button);
        this.btn5 = this.resolve(list, "btn5", "Button", Button);
        this.btn6 = this.resolve(list, "btn6", "Button", Button);
        this.btnSubtrair = this.resolve(list, "btnSubtrair", "Button", Button);
        this.btn1 = this.resolve(list, "btn1", "Button", Button);
        this.btn2 = this.resolve(list, "btn2", "Button", Button);
        this.btn3 = this.resolve(list, "btn3", "Button", Button);
        this.btnSomar = this.resolve(list, "btnSomar", "Button", Button);
        this.btnSinal = this.resolve(list, "btnSinal", "Button", Button);
        this.btn0 = this.resolve(list, "btn0", "Button", Button);
        this.btnVirgula = this.resolve(list, "btnVirgula", "Button", Button);
        this.btnIgual = this.resolve(list, "btnIgual", "Button", Button);
    }
    /** O Button deste form com o nome (como o FindComponent). */
    findButton(name: string): Button | undefined {
        switch (name) {
            case "btnLimparHistorico": return this.btnLimparHistorico;
            case "btnHistorico": return this.btnHistorico;
            case "btnMC": return this.btnMC;
            case "btnMR": return this.btnMR;
            case "btnMMais": return this.btnMMais;
            case "btnMMenos": return this.btnMMenos;
            case "btnMS": return this.btnMS;
            case "btnPorcento": return this.btnPorcento;
            case "btnCE": return this.btnCE;
            case "btnC": return this.btnC;
            case "btnApagar": return this.btnApagar;
            case "btnInverso": return this.btnInverso;
            case "btnQuadrado": return this.btnQuadrado;
            case "btnRaiz": return this.btnRaiz;
            case "btnDividir": return this.btnDividir;
            case "btn7": return this.btn7;
            case "btn8": return this.btn8;
            case "btn9": return this.btn9;
            case "btnMultiplicar": return this.btnMultiplicar;
            case "btn4": return this.btn4;
            case "btn5": return this.btn5;
            case "btn6": return this.btn6;
            case "btnSubtrair": return this.btnSubtrair;
            case "btn1": return this.btn1;
            case "btn2": return this.btn2;
            case "btn3": return this.btn3;
            case "btnSomar": return this.btnSomar;
            case "btnSinal": return this.btnSinal;
            case "btn0": return this.btn0;
            case "btnVirgula": return this.btnVirgula;
            case "btnIgual": return this.btnIgual;
        }
        return undefined;
    }
    /** O Button que disparou o evento. */
    senderAsButton(ev: AylisEvent): Button | undefined {
        return this.findButton(ev.sender.name);
    }
    /** O GridLayout deste form com o nome (como o FindComponent). */
    findGridLayout(name: string): GridLayout | undefined {
        switch (name) {
            case "grdMemoria": return this.grdMemoria;
            case "grdTeclado": return this.grdTeclado;
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
            case "lblTitulo": return this.lblTitulo;
            case "lblExpressao": return this.lblExpressao;
            case "lblVisor": return this.lblVisor;
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
            case "lytCorpo": return this.lytCorpo;
            case "lytTopo": return this.lytTopo;
        }
        return undefined;
    }
    /** O Layout que disparou o evento. */
    senderAsLayout(ev: AylisEvent): Layout | undefined {
        return this.findLayout(ev.sender.name);
    }
    /** O ListView deste form com o nome (como o FindComponent). */
    findListView(name: string): ListView | undefined {
        switch (name) {
            case "lvHistorico": return this.lvHistorico;
            case "lvMemoria": return this.lvMemoria;
        }
        return undefined;
    }
    /** O ListView que disparou o evento. */
    senderAsListView(ev: AylisEvent): ListView | undefined {
        return this.findListView(ev.sender.name);
    }
    /** O Panel deste form com o nome (como o FindComponent). */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlLateral": return this.pnlLateral;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O TabControl deste form com o nome (como o FindComponent). */
    findTabControl(name: string): TabControl | undefined {
        switch (name) {
            case "tabLateral": return this.tabLateral;
        }
        return undefined;
    }
    /** O TabControl que disparou o evento. */
    senderAsTabControl(ev: AylisEvent): TabControl | undefined {
        return this.findTabControl(ev.sender.name);
    }
    /** O TabItem deste form com o nome (como o FindComponent). */
    findTabItem(name: string): TabItem | undefined {
        switch (name) {
            case "tabHistorico": return this.tabHistorico;
            case "tabMemoria": return this.tabMemoria;
        }
        return undefined;
    }
    /** O TabItem que disparou o evento. */
    senderAsTabItem(ev: AylisEvent): TabItem | undefined {
        return this.findTabItem(ev.sender.name);
    }
}

/** A janela de frmCalculadora cujo evento está rodando (a primeira, fora dos handlers). */
export let frmCalculadora: FrmCalculadoraComponents;
registerFormClass("frmCalculadora", FrmCalculadoraComponents, (form) => {
    frmCalculadora = form as FrmCalculadoraComponents;
});
