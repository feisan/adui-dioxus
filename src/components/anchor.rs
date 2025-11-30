//! Anchor component - navigation links for scrolling within a page.
//!
//! Ported from Ant Design's Anchor component.

#![allow(unpredictable_function_pointer_comparisons)]

use crate::components::affix::Affix;
use dioxus::prelude::*;

/// Direction of the anchor navigation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnchorDirection {
    /// Vertical anchor navigation (default).
    #[default]
    Vertical,
    /// Horizontal anchor navigation.
    Horizontal,
}

impl AnchorDirection {
    fn as_class(&self) -> &'static str {
        match self {
            AnchorDirection::Vertical => "adui-anchor-vertical",
            AnchorDirection::Horizontal => "adui-anchor-horizontal",
        }
    }
}

/// Data model for a single anchor link item.
#[derive(Clone, PartialEq)]
pub struct AnchorLinkItem {
    /// Unique key for the item.
    pub key: String,
    /// The target anchor href (e.g., "#section-1").
    pub href: String,
    /// The display title for the link.
    pub title: String,
    /// Optional target attribute for the anchor element.
    pub target: Option<String>,
    /// Nested child items (only for vertical direction).
    pub children: Option<Vec<AnchorLinkItem>>,
}

impl AnchorLinkItem {
    /// Create a new anchor link item.
    pub fn new(key: impl Into<String>, href: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            href: href.into(),
            title: title.into(),
            target: None,
            children: None,
        }
    }

    /// Create an anchor link item with children.
    pub fn with_children(
        key: impl Into<String>,
        href: impl Into<String>,
        title: impl Into<String>,
        children: Vec<AnchorLinkItem>,
    ) -> Self {
        Self {
            key: key.into(),
            href: href.into(),
            title: title.into(),
            target: None,
            children: Some(children),
        }
    }
}

/// Context for Anchor component shared with AnchorLink children.
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct AnchorContext {
    /// Currently active link href.
    active_link: Signal<Option<String>>,
    /// Direction of the anchor.
    direction: AnchorDirection,
}

/// Props for the Anchor component.
#[derive(Props, Clone, PartialEq)]
#[allow(clippy::fn_address_comparisons)]
pub struct AnchorProps {
    /// List of anchor link items.
    pub items: Vec<AnchorLinkItem>,

    /// Whether to use Affix to fix the anchor when scrolling.
    /// Default is true.
    #[props(default = true)]
    pub affix: bool,

    /// Offset from the top of the viewport when affixed (in pixels).
    #[props(optional)]
    pub offset_top: Option<f64>,

    /// Bounding distance from the anchor when calculating active state (in pixels).
    /// Default is 5.
    #[props(default = 5.0)]
    pub bounds: f64,

    /// Scroll offset from the top when clicking an anchor (in pixels).
    /// If not set, equals `offset_top`.
    #[props(optional)]
    pub target_offset: Option<f64>,

    /// Direction of the anchor navigation.
    #[props(default)]
    pub direction: AnchorDirection,

    /// Whether to replace the current history entry when clicking an anchor.
    #[props(default)]
    pub replace: bool,

    /// Show ink indicator in fixed mode.
    #[props(default)]
    pub show_ink_in_fixed: bool,

    /// Callback when the active link changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<String>>,

    /// Callback when an anchor link is clicked.
    #[props(optional)]
    pub on_click: Option<EventHandler<AnchorClickInfo>>,

    /// Custom function to determine the current active anchor.
    /// Note: Function pointer comparison may not be meaningful.
    #[props(optional)]
    pub get_current_anchor: Option<fn(String) -> String>,

    /// Additional CSS class for the anchor wrapper.
    #[props(optional)]
    pub class: Option<String>,

    /// Additional inline styles for the anchor wrapper.
    #[props(optional)]
    pub style: Option<String>,
}

/// Information passed to the on_click callback.
#[derive(Clone, Debug)]
pub struct AnchorClickInfo {
    pub href: String,
    pub title: String,
}

/// Anchor component - provides navigation links that scroll to sections within a page.
///
/// # Example
///
/// ```rust,ignore
/// use adui_dioxus::{Anchor, AnchorLinkItem};
///
/// rsx! {
///     Anchor {
///         items: vec![
///             AnchorLinkItem::new("1", "#section-1", "Section 1"),
///             AnchorLinkItem::new("2", "#section-2", "Section 2"),
///         ],
///     }
/// }
/// ```
#[component]
pub fn Anchor(props: AnchorProps) -> Element {
    let AnchorProps {
        items,
        affix,
        offset_top,
        bounds,
        target_offset,
        direction,
        replace,
        show_ink_in_fixed,
        on_change,
        on_click,
        get_current_anchor,
        class,
        style,
    } = props;

    let active_link: Signal<Option<String>> = use_signal(|| None);
    let animating: Signal<bool> = use_signal(|| false);
    let registered_links: Signal<Vec<String>> = use_signal(Vec::new);

    // Calculate effective target offset
    let effective_target_offset = target_offset.unwrap_or(offset_top.unwrap_or(0.0));

    // Provide context
    use_context_provider(|| AnchorContext {
        active_link,
        direction,
    });

    // Collect all hrefs from items recursively
    let all_hrefs = collect_hrefs(&items);

    // Silence warnings for non-wasm targets
    let _ = (
        &animating,
        &registered_links,
        &bounds,
        &effective_target_offset,
        &on_change,
        &get_current_anchor,
        &all_hrefs,
    );

    // Set up scroll listener to update active link
    #[cfg(target_arch = "wasm32")]
    {
        let items_for_effect = items.clone();
        let target_offset_val = effective_target_offset;
        let bounds_val = bounds;
        let on_change_cb = on_change;
        let get_current_anchor_fn = get_current_anchor;
        let mut active_link_signal = active_link;
        let animating_signal = animating;

        use_effect(move || {
            use wasm_bindgen::{JsCast, closure::Closure};

            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };

            let items_clone = items_for_effect.clone();

            let handler = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(
                move |_evt: web_sys::Event| {
                    if *animating_signal.read() {
                        return;
                    }

                    let Some(window) = web_sys::window() else {
                        return;
                    };
                    let Some(document) = window.document() else {
                        return;
                    };

                    let hrefs = collect_hrefs(&items_clone);
                    let current = get_internal_current_anchor(
                        &document,
                        &hrefs,
                        target_offset_val,
                        bounds_val,
                    );

                    // Apply custom getCurrentAnchor if provided
                    let final_link = if let Some(custom_fn) = get_current_anchor_fn {
                        custom_fn(current.clone())
                    } else {
                        current
                    };

                    let prev = active_link_signal.read().clone();
                    if prev.as_deref() != Some(&final_link) && !final_link.is_empty() {
                        active_link_signal.set(Some(final_link.clone()));
                        if let Some(cb) = on_change_cb {
                            cb.call(final_link);
                        }
                    }
                },
            ));

            let _ =
                window.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref());
            handler.forget();
        });
    }

    // Build class names
    let mut class_list = vec![
        "adui-anchor-wrapper".to_string(),
        direction.as_class().to_string(),
    ];
    if !affix && !show_ink_in_fixed {
        class_list.push("adui-anchor-fixed".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    // Build style
    let style_attr = {
        let max_height = if let Some(top) = offset_top {
            format!("max-height: calc(100vh - {}px);", top)
        } else {
            "max-height: 100vh;".to_string()
        };
        let extra = style.unwrap_or_default();
        format!("{} {}", max_height, extra)
    };

    let current_active = active_link.read().clone();
    let on_click_cb = on_click;
    let replace_flag = replace;

    let anchor_content = rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            div { class: "adui-anchor",
                // Ink indicator
                AnchorInk {
                    active_link: current_active.clone(),
                    direction: Some(direction),
                }

                // Render items
                {items.iter().map(|item| {
                    let href = item.href.clone();
                    let title = item.title.clone();
                    let target = item.target.clone();
                    let nested = item.children.clone();
                    let key = item.key.clone();
                    let is_active = current_active.as_ref() == Some(&href);

                    rsx! {
                        AnchorLink {
                            key: "{key}",
                            href: href,
                            title: title,
                            target: target,
                            nested_items: nested,
                            active: is_active,
                            direction: direction,
                            replace: replace_flag,
                            on_click: on_click_cb,
                        }
                    }
                })}
            }
        }
    };

    if affix {
        rsx! {
            Affix { offset_top: offset_top.unwrap_or(0.0),
                {anchor_content}
            }
        }
    } else {
        anchor_content
    }
}

/// Props for the ink indicator.
#[derive(Props, Clone, PartialEq)]
struct AnchorInkProps {
    active_link: Option<String>,
    #[props(optional)]
    direction: Option<AnchorDirection>,
}

/// Ink indicator component for the anchor.
#[component]
fn AnchorInk(props: AnchorInkProps) -> Element {
    let AnchorInkProps {
        active_link,
        direction: _direction,
    } = props;

    let visible = active_link.is_some();

    let class_attr = format!(
        "adui-anchor-ink{}",
        if visible {
            " adui-anchor-ink-visible"
        } else {
            ""
        }
    );

    // The ink position would be calculated in WASM based on the active link element
    // For now, we just show/hide the ink
    rsx! {
        span { class: "{class_attr}" }
    }
}

/// Props for individual anchor link.
#[derive(Props, Clone, PartialEq)]
struct AnchorLinkProps {
    href: String,
    title: String,
    #[props(optional)]
    target: Option<String>,
    #[props(optional)]
    nested_items: Option<Vec<AnchorLinkItem>>,
    active: bool,
    direction: AnchorDirection,
    replace: bool,
    #[props(optional)]
    on_click: Option<EventHandler<AnchorClickInfo>>,
}

/// Individual anchor link component.
#[component]
fn AnchorLink(props: AnchorLinkProps) -> Element {
    let AnchorLinkProps {
        href,
        title,
        target,
        nested_items,
        active,
        direction,
        replace,
        on_click,
    } = props;

    let link_class = format!(
        "adui-anchor-link{}",
        if active {
            " adui-anchor-link-active"
        } else {
            ""
        }
    );

    let title_class = format!(
        "adui-anchor-link-title{}",
        if active {
            " adui-anchor-link-title-active"
        } else {
            ""
        }
    );

    let href_for_click = href.clone();
    let title_for_click = title.clone();
    let replace_for_click = replace;

    rsx! {
        div { class: "{link_class}",
            a {
                class: "{title_class}",
                href: "{href}",
                target: target.as_deref().unwrap_or(""),
                title: "{title}",
                onclick: move |evt| {
                    // Call user callback
                    if let Some(cb) = on_click {
                        cb.call(AnchorClickInfo {
                            href: href_for_click.clone(),
                            title: title_for_click.clone(),
                        });
                    }

                    // Handle scroll to target
                    #[cfg(target_arch = "wasm32")]
                    {
                        handle_anchor_click(&href_for_click, replace_for_click);
                    }

                    // Silence warning for non-wasm targets
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = replace_for_click;

                    // Prevent default for internal anchors
                    if href_for_click.starts_with('#') {
                        evt.prevent_default();
                    }
                },
                "{title}"
            }

            // Render nested children (only for vertical direction)
            if matches!(direction, AnchorDirection::Vertical) {
                if let Some(nested) = nested_items {
                    {nested.iter().map(|child| {
                        let child_href = child.href.clone();
                        let child_title = child.title.clone();
                        let child_target = child.target.clone();
                        let child_nested = child.children.clone();
                        let child_key = child.key.clone();
                        // For nested items, we'd need to check if they're active too
                        // This is simplified for MVP

                        rsx! {
                            AnchorLink {
                                key: "{child_key}",
                                href: child_href,
                                title: child_title,
                                target: child_target,
                                nested_items: child_nested,
                                active: false,
                                direction: direction,
                                replace: replace,
                                on_click: on_click,
                            }
                        }
                    })}
                }
            }
        }
    }
}

/// Collect all hrefs from anchor items recursively.
fn collect_hrefs(items: &[AnchorLinkItem]) -> Vec<String> {
    let mut hrefs = Vec::new();
    for item in items {
        hrefs.push(item.href.clone());
        if let Some(children) = &item.children {
            hrefs.extend(collect_hrefs(children));
        }
    }
    hrefs
}

/// Get the currently active anchor based on scroll position.
#[cfg(target_arch = "wasm32")]
fn get_internal_current_anchor(
    document: &web_sys::Document,
    hrefs: &[String],
    offset_top: f64,
    bounds: f64,
) -> String {
    use wasm_bindgen::JsCast;

    let mut sections: Vec<(String, f64)> = Vec::new();

    for href in hrefs {
        // Extract the ID from href (e.g., "#section-1" -> "section-1")
        if let Some(id) = href.strip_prefix('#') {
            if let Some(element) = document.get_element_by_id(id) {
                let rect = element.get_bounding_client_rect();
                let top = rect.top();

                // Check if element is within the threshold
                if top <= offset_top + bounds {
                    sections.push((href.clone(), top));
                }
            }
        }
    }

    // Return the section closest to the top
    if let Some((href, _)) = sections
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    {
        href.clone()
    } else {
        String::new()
    }
}

/// Handle anchor click - scroll to target and update history.
#[cfg(target_arch = "wasm32")]
fn handle_anchor_click(href: &str, replace: bool) {
    use wasm_bindgen::JsCast;

    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };

    let document = match window.document() {
        Some(d) => d,
        None => return,
    };

    // Check if it's an internal anchor
    if let Some(id) = href.strip_prefix('#') {
        if let Some(element) = document.get_element_by_id(id) {
            // Scroll to element
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_behavior(web_sys::ScrollBehavior::Smooth);
            options.set_block(web_sys::ScrollLogicalPosition::Start);
            element.scroll_into_view_with_scroll_into_view_options(&options);

            // Update history
            let history = window.history().ok();
            if let Some(h) = history {
                if replace {
                    let _ = h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(href));
                } else {
                    let _ = h.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(href));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_direction_classes_are_correct() {
        assert_eq!(AnchorDirection::Vertical.as_class(), "adui-anchor-vertical");
        assert_eq!(
            AnchorDirection::Horizontal.as_class(),
            "adui-anchor-horizontal"
        );
    }

    #[test]
    fn anchor_link_item_creation() {
        let item = AnchorLinkItem::new("key1", "#section", "Section Title");
        assert_eq!(item.key, "key1");
        assert_eq!(item.href, "#section");
        assert_eq!(item.title, "Section Title");
        assert!(item.children.is_none());
    }

    #[test]
    fn anchor_link_item_with_children() {
        let children = vec![AnchorLinkItem::new("child1", "#child", "Child Section")];
        let item =
            AnchorLinkItem::with_children("parent", "#parent", "Parent Section", children.clone());
        assert_eq!(item.children.unwrap().len(), 1);
    }

    #[test]
    fn collect_hrefs_works_recursively() {
        let items = vec![
            AnchorLinkItem::new("1", "#section-1", "Section 1"),
            AnchorLinkItem::with_children(
                "2",
                "#section-2",
                "Section 2",
                vec![AnchorLinkItem::new("2-1", "#section-2-1", "Section 2.1")],
            ),
        ];

        let hrefs = collect_hrefs(&items);
        assert_eq!(hrefs.len(), 3);
        assert!(hrefs.contains(&"#section-1".to_string()));
        assert!(hrefs.contains(&"#section-2".to_string()));
        assert!(hrefs.contains(&"#section-2-1".to_string()));
    }
}
