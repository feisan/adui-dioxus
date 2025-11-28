use crate::components::config_provider::ComponentSize;
use dioxus::prelude::*;

/// Data model for a single tab in the Tabs component.
#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub key: String,
    pub label: String,
    pub disabled: bool,
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
            content,
        }
    }

    /// Create a disabled tab without content.
    pub fn disabled(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            disabled: true,
            content: None,
        }
    }
}

/// Resolve the initial active key for uncontrolled Tabs.
fn resolve_initial_active_key(default_key: Option<String>, items: &[TabItem]) -> String {
    default_key
        .or_else(|| items.first().map(|t| t.key.clone()))
        .unwrap_or_default()
}

/// Props for the Tabs component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// Tab items with key/label/content.
    pub items: Vec<TabItem>,
    /// Controlled active key. When set, Tabs becomes controlled and will not
    /// manage its own active tab state.
    #[props(optional)]
    pub active_key: Option<String>,
    /// Default active key used in uncontrolled mode.
    #[props(optional)]
    pub default_active_key: Option<String>,
    /// Called when the active tab changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,
    /// Visual density for tab height and typography.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Extra class name for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Tabs (MVP, line style, top position only).
#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let TabsProps {
        items,
        active_key,
        default_active_key,
        on_change,
        size,
        class,
        style,
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
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-tabs-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-tabs-lg".into()),
        }
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Event handler snapshot.
    let on_change_cb = on_change;

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-tabs-nav",
                div { class: "adui-tabs-nav-list",
                    {items.iter().map(|item| {
                        let key = item.key.clone();
                        let label = item.label.clone();
                        let disabled = item.disabled;
                        let is_active = key == current_key;
                        let mut active_internal_for_tab = active_internal;
                        let on_change_for_tab = on_change_cb;

                        rsx! {
                            button {
                                r#type: "button",
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
                                disabled: "{disabled}",
                                onclick: move |_| {
                                    if disabled {
                                        return;
                                    }
                                    if !is_controlled {
                                        let mut signal = active_internal_for_tab;
                                        signal.set(key.clone());
                                    }
                                    if let Some(cb) = on_change_for_tab {
                                        cb.call(key.clone());
                                    }
                                },
                                "{label}"
                            }
                        }
                    })}
                }
            }

            div { class: "adui-tabs-content",
                {items.iter().filter(|item| item.key == current_key).map(|item| {
                    let content = item.content.clone();
                    rsx! {
                        div { class: "adui-tabs-tabpane", role: "tabpanel",
                            if let Some(node) = content { {node} }
                        }
                    }
                })}
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
}
