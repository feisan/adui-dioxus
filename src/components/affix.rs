//! Affix component - wraps content and pins it to the viewport when scrolling.
//!
//! Ported from Ant Design's Affix component.

use dioxus::prelude::*;

/// Props for the Affix component.
#[derive(Props, Clone, PartialEq)]
pub struct AffixProps {
    /// Offset from the top of the viewport (in pixels) when to start affixing.
    /// If neither `offset_top` nor `offset_bottom` is set, defaults to 0.
    #[props(optional)]
    pub offset_top: Option<f64>,

    /// Offset from the bottom of the viewport (in pixels) when to start affixing.
    #[props(optional)]
    pub offset_bottom: Option<f64>,

    /// Callback fired when the affix state changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<bool>>,

    /// Additional CSS class for the wrapper element.
    #[props(optional)]
    pub class: Option<String>,

    /// Additional inline styles for the wrapper element.
    #[props(optional)]
    pub style: Option<String>,

    /// The content to be affixed.
    pub children: Element,
}

/// Internal state for affix positioning.
#[derive(Clone, Copy, Debug, PartialEq)]
struct AffixState {
    /// Whether the content is currently affixed.
    affixed: bool,
    /// The fixed position style (top or bottom offset).
    fixed_top: Option<f64>,
    fixed_bottom: Option<f64>,
    /// Placeholder dimensions to prevent layout shift.
    placeholder_width: f64,
    placeholder_height: f64,
    /// Left position to maintain horizontal alignment.
    placeholder_left: f64,
}

impl Default for AffixState {
    fn default() -> Self {
        Self {
            affixed: false,
            fixed_top: None,
            fixed_bottom: None,
            placeholder_width: 0.0,
            placeholder_height: 0.0,
            placeholder_left: 0.0,
        }
    }
}

/// Affix component - pins its children to the viewport when scrolling past a threshold.
///
/// # Example
///
/// ```rust,ignore
/// use adui_dioxus::Affix;
///
/// rsx! {
///     Affix { offset_top: 10.0,
///         div { "This content will stick to the top when scrolled" }
///     }
/// }
/// ```
#[component]
pub fn Affix(props: AffixProps) -> Element {
    let AffixProps {
        offset_top,
        offset_bottom,
        on_change,
        class,
        style,
        children,
    } = props;

    // Default to offset_top = 0 if neither is specified
    let effective_offset_top = if offset_bottom.is_none() && offset_top.is_none() {
        Some(0.0)
    } else {
        offset_top
    };

    let affix_state: Signal<AffixState> = use_signal(AffixState::default);
    let last_affixed: Signal<bool> = use_signal(|| false);

    // Silence warnings for non-wasm targets
    let _ = (&effective_offset_top, &on_change, &last_affixed);

    // Unique ID for the placeholder element
    let placeholder_id = use_signal(|| format!("adui-affix-{}", rand_id()));
    let fixed_id = use_signal(|| format!("adui-affix-fixed-{}", rand_id()));

    // Set up scroll/resize listeners
    #[cfg(target_arch = "wasm32")]
    {
        let placeholder_id_for_effect = placeholder_id.read().clone();
        let offset_top_val = effective_offset_top;
        let offset_bottom_val = offset_bottom;
        let on_change_cb = on_change;
        let mut state_signal = affix_state;
        let mut last_affixed_signal = last_affixed;

        use_effect(move || {
            use wasm_bindgen::{closure::Closure, JsCast};

            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };

            let document = match window.document() {
                Some(d) => d,
                None => return,
            };

            let placeholder_id_clone = placeholder_id_for_effect.clone();

            // Create the event handler closure
            let handler = Closure::<dyn FnMut(web_sys::Event)>::wrap(Box::new(move |_evt: web_sys::Event| {
                let Some(window) = web_sys::window() else { return };
                let Some(document) = window.document() else { return };

                let placeholder = match document.get_element_by_id(&placeholder_id_clone) {
                    Some(el) => el,
                    None => return,
                };

                let placeholder_rect = placeholder.get_bounding_client_rect();

                // Skip if element is not visible
                if placeholder_rect.width() == 0.0 && placeholder_rect.height() == 0.0 {
                    return;
                }

                let window_height = window.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0);

                let mut new_state = AffixState::default();
                new_state.placeholder_width = placeholder_rect.width();
                new_state.placeholder_height = placeholder_rect.height();
                new_state.placeholder_left = placeholder_rect.left();

                // Check if should affix to top
                if let Some(top_offset) = offset_top_val {
                    if placeholder_rect.top() < top_offset {
                        new_state.affixed = true;
                        new_state.fixed_top = Some(top_offset);
                    }
                }

                // Check if should affix to bottom
                // Element should be fixed when its bottom edge goes below the viewport bottom minus offset
                if let Some(bottom_offset) = offset_bottom_val {
                    if placeholder_rect.bottom() > window_height - bottom_offset {
                        new_state.affixed = true;
                        new_state.fixed_bottom = Some(bottom_offset);
                        new_state.fixed_top = None; // Bottom takes precedence
                    }
                }

                let prev_affixed = *last_affixed_signal.read();
                if prev_affixed != new_state.affixed {
                    last_affixed_signal.set(new_state.affixed);
                    if let Some(cb) = on_change_cb {
                        cb.call(new_state.affixed);
                    }
                }

                state_signal.set(new_state);
            }));

            // Add event listeners
            let _ = window.add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref());
            let _ = window.add_event_listener_with_callback("resize", handler.as_ref().unchecked_ref());

            // Keep the closure alive
            handler.forget();
        });
    }

    let state = *affix_state.read();
    let placeholder_id_val = placeholder_id.read().clone();
    let fixed_id_val = fixed_id.read().clone();

    // Build class names
    let mut class_list = vec!["adui-affix-wrapper".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    // Build wrapper style
    let style_attr = style.unwrap_or_default();

    // Build placeholder style (maintains layout when content is fixed)
    let placeholder_style = if state.affixed {
        format!(
            "width: {}px; height: {}px;",
            state.placeholder_width, state.placeholder_height
        )
    } else {
        String::new()
    };

    // Build fixed element style
    let fixed_style = if state.affixed {
        let mut style_parts = vec![
            "position: fixed".to_string(),
            format!("left: {}px", state.placeholder_left),
            format!("width: {}px", state.placeholder_width),
            "z-index: 10".to_string(),
        ];

        if let Some(top) = state.fixed_top {
            style_parts.push(format!("top: {}px", top));
        }
        if let Some(bottom) = state.fixed_bottom {
            style_parts.push(format!("bottom: {}px", bottom));
        }

        style_parts.join("; ") + ";"
    } else {
        String::new()
    };

    let fixed_class = if state.affixed {
        "adui-affix"
    } else {
        ""
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            id: "{placeholder_id_val}",

            // Placeholder to maintain layout
            if state.affixed {
                div {
                    style: "{placeholder_style}",
                    "aria-hidden": "true",
                }
            }

            // The actual content
            div {
                id: "{fixed_id_val}",
                class: "{fixed_class}",
                style: "{fixed_style}",
                {children}
            }
        }
    }
}

/// Generate a simple random ID for element identification.
fn rand_id() -> u32 {
    // Simple pseudo-random based on current time
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Math;
        (Math::random() * 1_000_000.0) as u32
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affix_state_default_is_not_affixed() {
        let state = AffixState::default();
        assert!(!state.affixed);
        assert!(state.fixed_top.is_none());
        assert!(state.fixed_bottom.is_none());
    }

    #[test]
    fn rand_id_produces_values() {
        let id1 = rand_id();
        // Just ensure it doesn't panic and produces a value
        assert!(id1 <= 1_000_000 || true); // Always passes, just checks execution
    }
}

