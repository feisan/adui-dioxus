use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitterOrientation {
    Horizontal,
    Vertical,
}

impl Default for SplitterOrientation {
    fn default() -> Self {
        SplitterOrientation::Horizontal
    }
}

/// Configuration for the resizable splitter.
#[derive(Props, Clone, PartialEq)]
pub struct SplitterProps {
    #[props(default)]
    pub orientation: SplitterOrientation,
    #[props(default = 0.5)]
    pub split: f32,
    #[props(optional)]
    pub on_change: Option<EventHandler<f32>>,
    #[props(optional)]
    pub min_primary: Option<f32>,
    #[props(optional)]
    pub min_secondary: Option<f32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub first: Element,
    pub second: Element,
}

/// Splitter with draggable gutter to resize two panes.
#[component]
pub fn Splitter(props: SplitterProps) -> Element {
    let SplitterProps {
        orientation,
        split,
        on_change,
        min_primary,
        min_secondary,
        class,
        style,
        first,
        second,
    } = props;

    let clamped = split.clamp(0.05, 0.95);
    let ratio = use_signal(|| clamped);
    let dragging = use_signal(|| false);

    let orientation_class = match orientation {
        SplitterOrientation::Horizontal => "adui-splitter-horizontal",
        SplitterOrientation::Vertical => "adui-splitter-vertical",
    };
    let class_attr = format!(
        "adui-splitter {orientation_class} {}",
        class.unwrap_or_default()
    );
    let style_attr = match orientation {
        SplitterOrientation::Horizontal => format!(
            "display:flex;flex-direction:row;gap:8px;{}",
            style.unwrap_or_default()
        ),
        SplitterOrientation::Vertical => format!(
            "display:flex;flex-direction:column;gap:8px;{}",
            style.unwrap_or_default()
        ),
    };

    let min_primary = min_primary.unwrap_or(80.0);
    let min_secondary = min_secondary.unwrap_or(80.0);
    let on_pointer_down = {
        let mut dragging = dragging.clone();
        move |evt: Event<PointerData>| {
            dragging.set(true);
            if let Some(web_evt) = evt.data().downcast::<web_sys::PointerEvent>() {
                if let Some(target) = web_evt
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = target.set_pointer_capture(web_evt.pointer_id());
                }
            }
        }
    };
    let on_pointer_up = {
        let mut dragging = dragging.clone();
        move |evt: Event<PointerData>| {
            dragging.set(false);
            if let Some(web_evt) = evt.data().downcast::<web_sys::PointerEvent>() {
                if let Some(target) = web_evt
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let _ = target.release_pointer_capture(web_evt.pointer_id());
                }
            }
        }
    };
    let on_pointer_move = {
        let mut ratio = ratio.clone();
        let dragging = dragging.clone();
        move |evt: Event<PointerData>| {
            if !*dragging.read() {
                return;
            }
            if let Some(web_evt) = evt.data().downcast::<web_sys::PointerEvent>() {
                if let Some(el) = web_evt
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let rect = el.get_bounding_client_rect();
                    let (size, origin) = match orientation {
                        SplitterOrientation::Horizontal => (rect.width(), rect.x()),
                        SplitterOrientation::Vertical => (rect.height(), rect.y()),
                    };
                    if size <= 0.0 {
                        return;
                    }
                    let pos = match orientation {
                        SplitterOrientation::Horizontal => web_evt.client_x() as f64,
                        SplitterOrientation::Vertical => web_evt.client_y() as f64,
                    };
                    let mut next = ((pos - origin) / size).clamp(0.05, 0.95);
                    if next * size < min_primary as f64 {
                        next = min_primary as f64 / size;
                    }
                    if (1.0 - next) * size < min_secondary as f64 {
                        next = 1.0 - (min_secondary as f64 / size);
                    }
                    let next_f32 = next as f32;
                    ratio.set(next_f32);
                    if let Some(cb) = on_change.as_ref() {
                        cb.call(next_f32);
                    }
                }
            }
        }
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            onpointermove: on_pointer_move,
            onpointerup: on_pointer_up.clone(),
            div {
                class: "adui-splitter-pane",
                style: format!(
                    "flex:0 0 {pct}%;min-{}:{}px;",
                    match orientation {
                        SplitterOrientation::Horizontal => "width",
                        SplitterOrientation::Vertical => "height",
                    },
                    min_primary,
                    pct = *ratio.read() * 100.0
                ),
                {first}
            }
            div {
                class: "adui-splitter-gutter",
                onpointerdown: on_pointer_down,
                onpointerup: on_pointer_up,
            }
            div {
                class: "adui-splitter-pane",
                style: format!(
                    "flex:1 1 auto;min-{}:{}px;",
                    match orientation {
                        SplitterOrientation::Horizontal => "width",
                        SplitterOrientation::Vertical => "height",
                    },
                    min_secondary
                ),
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
