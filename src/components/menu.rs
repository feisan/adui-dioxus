use dioxus::prelude::*;

/// Menu display mode, aligned with Ant Design's `inline` and `horizontal` modes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuMode {
    #[default]
    Inline,
    Horizontal,
}

/// Data model for a single menu item.
#[derive(Clone, PartialEq)]
pub struct MenuItemNode {
    pub id: String,
    pub label: String,
    pub icon: Option<Element>,
    pub disabled: bool,
    pub children: Option<Vec<MenuItemNode>>,
}

impl MenuItemNode {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            disabled: false,
            children: None,
        }
    }
}

/// Props for the Menu component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    /// Menu items in a tree-like structure (MVP: at most two levels).
    pub items: Vec<MenuItemNode>,
    /// Display mode: inline (sider) or horizontal (header).
    #[props(default)]
    pub mode: MenuMode,
    /// Controlled selected keys.
    #[props(optional)]
    pub selected_keys: Option<Vec<String>>,
    /// Default selected keys (used when `selected_keys` is None).
    #[props(optional)]
    pub default_selected_keys: Option<Vec<String>>,
    /// Controlled open keys (only meaningful in inline mode).
    #[props(optional)]
    pub open_keys: Option<Vec<String>>,
    /// Default open keys for uncontrolled mode (inline).
    #[props(optional)]
    pub default_open_keys: Option<Vec<String>>,
    /// Called when a leaf menu item is selected.
    #[props(optional)]
    pub on_select: Option<EventHandler<String>>,
    /// Called when open keys change (inline mode).
    #[props(optional)]
    pub on_open_change: Option<EventHandler<Vec<String>>>,
    /// When true, inline menu is collapsed (typically used with Sider).
    #[props(default)]
    pub inline_collapsed: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Menu (MVP).
#[component]
pub fn Menu(props: MenuProps) -> Element {
    let MenuProps {
        items,
        mode,
        selected_keys,
        default_selected_keys,
        open_keys,
        default_open_keys,
        on_select,
        on_open_change,
        inline_collapsed,
        class,
        style,
    } = props;

    // Internal state for uncontrolled selected keys.
    let selected_internal: Signal<Vec<String>> =
        use_signal(|| default_selected_keys.unwrap_or_else(Vec::new));

    // Internal state for uncontrolled open keys (inline mode only).
    let open_internal: Signal<Vec<String>> =
        use_signal(|| default_open_keys.unwrap_or_else(Vec::new));

    let current_selected = selected_keys
        .clone()
        .unwrap_or_else(|| selected_internal.read().clone());
    let current_open = open_keys
        .clone()
        .unwrap_or_else(|| open_internal.read().clone());

    // Root classes.
    let mut class_list = vec!["adui-menu".to_string()];
    match mode {
        MenuMode::Inline => class_list.push("adui-menu-inline".into()),
        MenuMode::Horizontal => class_list.push("adui-menu-horizontal".into()),
    }
    if inline_collapsed && matches!(mode, MenuMode::Inline) {
        class_list.push("adui-menu-inline-collapsed".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Event handlers use cloned signals to update internal state when uncontrolled.
    let on_select_cb = on_select;
    let on_open_change_cb = on_open_change;
    let selected_signal = selected_internal;
    let open_signal = open_internal;
    let is_selected_controlled = selected_keys.is_some();
    let is_open_controlled = open_keys.is_some();

    rsx! {
        nav {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "menu",
            ul {
                class: "adui-menu-list",
                {items.into_iter().map(|item| {
                    let key = item.id.clone();
                    let label = item.label.clone();
                    let icon = item.icon.clone();
                    let disabled = item.disabled;
                    let children = item.children.clone().unwrap_or_default();
                    let is_leaf = children.is_empty();
                    let selected_snapshot = current_selected.clone();
                    let open_snapshot = current_open.clone();
                    let selected_signal_for_item = selected_signal;
                    let open_signal_for_item = open_signal;
                    let on_select_item = on_select_cb;
                    let on_open_change_item = on_open_change_cb;

                    let is_selected = selected_snapshot.contains(&key);
                    let is_open = open_snapshot.contains(&key);

                    rsx! {
                        li {
                            class: {
                                let mut classes = vec!["adui-menu-item".to_string()];
                                if !is_leaf {
                                    classes.push("adui-menu-submenu".into());
                                }
                                if is_selected {
                                    classes.push("adui-menu-item-selected".into());
                                }
                                if is_open && !inline_collapsed && matches!(mode, MenuMode::Inline) {
                                    classes.push("adui-menu-submenu-open".into());
                                }
                                if disabled {
                                    classes.push("adui-menu-item-disabled".into());
                                }
                                classes.join(" ")
                            },
                            role: "menuitem",
                            onclick: move |_| {
                                if disabled {
                                    return;
                                }
                                if is_leaf {
                                    // Update selected keys in uncontrolled mode.
                                    if !is_selected_controlled {
                                        let mut signal = selected_signal_for_item;
                                        signal.set(vec![key.clone()]);
                                    }
                                    if let Some(cb) = on_select_item {
                                        cb.call(key.clone());
                                    }
                                } else if matches!(mode, MenuMode::Inline) {
                                    // Toggle open state for inline submenu.
                                    let mut next = open_snapshot.clone();
                                    if let Some(pos) = next.iter().position(|k| k == &key) {
                                        next.remove(pos);
                                    } else {
                                        next.push(key.clone());
                                    }
                                    if !is_open_controlled {
                                        let mut signal = open_signal_for_item;
                                        signal.set(next.clone());
                                    }
                                    if let Some(cb) = on_open_change_item {
                                        cb.call(next);
                                    }
                                }
                            },
                            div { class: "adui-menu-item-title",
                                if let Some(icon_node) = icon {
                                    span { class: "adui-menu-item-icon", {icon_node} }
                                }
                                span { class: "adui-menu-item-label", "{label}" }
                            }
                            if !children.is_empty() && matches!(mode, MenuMode::Inline) && !inline_collapsed {
                                ul {
                                    class: "adui-menu-submenu-list",
                                    style: if is_open { "display: block;" } else { "display: none;" },
                                    {children.into_iter().map(|child| {
                                        let child_key = child.id.clone();
                                        let child_label = child.label.clone();
                                        let child_icon = child.icon.clone();
                                        let child_disabled = child.disabled;
                                        let selected_snapshot = selected_signal.read().clone();
                                        let is_selected_child = selected_snapshot.contains(&child_key);
                                        let selected_signal_child = selected_signal;
                                        let on_select_child = on_select_cb;

                                        rsx! {
                                            li {
                                                class: {
                                                    let mut classes = vec!["adui-menu-item".to_string()];
                                                    classes.push("adui-menu-submenu-item".into());
                                                    if is_selected_child {
                                                        classes.push("adui-menu-item-selected".into());
                                                    }
                                                    if child_disabled {
                                                        classes.push("adui-menu-item-disabled".into());
                                                    }
                                                    classes.join(" ")
                                                },
                                                role: "menuitem",
                                                onclick: move |_| {
                                                    if child_disabled {
                                                        return;
                                                    }
                                                    if !is_selected_controlled {
                                                        let mut signal = selected_signal_child;
                                                        signal.set(vec![child_key.clone()]);
                                                    }
                                                    if let Some(cb) = on_select_child {
                                                        cb.call(child_key.clone());
                                                    }
                                                },
                                                div { class: "adui-menu-item-title",
                                                    if let Some(icon_node) = child_icon {
                                                        span { class: "adui-menu-item-icon", {icon_node} }
                                                    }
                                                    span { class: "adui-menu-item-label", "{child_label}" }
                                                }
                                            }
                                        }
                                    })}
                                }
                            }
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
    fn menu_mode_default() {
        assert_eq!(MenuMode::default(), MenuMode::Inline);
    }

    #[test]
    fn menu_mode_all_variants() {
        assert_eq!(MenuMode::Inline, MenuMode::Inline);
        assert_eq!(MenuMode::Horizontal, MenuMode::Horizontal);
        assert_ne!(MenuMode::Inline, MenuMode::Horizontal);
    }

    #[test]
    fn menu_mode_clone() {
        let original = MenuMode::Horizontal;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn menu_item_node_leaf() {
        let item = MenuItemNode::leaf("item1", "Item 1");
        assert_eq!(item.id, "item1");
        assert_eq!(item.label, "Item 1");
        assert!(item.icon.is_none());
        assert_eq!(item.disabled, false);
        assert!(item.children.is_none());
    }

    #[test]
    fn menu_item_node_leaf_with_strings() {
        let item = MenuItemNode::leaf(String::from("item2"), String::from("Item 2"));
        assert_eq!(item.id, "item2");
        assert_eq!(item.label, "Item 2");
        assert_eq!(item.disabled, false);
    }

    #[test]
    fn menu_item_node_clone() {
        let item1 = MenuItemNode::leaf("item1", "Item 1");
        let item2 = item1.clone();
        assert!(item1 == item2);
    }

    #[test]
    fn menu_item_node_partial_eq() {
        let item1 = MenuItemNode::leaf("item1", "Item 1");
        let item2 = MenuItemNode::leaf("item1", "Item 1");
        let item3 = MenuItemNode::leaf("item2", "Item 2");
        assert!(item1 == item2);
        assert!(item1 != item3);
    }

    #[test]
    fn menu_item_node_with_children() {
        let child1 = MenuItemNode::leaf("child1", "Child 1");
        let child2 = MenuItemNode::leaf("child2", "Child 2");
        let parent = MenuItemNode {
            id: "parent".to_string(),
            label: "Parent".to_string(),
            icon: None,
            disabled: false,
            children: Some(vec![child1, child2]),
        };
        assert_eq!(parent.id, "parent");
        assert_eq!(parent.label, "Parent");
        assert!(parent.children.is_some());
        assert_eq!(parent.children.as_ref().unwrap().len(), 2);
    }
}
