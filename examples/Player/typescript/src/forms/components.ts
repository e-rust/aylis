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

/** Um Icon de um form. */
export class Icon extends AylisComponent {
    get icon(): Promise<string> {
        return this.getProp("icon");
    }
    set icon(value: string) {
        this.setProp("icon", value);
    }
    get color(): Promise<string> {
        return this.getProp("color");
    }
    set color(value: string) {
        this.setProp("color", value);
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

/** Um SeekBar de um form. */
export class SeekBar extends AylisComponent {
    get position(): Promise<number> {
        return this.getProp("position");
    }
    set position(value: number) {
        this.setProp("position", value);
    }
    get duration(): Promise<number> {
        return this.getProp("duration");
    }
    set duration(value: number) {
        this.setProp("duration", value);
    }
    get buffered(): Promise<number> {
        return this.getProp("buffered");
    }
    set buffered(value: number) {
        this.setProp("buffered", value);
    }
    get markers(): Promise<string> {
        return this.getProp("markers");
    }
    set markers(value: string) {
        this.setProp("markers", value);
    }
    get showTime(): Promise<boolean> {
        return this.getProp("show_time");
    }
    set showTime(value: boolean) {
        this.setProp("show_time", value);
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

/** Um Timer de um form. */
export class Timer extends AylisComponent {
    get interval(): Promise<number> {
        return this.getProp("interval");
    }
    set interval(value: number) {
        this.setProp("interval", value);
    }
    get enabled(): Promise<boolean> {
        return this.getProp("enabled");
    }
    set enabled(value: boolean) {
        this.setProp("enabled", value);
    }
    get precision(): Promise<string> {
        return this.getProp("precision");
    }
    set precision(value: string) {
        this.setProp("precision", value);
    }
    get background(): Promise<boolean> {
        return this.getProp("background");
    }
    set background(value: boolean) {
        this.setProp("background", value);
    }
    get drift(): Promise<number> {
        return this.getProp("drift");
    }
    set drift(value: number) {
        this.setProp("drift", value);
    }
}

/** Um TrackBar de um form. */
export class TrackBar extends AylisComponent {
    get value(): Promise<number> {
        return this.getProp("value");
    }
    set value(value: number) {
        this.setProp("value", value);
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
    get frequency(): Promise<number> {
        return this.getProp("frequency");
    }
    set frequency(value: number) {
        this.setProp("frequency", value);
    }
    get vertical(): Promise<boolean> {
        return this.getProp("vertical");
    }
    set vertical(value: boolean) {
        this.setProp("vertical", value);
    }
    get thumbSize(): Promise<number> {
        return this.getProp("thumb_size");
    }
    set thumbSize(value: number) {
        this.setProp("thumb_size", value);
    }
    get trackSize(): Promise<number> {
        return this.getProp("track_size");
    }
    set trackSize(value: number) {
        this.setProp("track_size", value);
    }
    get solidThumb(): Promise<boolean> {
        return this.getProp("solid_thumb");
    }
    set solidThumb(value: boolean) {
        this.setProp("solid_thumb", value);
    }
    get limit(): Promise<number> {
        return this.getProp("limit");
    }
    set limit(value: number) {
        this.setProp("limit", value);
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

/** Um VideoPlayer de um form. */
export class VideoPlayer extends AylisComponent {
    get file(): Promise<string> {
        return this.getProp("file");
    }
    set file(value: string) {
        this.setProp("file", value);
    }
    get autoplay(): Promise<boolean> {
        return this.getProp("autoplay");
    }
    set autoplay(value: boolean) {
        this.setProp("autoplay", value);
    }
    get looping(): Promise<boolean> {
        return this.getProp("looping");
    }
    set looping(value: boolean) {
        this.setProp("looping", value);
    }
    get volume(): Promise<number> {
        return this.getProp("volume");
    }
    set volume(value: number) {
        this.setProp("volume", value);
    }
    get muted(): Promise<boolean> {
        return this.getProp("muted");
    }
    set muted(value: boolean) {
        this.setProp("muted", value);
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
    get rate(): Promise<number> {
        return this.getProp("rate");
    }
    set rate(value: number) {
        this.setProp("rate", value);
    }
    get output(): Promise<string> {
        return this.getProp("output");
    }
    set output(value: string) {
        this.setProp("output", value);
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
    play() {
        this.aylis.send("media", { target: this.id, op: "play" });
    }
    pause() {
        this.aylis.send("media", { target: this.id, op: "pause" });
    }
    stop() {
        this.aylis.send("media", { target: this.id, op: "stop" });
    }
    seek(seconds: number) {
        this.aylis.send("media", { target: this.id, op: "seek", seconds: seconds });
    }
    position(): Promise<number> {
        return this.aylis.call("media", { target: this.id, op: "position" }) as Promise<any>;
    }
    duration(): Promise<number> {
        return this.aylis.call("media", { target: this.id, op: "duration" }) as Promise<any>;
    }
}

/** O form frmPlayer e os componentes dele (resolvidos ao abrir; cada janela tem a sua instância). */
export class FrmPlayerComponents extends Form {
    mnuPrincipal!: MainMenu;
    mnuArquivo!: MenuItem;
    mnuAbrir!: MenuItem;
    mnuLimparFila!: MenuItem;
    mnuSep1!: MenuItem;
    mnuSair!: MenuItem;
    mnuReproduzir!: MenuItem;
    mnuReproduzirPausar!: MenuItem;
    mnuParar!: MenuItem;
    mnuAnterior!: MenuItem;
    mnuProxima!: MenuItem;
    mnuSep2!: MenuItem;
    mnuEmbaralhar!: MenuItem;
    mnuRepetir!: MenuItem;
    mnuExibir!: MenuItem;
    mnuAgora!: MenuItem;
    mnuFila!: MenuItem;
    pnlControles!: Panel;
    lytBarra!: Layout;
    lblTempo!: Label;
    lblDuracao!: Label;
    sbPosicao!: SeekBar;
    lytInfo!: Layout;
    icoCapa!: Icon;
    lytTextos!: Layout;
    lblTitulo!: Label;
    lblDetalhe!: Label;
    lytVolume!: Layout;
    btnMudo!: Button;
    tbVolume!: TrackBar;
    lytBotoes!: Layout;
    lytBotoesCentro!: Layout;
    btnEmbaralhar!: Button;
    btnAnterior!: Button;
    btnReproduzir!: Button;
    btnProxima!: Button;
    btnRepetir!: Button;
    lytNavegacao!: Layout;
    btnAgora!: Button;
    btnFila!: Button;
    divNavegacao!: Divider;
    btnAbrirArquivos!: Button;
    pnlAgora!: Panel;
    vpVideo!: VideoPlayer;
    pnlFila!: Panel;
    lblFila!: Label;
    lytFilaBotoes!: Layout;
    btnAdicionar!: Button;
    btnLimparFila!: Button;
    lvFila!: ListView;
    dlgAbrir!: OpenDialog;
    tmrPosicao!: Timer;
    bind(list: ComponentInfo[]) {
        this.mnuPrincipal = this.resolve(list, "mnuPrincipal", "MainMenu", MainMenu);
        this.mnuArquivo = this.resolve(list, "mnuArquivo", "MenuItem", MenuItem);
        this.mnuAbrir = this.resolve(list, "mnuAbrir", "MenuItem", MenuItem);
        this.mnuLimparFila = this.resolve(list, "mnuLimparFila", "MenuItem", MenuItem);
        this.mnuSep1 = this.resolve(list, "mnuSep1", "MenuItem", MenuItem);
        this.mnuSair = this.resolve(list, "mnuSair", "MenuItem", MenuItem);
        this.mnuReproduzir = this.resolve(list, "mnuReproduzir", "MenuItem", MenuItem);
        this.mnuReproduzirPausar = this.resolve(list, "mnuReproduzirPausar", "MenuItem", MenuItem);
        this.mnuParar = this.resolve(list, "mnuParar", "MenuItem", MenuItem);
        this.mnuAnterior = this.resolve(list, "mnuAnterior", "MenuItem", MenuItem);
        this.mnuProxima = this.resolve(list, "mnuProxima", "MenuItem", MenuItem);
        this.mnuSep2 = this.resolve(list, "mnuSep2", "MenuItem", MenuItem);
        this.mnuEmbaralhar = this.resolve(list, "mnuEmbaralhar", "MenuItem", MenuItem);
        this.mnuRepetir = this.resolve(list, "mnuRepetir", "MenuItem", MenuItem);
        this.mnuExibir = this.resolve(list, "mnuExibir", "MenuItem", MenuItem);
        this.mnuAgora = this.resolve(list, "mnuAgora", "MenuItem", MenuItem);
        this.mnuFila = this.resolve(list, "mnuFila", "MenuItem", MenuItem);
        this.pnlControles = this.resolve(list, "pnlControles", "Panel", Panel);
        this.lytBarra = this.resolve(list, "lytBarra", "Layout", Layout);
        this.lblTempo = this.resolve(list, "lblTempo", "Label", Label);
        this.lblDuracao = this.resolve(list, "lblDuracao", "Label", Label);
        this.sbPosicao = this.resolve(list, "sbPosicao", "SeekBar", SeekBar);
        this.lytInfo = this.resolve(list, "lytInfo", "Layout", Layout);
        this.icoCapa = this.resolve(list, "icoCapa", "Icon", Icon);
        this.lytTextos = this.resolve(list, "lytTextos", "Layout", Layout);
        this.lblTitulo = this.resolve(list, "lblTitulo", "Label", Label);
        this.lblDetalhe = this.resolve(list, "lblDetalhe", "Label", Label);
        this.lytVolume = this.resolve(list, "lytVolume", "Layout", Layout);
        this.btnMudo = this.resolve(list, "btnMudo", "Button", Button);
        this.tbVolume = this.resolve(list, "tbVolume", "TrackBar", TrackBar);
        this.lytBotoes = this.resolve(list, "lytBotoes", "Layout", Layout);
        this.lytBotoesCentro = this.resolve(list, "lytBotoesCentro", "Layout", Layout);
        this.btnEmbaralhar = this.resolve(list, "btnEmbaralhar", "Button", Button);
        this.btnAnterior = this.resolve(list, "btnAnterior", "Button", Button);
        this.btnReproduzir = this.resolve(list, "btnReproduzir", "Button", Button);
        this.btnProxima = this.resolve(list, "btnProxima", "Button", Button);
        this.btnRepetir = this.resolve(list, "btnRepetir", "Button", Button);
        this.lytNavegacao = this.resolve(list, "lytNavegacao", "Layout", Layout);
        this.btnAgora = this.resolve(list, "btnAgora", "Button", Button);
        this.btnFila = this.resolve(list, "btnFila", "Button", Button);
        this.divNavegacao = this.resolve(list, "divNavegacao", "Divider", Divider);
        this.btnAbrirArquivos = this.resolve(list, "btnAbrirArquivos", "Button", Button);
        this.pnlAgora = this.resolve(list, "pnlAgora", "Panel", Panel);
        this.vpVideo = this.resolve(list, "vpVideo", "VideoPlayer", VideoPlayer);
        this.pnlFila = this.resolve(list, "pnlFila", "Panel", Panel);
        this.lblFila = this.resolve(list, "lblFila", "Label", Label);
        this.lytFilaBotoes = this.resolve(list, "lytFilaBotoes", "Layout", Layout);
        this.btnAdicionar = this.resolve(list, "btnAdicionar", "Button", Button);
        this.btnLimparFila = this.resolve(list, "btnLimparFila", "Button", Button);
        this.lvFila = this.resolve(list, "lvFila", "ListView", ListView);
        this.dlgAbrir = this.resolve(list, "dlgAbrir", "OpenDialog", OpenDialog);
        this.tmrPosicao = this.resolve(list, "tmrPosicao", "Timer", Timer);
    }
    /** O Button deste form com o nome (como o FindComponent). */
    findButton(name: string): Button | undefined {
        switch (name) {
            case "btnMudo": return this.btnMudo;
            case "btnEmbaralhar": return this.btnEmbaralhar;
            case "btnAnterior": return this.btnAnterior;
            case "btnReproduzir": return this.btnReproduzir;
            case "btnProxima": return this.btnProxima;
            case "btnRepetir": return this.btnRepetir;
            case "btnAgora": return this.btnAgora;
            case "btnFila": return this.btnFila;
            case "btnAbrirArquivos": return this.btnAbrirArquivos;
            case "btnAdicionar": return this.btnAdicionar;
            case "btnLimparFila": return this.btnLimparFila;
        }
        return undefined;
    }
    /** O Button que disparou o evento. */
    senderAsButton(ev: AylisEvent): Button | undefined {
        return this.findButton(ev.sender.name);
    }
    /** O Divider deste form com o nome (como o FindComponent). */
    findDivider(name: string): Divider | undefined {
        switch (name) {
            case "divNavegacao": return this.divNavegacao;
        }
        return undefined;
    }
    /** O Divider que disparou o evento. */
    senderAsDivider(ev: AylisEvent): Divider | undefined {
        return this.findDivider(ev.sender.name);
    }
    /** O Icon deste form com o nome (como o FindComponent). */
    findIcon(name: string): Icon | undefined {
        switch (name) {
            case "icoCapa": return this.icoCapa;
        }
        return undefined;
    }
    /** O Icon que disparou o evento. */
    senderAsIcon(ev: AylisEvent): Icon | undefined {
        return this.findIcon(ev.sender.name);
    }
    /** O Label deste form com o nome (como o FindComponent). */
    findLabel(name: string): Label | undefined {
        switch (name) {
            case "lblTempo": return this.lblTempo;
            case "lblDuracao": return this.lblDuracao;
            case "lblTitulo": return this.lblTitulo;
            case "lblDetalhe": return this.lblDetalhe;
            case "lblFila": return this.lblFila;
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
            case "lytBarra": return this.lytBarra;
            case "lytInfo": return this.lytInfo;
            case "lytTextos": return this.lytTextos;
            case "lytVolume": return this.lytVolume;
            case "lytBotoes": return this.lytBotoes;
            case "lytBotoesCentro": return this.lytBotoesCentro;
            case "lytNavegacao": return this.lytNavegacao;
            case "lytFilaBotoes": return this.lytFilaBotoes;
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
            case "lvFila": return this.lvFila;
        }
        return undefined;
    }
    /** O ListView que disparou o evento. */
    senderAsListView(ev: AylisEvent): ListView | undefined {
        return this.findListView(ev.sender.name);
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
            case "mnuAbrir": return this.mnuAbrir;
            case "mnuLimparFila": return this.mnuLimparFila;
            case "mnuSep1": return this.mnuSep1;
            case "mnuSair": return this.mnuSair;
            case "mnuReproduzir": return this.mnuReproduzir;
            case "mnuReproduzirPausar": return this.mnuReproduzirPausar;
            case "mnuParar": return this.mnuParar;
            case "mnuAnterior": return this.mnuAnterior;
            case "mnuProxima": return this.mnuProxima;
            case "mnuSep2": return this.mnuSep2;
            case "mnuEmbaralhar": return this.mnuEmbaralhar;
            case "mnuRepetir": return this.mnuRepetir;
            case "mnuExibir": return this.mnuExibir;
            case "mnuAgora": return this.mnuAgora;
            case "mnuFila": return this.mnuFila;
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
    /** O Panel deste form com o nome (como o FindComponent). */
    findPanel(name: string): Panel | undefined {
        switch (name) {
            case "pnlControles": return this.pnlControles;
            case "pnlAgora": return this.pnlAgora;
            case "pnlFila": return this.pnlFila;
        }
        return undefined;
    }
    /** O Panel que disparou o evento. */
    senderAsPanel(ev: AylisEvent): Panel | undefined {
        return this.findPanel(ev.sender.name);
    }
    /** O SeekBar deste form com o nome (como o FindComponent). */
    findSeekBar(name: string): SeekBar | undefined {
        switch (name) {
            case "sbPosicao": return this.sbPosicao;
        }
        return undefined;
    }
    /** O SeekBar que disparou o evento. */
    senderAsSeekBar(ev: AylisEvent): SeekBar | undefined {
        return this.findSeekBar(ev.sender.name);
    }
    /** O Timer deste form com o nome (como o FindComponent). */
    findTimer(name: string): Timer | undefined {
        switch (name) {
            case "tmrPosicao": return this.tmrPosicao;
        }
        return undefined;
    }
    /** O Timer que disparou o evento. */
    senderAsTimer(ev: AylisEvent): Timer | undefined {
        return this.findTimer(ev.sender.name);
    }
    /** O TrackBar deste form com o nome (como o FindComponent). */
    findTrackBar(name: string): TrackBar | undefined {
        switch (name) {
            case "tbVolume": return this.tbVolume;
        }
        return undefined;
    }
    /** O TrackBar que disparou o evento. */
    senderAsTrackBar(ev: AylisEvent): TrackBar | undefined {
        return this.findTrackBar(ev.sender.name);
    }
    /** O VideoPlayer deste form com o nome (como o FindComponent). */
    findVideoPlayer(name: string): VideoPlayer | undefined {
        switch (name) {
            case "vpVideo": return this.vpVideo;
        }
        return undefined;
    }
    /** O VideoPlayer que disparou o evento. */
    senderAsVideoPlayer(ev: AylisEvent): VideoPlayer | undefined {
        return this.findVideoPlayer(ev.sender.name);
    }
}

/** A janela de frmPlayer cujo evento está rodando (a primeira, fora dos handlers). */
export let frmPlayer: FrmPlayerComponents;
registerFormClass("frmPlayer", FrmPlayerComponents, (form) => {
    frmPlayer = form as FrmPlayerComponents;
});
