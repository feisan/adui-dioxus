use crate::components::config_provider::{ComponentSize, use_config};
use crate::components::icon::{Icon, IconKind};
use crate::foundation::{
    ClassListExt, CollapseClassNames, CollapseSemantic, CollapseStyles, StyleStringExt,
};
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Function type for custom expand icon rendering.
/// Takes (panel_props, is_active) and returns Element.
pub type ExpandIconRenderFn = fn(&CollapsePanel, bool) -> Element;

/// Collapsible trigger type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapsibleType {
    /// Trigger by clicking header.
    Header,
    /// Trigger by clicking icon only.
    Icon,
    /// Disabled, cannot be triggered.
    Disabled,
}

/// Size variants for Collapse.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CollapseSize {
    Small,
    #[default]
    Middle,
    Large,
}

impl CollapseSize {
    fn from_global(size: ComponentSize) -> Self {
        match size {
            ComponentSize::Small => CollapseSize::Small,
            ComponentSize::Large => CollapseSize::Large,
            ComponentSize::Middle => CollapseSize::Middle,
        }
    }

    fn as_class(&self) -> &'static str {
        match self {
            CollapseSize::Small => "adui-collapse-sm",
            CollapseSize::Middle => "adui-collapse-md",
            CollapseSize::Large => "adui-collapse-lg",
        }
    }
}

/// Placement of the expand icon.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExpandIconPlacement {
    #[default]
    Start,
    End,
}

impl ExpandIconPlacement {
    fn as_class(&self) -> &'static str {
        match self {
            ExpandIconPlacement::Start => "adui-collapse-icon-start",
            ExpandIconPlacement::End => "adui-collapse-icon-end",
        }
    }
}

/// Data model for a single collapse panel.
#[derive(Clone, PartialEq)]
pub struct CollapsePanel {
    pub key: String,
    pub header: Element,
    pub content: Element,
    pub disabled: bool,
    pub show_arrow: bool,
    pub collapsible: Option<CollapsibleType>,
    pub extra: Option<Element>,
}

impl CollapsePanel {
    pub fn new(key: impl Into<String>, header: Element, content: Element) -> Self {
        Self {
            key: key.into(),
            header,
            content,
            disabled: false,
            show_arrow: true,
            collapsible: None,
            extra: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    pub fn collapsible(mut self, collapsible: CollapsibleType) -> Self {
        self.collapsible = Some(collapsible);
        self
    }

    pub fn extra(mut self, extra: Element) -> Self {
        self.extra = Some(extra);
        self
    }
}

// Function pointer only used for props equality in diffing.
#[allow(unpredictable_function_pointer_comparisons)]
/// Props for the Collapse component.
#[derive(Props, Clone, PartialEq)]
pub struct CollapseProps {
    /// Panel items to display.
    pub items: Vec<CollapsePanel>,
    /// Controlled active keys (expanded panels).
    #[props(optional)]
    pub active_key: Option<Vec<String>>,
    /// Default active keys for uncontrolled mode.
    #[props(optional)]
    pub default_active_key: Option<Vec<String>>,
    /// Called when active keys change.
    #[props(optional)]
    pub on_change: Option<EventHandler<Vec<String>>>,
    /// Accordion mode (only one panel can be expanded at a time).
    #[props(default)]
    pub accordion: bool,
    /// Whether to show border.
    #[props(default = true)]
    pub bordered: bool,
    /// Ghost mode (transparent background).
    #[props(default)]
    pub ghost: bool,
    /// Size variant.
    #[props(optional)]
    pub size: Option<CollapseSize>,
    /// Expand icon placement.
    #[props(default)]
    pub expand_icon_placement: ExpandIconPlacement,
    /// Default collapsible type for all panels.
    #[props(optional)]
    pub collapsible: Option<CollapsibleType>,
    /// Whether to destroy inactive panel content.
    #[props(default = true)]
    pub destroy_on_hidden: bool,
    /// Custom expand icon render function.
    #[props(optional)]
    pub expand_icon: Option<ExpandIconRenderFn>,
    /// Extra class name.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style.
    #[props(optional)]
    pub style: Option<String>,
    /// Semantic class names.
    #[props(optional)]
    pub class_names: Option<CollapseClassNames>,
    /// Semantic styles.
    #[props(optional)]
    pub styles: Option<CollapseStyles>,
}

/// Ant Design flavored Collapse component.
#[component]
pub fn Collapse(props: CollapseProps) -> Element {
    let CollapseProps {
        items,
        active_key,
        default_active_key,
        on_change,
        accordion,
        bordered,
        ghost,
        size,
        expand_icon_placement,
        collapsible,
        destroy_on_hidden,
        expand_icon,
        class,
        style,
        class_names,
        styles,
    } = props;

    let config = use_config();
    let theme = use_theme();
    let tokens = theme.tokens();

    // Resolve size
    let resolved_size = if let Some(s) = size {
        s
    } else {
        CollapseSize::from_global(config.size)
    };

    // Initialize active keys for uncontrolled mode
    let initial_keys = default_active_key.unwrap_or_default();
    let active_keys_internal: Signal<Vec<String>> = use_signal(|| initial_keys);

    let is_controlled = active_key.is_some();
    let current_active_keys = if is_controlled {
        active_key.clone().unwrap_or_default()
    } else {
        active_keys_internal.read().clone()
    };

    // Build root classes
    let mut class_list = vec!["adui-collapse".to_string()];
    class_list.push(resolved_size.as_class().to_string());
    class_list.push(expand_icon_placement.as_class().to_string());
    if !bordered {
        class_list.push("adui-collapse-borderless".into());
    }
    if ghost {
        class_list.push("adui-collapse-ghost".into());
    }
    class_list.push_semantic(&class_names, CollapseSemantic::Root);
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut style_attr = format!("border-color:{};", tokens.color_border);
    style_attr.push_str(&style.unwrap_or_default());
    style_attr.append_semantic(&styles, CollapseSemantic::Root);

    let on_change_cb = on_change;

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "group",
            {items.iter().map(|panel| {
                let key = panel.key.clone();
                let is_active = current_active_keys.contains(&key);
                let panel_disabled = panel.disabled;
                let panel_collapsible = panel.collapsible.or(collapsible).unwrap_or(CollapsibleType::Header);
                let show_arrow = panel.show_arrow;
                let header = panel.header.clone();
                let content = panel.content.clone();
                let extra = panel.extra.clone();

                let is_icon_only = matches!(panel_collapsible, CollapsibleType::Icon);
                let is_disabled = panel_disabled || matches!(panel_collapsible, CollapsibleType::Disabled);

                let mut panel_class = vec!["adui-collapse-item".to_string()];
                if is_active {
                    panel_class.push("adui-collapse-item-active".into());
                }
                if is_disabled {
                    panel_class.push("adui-collapse-item-disabled".into());
                }
                let panel_class_attr = panel_class.join(" ");

                let active_keys_for_toggle = active_keys_internal;
                let on_change_for_toggle = on_change_cb;
                let key_for_toggle = key.clone();
                let active_key_for_toggle = active_key.clone();

                rsx! {
                    div {
                        key: "{key}",
                        class: "{panel_class_attr}",
                        div {
                            class: "adui-collapse-header",
                            role: "button",
                            tabindex: if is_disabled { "-1" } else { "0" },
                            "aria-expanded": "{is_active}",
                            "aria-disabled": "{is_disabled}",
                            onclick: move |_| {
                                if is_disabled || is_icon_only {
                                    return;
                                }

                                if !is_controlled {
                                    let mut keys = active_keys_for_toggle;
                                    let current = keys.read().clone();
                                    let new_keys = if accordion {
                                        if current.contains(&key_for_toggle) {
                                            vec![]
                                        } else {
                                            vec![key_for_toggle.clone()]
                                        }
                                    } else {
                                        if current.contains(&key_for_toggle) {
                                            current.into_iter().filter(|k| k != &key_for_toggle).collect()
                                        } else {
                                            let mut new = current;
                                            new.push(key_for_toggle.clone());
                                            new
                                        }
                                    };
                                    keys.set(new_keys.clone());
                                    if let Some(cb) = on_change_for_toggle {
                                        cb.call(new_keys);
                                    }
                                } else {
                                    if let Some(cb) = on_change_for_toggle {
                                        let current = active_key_for_toggle.clone().unwrap_or_default();
                                        let new_keys = if accordion {
                                            if current.contains(&key_for_toggle) {
                                                vec![]
                                            } else {
                                                vec![key_for_toggle.clone()]
                                            }
                                        } else {
                                            if current.contains(&key_for_toggle) {
                                                current.into_iter().filter(|k| k != &key_for_toggle).collect()
                                            } else {
                                                let mut new = current;
                                                new.push(key_for_toggle.clone());
                                                new
                                            }
                                        };
                                        cb.call(new_keys);
                                    }
                                }
                            },
                            {show_arrow.then(|| {
                                let active_keys_for_icon = active_keys_internal;
                                let on_change_for_icon = on_change_cb;
                                let key_for_icon = key.clone();
                                let active_key_for_icon = active_key.clone();

                                // Use custom expand_icon if provided
                                let icon_element = if let Some(render_fn) = expand_icon {
                                    render_fn(panel, is_active)
                                } else {
                                    rsx! {
                                        Icon {
                                            kind: IconKind::ArrowRight,
                                            rotate: if is_active { Some(90.0) } else { None },
                                            aria_label: if is_active { "collapse" } else { "expand" },
                                        }
                                    }
                                };

                                rsx! {
                                    span {
                                        class: "adui-collapse-expand-icon",
                                        onclick: move |_| {
                                            if is_disabled || !is_icon_only {
                                                return;
                                            }

                                            if !is_controlled {
                                                let mut keys = active_keys_for_icon;
                                                let current = keys.read().clone();
                                                let new_keys = if accordion {
                                                    if current.contains(&key_for_icon) {
                                                        vec![]
                                                    } else {
                                                        vec![key_for_icon.clone()]
                                                    }
                                                } else {
                                                    if current.contains(&key_for_icon) {
                                                        current.into_iter().filter(|k| k != &key_for_icon).collect()
                                                    } else {
                                                        let mut new = current;
                                                        new.push(key_for_icon.clone());
                                                        new
                                                    }
                                                };
                                                keys.set(new_keys.clone());
                                                if let Some(cb) = on_change_for_icon {
                                                    cb.call(new_keys);
                                                }
                                            } else {
                                                if let Some(cb) = on_change_for_icon {
                                                    let current = active_key_for_icon.clone().unwrap_or_default();
                                                    let new_keys = if accordion {
                                                        if current.contains(&key_for_icon) {
                                                            vec![]
                                                        } else {
                                                            vec![key_for_icon.clone()]
                                                        }
                                                    } else {
                                                        if current.contains(&key_for_icon) {
                                                            current.into_iter().filter(|k| k != &key_for_icon).collect()
                                                        } else {
                                                            let mut new = current;
                                                            new.push(key_for_icon.clone());
                                                            new
                                                        }
                                                    };
                                                    cb.call(new_keys);
                                                }
                                            }
                                        },
                                        {icon_element}
                                    }
                                }
                            })},
                            span { class: "adui-collapse-header-text",
                                {header}
                            },
                            {extra.map(|e| rsx! {
                                span { class: "adui-collapse-extra",
                                    {e}
                                }
                            })},
                        },
                        // Content rendering based on destroy_on_hidden
                        if destroy_on_hidden {
                            {is_active.then(|| rsx! {
                                div {
                                    class: "adui-collapse-content",
                                    role: "region",
                                    div { class: "adui-collapse-content-box",
                                        {content}
                                    }
                                }
                            })}
                        } else {
                            div {
                                class: if is_active { "adui-collapse-content" } else { "adui-collapse-content adui-collapse-content-hidden" },
                                role: "region",
                                hidden: !is_active,
                                div { class: "adui-collapse-content-box",
                                    {content}
                                }
                            }
                        },
                    }
                }
            })}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapse_size_class_mapping_is_stable() {
        assert_eq!(CollapseSize::Small.as_class(), "adui-collapse-sm");
        assert_eq!(CollapseSize::Middle.as_class(), "adui-collapse-md");
        assert_eq!(CollapseSize::Large.as_class(), "adui-collapse-lg");
    }

    #[test]
    fn expand_icon_placement_class_mapping_is_stable() {
        assert_eq!(
            ExpandIconPlacement::Start.as_class(),
            "adui-collapse-icon-start"
        );
        assert_eq!(
            ExpandIconPlacement::End.as_class(),
            "adui-collapse-icon-end"
        );
    }
}
