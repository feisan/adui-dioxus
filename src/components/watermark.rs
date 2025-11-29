//! Watermark component for adding watermarks to content areas.
//!
//! Ported from Ant Design 6.x watermark component.

use dioxus::prelude::*;

/// Font configuration for text watermarks.
#[derive(Clone, Debug, PartialEq)]
pub struct WatermarkFont {
    /// Font color. Defaults to `rgba(0, 0, 0, 0.15)`.
    pub color: String,
    /// Font size in pixels. Defaults to 16.
    pub font_size: f32,
    /// Font weight (e.g., "normal", "bold", or numeric like 400, 700).
    pub font_weight: String,
    /// Font style (e.g., "normal", "italic").
    pub font_style: String,
    /// Font family. Defaults to "sans-serif".
    pub font_family: String,
    /// Text alignment. Defaults to "center".
    pub text_align: String,
}

impl Default for WatermarkFont {
    fn default() -> Self {
        Self {
            color: "rgba(0, 0, 0, 0.15)".into(),
            font_size: 16.0,
            font_weight: "normal".into(),
            font_style: "normal".into(),
            font_family: "sans-serif".into(),
            text_align: "center".into(),
        }
    }
}

/// Props for the Watermark component.
#[derive(Props, Clone, PartialEq)]
pub struct WatermarkProps {
    /// Z-index of the watermark layer. Defaults to 9.
    #[props(default = 9)]
    pub z_index: i32,

    /// Rotation angle in degrees. Defaults to -22.
    #[props(default = -22.0)]
    pub rotate: f32,

    /// Width of the watermark. Auto-calculated if not provided.
    #[props(optional)]
    pub width: Option<f32>,

    /// Height of the watermark. Auto-calculated if not provided.
    #[props(optional)]
    pub height: Option<f32>,

    /// Image URL for image watermarks. Takes precedence over content.
    #[props(optional)]
    pub image: Option<String>,

    /// Text content for the watermark. Can be a single string or multiple lines.
    #[props(optional)]
    pub content: Option<Vec<String>>,

    /// Font configuration for text watermarks.
    #[props(optional)]
    pub font: Option<WatermarkFont>,

    /// Gap between watermarks as `[horizontal, vertical]`. Defaults to `[100, 100]`.
    #[props(optional)]
    pub gap: Option<[f32; 2]>,

    /// Offset of the watermark from top-left as `[left, top]`.
    #[props(optional)]
    pub offset: Option<[f32; 2]>,

    /// Extra class name for the wrapper.
    #[props(optional)]
    pub class: Option<String>,

    /// Extra class name for the root element.
    #[props(optional)]
    pub root_class: Option<String>,

    /// Inline style for the wrapper.
    #[props(optional)]
    pub style: Option<String>,

    /// Whether nested watermark contexts should inherit this watermark.
    #[props(default = true)]
    pub inherit: bool,

    /// Content to be watermarked.
    pub children: Element,
}

/// Watermark context for nested support (e.g., Modal, Drawer).
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct WatermarkContext {
    /// Whether parent has watermark enabled.
    has_watermark: bool,
}

/// Watermark component that adds a watermark layer over its children.
///
/// Supports both text and image watermarks with customizable appearance.
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     Watermark {
///         content: vec!["Confidential".to_string()],
///         div { class: "content",
///             "Protected content here"
///         }
///     }
/// }
/// ```
#[component]
pub fn Watermark(props: WatermarkProps) -> Element {
    let WatermarkProps {
        z_index,
        rotate,
        width,
        height,
        image,
        content,
        font,
        gap,
        offset,
        class,
        root_class,
        style,
        inherit,
        children,
    } = props;

    // Merge font with defaults
    let font = font.unwrap_or_default();
    let gap = gap.unwrap_or([100.0, 100.0]);
    let [gap_x, gap_y] = gap;
    let gap_x_center = gap_x / 2.0;
    let gap_y_center = gap_y / 2.0;
    let offset_left = offset.map(|o| o[0]).unwrap_or(gap_x_center);
    let offset_top = offset.map(|o| o[1]).unwrap_or(gap_y_center);

    // Calculate watermark dimensions
    let (mark_width, mark_height) =
        calculate_mark_size(width, height, &content, &font, image.is_some());

    // Generate watermark pattern
    let watermark_style = generate_watermark_style(
        z_index,
        rotate,
        mark_width,
        mark_height,
        &image,
        &content,
        &font,
        gap_x,
        gap_y,
        offset_left,
        offset_top,
        gap_x_center,
        gap_y_center,
    );

    // Provide context for nested watermarks
    if inherit {
        use_context_provider(|| WatermarkContext {
            has_watermark: true,
        });
    }

    // Build class list
    let mut class_list = vec!["adui-watermark".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let mut root_class_list = vec!["adui-watermark-wrapper".to_string()];
    if let Some(extra) = root_class {
        root_class_list.push(extra);
    }
    let root_class_attr = root_class_list.join(" ");

    let wrapper_style = format!(
        "position: relative; overflow: hidden; {}",
        style.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{root_class_attr}",
            style: "{wrapper_style}",
            {children}
            div {
                class: "{class_attr}",
                style: "{watermark_style}",
            }
        }
    }
}

/// Calculate the size of a single watermark cell.
fn calculate_mark_size(
    width: Option<f32>,
    height: Option<f32>,
    content: &Option<Vec<String>>,
    font: &WatermarkFont,
    is_image: bool,
) -> (f32, f32) {
    if is_image {
        // Default image dimensions
        (width.unwrap_or(120.0), height.unwrap_or(64.0))
    } else if let Some(lines) = content {
        // Estimate text dimensions based on content
        let font_gap = 3.0;
        let line_count = lines.len().max(1) as f32;

        // Estimate width based on longest line
        let max_chars = lines.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        let estimated_width = width.unwrap_or((max_chars as f32 * font.font_size * 0.6).max(60.0));

        // Height based on line count
        let estimated_height =
            height.unwrap_or(line_count * font.font_size + (line_count - 1.0).max(0.0) * font_gap);

        (estimated_width, estimated_height)
    } else {
        (width.unwrap_or(120.0), height.unwrap_or(64.0))
    }
}

/// Generate the CSS style for the watermark overlay.
fn generate_watermark_style(
    z_index: i32,
    rotate: f32,
    mark_width: f32,
    mark_height: f32,
    image: &Option<String>,
    content: &Option<Vec<String>>,
    font: &WatermarkFont,
    gap_x: f32,
    gap_y: f32,
    offset_left: f32,
    offset_top: f32,
    gap_x_center: f32,
    gap_y_center: f32,
) -> String {
    // Calculate position offset
    let position_left = (offset_left - gap_x_center).max(0.0);
    let position_top = (offset_top - gap_y_center).max(0.0);

    // Calculate background position
    let bg_position_left = if offset_left > gap_x_center {
        0.0
    } else {
        offset_left - gap_x_center
    };
    let bg_position_top = if offset_top > gap_y_center {
        0.0
    } else {
        offset_top - gap_y_center
    };

    // Generate SVG-based watermark for better cross-browser support
    let svg_content = generate_svg_watermark(
        rotate,
        mark_width,
        mark_height,
        image,
        content,
        font,
        gap_x,
        gap_y,
    );

    // Base64 encode the SVG for use as background
    let svg_base64 = base64_encode(&svg_content);
    let background_image = format!("url('data:image/svg+xml;base64,{}')", svg_base64);

    // Calculate rotation for pattern size
    let angle_rad = rotate * std::f32::consts::PI / 180.0;
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();
    let rotated_width = (mark_width * cos_a.abs() + mark_height * sin_a.abs()).ceil();
    let rotated_height = (mark_width * sin_a.abs() + mark_height * cos_a.abs()).ceil();

    // Calculate the full pattern size (matches SVG dimensions)
    let cell_width = rotated_width + gap_x;
    let cell_height = rotated_height + gap_y;
    let pattern_width = cell_width * 2.0;
    let pattern_height = cell_height * 2.0;

    let mut style = format!(
        "position: absolute; \
         z-index: {}; \
         left: {}px; \
         top: {}px; \
         width: calc(100% - {}px); \
         height: calc(100% - {}px); \
         pointer-events: none; \
         background-repeat: repeat; \
         background-image: {}; \
         background-size: {}px {}px; \
         background-position: {}px {}px;",
        z_index,
        position_left,
        position_top,
        position_left,
        position_top,
        background_image,
        pattern_width,
        pattern_height,
        bg_position_left,
        bg_position_top,
    );

    // Add visibility important to prevent hiding via CSS
    style.push_str(" visibility: visible !important;");

    style
}

/// Generate an SVG watermark pattern.
fn generate_svg_watermark(
    rotate: f32,
    mark_width: f32,
    mark_height: f32,
    image: &Option<String>,
    content: &Option<Vec<String>>,
    font: &WatermarkFont,
    gap_x: f32,
    gap_y: f32,
) -> String {
    let font_gap = 3.0;

    // Calculate rotation
    let angle_rad = rotate * std::f32::consts::PI / 180.0;
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    // Calculate rotated bounding box
    let rotated_width = (mark_width * cos_a.abs() + mark_height * sin_a.abs()).ceil();
    let rotated_height = (mark_width * sin_a.abs() + mark_height * cos_a.abs()).ceil();

    // Pattern dimensions - account for alternating offset
    let cell_width = rotated_width + gap_x;
    let cell_height = rotated_height + gap_y;
    let pattern_width = cell_width * 2.0;
    let pattern_height = cell_height * 2.0;

    let content_svg = if let Some(url) = image {
        // Image watermark
        let cx = rotated_width / 2.0;
        let cy = rotated_height / 2.0;
        format!(
            r#"<image href="{}" width="{}" height="{}" x="{}" y="{}" transform="rotate({} {} {})" preserveAspectRatio="xMidYMid meet"/>"#,
            escape_xml(url),
            mark_width,
            mark_height,
            cx - mark_width / 2.0,
            cy - mark_height / 2.0,
            rotate,
            cx,
            cy
        )
    } else if let Some(lines) = content {
        // Text watermark
        let cx = rotated_width / 2.0;
        let cy = rotated_height / 2.0;
        let line_height = font.font_size + font_gap;
        let total_height = lines.len() as f32 * line_height - font_gap;
        let start_y = cy - total_height / 2.0 + font.font_size;

        let text_elements: String = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let y = start_y + i as f32 * line_height;
                format!(
                    r#"<text x="{}" y="{}" text-anchor="middle" fill="{}" font-size="{}px" font-weight="{}" font-style="{}" font-family="{}">{}</text>"#,
                    cx,
                    y,
                    escape_xml(&font.color),
                    font.font_size,
                    escape_xml(&font.font_weight),
                    escape_xml(&font.font_style),
                    escape_xml(&font.font_family),
                    escape_xml(line)
                )
            })
            .collect();

        format!(
            r#"<g transform="rotate({} {} {})">{}</g>"#,
            rotate, cx, cy, text_elements
        )
    } else {
        String::new()
    };

    // Create alternating pattern with 4 cells (2x2 grid with offset)
    // Row 0: positions (0,0) and (cell_width, cell_height/2)
    // Row 1: positions (0, cell_height) and (cell_width, cell_height + cell_height/2)
    let half_cell_height = cell_height / 2.0;

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
            <g transform="translate(0, 0)">{}</g>
            <g transform="translate({}, {})">{}</g>
            <g transform="translate(0, {})">{}</g>
            <g transform="translate({}, {})">{}</g>
        </svg>"#,
        pattern_width,
        pattern_height,
        pattern_width,
        pattern_height,
        // First row, first column
        content_svg,
        // First row, second column (offset down by half)
        cell_width,
        half_cell_height,
        content_svg,
        // Second row, first column
        cell_height,
        content_svg,
        // Second row, second column (offset down by half)
        cell_width,
        cell_height + half_cell_height,
        content_svg
    )
}

/// Escape XML special characters.
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Simple base64 encoding for ASCII strings.
fn base64_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = input.as_bytes();
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;

        let n = (b0 << 16) | (b1 << 8) | b2;

        result.push(ALPHABET[(n >> 18 & 0x3F) as usize] as char);
        result.push(ALPHABET[(n >> 12 & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[(n >> 6 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_font_has_expected_values() {
        let font = WatermarkFont::default();
        assert_eq!(font.font_size, 16.0);
        assert_eq!(font.font_weight, "normal");
        assert_eq!(font.font_family, "sans-serif");
        assert!(font.color.contains("rgba"));
    }

    #[test]
    fn calculate_mark_size_returns_defaults_for_image() {
        let (w, h) = calculate_mark_size(None, None, &None, &WatermarkFont::default(), true);
        assert_eq!(w, 120.0);
        assert_eq!(h, 64.0);
    }

    #[test]
    fn calculate_mark_size_respects_explicit_dimensions() {
        let (w, h) = calculate_mark_size(
            Some(200.0),
            Some(100.0),
            &None,
            &WatermarkFont::default(),
            true,
        );
        assert_eq!(w, 200.0);
        assert_eq!(h, 100.0);
    }

    #[test]
    fn escape_xml_handles_special_characters() {
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("\"quote\""), "&quot;quote&quot;");
    }

    #[test]
    fn base64_encode_produces_valid_output() {
        // "Hello" should encode to "SGVsbG8="
        assert_eq!(base64_encode("Hello"), "SGVsbG8=");
        // Empty string
        assert_eq!(base64_encode(""), "");
        // Single character
        assert_eq!(base64_encode("a"), "YQ==");
    }

    #[test]
    fn generate_svg_watermark_creates_valid_svg() {
        let svg = generate_svg_watermark(
            -22.0,
            120.0,
            64.0,
            &None,
            &Some(vec!["Test".to_string()]),
            &WatermarkFont::default(),
            100.0,
            100.0,
        );
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("Test"));
        assert!(svg.contains("rotate(-22"));
    }
}
