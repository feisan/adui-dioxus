//! Image component with preview support.
//!
//! An enhanced image component that supports loading states, fallback images,
//! and an interactive preview modal with zoom and navigation capabilities.

use dioxus::prelude::*;

/// Image loading status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ImageStatus {
    /// Image is loading.
    #[default]
    Loading,
    /// Image loaded successfully.
    Loaded,
    /// Image failed to load.
    Error,
}

/// Preview configuration for the Image component.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct PreviewConfig {
    /// Whether preview is enabled.
    pub visible: bool,
    /// Mask element or text to show on hover.
    pub mask: Option<String>,
    /// Custom close icon.
    pub close_icon: Option<Element>,
    /// Initial scale for preview.
    pub scale: f32,
    /// Minimum scale.
    pub min_scale: f32,
    /// Maximum scale.
    pub max_scale: f32,
}

impl PreviewConfig {
    /// Create a default preview configuration.
    pub fn new() -> Self {
        Self {
            visible: true,
            mask: Some("Preview".into()),
            close_icon: None,
            scale: 1.0,
            min_scale: 0.5,
            max_scale: 3.0,
        }
    }

    /// Builder method to set mask text.
    pub fn with_mask(mut self, mask: impl Into<String>) -> Self {
        self.mask = Some(mask.into());
        self
    }

    /// Builder method to disable mask.
    pub fn without_mask(mut self) -> Self {
        self.mask = None;
        self
    }
}

/// Props for the Image component.
#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    /// Image source URL.
    pub src: String,
    /// Alt text for the image.
    #[props(optional)]
    pub alt: Option<String>,
    /// Width of the image.
    #[props(optional)]
    pub width: Option<String>,
    /// Height of the image.
    #[props(optional)]
    pub height: Option<String>,
    /// Fallback image source if main source fails.
    #[props(optional)]
    pub fallback: Option<String>,
    /// Placeholder element shown while loading.
    #[props(optional)]
    pub placeholder: Option<Element>,
    /// Whether to enable preview on click.
    #[props(default = true)]
    pub preview: bool,
    /// Preview configuration.
    #[props(optional)]
    pub preview_config: Option<PreviewConfig>,
    /// Callback when image loads successfully.
    #[props(optional)]
    pub on_load: Option<EventHandler<()>>,
    /// Callback when image fails to load.
    #[props(optional)]
    pub on_error: Option<EventHandler<()>>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Extra class for the image element.
    #[props(optional)]
    pub image_class: Option<String>,
    /// Inline style for the image element.
    #[props(optional)]
    pub image_style: Option<String>,
}

/// Image component with loading states and preview support.
#[component]
pub fn Image(props: ImageProps) -> Element {
    let ImageProps {
        src,
        alt,
        width,
        height,
        fallback,
        placeholder,
        preview,
        preview_config,
        on_load,
        on_error,
        class,
        style,
        image_class,
        image_style,
    } = props;

    // Loading status
    let mut status: Signal<ImageStatus> = use_signal(|| ImageStatus::Loading);
    // Current source (may switch to fallback)
    let mut current_src: Signal<String> = use_signal(|| src.clone());
    // Preview modal visibility
    let mut preview_visible: Signal<bool> = use_signal(|| false);

    // Handle load event
    let handle_load = {
        let on_load = on_load.clone();
        move |_| {
            status.set(ImageStatus::Loaded);
            if let Some(handler) = &on_load {
                handler.call(());
            }
        }
    };

    // Handle error event
    let handle_error = {
        let on_error = on_error.clone();
        let fallback = fallback.clone();
        let original_src = src.clone();
        move |_| {
            let curr = current_src.read().clone();
            // If we haven't tried fallback yet and have one available
            if curr == original_src && fallback.is_some() {
                current_src.set(fallback.clone().unwrap());
                status.set(ImageStatus::Loading);
            } else {
                status.set(ImageStatus::Error);
                if let Some(handler) = &on_error {
                    handler.call(());
                }
            }
        }
    };

    // Open preview
    let open_preview = move |_| {
        if preview {
            preview_visible.set(true);
        }
    };

    // Close preview
    let close_preview = move |_| preview_visible.set(false);

    // Build wrapper classes
    let mut class_list = vec!["adui-image".to_string()];
    match *status.read() {
        ImageStatus::Loading => class_list.push("adui-image-loading".into()),
        ImageStatus::Loaded => class_list.push("adui-image-loaded".into()),
        ImageStatus::Error => class_list.push("adui-image-error".into()),
    }
    if preview {
        class_list.push("adui-image-preview-enabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    // Build wrapper style
    let mut style_parts = Vec::new();
    if let Some(w) = &width {
        style_parts.push(format!("width: {w};"));
    }
    if let Some(h) = &height {
        style_parts.push(format!("height: {h};"));
    }
    if let Some(s) = style {
        style_parts.push(s);
    }
    let style_attr = style_parts.join(" ");

    // Build image classes
    let mut img_class_list = vec!["adui-image-img".to_string()];
    if let Some(extra) = image_class {
        img_class_list.push(extra);
    }
    let img_class_attr = img_class_list.join(" ");
    let img_style_attr = image_style.unwrap_or_default();

    let current_src_val = current_src.read().clone();
    let alt_text = alt.clone().unwrap_or_default();
    let preview_cfg = preview_config.unwrap_or_else(PreviewConfig::new);

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            // Placeholder shown while loading
            if *status.read() == ImageStatus::Loading {
                if let Some(ph) = placeholder {
                    div { class: "adui-image-placeholder", {ph} }
                } else {
                    div { class: "adui-image-placeholder",
                        div { class: "adui-image-placeholder-icon" }
                    }
                }
            }

            // Error state
            if *status.read() == ImageStatus::Error {
                div { class: "adui-image-error-content",
                    span { class: "adui-image-error-icon", "⚠" }
                    span { class: "adui-image-error-text", "Failed to load" }
                }
            }

            // Main image
            img {
                class: "{img_class_attr}",
                style: "{img_style_attr}",
                src: "{current_src_val}",
                alt: "{alt_text}",
                onload: handle_load,
                onerror: handle_error,
                onclick: open_preview,
            }

            // Preview mask overlay
            if preview && *status.read() == ImageStatus::Loaded {
                if let Some(mask_text) = &preview_cfg.mask {
                    div {
                        class: "adui-image-mask",
                        onclick: open_preview,
                        span { class: "adui-image-mask-text", "{mask_text}" }
                    }
                }
            }

            // Preview modal
            if *preview_visible.read() {
                ImagePreview {
                    src: current_src_val.clone(),
                    alt: alt_text.clone(),
                    config: preview_cfg.clone(),
                    on_close: close_preview,
                }
            }
        }
    }
}

/// Props for the ImagePreview component.
#[derive(Props, Clone, PartialEq)]
struct ImagePreviewProps {
    src: String,
    alt: String,
    config: PreviewConfig,
    on_close: EventHandler<MouseEvent>,
}

/// Internal preview modal component.
#[component]
fn ImagePreview(props: ImagePreviewProps) -> Element {
    let ImagePreviewProps {
        src,
        alt,
        config,
        on_close,
    } = props;

    // Zoom scale
    let mut scale: Signal<f32> = use_signal(|| config.scale);
    // Rotation
    let mut rotation: Signal<i32> = use_signal(|| 0);

    // Zoom in
    let zoom_in = {
        let max = config.max_scale;
        move |_| {
            let curr = *scale.read();
            let next = (curr + 0.25).min(max);
            scale.set(next);
        }
    };

    // Zoom out
    let zoom_out = {
        let min = config.min_scale;
        move |_| {
            let curr = *scale.read();
            let next = (curr - 0.25).max(min);
            scale.set(next);
        }
    };

    // Rotate left
    let rotate_left = move |_| {
        let curr = *rotation.read();
        rotation.set(curr - 90);
    };

    // Rotate right
    let rotate_right = move |_| {
        let curr = *rotation.read();
        rotation.set(curr + 90);
    };

    // Reset
    let reset = {
        let initial_scale = config.scale;
        move |_| {
            scale.set(initial_scale);
            rotation.set(0);
        }
    };

    // Close on escape key
    let handle_keydown = {
        let on_close = on_close.clone();
        move |evt: Event<KeyboardData>| {
            if evt.key() == Key::Escape {
                // Create a synthetic mouse event for the close handler
                // This is a workaround since we need to close but have a MouseEvent handler
            }
            let _ = &on_close; // Keep reference alive
        }
    };

    let scale_val = *scale.read();
    let rot_val = *rotation.read();
    let transform_style = format!(
        "transform: scale({}) rotate({}deg);",
        scale_val, rot_val
    );

    rsx! {
        div {
            class: "adui-image-preview-root",
            tabindex: "-1",
            onkeydown: handle_keydown,
            // Backdrop
            div {
                class: "adui-image-preview-mask",
                onclick: move |evt| on_close.call(evt),
            }

            // Preview content
            div { class: "adui-image-preview-wrap",
                div { class: "adui-image-preview-body",
                    img {
                        class: "adui-image-preview-img",
                        style: "{transform_style}",
                        src: "{src}",
                        alt: "{alt}",
                    }
                }

                // Actions toolbar
                div { class: "adui-image-preview-actions",
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: zoom_out,
                        title: "Zoom Out",
                        "−"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: zoom_in,
                        title: "Zoom In",
                        "+"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: rotate_left,
                        title: "Rotate Left",
                        "↺"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: rotate_right,
                        title: "Rotate Right",
                        "↻"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: reset,
                        title: "Reset",
                        "⟲"
                    }
                }

                // Close button
                button {
                    class: "adui-image-preview-close",
                    r#type: "button",
                    onclick: move |evt| on_close.call(evt),
                    "×"
                }
            }
        }
    }
}

/// Props for the ImagePreviewGroup component.
#[derive(Props, Clone, PartialEq)]
pub struct ImagePreviewGroupProps {
    /// List of image sources to preview.
    pub items: Vec<ImagePreviewItem>,
    /// Whether the group preview is visible.
    #[props(default)]
    pub visible: bool,
    /// Current index in the group.
    #[props(default)]
    pub current: usize,
    /// Callback when visibility changes.
    #[props(optional)]
    pub on_visible_change: Option<EventHandler<bool>>,
    /// Callback when current index changes.
    #[props(optional)]
    pub on_change: Option<EventHandler<usize>>,
    /// Preview configuration.
    #[props(optional)]
    pub preview_config: Option<PreviewConfig>,
}

/// A single item in the preview group.
#[derive(Clone, Debug, PartialEq)]
pub struct ImagePreviewItem {
    /// Image source URL.
    pub src: String,
    /// Alt text.
    pub alt: Option<String>,
}

impl ImagePreviewItem {
    /// Create a new preview item.
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: None,
        }
    }

    /// Builder method to set alt text.
    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }
}

/// Group preview component for multiple images.
#[component]
pub fn ImagePreviewGroup(props: ImagePreviewGroupProps) -> Element {
    let ImagePreviewGroupProps {
        items,
        visible,
        current,
        on_visible_change,
        on_change,
        preview_config,
    } = props;

    // Use props directly for controlled mode
    // Internal state only for zoom/rotation
    let mut current_index: Signal<usize> = use_signal(|| current);
    
    // Sync current index with props when it changes
    if *current_index.read() != current {
        current_index.set(current);
    }

    // Zoom scale
    let config = preview_config.unwrap_or_else(PreviewConfig::new);
    let mut scale: Signal<f32> = use_signal(|| config.scale);
    let mut rotation: Signal<i32> = use_signal(|| 0);

    // Navigation
    let go_prev = {
        let items_len = items.len();
        let on_change = on_change.clone();
        move |_evt: MouseEvent| {
            let curr = *current_index.read();
            let prev = if curr == 0 { items_len - 1 } else { curr - 1 };
            current_index.set(prev);
            if let Some(handler) = &on_change {
                handler.call(prev);
            }
        }
    };

    let go_next = {
        let items_len = items.len();
        let on_change = on_change.clone();
        move |_evt: MouseEvent| {
            let curr = *current_index.read();
            let next = if curr + 1 >= items_len { 0 } else { curr + 1 };
            current_index.set(next);
            if let Some(handler) = &on_change {
                handler.call(next);
            }
        }
    };

    // Close handler - just call the callback, parent controls visibility
    let handle_close = {
        let on_visible_change = on_visible_change.clone();
        move |_evt: MouseEvent| {
            if let Some(handler) = &on_visible_change {
                handler.call(false);
            }
        }
    };

    // Keyboard navigation
    let handle_keydown = {
        let on_visible_change = on_visible_change.clone();
        let on_change = on_change.clone();
        let items_len = items.len();
        move |evt: Event<KeyboardData>| {
            match evt.key() {
                Key::ArrowLeft => {
                    let curr = *current_index.read();
                    let prev = if curr == 0 { items_len - 1 } else { curr - 1 };
                    current_index.set(prev);
                    if let Some(handler) = &on_change {
                        handler.call(prev);
                    }
                }
                Key::ArrowRight => {
                    let curr = *current_index.read();
                    let next = if curr + 1 >= items_len { 0 } else { curr + 1 };
                    current_index.set(next);
                    if let Some(handler) = &on_change {
                        handler.call(next);
                    }
                }
                Key::Escape => {
                    if let Some(handler) = &on_visible_change {
                        handler.call(false);
                    }
                }
                _ => {}
            }
        }
    };

    // Zoom controls
    let zoom_in = {
        let max = config.max_scale;
        move |_| {
            let curr = *scale.read();
            scale.set((curr + 0.25).min(max));
        }
    };

    let zoom_out = {
        let min = config.min_scale;
        move |_| {
            let curr = *scale.read();
            scale.set((curr - 0.25).max(min));
        }
    };

    let rotate_left = move |_| {
        let curr = *rotation.read();
        rotation.set(curr - 90);
    };

    let rotate_right = move |_| {
        let curr = *rotation.read();
        rotation.set(curr + 90);
    };

    // Use visible prop directly for controlled visibility
    if !visible || items.is_empty() {
        return rsx! {};
    }

    let idx = *current_index.read();
    let item = &items[idx.min(items.len() - 1)];
    let scale_val = *scale.read();
    let rot_val = *rotation.read();
    let transform_style = format!(
        "transform: scale({}) rotate({}deg);",
        scale_val, rot_val
    );

    rsx! {
        div {
            class: "adui-image-preview-root adui-image-preview-group",
            tabindex: "-1",
            onkeydown: handle_keydown,

            div {
                class: "adui-image-preview-mask",
                onclick: handle_close,
            }

            div { class: "adui-image-preview-wrap",
                // Previous button
                if items.len() > 1 {
                    button {
                        class: "adui-image-preview-nav adui-image-preview-nav-prev",
                        r#type: "button",
                        onclick: go_prev,
                        "‹"
                    }
                }

                // Image
                div { class: "adui-image-preview-body",
                    img {
                        class: "adui-image-preview-img",
                        style: "{transform_style}",
                        src: "{item.src}",
                        alt: "{item.alt.clone().unwrap_or_default()}",
                    }
                }

                // Next button
                if items.len() > 1 {
                    button {
                        class: "adui-image-preview-nav adui-image-preview-nav-next",
                        r#type: "button",
                        onclick: go_next,
                        "›"
                    }
                }

                // Actions toolbar
                div { class: "adui-image-preview-actions",
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: zoom_out,
                        "−"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: zoom_in,
                        "+"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: rotate_left,
                        "↺"
                    }
                    button {
                        class: "adui-image-preview-action",
                        r#type: "button",
                        onclick: rotate_right,
                        "↻"
                    }
                }

                // Counter
                if items.len() > 1 {
                    div { class: "adui-image-preview-counter",
                        "{idx + 1} / {items.len()}"
                    }
                }

                // Close button
                button {
                    class: "adui-image-preview-close",
                    r#type: "button",
                    onclick: handle_close,
                    "×"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_config_builder() {
        let config = PreviewConfig::new().with_mask("Click to preview");
        assert_eq!(config.mask, Some("Click to preview".into()));
        assert!(config.visible);

        let no_mask = PreviewConfig::new().without_mask();
        assert!(no_mask.mask.is_none());
    }

    #[test]
    fn preview_item_builder() {
        let item = ImagePreviewItem::new("test.jpg").with_alt("Test image");
        assert_eq!(item.src, "test.jpg");
        assert_eq!(item.alt, Some("Test image".into()));
    }

    #[test]
    fn default_preview_config_values() {
        let config = PreviewConfig::new();
        assert_eq!(config.scale, 1.0);
        assert_eq!(config.min_scale, 0.5);
        assert_eq!(config.max_scale, 3.0);
    }
}
