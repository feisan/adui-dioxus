//! Tabs component aligned with Ant Design 6.0.
//!
//! Features:
//! - Multiple visual types (line/card/editable-card)
//! - Tab placement (top/right/bottom/left)
//! - Centered tabs
//! - Editable tabs with add/remove functionality

use crate::components::config_provider::ComponentSize;
use crate::components::icon::{Icon, IconKind};
use crate::foundation::{ClassListExt, StyleStringExt, TabsClassNames, TabsSemantic, TabsStyles};
use dioxus::prelude::*;

/// Visual type for Tabs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsType {
    #[default]
    Line,
    Card,
    EditableCard,
}

impl TabsType {
    fn as_class(&self) -> &'static str {
        match self {
            TabsType::Line => "adui-tabs-line",
            TabsType::Card => "adui-tabs-card",
            TabsType::EditableCard => "adui-tabs-editable-card",
        }
    }
}

/// Tab placement position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabPlacement {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TabPlacement {
    fn as_class(&self) -> &'static str {
        match self {
            TabPlacement::Top => "adui-tabs-top",
            TabPlacement::Right => "adui-tabs-right",
            TabPlacement::Bottom => "adui-tabs-bottom",
            TabPlacement::Left => "adui-tabs-left",
        }
    }
}

/// Edit action for editable-card tabs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabEditAction {
    Add,
    Remove(String),
}

/// Data model for a single tab in the Tabs component.
#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub key: String,
    pub label: String,
    pub disabled: bool,
    /// Whether this tab can be closed (for editable-card type).
    pub closable: bool,
    /// Custom icon for the tab.
    pub icon: Option<Element>,
    /// Optional tab content. When None, the caller can render content nearby.
    pub content: Option<Element>,
}

impl TabItem {
    /// Create a new tab item with the given key, label and optional content.
    pub fn new(key: impl Into<String>, label: impl Into<String>, content: Option<Element>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: false,
            closable: true,
            icon: None,
            content,
        }
    }

    /// Create a disabled tab without content.
    pub fn disabled(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: true,
            closable: false,
            icon: None,
            content: None,
        }
    }

    /// Set closable state.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Set custom icon.
    pub fn icon(mut self, icon: Element) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Resolve the initial active key for uncontrolled Tabs.
fn resolve_initial_active_key(default_key: Option<String>, items: &[TabItem]) -> String {
    default_key
        .or_else(|| items.first().map(|t| t.key.clone()))
        .unwrap_or_default()
}

/// Props for the Tabs component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Tab items with key/label/content.
    pub items: Vec<TabItem>,
    /// Controlled active key. When set, Tabs becomes controlled.
    #[props(optional)]
    pub active_key: Option<String>,
    /// Default active key used in uncontrolled mode.
    #[props(optional)]
    pub default_active_key: Option<String>,
    /// Called when the active tab changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Visual type (line/card/editable-card).
    #[props(default)]
    pub r#type: TabsType,
    /// Tab placement position.
    #[props(default)]
    pub tab_placement: TabPlacement,
    /// Whether to center tabs.
    #[props(default)]
    pub centered: bool,
    /// Whether to hide the add button (for editable-card).
    #[props(default)]
    pub hide_add: bool,
    /// Called when tabs are added or removed (for editable-card).
    #[props(optional)]
    pub on_edit: Option<EventHandler<TabEditAction>>,
    /// Custom add icon.
    #[props(optional)]
    pub add_icon: Option<Element>,
    /// Custom close icon.
    #[props(optional)]
    pub remove_icon: Option<Element>,
    /// Custom more icon (for overflow).
    #[props(optional)]
    pub more_icon: Option<Element>,
    /// Visual density for tab height and typography.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Whether to destroy inactive tab panels.
    #[props(default)]
    pub destroy_inactive_tab_pane: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names.
    #[props(optional)]
    pub class_names: Option<TabsClassNames>,
    /// Semantic styles.
    #[props(optional)]
    pub styles: Option<TabsStyles>,
}

/// Ant Design flavored Tabs.
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let TabsProps {
        items,
        active_key,
        default_active_key,
        on_change,
        r#type,
        tab_placement,
        centered,
        hide_add,
        on_edit,
        add_icon,
        remove_icon,
        more_icon: _more_icon,
        size,
        destroy_inactive_tab_pane,
        class,
        style,
        class_names,
        styles,
    } = props;

    // Determine initial active key for uncontrolled mode.
    let initial_key = resolve_initial_active_key(default_active_key.clone(), &items);

    let active_internal: Signal<String> = use_signal(|| initial_key);

    let is_controlled = active_key.is_some();
    let current_key = active_key
        .clone()
        .unwrap_or_else(|| active_internal.read().clone());

    // Root classes.
    let mut class_list = vec!["adui-tabs".to_string()];
    class_list.push(r#type.as_class().to_string());
    class_list.push(tab_placement.as_class().to_string());
    if centered {
        class_list.push("adui-tabs-centered".into());
    }
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-tabs-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-tabs-lg".into()),
        }
    }
    class_list.push_semantic(&class_names, TabsSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut style_attr = style.unwrap_or_default();
    style_attr.append_semantic(&styles, TabsSemantic::Root);

    // Event handler snapshot.
    let on_change_cb = on_change;
    let on_edit_cb = on_edit;
    let is_editable = matches!(r#type, TabsType::EditableCard);

    // Default icons
    let add_icon_element = add_icon.unwrap_or_else(|| {
        rsx! { Icon { kind: IconKind::Plus, size: 14.0 } }
    });

    let close_icon_element = remove_icon.unwrap_or_else(|| {
        rsx! { Icon { kind: IconKind::Close, size: 12.0 } }
    });

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-tabs-nav",
                div { class: "adui-tabs-nav-wrap",
                    div { class: "adui-tabs-nav-list",
                        {items.iter().map(|item| {
                            let key = item.key.clone();
                            let key_for_change = key.clone();
                            let key_for_close = key.clone();
                            let label = item.label.clone();
                            let disabled = item.disabled;
                            let closable = item.closable;
                            let icon = item.icon.clone();
                            let is_active = key == current_key;
                            let active_internal_for_tab = active_internal;
                            let on_change_for_tab = on_change_cb;
                            let on_edit_for_close = on_edit_cb;

                            rsx! {
                                div {
                                    class: {
                                        let mut classes = vec!["adui-tabs-tab".to_string()];
                                        if is_active {
                                            classes.push("adui-tabs-tab-active".into());
                                        }
                                        if disabled {
                                            classes.push("adui-tabs-tab-disabled".into());
                                        }
                                        classes.join(" ")
                                    },
                                    role: "tab",
                                    aria_selected: "{is_active}",
                                    button {
                                        r#type: "button",
                                        class: "adui-tabs-tab-btn",
                                        disabled: disabled,
                                        onclick: move |_| {
                                            if disabled {
                                                return;
                                            }
                                            if !is_controlled {
                                                let mut signal = active_internal_for_tab;
                                                signal.set(key_for_change.clone());
                                            }
                                            if let Some(cb) = on_change_for_tab {
                                                cb.call(key_for_change.clone());
                                            }
                                        },
                                        if let Some(icon_el) = icon {
                                            span { class: "adui-tabs-tab-icon", {icon_el} }
                                        }
                                        "{label}"
                                    }
                                    if is_editable && closable {
                                        button {
                                            r#type: "button",
                                            class: "adui-tabs-tab-remove",
                                            onclick: move |evt| {
                                                evt.stop_propagation();
                                                if let Some(cb) = on_edit_for_close {
                                                    cb.call(TabEditAction::Remove(key_for_close.clone()));
                                                }
                                            },
                                            {close_icon_element.clone()}
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
                if is_editable && !hide_add {
                    button {
                        r#type: "button",
                        class: "adui-tabs-nav-add",
                        onclick: move |_| {
                            if let Some(cb) = on_edit_cb {
                                cb.call(TabEditAction::Add);
                            }
                        },
                        {add_icon_element}
                    }
                }
            }

            div { class: "adui-tabs-content-holder",
                div { class: "adui-tabs-content",
                    {items.iter().map(|item| {
                        let key = item.key.clone();
                        let content = item.content.clone();
                        let is_active = key == current_key;

                        // If destroy_inactive_tab_pane, only render active content
                        if destroy_inactive_tab_pane && !is_active {
                            return rsx! {};
                        }

                        let pane_class = if is_active {
                            "adui-tabs-tabpane adui-tabs-tabpane-active"
                        } else {
                            "adui-tabs-tabpane adui-tabs-tabpane-hidden"
                        };

                        rsx! {
                            div {
                                key: "{key}",
                                class: "{pane_class}",
                                role: "tabpanel",
                                hidden: !is_active,
                                if let Some(node) = content { {node} }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_item_new_and_disabled_work() {
        let t = TabItem::new("k1", "Label", None);
        assert_eq!(t.key, "k1");
        assert_eq!(t.label, "Label");
        assert!(!t.disabled);

        let t2 = TabItem::disabled("k2", "Other");
        assert_eq!(t2.key, "k2");
        assert_eq!(t2.label, "Other");
        assert!(t2.disabled);
        assert!(t2.content.is_none());
    }

    #[test]
    fn resolve_initial_active_key_prefers_default_then_first_item() {
        let items = vec![TabItem::new("a", "A", None), TabItem::new("b", "B", None)];

        assert_eq!(resolve_initial_active_key(Some("x".into()), &items), "x");
        assert_eq!(resolve_initial_active_key(None, &items), "a");
        assert_eq!(resolve_initial_active_key(None, &[]), "");
    }

    #[test]
    fn tabs_type_classes() {
        assert_eq!(TabsType::Line.as_class(), "adui-tabs-line");
        assert_eq!(TabsType::Card.as_class(), "adui-tabs-card");
        assert_eq!(TabsType::EditableCard.as_class(), "adui-tabs-editable-card");
    }

    #[test]
    fn tab_placement_classes() {
        assert_eq!(TabPlacement::Top.as_class(), "adui-tabs-top");
        assert_eq!(TabPlacement::Left.as_class(), "adui-tabs-left");
    }
}
