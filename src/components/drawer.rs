use crate::components::overlay::{OverlayKey, OverlayKind, use_overlay};
use dioxus::prelude::*;

/// Placement for Drawer panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawerPlacement {
    Left,
    Right,
    Top,
    Bottom,
}

/// Basic Drawer props.
#[derive(Props, Clone, PartialEq)]
pub struct DrawerProps {
    pub open: bool,
    #[props(optional)]
    pub title: Option<String>,
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default = true)]
    pub mask_closable: bool,
    #[props(default = true)]
    pub closable: bool,
    /// Destroy contents when closed.
    #[props(default)]
    pub destroy_on_close: bool,
    /// Drawer side.
    #[props(default = DrawerPlacement::Right)]
    pub placement: DrawerPlacement,
    /// Logical size (width for left/right, height for top/bottom).
    #[props(optional)]
    pub size: Option<f32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Simple Ant Design flavored Drawer.
#[component]
pub fn Drawer(props: DrawerProps) -> Element {
    let DrawerProps {
        open,
        title,
        on_close,
        mask_closable,
        closable,
        destroy_on_close,
        placement,
        size,
        class,
        style,
        children,
    } = props;

    let overlay = use_overlay();
    let drawer_key: Signal<Option<OverlayKey>> = use_signal(|| None);
    let z_index: Signal<i32> = use_signal(|| 1000);

    {
        let overlay = overlay.clone();
        let mut key_signal = drawer_key;
        let mut z_signal = z_index;
        use_effect(move || {
            if let Some(handle) = overlay.clone() {
                let current_key = {
                    let guard = key_signal.read();
                    *guard
                };
                if open {
                    if current_key.is_none() {
                        let (key, meta) = handle.open(OverlayKind::Drawer, true);
                        z_signal.set(meta.z_index);
                        key_signal.set(Some(key));
                    }
                } else if let Some(key) = current_key {
                    handle.close(key);
                    key_signal.set(None);
                }
            }
        });
    }

    if !open && destroy_on_close {
        return rsx! {};
    }

    let current_z = *z_index.read();
    let logical_size = size.unwrap_or(378.0);
    let class_attr = class.unwrap_or_else(|| "adui-drawer".to_string());
    let style_attr = style.unwrap_or_default();

    let close = move || {
        if let Some(cb) = on_close {
            cb.call(());
        }
    };

    // Compute positioning styles based on placement.
    let (panel_style, wrapper_align_style) = match placement {
        DrawerPlacement::Left => (
            format!("left: 0; top: 0; bottom: 0; width: {logical_size}px;",),
            "justify-content: flex-start; align-items: stretch;".to_string(),
        ),
        DrawerPlacement::Right => (
            format!("right: 0; top: 0; bottom: 0; width: {logical_size}px;",),
            "justify-content: flex-end; align-items: stretch;".to_string(),
        ),
        DrawerPlacement::Top => (
            format!("top: 0; left: 0; right: 0; height: {logical_size}px;",),
            "justify-content: flex-start; align-items: stretch;".to_string(),
        ),
        DrawerPlacement::Bottom => (
            format!("bottom: 0; left: 0; right: 0; height: {logical_size}px;",),
            "justify-content: flex-end; align-items: stretch;".to_string(),
        ),
    };

    rsx! {
        if open {
            // Mask layer
            div {
                class: "adui-drawer-mask",
                style: "position: fixed; inset: 0; background: rgba(0,0,0,0.45); z-index: {current_z};",
                onclick: move |_| {
                    if mask_closable {
                        close();
                    }
                }
            }
            // Drawer panel
            div {
                class: "{class_attr}",
                style: "position: fixed; inset: 0; display: flex; {wrapper_align_style} z-index: {current_z + 1}; {style_attr}",
                div {
                    class: "adui-drawer-panel",
                    style: "position: absolute; {panel_style} background: var(--adui-color-bg-container); border-radius: 0; box-shadow: var(--adui-shadow-secondary); border: 1px solid var(--adui-color-border); display: flex; flex-direction: column;",
                    // Header
                    if title.is_some() || closable {
                        div {
                            class: "adui-drawer-header",
                            style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--adui-color-border);",
                            if let Some(text) = title {
                                div { class: "adui-drawer-title", "{text}" }
                            }
                            if closable {
                                button {
                                    class: "adui-drawer-close",
                                    r#type: "button",
                                    style: "border: none; background: none; cursor: pointer; font-size: 16px;",
                                    onclick: move |_| close(),
                                    "×"
                                }
                            }
                        }
                    }
                    // Body
                    div {
                        class: "adui-drawer-body",
                        style: "padding: 16px; flex: 1; overflow: auto;",
                        {children}
                    }
                }
            }
        }
    }
}
