use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::control::{ControlStatus, push_status_class};
use crate::components::form::{FormItemControlContext, use_form_item_control};
use crate::components::select_base::{
    CascaderNode, DropdownLayer, OptionKey, path_to_value, use_dropdown_layer, value_to_path,
};
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;
use serde_json::Value;

/// Props for the Cascader component (MVP subset).
///
/// 当前版本重点支持单选路径，不实现多选逻辑：
/// - `value` 表示选中的路径，如 `Some(vec!["zhejiang", "hangzhou"])`；
/// - `multiple` 字段预留，暂未生效（仅作为将来扩展的 API 占位）。
#[derive(Props, Clone, PartialEq)]
pub struct CascaderProps {
    /// 级联选项树。
    pub options: Vec<CascaderNode>,
    /// 单选模式下的受控路径值。
    #[props(optional)]
    pub value: Option<Vec<String>>,
    /// 预留多选开关，目前实现中未启用。
    #[props(default)]
    pub multiple: bool,
    /// 占位文案。
    #[props(optional)]
    pub placeholder: Option<String>,
    /// 是否展示清除按钮。
    #[props(default)]
    pub allow_clear: bool,
    /// 禁用整个组件。
    #[props(default)]
    pub disabled: bool,
    /// 视觉状态（success / warning / error）。
    #[props(optional)]
    pub status: Option<ControlStatus>,
    /// 组件尺寸，默认跟随 ConfigProvider。
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// 自定义类名与样式。
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// 弹层额外类名与样式。
    #[props(optional)]
    pub dropdown_class: Option<String>,
    #[props(optional)]
    pub dropdown_style: Option<String>,
    /// 选中路径变更回调。
    #[props(optional)]
    pub on_change: Option<EventHandler<Vec<String>>>,
}

/// Ant Design flavored Cascader (MVP).
#[component]
pub fn Cascader(props: CascaderProps) -> Element {
    let CascaderProps {
        options,
        value,
        multiple,
        placeholder,
        allow_clear,
        disabled,
        status,
        size,
        class,
        style,
        dropdown_class,
        dropdown_style,
        on_change,
    } = props;

    // `multiple` 目前只作为占位参数，尚未在内部实现多选逻辑。
    let _multiple_flag = multiple;

    let config = use_config();
    let form_control = use_form_item_control();

    let final_size = size.unwrap_or(config.size);

    let is_disabled =
        disabled || config.disabled || form_control.as_ref().is_some_and(|ctx| ctx.is_disabled());

    // 内部仅在非受控、非 Form 场景下使用的选中路径。
    let internal_selected: Signal<Vec<OptionKey>> = use_signal(Vec::new);

    let has_form = form_control.is_some();
    let prop_value = value.clone();

    // 当前已选路径快照。
    let selected_path: Vec<OptionKey> = if let Some(ctx) = form_control.as_ref() {
        value_to_path(ctx.value())
    } else if let Some(v) = prop_value {
        v
    } else {
        internal_selected.read().clone()
    };

    let controlled_by_prop = has_form || value.is_some();

    // 当前 UI 导航使用的路径，与选中路径解耦：
    // - 初次时为空，渲染时用 `selected_path` 作为 fallback；
    // - 点击中间节点会更新 `active_path`，只在点击叶子时更新选中路径。
    let active_path: Signal<Vec<OptionKey>> = use_signal(Vec::new);

    // 打开/关闭状态。
    let open_state: Signal<bool> = use_signal(|| false);

    // 内外点击标记，用于实现点击空白关闭。
    let internal_click_flag: Signal<bool> = use_signal(|| false);

    // document 级别点击监听，仅 wasm 目标生效。
    #[cfg(target_arch = "wasm32")]
    {
        let mut open_for_global = open_state;
        let mut internal_flag = internal_click_flag;
        use_effect(move || {
            use wasm_bindgen::{JsCast, closure::Closure};

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    let target: web_sys::EventTarget = document.into();
                    let handler = Closure::<dyn FnMut(web_sys::MouseEvent)>::wrap(Box::new(
                        move |_evt: web_sys::MouseEvent| {
                            let mut flag = internal_flag;
                            if *flag.read() {
                                flag.set(false);
                                return;
                            }
                            let mut open_signal = open_for_global;
                            if *open_signal.read() {
                                open_signal.set(false);
                            }
                        },
                    ));
                    let _ = target.add_event_listener_with_callback(
                        "click",
                        handler.as_ref().unchecked_ref(),
                    );
                    handler.forget();
                }
            }
        });
    }

    let open_flag = *open_state.read();
    let DropdownLayer { z_index, .. } = use_dropdown_layer(open_flag);
    let current_z = *z_index.read();

    let placeholder_str = placeholder.unwrap_or_default();

    // UI 路径 = active_path 非空时优先，否则使用 selected_path。
    let ui_path: Vec<OptionKey> = {
        let a = active_path.read().clone();
        if a.is_empty() {
            selected_path.clone()
        } else {
            a
        }
    };

    // 计算下拉多列中的每一列。
    // 为避免生命周期复杂度，这里按列复制节点（Cascader 通常数据量较小）。
    fn build_columns(options: &[CascaderNode], path: &[OptionKey]) -> Vec<Vec<CascaderNode>> {
        let mut cols: Vec<Vec<CascaderNode>> = Vec::new();
        let mut current_level: &[CascaderNode] = options;
        cols.push(current_level.to_vec());

        for key in path {
            if let Some(node) = current_level.iter().find(|n| &n.key == key) {
                if node.children.is_empty() {
                    break;
                }
                current_level = &node.children;
                cols.push(current_level.to_vec());
            } else {
                break;
            }
        }

        cols
    }

    // 构造路径的展示 label，例如 "Zhejiang / Hangzhou"。
    fn build_path_label(options: &[CascaderNode], path: &[OptionKey]) -> String {
        fn find_label<'a>(nodes: &'a [CascaderNode], key: &str) -> Option<&'a CascaderNode> {
            nodes.iter().find(|n| n.key == key)
        }

        let mut labels: Vec<String> = Vec::new();
        let mut current = options;
        for key in path {
            if let Some(node) = find_label(current, key) {
                labels.push(node.label.clone());
                if node.children.is_empty() {
                    break;
                }
                current = &node.children;
            } else {
                break;
            }
        }

        if labels.is_empty() {
            String::new()
        } else {
            labels.join(" / ")
        }
    }

    let columns = build_columns(&options, &ui_path);
    let display_label = build_path_label(&options, &selected_path);

    let display_node = if selected_path.is_empty() {
        rsx! { span { class: "adui-select-selection-placeholder", "{placeholder_str}" } }
    } else {
        rsx! { span { class: "adui-select-selection-item", "{display_label}" } }
    };

    // 事件处理所需的共享上下文。
    let form_for_handlers = form_control.clone();
    let internal_selected_for_handlers = internal_selected;
    let on_change_cb = on_change;
    let controlled_flag = controlled_by_prop;

    let mut open_for_toggle = open_state;
    let is_disabled_flag = is_disabled;
    let mut active_path_for_handlers = active_path;

    // 键盘：只处理打开/关闭与 Esc，暂不实现列内导航。
    let mut open_for_keydown = open_for_toggle;
    let internal_click_for_keydown = internal_click_flag;

    let mut internal_click_for_toggle = internal_click_flag;

    let dropdown_class_attr = {
        let mut list = vec!["adui-select-dropdown".to_string()];
        if let Some(extra) = dropdown_class {
            list.push(extra);
        }
        list.join(" ")
    };
    let dropdown_style_attr = format!(
        "position: absolute; top: 100%; left: 0; min-width: 100%; z-index: {}; {}",
        current_z,
        dropdown_style.unwrap_or_default()
    );

    // 触发区 class 复用 Select 的样式体系，便于保持视觉一致。
    let mut class_list = vec!["adui-select".to_string()];
    if is_disabled_flag {
        class_list.push("adui-select-disabled".into());
    }
    if open_flag {
        class_list.push("adui-select-open".into());
    }
    match final_size {
        ComponentSize::Small => class_list.push("adui-select-sm".into()),
        ComponentSize::Large => class_list.push("adui-select-lg".into()),
        ComponentSize::Middle => {}
    }
    push_status_class(&mut class_list, status);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    rsx! {
        div {
            class: "adui-select-root",
            style: "position: relative; display: inline-block;",
            div {
                class: "{class_attr}",
                style: "{style_attr}",
                role: "combobox",
                tabindex: 0,
                "aria-expanded": open_flag,
                "aria-disabled": is_disabled_flag,
                onclick: move |_| {
                    if is_disabled_flag {
                        return;
                    }
                    let mut flag = internal_click_for_toggle;
                    flag.set(true);

                    let mut open_signal = open_for_toggle;
                    let current = *open_signal.read();
                    open_signal.set(!current);
                },
                onkeydown: move |evt: KeyboardEvent| {
                    if is_disabled_flag {
                        return;
                    }
                    use dioxus::prelude::Key;

                    let open_now = *open_for_keydown.read();
                    if !open_now {
                        match evt.key() {
                            Key::Enter | Key::ArrowDown => {
                                evt.prevent_default();
                                let mut open_signal = open_for_keydown;
                                open_signal.set(true);
                            }
                            Key::Escape => {
                                // 关闭状态下按 Esc 不做任何事。
                            }
                            _ => {}
                        }
                        return;
                    }

                    if matches!(evt.key(), Key::Escape) {
                        let mut open_signal = open_for_keydown;
                        open_signal.set(false);
                        return;
                    }

                    // 内部键盘交互视为内部事件，避免被 document click 误伤。
                    let mut flag = internal_click_for_keydown;
                    flag.set(true);
                },
                div { class: "adui-select-selector", {display_node} }
                if allow_clear && !selected_path.is_empty() && !is_disabled_flag {
                    span {
                        class: "adui-select-clear",
                        onclick: move |_| {
                            apply_selected_path(
                                &form_for_handlers,
                                controlled_flag,
                                &internal_selected_for_handlers,
                                on_change_cb,
                                Vec::new(),
                            );
                            // 清空时也同步重置 active_path。
                            active_path_for_handlers.set(Vec::new());
                        },
                        "×"
                    }
                }
            }
            if open_flag {
                div {
                    class: "{dropdown_class_attr}",
                    style: "{dropdown_style_attr}",
                    role: "menu",
                    div {
                        style: "display: flex; max-height: 240px;",
                        {columns.iter().enumerate().map(|(col_index, col_items)| {
                            let ui_path_snapshot_col = ui_path.clone();
                            let selected_path_snapshot_col = selected_path.clone();
                            let form_for_click_col = form_control.clone();
                            let internal_selected_for_click_col = internal_selected;
                            let mut open_for_click_col = open_state;
                            let mut active_path_for_click_col = active_path;
                            let mut internal_click_for_col = internal_click_flag;
                            rsx! {
                                ul {
                                    class: "adui-select-item-list",
                                    style: "min-width: 140px; border-right: 1px solid var(--adui-color-split, var(--adui-color-border)); overflow-y: auto;",
                                    {col_items.iter().map(|node| {
                                        let key = node.key.clone();
                                        let label = node.label.clone();
                                        let disabled_opt = node.disabled || is_disabled_flag;
                                        let has_children = !node.children.is_empty();

                                        let is_in_path = ui_path_snapshot_col
                                            .get(col_index)
                                            .map(|k| k == &key)
                                            .unwrap_or(false);

                                        // 计算点击此项后将形成的路径，用于选中状态与提交。
                                        let next_path = {
                                            let mut path = ui_path_snapshot_col.clone();
                                            if path.len() > col_index + 1 {
                                                path.truncate(col_index + 1);
                                            }
                                            if path.len() == col_index {
                                                path.push(key.clone());
                                            } else if path.get(col_index) != Some(&key) {
                                                path[col_index] = key.clone();
                                            }
                                            path
                                        };
                                        let is_selected = selected_path_snapshot_col == next_path;

                                        let form_for_click_item = form_for_click_col.clone();
                                        let next_path_item = next_path.clone();

                                        rsx! {
                                            li {
                                                class: {
                                                    let mut classes = vec!["adui-select-item".to_string()];
                                                    if is_in_path {
                                                        classes.push("adui-select-item-option-active".into());
                                                    }
                                                    if is_selected {
                                                        classes.push("adui-select-item-option-selected".into());
                                                    }
                                                    if disabled_opt {
                                                        classes.push("adui-select-item-option-disabled".into());
                                                    }
                                                    classes.join(" ")
                                                },
                                                role: "menuitem",
                                                onclick: move |_| {
                                                    if disabled_opt {
                                                        return;
                                                    }
                                                    let mut flag = internal_click_for_col;
                                                    flag.set(true);

                                                    // 更新 active_path：保留当前列之前的路径，并设置当前列为点击的 key。
                                                    active_path_for_click_col.set(next_path_item.clone());

                                                    // 若有子节点，仅展开下一列，不立即提交。
                                                    if has_children {
                                                        return;
                                                    }

                                                    // 叶子节点：提交选中路径。
                                                    apply_selected_path(
                                                        &form_for_click_item,
                                                        controlled_flag,
                                                        &internal_selected_for_click_col,
                                                        on_change_cb,
                                                        next_path_item.clone(),
                                                    );

                                                    // 叶子选择后关闭弹层。
                                                    let mut open_signal = open_for_click_col;
                                                    open_signal.set(false);
                                                },
                                                "{label}"
                                            }
                                        }
                                    })}
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}

fn apply_selected_path(
    form_control: &Option<FormItemControlContext>,
    controlled_by_prop: bool,
    selected_signal: &Signal<Vec<OptionKey>>,
    on_change: Option<EventHandler<Vec<String>>>,
    new_path: Vec<OptionKey>,
) {
    if let Some(ctx) = form_control {
        if new_path.is_empty() {
            ctx.set_value(Value::Null);
        } else {
            let json = path_to_value(&new_path);
            ctx.set_value(json);
        }
    } else if !controlled_by_prop {
        let mut signal = *selected_signal;
        signal.set(new_path.clone());
    }

    if let Some(cb) = on_change {
        cb.call(new_path);
    }
}
