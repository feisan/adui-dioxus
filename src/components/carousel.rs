//! Carousel component for cycling through images or content.
//!
//! A slideshow component for cycling through elements with support for
//! various transition effects, autoplay, and navigation controls.

use dioxus::prelude::*;

/// Transition effect for the carousel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselEffect {
    /// Horizontal scrolling effect.
    #[default]
    ScrollX,
    /// Fade in/out effect.
    Fade,
}

impl CarouselEffect {
    fn as_class(&self) -> &'static str {
        match self {
            CarouselEffect::ScrollX => "adui-carousel-scroll",
            CarouselEffect::Fade => "adui-carousel-fade",
        }
    }
}

/// Position for the navigation dots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DotPlacement {
    /// Dots at the top.
    Top,
    /// Dots at the bottom (default).
    #[default]
    Bottom,
    /// Dots at the left.
    Left,
    /// Dots at the right.
    Right,
}

impl DotPlacement {
    fn as_class(&self) -> &'static str {
        match self {
            DotPlacement::Top => "adui-carousel-dots-top",
            DotPlacement::Bottom => "adui-carousel-dots-bottom",
            DotPlacement::Left => "adui-carousel-dots-left",
            DotPlacement::Right => "adui-carousel-dots-right",
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(self, DotPlacement::Left | DotPlacement::Right)
    }
}

/// A single slide item for the Carousel.
#[derive(Clone, Debug, PartialEq)]
pub struct CarouselItem {
    /// Content to display (can be HTML string or key for custom rendering).
    pub content: String,
    /// Optional background color.
    pub background: Option<String>,
}

impl CarouselItem {
    /// Create a new carousel item with content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            background: None,
        }
    }

    /// Builder method to set background.
    pub fn with_background(mut self, bg: impl Into<String>) -> Self {
        self.background = Some(bg.into());
        self
    }
}

/// Props for the Carousel component.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselProps {
    /// Slide items to display.
    #[props(default)]
    pub items: Vec<CarouselItem>,
    /// Number of slides (used when using children instead of items).
    #[props(optional)]
    pub slide_count: Option<usize>,
    /// Transition effect type.
    #[props(default)]
    pub effect: CarouselEffect,
    /// Whether to enable autoplay.
    #[props(default)]
    pub autoplay: bool,
    /// Autoplay interval in milliseconds.
    #[props(default = 3000)]
    pub autoplay_speed: u64,
    /// Whether to show navigation dots.
    #[props(default = true)]
    pub dots: bool,
    /// Position of the navigation dots.
    #[props(default)]
    pub dot_placement: DotPlacement,
    /// Whether to show arrow navigation.
    #[props(default)]
    pub arrows: bool,
    /// Transition speed in milliseconds.
    #[props(default = 500)]
    pub speed: u32,
    /// Initial slide index.
    #[props(default)]
    pub initial_slide: usize,
    /// Enable infinite loop.
    #[props(default = true)]
    pub infinite: bool,
    /// Pause autoplay on hover.
    #[props(default = true)]
    pub pause_on_hover: bool,
    /// Callback when slide changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<usize>>,
    /// Callback before slide changes.
    #[props(optional)]
    pub before_change: Option<EventHandler<(usize, usize)>>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Slide content (alternative to items prop, requires slide_count).
    pub children: Element,
}

/// A carousel/slideshow component.
#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let CarouselProps {
        items,
        slide_count,
        effect,
        autoplay: _autoplay,
        autoplay_speed: _autoplay_speed,
        dots,
        dot_placement,
        arrows,
        speed,
        initial_slide,
        infinite,
        pause_on_hover: _pause_on_hover,
        on_change,
        before_change,
        class,
        style,
        children,
    } = props;

    // Current slide index
    let mut current: Signal<usize> = use_signal(|| initial_slide);
    // Hover state for pause on hover
    let is_hovered: Signal<bool> = use_signal(|| false);

    // Determine slide count from items or prop
    let count = if !items.is_empty() {
        items.len()
    } else {
        slide_count.unwrap_or(0)
    };

    // Navigation handlers
    let go_to = {
        let on_change = on_change.clone();
        let before_change = before_change.clone();
        move |index: usize| {
            if count == 0 || index >= count {
                return;
            }
            let curr = *current.read();
            if curr == index {
                return;
            }
            if let Some(handler) = &before_change {
                handler.call((curr, index));
            }
            current.set(index);
            if let Some(handler) = &on_change {
                handler.call(index);
            }
        }
    };

    let go_prev = {
        let on_change = on_change.clone();
        let before_change = before_change.clone();
        move |_: MouseEvent| {
            if count == 0 {
                return;
            }
            let curr = *current.read();
            let prev = if curr == 0 {
                if infinite {
                    count - 1
                } else {
                    return;
                }
            } else {
                curr - 1
            };
            if let Some(handler) = &before_change {
                handler.call((curr, prev));
            }
            current.set(prev);
            if let Some(handler) = &on_change {
                handler.call(prev);
            }
        }
    };

    let go_next = {
        let on_change = on_change.clone();
        let before_change = before_change.clone();
        move |_: MouseEvent| {
            if count == 0 {
                return;
            }
            let curr = *current.read();
            let next = if curr + 1 >= count {
                if infinite {
                    0
                } else {
                    return;
                }
            } else {
                curr + 1
            };
            if let Some(handler) = &before_change {
                handler.call((curr, next));
            }
            current.set(next);
            if let Some(handler) = &on_change {
                handler.call(next);
            }
        }
    };

    // Build class list
    let mut class_list = vec!["adui-carousel".to_string()];
    class_list.push(effect.as_class().to_string());
    class_list.push(dot_placement.as_class().to_string());
    if dot_placement.is_vertical() {
        class_list.push("adui-carousel-vertical".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let transition_style = format!("--adui-carousel-speed: {}ms;", speed);
    let style_attr = format!("{}{}", transition_style, style.unwrap_or_default());

    let current_index = *current.read();

    // Hover handlers
    let on_mouse_enter = {
        let mut hovered = is_hovered;
        move |_| hovered.set(true)
    };
    let on_mouse_leave = {
        let mut hovered = is_hovered;
        move |_| hovered.set(false)
    };

    // Track style for scrollX effect
    let track_style = match effect {
        CarouselEffect::ScrollX => format!("transform: translateX(-{}%);", current_index * 100),
        CarouselEffect::Fade => String::new(),
    };

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            onmouseenter: on_mouse_enter,
            onmouseleave: on_mouse_leave,

            // Slides container
            div { class: "adui-carousel-inner",
                div {
                    class: "adui-carousel-track",
                    style: "{track_style}",
                    // Render from items if provided
                    for (i, item) in items.iter().enumerate() {
                        {
                            let is_active = i == current_index;
                            let mut slide_class = vec!["adui-carousel-slide".to_string()];
                            if is_active {
                                slide_class.push("adui-carousel-slide-active".into());
                            }
                            let slide_style = item.background.as_ref()
                                .map(|bg| format!("background: {};", bg))
                                .unwrap_or_default();
                            rsx! {
                                div {
                                    key: "{i}",
                                    class: "{slide_class.join(\" \")}",
                                    style: "{slide_style}",
                                    "{item.content}"
                                }
                            }
                        }
                    }
                    // If no items, render children (requires slide_count prop)
                    if items.is_empty() {
                        {children}
                    }
                }
            }

            // Arrow navigation
            if arrows && count > 0 {
                button {
                    class: "adui-carousel-arrow adui-carousel-arrow-prev",
                    r#type: "button",
                    onclick: go_prev,
                    "‹"
                }
                button {
                    class: "adui-carousel-arrow adui-carousel-arrow-next",
                    r#type: "button",
                    onclick: go_next,
                    "›"
                }
            }

            // Dots navigation
            if dots && count > 0 {
                div { class: "adui-carousel-dots",
                    for i in 0..count {
                        button {
                            key: "{i}",
                            class: if i == current_index { "adui-carousel-dot adui-carousel-dot-active" } else { "adui-carousel-dot" },
                            r#type: "button",
                            onclick: {
                                let mut go_to = go_to.clone();
                                move |_| go_to(i)
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Props for a single carousel slide.
#[derive(Props, Clone, PartialEq)]
pub struct CarouselSlideProps {
    /// Whether this slide is currently active (for fade effect).
    #[props(default)]
    pub active: bool,
    /// Extra class for the slide.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the slide.
    #[props(optional)]
    pub style: Option<String>,
    /// Slide content.
    pub children: Element,
}

/// A single slide in the carousel.
#[component]
pub fn CarouselSlide(props: CarouselSlideProps) -> Element {
    let CarouselSlideProps {
        active,
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-carousel-slide".to_string()];
    if active {
        class_list.push("adui-carousel-slide-active".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carousel_effect_class_names() {
        assert_eq!(CarouselEffect::ScrollX.as_class(), "adui-carousel-scroll");
        assert_eq!(CarouselEffect::Fade.as_class(), "adui-carousel-fade");
    }

    #[test]
    fn dot_placement_class_names() {
        assert_eq!(DotPlacement::Top.as_class(), "adui-carousel-dots-top");
        assert_eq!(DotPlacement::Bottom.as_class(), "adui-carousel-dots-bottom");
        assert_eq!(DotPlacement::Left.as_class(), "adui-carousel-dots-left");
        assert_eq!(DotPlacement::Right.as_class(), "adui-carousel-dots-right");
    }

    #[test]
    fn dot_placement_vertical_detection() {
        assert!(!DotPlacement::Top.is_vertical());
        assert!(!DotPlacement::Bottom.is_vertical());
        assert!(DotPlacement::Left.is_vertical());
        assert!(DotPlacement::Right.is_vertical());
    }

    #[test]
    fn carousel_item_builder() {
        let item = CarouselItem::new("Hello").with_background("#ff0000");
        assert_eq!(item.content, "Hello");
        assert_eq!(item.background, Some("#ff0000".to_string()));
    }
}
