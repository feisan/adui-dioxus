use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Configuration for the resizable splitter.
#[derive(Props, Clone, PartialEq)]
pub struct SplitterProps {
    #[props(default)]
    pub orientation: SplitterOrientation,
    #[props(optional)]
    pub split: Option<f32>,
    #[props(default = 0.5)]
    pub default_split: f32,
    #[props(optional)]
    pub on_change: Option<EventHandler<f32>>,
    #[props(optional)]
    pub on_moving: Option<EventHandler<f32>>,
    #[props(optional)]
    pub on_release: Option<EventHandler<f32>>,
    #[props(optional)]
    pub min_primary: Option<f32>,
    #[props(optional)]
    pub min_secondary: Option<f32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub gutter_aria_label: Option<String>,
    pub first: Element,
    pub second: Element,
}

/// Splitter with draggable gutter to resize two panes.
#[component]
pub fn Splitter(props: SplitterProps) -> Element {
    let SplitterProps {
        orientation,
        split,
        default_split,
        on_change,
        on_moving,
        on_release,
        min_primary,
        min_secondary,
        class,
        style,
        gutter_aria_label,
        first,
        second,
    } = props;

    let initial = split.unwrap_or(default_split).clamp(0.05, 0.95);
    let mut ratio = use_signal(|| initial);
    if let Some(controlled) = split {
        ratio.set(controlled.clamp(0.05, 0.95));
    }
    let dragging = use_signal(|| false);
    let active_pointer = use_signal(|| None::<i32>);

    let min_primary = min_primary.unwrap_or(80.0);
    let min_secondary = min_secondary.unwrap_or(80.0);

    let orientation_class = match orientation {
        SplitterOrientation::Horizontal => "adui-splitter-horizontal",
        SplitterOrientation::Vertical => "adui-splitter-vertical",
    };
    let class_attr = format!(
        "adui-splitter {orientation_class} {}",
        class.unwrap_or_default()
    );
    let style_attr = format!(
        "display:flex;flex-direction:{};gap:8px;{}",
        match orientation {
            SplitterOrientation::Horizontal => "row",
            SplitterOrientation::Vertical => "column",
        },
        style.unwrap_or_default()
    );

    let mut set_ratio_with_constraints = {
        let mut ratio_signal = ratio;
        move |next: f32, size: f64| {
            let primary_px = (next as f64) * size;
            let secondary_px = (1.0 - next as f64) * size;
            let mut adjusted = next;
            if primary_px < min_primary as f64 {
                adjusted = (min_primary as f64 / size) as f32;
            }
            if secondary_px < min_secondary as f64 {
                adjusted = 1.0 - (min_secondary as f64 / size) as f32;
            }
            ratio_signal.set(adjusted.clamp(0.05, 0.95));
            adjusted
        }
    };

    let handle_move = {
        let mut ratio_signal = ratio;
        let dragging_signal = dragging;
        let on_change_cb = on_change;
        let on_moving_cb = on_moving;
        move |evt: Event<PointerData>| {
            if !*dragging_signal.read() {
                return;
            }
            let binding = evt.data();
            let Some(web_evt) = binding.downcast::<web_sys::PointerEvent>() else {
                return;
            };
            let Some(el) = web_evt
                .current_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            else {
                return;
            };
            let rect = el.get_bounding_client_rect();
            let (size, origin) = match orientation {
                SplitterOrientation::Horizontal => (rect.width(), rect.x()),
                SplitterOrientation::Vertical => (rect.height(), rect.y()),
            };
            if size <= 0.0 {
                return;
            }
            let pointer_pos = match orientation {
                SplitterOrientation::Horizontal => web_evt.client_x() as f64,
                SplitterOrientation::Vertical => web_evt.client_y() as f64,
            };
            let mut next = ((pointer_pos - origin) / size).clamp(0.05, 0.95) as f32;
            next = set_ratio_with_constraints(next, size);
            ratio_signal.set(next);
            if let Some(cb) = on_moving_cb.as_ref() {
                cb.call(next);
            }
            if let Some(cb) = on_change_cb.as_ref() {
                cb.call(next);
            }
        }
    };

    let handle_pointer_down = {
        let mut dragging_signal = dragging;
        let mut active_pointer = active_pointer;
        move |evt: Event<PointerData>| {
            if let Some(web_evt) = evt.data().downcast::<web_sys::PointerEvent>() {
                active_pointer.set(Some(web_evt.pointer_id()));
                dragging_signal.set(true);
                if let Some(target) = web_evt
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = target.set_pointer_capture(web_evt.pointer_id());
                }
            }
        }
    };

    let handle_pointer_up = {
        let mut dragging_signal = dragging;
        let mut active_pointer = active_pointer;
        let on_release_cb = on_release;
        move |evt: Event<PointerData>| {
            if let Some(web_evt) = evt.data().downcast::<web_sys::PointerEvent>()
                && Some(web_evt.pointer_id()) == *active_pointer.read()
            {
                active_pointer.set(None);
                dragging_signal.set(false);
                if let Some(cb) = on_release_cb.as_ref() {
                    cb.call(*ratio.read());
                }
                if let Some(target) = web_evt
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = target.release_pointer_capture(web_evt.pointer_id());
                }
            }
        }
    };

    let pane_primary_style = format!(
        "flex:0 0 {pct}%;min-{}:{}px;",
        match orientation {
            SplitterOrientation::Horizontal => "width",
            SplitterOrientation::Vertical => "height",
        },
        min_primary,
        pct = (*ratio.read() * 100.0)
    );
    let pane_secondary_style = format!(
        "flex:1 1 auto;min-{}:{}px;",
        match orientation {
            SplitterOrientation::Horizontal => "width",
            SplitterOrientation::Vertical => "height",
        },
        min_secondary
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            onpointermove: handle_move,
            onpointerup: handle_pointer_up,
            onpointerleave: handle_pointer_up,
            div {
                class: "adui-splitter-pane",
                style: "{pane_primary_style}",
                {first}
            }
            div {
                class: "adui-splitter-gutter",
                role: "separator",
                tabindex: "0",
                "aria-label": gutter_aria_label.unwrap_or_else(|| "Resize panels".into()),
                onpointerdown: handle_pointer_down,
                onpointerup: handle_pointer_up,
            }
            div {
                class: "adui-splitter-pane",
                style: "{pane_secondary_style}",
                {second}
            }
        }
    }
}

/// Wrapper for clarity; currently just renders children.
#[derive(Props, Clone, PartialEq)]
pub struct SplitterPaneProps {
    pub children: Element,
}

#[component]
pub fn SplitterPane(props: SplitterPaneProps) -> Element {
    rsx! { {props.children} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitter_orientation_default() {
        assert_eq!(
            SplitterOrientation::default(),
            SplitterOrientation::Horizontal
        );
    }

    #[test]
    fn splitter_orientation_all_variants() {
        assert_eq!(
            SplitterOrientation::Horizontal,
            SplitterOrientation::Horizontal
        );
        assert_eq!(
            SplitterOrientation::Vertical,
            SplitterOrientation::Vertical
        );
        assert_ne!(
            SplitterOrientation::Horizontal,
            SplitterOrientation::Vertical
        );
    }

    #[test]
    fn splitter_orientation_clone() {
        let original = SplitterOrientation::Vertical;
        let cloned = original;
        assert_eq!(original, cloned);
    }
}
