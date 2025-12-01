//! Tour component for guided user onboarding.
//!
//! A popup component for guiding users through a product. It displays a series
//! of steps, highlighting UI elements and providing descriptions.
//!
//! # Features
//! - Step-by-step guidance with navigation
//! - Customizable placement for each step
//! - Highlight mask for focused elements
//! - Keyboard navigation (arrow keys, Escape)
//! - Primary and default visual variants
//!
//! # Example
//! ```rust,ignore
//! use adui_dioxus::components::tour::{Tour, TourStep};
//!
//! let steps = vec![
//!     TourStep::new("step1", "Welcome", "This is the first step"),
//!     TourStep::new("step2", "Feature", "Explore this feature"),
//! ];
//!
//! rsx! {
//!     Tour {
//!         open: show_tour(),
//!         steps: steps,
//!         on_close: move |_| set_show_tour(false),
//!         on_finish: move |_| { /* tour completed */ },
//!     }
//! }
//! ```

use crate::components::button::{Button, ButtonColor, ButtonVariant};
use crate::components::overlay::{OverlayKey, OverlayKind, use_overlay};
use crate::components::tooltip::TooltipPlacement;
use crate::theme::use_theme;
use dioxus::events::KeyboardEvent;
use dioxus::prelude::*;

/// Visual type of the tour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TourType {
    /// Default style with light background.
    #[default]
    Default,
    /// Primary style with colored background.
    Primary,
}

impl TourType {
    fn as_class(&self) -> &'static str {
        match self {
            TourType::Default => "adui-tour-default",
            TourType::Primary => "adui-tour-primary",
        }
    }
}

/// Data model for a single tour step.
#[derive(Clone, PartialEq)]
pub struct TourStep {
    /// Unique key for the step.
    pub key: String,
    /// Title displayed in the tour panel.
    pub title: Option<String>,
    /// Description text or element.
    pub description: Option<Element>,
    /// Cover image or element displayed above the content.
    pub cover: Option<Element>,
    /// Placement of the tour panel relative to the target.
    pub placement: Option<TooltipPlacement>,
    /// CSS selector for the target element to highlight.
    /// When None, the panel is centered on screen.
    pub target: Option<String>,
    /// Custom "Next" button text.
    pub next_button_text: Option<String>,
    /// Custom "Previous" button text.
    pub prev_button_text: Option<String>,
}

impl TourStep {
    /// Create a new tour step with title and description.
    pub fn new(
        key: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let desc_text = description.into();
        Self {
            key: key.into(),
            title: Some(title.into()),
            description: Some(rsx! { "{desc_text}" }),
            cover: None,
            placement: None,
            target: None,
            next_button_text: None,
            prev_button_text: None,
        }
    }

    /// Set the target CSS selector.
    pub fn target(mut self, selector: impl Into<String>) -> Self {
        self.target = Some(selector.into());
        self
    }

    /// Set the placement of the tour panel.
    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = Some(placement);
        self
    }

    /// Set a cover element.
    pub fn cover(mut self, cover: Element) -> Self {
        self.cover = Some(cover);
        self
    }

    /// Set custom description element.
    pub fn description_element(mut self, desc: Element) -> Self {
        self.description = Some(desc);
        self
    }

    /// Set custom next button text.
    pub fn next_button(mut self, text: impl Into<String>) -> Self {
        self.next_button_text = Some(text.into());
        self
    }

    /// Set custom prev button text.
    pub fn prev_button(mut self, text: impl Into<String>) -> Self {
        self.prev_button_text = Some(text.into());
        self
    }
}

/// Props for the Tour component.
#[derive(Props, Clone, PartialEq)]
pub struct TourProps {
    /// Whether the tour is visible.
    pub open: bool,
    /// Tour steps to display.
    pub steps: Vec<TourStep>,
    /// Controlled current step index.
    #[props(optional)]
    pub current: Option<usize>,
    /// Called when the tour is closed (via close button, mask click, or Escape).
    #[props(optional)]
    pub on_close: Option<EventHandler<()>>,
    /// Called when the current step changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<usize>>,
    /// Called when the user completes the tour.
    #[props(optional)]
    pub on_finish: Option<EventHandler<()>>,
    /// Visual type of the tour.
    #[props(default)]
    pub r#type: TourType,
    /// Whether clicking the mask should close the tour.
    #[props(default = true)]
    pub mask_closable: bool,
    /// Whether to show the close button.
    #[props(default = true)]
    pub closable: bool,
    /// Whether to show step indicators.
    #[props(default = true)]
    pub show_indicators: bool,
    /// Text for the "Next" button.
    #[props(optional)]
    pub next_button_text: Option<String>,
    /// Text for the "Previous" button.
    #[props(optional)]
    pub prev_button_text: Option<String>,
    /// Text for the "Finish" button.
    #[props(optional)]
    pub finish_button_text: Option<String>,
    /// Additional CSS class on the root container.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline styles applied to the root container.
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Tour component for user onboarding.
#[component]
pub fn Tour(props: TourProps) -> Element {
    let TourProps {
        open,
        steps,
        current,
        on_close,
        on_change,
        on_finish,
        r#type,
        mask_closable,
        closable,
        show_indicators,
        next_button_text,
        prev_button_text,
        finish_button_text,
        class,
        style,
    } = props;

    let theme = use_theme();
    let tokens = theme.tokens();

    // Overlay management for z-index
    let overlay = use_overlay();
    let tour_key: Signal<Option<OverlayKey>> = use_signal(|| None);
    let z_index: Signal<i32> = use_signal(|| 1000);

    {
        let overlay = overlay.clone();
        let mut key_signal = tour_key;
        let mut z_signal = z_index;
        use_effect(move || {
            if let Some(handle) = overlay.clone() {
                let current_key = *key_signal.read();
                if open {
                    if current_key.is_none() {
                        let (key, meta) = handle.open(OverlayKind::Modal, true);
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

    // Internal step state (uncontrolled mode)
    let internal_current: Signal<usize> = use_signal(|| 0);
    let is_controlled = current.is_some();
    let current_step = current.unwrap_or_else(|| *internal_current.read());

    // Reset internal state when tour opens
    {
        let mut internal = internal_current;
        use_effect(move || {
            if open && !is_controlled {
                internal.set(0);
            }
        });
    }

    if !open || steps.is_empty() {
        return rsx! {};
    }

    let total_steps = steps.len();
    let step = steps.get(current_step).cloned();

    let Some(step) = step else {
        return rsx! {};
    };

    let current_z = *z_index.read();
    let is_first = current_step == 0;
    let is_last = current_step == total_steps - 1;

    // Text for buttons
    let prev_text = step
        .prev_button_text
        .as_ref()
        .or(prev_button_text.as_ref())
        .cloned()
        .unwrap_or_else(|| "Previous".to_string());
    let next_text = step
        .next_button_text
        .as_ref()
        .or(next_button_text.as_ref())
        .cloned()
        .unwrap_or_else(|| "Next".to_string());
    let finish_text = finish_button_text
        .clone()
        .unwrap_or_else(|| "Finish".to_string());

    // Placement CSS
    let placement = step.placement.unwrap_or(TooltipPlacement::Bottom);
    let placement_style = match placement {
        TooltipPlacement::Top => "bottom: 60%; left: 50%; transform: translateX(-50%);",
        TooltipPlacement::Bottom => "top: 40%; left: 50%; transform: translateX(-50%);",
        TooltipPlacement::Left => "right: 60%; top: 50%; transform: translateY(-50%);",
        TooltipPlacement::Right => "left: 60%; top: 50%; transform: translateY(-50%);",
    };

    // Build root classes
    let mut class_list = vec!["adui-tour".to_string(), r#type.as_class().to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // Panel background based on type
    let panel_bg = match r#type {
        TourType::Default => tokens.color_bg_container.clone(),
        TourType::Primary => tokens.color_primary.clone(),
    };
    let panel_text = match r#type {
        TourType::Default => tokens.color_text.clone(),
        TourType::Primary => "#ffffff".to_string(),
    };

    let on_close_cb = on_close;
    let on_change_cb = on_change;
    let on_finish_cb = on_finish;

    let handle_close = move || {
        if let Some(cb) = on_close_cb {
            cb.call(());
        }
    };

    let handle_prev = {
        let on_change = on_change_cb;
        move || {
            if current_step > 0 {
                let next_step = current_step - 1;
                if let Some(cb) = on_change {
                    cb.call(next_step);
                }
                if !is_controlled {
                    let mut sig = internal_current;
                    sig.set(next_step);
                }
            }
        }
    };

    let handle_next = {
        let on_change = on_change_cb;
        let on_finish = on_finish_cb;
        move || {
            if is_last {
                if let Some(cb) = on_finish {
                    cb.call(());
                }
                if let Some(cb) = on_close_cb {
                    cb.call(());
                }
            } else {
                let next_step = current_step + 1;
                if let Some(cb) = on_change {
                    cb.call(next_step);
                }
                if !is_controlled {
                    let mut sig = internal_current;
                    sig.set(next_step);
                }
            }
        }
    };

    let handle_keydown = {
        move |evt: KeyboardEvent| {
            use dioxus::prelude::Key;

            match evt.key() {
                Key::Escape => {
                    evt.prevent_default();
                    handle_close();
                }
                Key::ArrowLeft => {
                    evt.prevent_default();
                    if !is_first {
                        handle_prev();
                    }
                }
                Key::ArrowRight | Key::Enter => {
                    evt.prevent_default();
                    handle_next();
                }
                _ => {}
            }
        }
    };

    rsx! {
        // Mask layer
        div {
            class: "adui-tour-mask",
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.45); z-index: {current_z};",
            onclick: move |_| {
                if mask_closable {
                    handle_close();
                }
            }
        }
        // Tour panel
        div {
            class: "{class_attr}",
            style: "position: fixed; {placement_style} z-index: {current_z + 1}; {style_attr}",
            tabindex: 0,
            onkeydown: handle_keydown,
            div {
                class: "adui-tour-content",
                style: "background: {panel_bg}; color: {panel_text}; border-radius: 8px; box-shadow: 0 6px 16px rgba(0,0,0,0.08), 0 3px 6px -4px rgba(0,0,0,0.12); max-width: 520px; min-width: 300px;",
                onclick: move |evt| {
                    evt.stop_propagation();
                },
                // Close button
                if closable {
                    button {
                        class: "adui-tour-close",
                        style: "position: absolute; top: 8px; right: 8px; border: none; background: none; cursor: pointer; font-size: 16px; color: {panel_text}; opacity: 0.65;",
                        r#type: "button",
                        onclick: move |_| handle_close(),
                        "×"
                    }
                }
                // Cover image
                if let Some(cover) = step.cover {
                    div {
                        class: "adui-tour-cover",
                        style: "padding: 16px 16px 0;",
                        {cover}
                    }
                }
                // Header with title
                div {
                    class: "adui-tour-header",
                    style: "padding: 16px 16px 8px;",
                    if let Some(title) = step.title {
                        div {
                            class: "adui-tour-title",
                            style: "font-weight: 600; font-size: 16px;",
                            "{title}"
                        }
                    }
                }
                // Description
                if let Some(desc) = step.description {
                    div {
                        class: "adui-tour-description",
                        style: "padding: 0 16px 16px; font-size: 14px; line-height: 1.5;",
                        {desc}
                    }
                }
                // Footer with indicators and buttons
                div {
                    class: "adui-tour-footer",
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-top: 1px solid rgba(128,128,128,0.2);",
                    // Indicators
                    if show_indicators && total_steps > 1 {
                        div {
                            class: "adui-tour-indicators",
                            style: "display: flex; gap: 4px;",
                            {(0..total_steps).map(|idx| {
                                let is_active = idx == current_step;
                                let indicator_bg = if is_active {
                                    match r#type {
                                        TourType::Default => tokens.color_primary.clone(),
                                        TourType::Primary => "#ffffff".to_string(),
                                    }
                                } else {
                                    "rgba(128,128,128,0.3)".to_string()
                                };
                                rsx! {
                                    span {
                                        key: "indicator-{idx}",
                                        class: "adui-tour-indicator",
                                        style: "width: 6px; height: 6px; border-radius: 50%; background: {indicator_bg}; transition: background 0.2s;",
                                    }
                                }
                            })}
                        }
                    } else {
                        div { class: "adui-tour-indicators-placeholder" }
                    }
                    // Action buttons
                    div {
                        class: "adui-tour-actions",
                        style: "display: flex; gap: 8px;",
                        if !is_first {
                            Button {
                                variant: Some(ButtonVariant::Outlined),
                                color: if r#type == TourType::Primary { Some(ButtonColor::Default) } else { None },
                                onclick: move |_| handle_prev(),
                                "{prev_text}"
                            }
                        }
                        Button {
                            variant: Some(ButtonVariant::Solid),
                            color: if r#type == TourType::Primary { Some(ButtonColor::Default) } else { Some(ButtonColor::Primary) },
                            onclick: move |_| handle_next(),
                            if is_last { "{finish_text}" } else { "{next_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tour_step_builder_works() {
        let step = TourStep::new("s1", "Title", "Description")
            .target("#my-element")
            .placement(TooltipPlacement::Top);

        assert_eq!(step.key, "s1");
        assert_eq!(step.title, Some("Title".to_string()));
        assert_eq!(step.target, Some("#my-element".to_string()));
        assert_eq!(step.placement, Some(TooltipPlacement::Top));
    }

    #[test]
    fn tour_step_with_all_options() {
        let step = TourStep::new("s2", "Step 2", "Description")
            .target("#target")
            .placement(TooltipPlacement::Bottom)
            .next_button("Continue")
            .prev_button("Back");

        assert_eq!(step.key, "s2");
        assert_eq!(step.title, Some("Step 2".to_string()));
        assert_eq!(step.target, Some("#target".to_string()));
        assert_eq!(step.placement, Some(TooltipPlacement::Bottom));
        assert_eq!(step.next_button_text, Some("Continue".to_string()));
        assert_eq!(step.prev_button_text, Some("Back".to_string()));
    }

    #[test]
    fn tour_step_minimal() {
        let step = TourStep::new("s3", "Title", "Description");
        assert_eq!(step.key, "s3");
        assert_eq!(step.title, Some("Title".to_string()));
        assert!(step.target.is_none());
        assert!(step.placement.is_none());
    }

    #[test]
    fn tour_step_clone() {
        let step1 = TourStep::new("s1", "Title", "Description")
            .target("#target")
            .placement(TooltipPlacement::Top);
        let step2 = step1.clone();
        assert_eq!(step1.key, step2.key);
        assert_eq!(step1.title, step2.title);
        assert_eq!(step1.target, step2.target);
        assert_eq!(step1.placement, step2.placement);
    }

    #[test]
    fn tour_type_class_names() {
        assert_eq!(TourType::Default.as_class(), "adui-tour-default");
        assert_eq!(TourType::Primary.as_class(), "adui-tour-primary");
    }

    #[test]
    fn tour_type_default() {
        assert_eq!(TourType::default(), TourType::Default);
    }

    #[test]
    fn tour_type_equality() {
        assert_eq!(TourType::Default, TourType::Default);
        assert_eq!(TourType::Primary, TourType::Primary);
        assert_ne!(TourType::Default, TourType::Primary);
    }

    #[test]
    fn tour_type_clone() {
        let t1 = TourType::Primary;
        let t2 = t1;
        assert_eq!(t1, t2);
    }
}
