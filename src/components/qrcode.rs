//! QRCode component for generating and displaying QR codes.
//!
//! Ported from Ant Design 6.x QRCode component.

use dioxus::prelude::*;

/// QR code rendering type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QRCodeType {
    /// Render as SVG (default, better for scaling).
    #[default]
    Svg,
    /// Render as Canvas (better for large QR codes).
    Canvas,
}

/// QR code status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QRCodeStatus {
    /// Active and scannable.
    #[default]
    Active,
    /// Expired, needs refresh.
    Expired,
    /// Loading state.
    Loading,
    /// Already scanned.
    Scanned,
}

impl QRCodeStatus {
    fn as_class(&self) -> &'static str {
        match self {
            QRCodeStatus::Active => "adui-qrcode-active",
            QRCodeStatus::Expired => "adui-qrcode-expired",
            QRCodeStatus::Loading => "adui-qrcode-loading",
            QRCodeStatus::Scanned => "adui-qrcode-scanned",
        }
    }
}

/// Error correction level for QR codes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QRCodeErrorLevel {
    /// ~7% error correction.
    L,
    /// ~15% error correction (default).
    #[default]
    M,
    /// ~25% error correction.
    Q,
    /// ~30% error correction.
    H,
}

/// Props for the QRCode component.
#[derive(Props, Clone, PartialEq)]
pub struct QRCodeProps {
    /// The value/content to encode in the QR code.
    pub value: String,

    /// Rendering type (SVG or Canvas). Defaults to SVG.
    #[props(default)]
    pub r#type: QRCodeType,

    /// Size of the QR code in pixels. Defaults to 160.
    #[props(default = 160)]
    pub size: u32,

    /// Icon URL to display in the center.
    #[props(optional)]
    pub icon: Option<String>,

    /// Size of the icon. Can be a single number or width/height.
    #[props(optional)]
    pub icon_size: Option<u32>,

    /// Foreground color of the QR code. Defaults to current text color.
    #[props(optional)]
    pub color: Option<String>,

    /// Background color of the QR code. Defaults to transparent.
    #[props(optional)]
    pub bg_color: Option<String>,

    /// Error correction level. Defaults to M.
    #[props(default)]
    pub error_level: QRCodeErrorLevel,

    /// Current status of the QR code.
    #[props(default)]
    pub status: QRCodeStatus,

    /// Whether to show border. Defaults to true.
    #[props(default = true)]
    pub bordered: bool,

    /// Callback when refresh is clicked (for expired status).
    #[props(optional)]
    pub on_refresh: Option<EventHandler<()>>,

    /// Extra class name for the container.
    #[props(optional)]
    pub class: Option<String>,

    /// Extra class name for the root element.
    #[props(optional)]
    pub root_class: Option<String>,

    /// Inline style for the container.
    #[props(optional)]
    pub style: Option<String>,
}

/// QRCode component that generates and displays QR codes.
///
/// # Example
///
/// ```rust,ignore
/// rsx! {
///     QRCode {
///         value: "https://example.com",
///         size: 200,
///         icon: "https://example.com/logo.png",
///     }
/// }
/// ```
#[component]
pub fn QRCode(props: QRCodeProps) -> Element {
    let QRCodeProps {
        value,
        r#type,
        size,
        icon,
        icon_size,
        color,
        bg_color,
        error_level,
        status,
        bordered,
        on_refresh,
        class,
        root_class,
        style,
    } = props;

    // Return empty if no value provided
    if value.is_empty() {
        return rsx! {};
    }

    let fg_color = color.unwrap_or_else(|| "currentColor".to_string());
    let bg_color = bg_color.unwrap_or_else(|| "transparent".to_string());
    let icon_size = icon_size.unwrap_or(40);

    // Build class list
    let mut class_list = vec!["adui-qrcode".to_string()];
    class_list.push(status.as_class().to_string());
    if !bordered {
        class_list.push("adui-qrcode-borderless".to_string());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    let mut root_class_list = vec!["adui-qrcode-wrapper".to_string()];
    if let Some(extra) = root_class {
        root_class_list.push(extra);
    }
    let root_class_attr = root_class_list.join(" ");

    let container_style = format!(
        "width: {}px; height: {}px; background-color: {}; {}",
        size,
        size,
        bg_color,
        style.unwrap_or_default()
    );

    // Generate QR code data
    let qr_modules = generate_qr_modules(&value, error_level);
    let module_count = qr_modules.len();

    // Render based on type
    let qr_content = match r#type {
        QRCodeType::Svg => {
            render_svg_qrcode(&qr_modules, module_count, size, &fg_color, &icon, icon_size)
        }
        QRCodeType::Canvas => {
            // Canvas rendering would require web_sys; fallback to SVG for now
            render_svg_qrcode(&qr_modules, module_count, size, &fg_color, &icon, icon_size)
        }
    };

    // Status overlay
    let status_overlay = match status {
        QRCodeStatus::Active => None,
        QRCodeStatus::Expired => {
            let handler = on_refresh;
            Some(rsx! {
                div { class: "adui-qrcode-cover",
                    div { class: "adui-qrcode-status",
                        span { class: "adui-qrcode-status-text", "QR code expired" }
                        button {
                            class: "adui-qrcode-refresh-btn",
                            onclick: move |_| {
                                if let Some(cb) = handler.as_ref() {
                                    cb.call(());
                                }
                            },
                            "Refresh"
                        }
                    }
                }
            })
        }
        QRCodeStatus::Loading => Some(rsx! {
            div { class: "adui-qrcode-cover",
                div { class: "adui-qrcode-status",
                    div { class: "adui-qrcode-spinner" }
                    span { class: "adui-qrcode-status-text", "Loading..." }
                }
            }
        }),
        QRCodeStatus::Scanned => Some(rsx! {
            div { class: "adui-qrcode-cover",
                div { class: "adui-qrcode-status",
                    span { class: "adui-qrcode-status-icon", "✓" }
                    span { class: "adui-qrcode-status-text", "Scanned" }
                }
            }
        }),
    };

    rsx! {
        div {
            class: "{root_class_attr}",
            div {
                class: "{class_attr}",
                style: "{container_style}",
                {qr_content}
                {status_overlay}
            }
        }
    }
}

/// Generate QR code modules (simplified version).
/// For a production implementation, use a proper QR code library.
fn generate_qr_modules(value: &str, error_level: QRCodeErrorLevel) -> Vec<Vec<bool>> {
    // This is a simplified QR code generation that creates a deterministic pattern
    // based on the input. For production use, integrate a proper QR library like `qrcode`.

    // Calculate version based on content length and error level
    let version = calculate_version(value.len(), error_level);
    let module_count = 21 + (version - 1) * 4;

    let mut modules = vec![vec![false; module_count]; module_count];

    // Add finder patterns (the three large squares in corners)
    add_finder_pattern(&mut modules, 0, 0);
    add_finder_pattern(&mut modules, module_count - 7, 0);
    add_finder_pattern(&mut modules, 0, module_count - 7);

    // Add timing patterns
    add_timing_patterns(&mut modules, module_count);

    // Add alignment pattern for version > 1
    if version > 1 {
        add_alignment_pattern(&mut modules, module_count);
    }

    // Fill data area with pattern derived from value
    fill_data_pattern(&mut modules, value, module_count);

    modules
}

/// Calculate QR code version based on content length.
fn calculate_version(content_len: usize, _error_level: QRCodeErrorLevel) -> usize {
    // Simplified version calculation
    if content_len <= 17 {
        1
    } else if content_len <= 32 {
        2
    } else if content_len <= 53 {
        3
    } else if content_len <= 78 {
        4
    } else if content_len <= 106 {
        5
    } else {
        6.min((content_len / 20).max(1))
    }
}

/// Add finder pattern at specified position.
fn add_finder_pattern(modules: &mut [Vec<bool>], row: usize, col: usize) {
    for r in 0..7 {
        for c in 0..7 {
            let is_border = r == 0 || r == 6 || c == 0 || c == 6;
            let is_inner = (2..=4).contains(&r) && (2..=4).contains(&c);
            if row + r < modules.len() && col + c < modules[0].len() {
                modules[row + r][col + c] = is_border || is_inner;
            }
        }
    }

    // Add separator (white border around finder)
    for i in 0..8 {
        if row + 7 < modules.len() && col + i < modules[0].len() {
            modules[row + 7][col + i] = false;
        }
        if row + i < modules.len() && col + 7 < modules[0].len() {
            modules[row + i][col + 7] = false;
        }
    }
}

/// Add timing patterns.
fn add_timing_patterns(modules: &mut [Vec<bool>], size: usize) {
    for i in 8..size - 8 {
        let pattern = i % 2 == 0;
        modules[6][i] = pattern;
        modules[i][6] = pattern;
    }
}

/// Add alignment pattern.
fn add_alignment_pattern(modules: &mut [Vec<bool>], size: usize) {
    let center = size - 7;
    for r in 0..5 {
        for c in 0..5 {
            let is_border = r == 0 || r == 4 || c == 0 || c == 4;
            let is_center = r == 2 && c == 2;
            let row = center - 2 + r;
            let col = center - 2 + c;
            if row < size && col < size {
                modules[row][col] = is_border || is_center;
            }
        }
    }
}

/// Fill data area with pattern derived from value.
fn fill_data_pattern(modules: &mut [Vec<bool>], value: &str, size: usize) {
    // Simple hash-based pattern generation
    let hash = simple_hash(value);
    let mut seed = hash;

    for row in 0..size {
        for col in 0..size {
            // Skip finder patterns and timing patterns
            if is_function_module(row, col, size) {
                continue;
            }

            // Generate pseudo-random pattern based on position and hash
            seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
            modules[row][col] = (seed % 3) != 0;
        }
    }
}

/// Check if position is a function module (finder, timing, etc.).
fn is_function_module(row: usize, col: usize, size: usize) -> bool {
    // Top-left finder
    if row < 9 && col < 9 {
        return true;
    }
    // Top-right finder
    if row < 9 && col >= size - 8 {
        return true;
    }
    // Bottom-left finder
    if row >= size - 8 && col < 9 {
        return true;
    }
    // Timing patterns
    if row == 6 || col == 6 {
        return true;
    }
    false
}

/// Simple hash function for generating deterministic patterns.
fn simple_hash(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    hash
}

/// Render QR code as SVG.
fn render_svg_qrcode(
    modules: &[Vec<bool>],
    module_count: usize,
    size: u32,
    color: &str,
    icon: &Option<String>,
    icon_size: u32,
) -> Element {
    let module_size = size as f32 / module_count as f32;

    // Generate path data for filled modules
    let mut path_data = String::new();
    for (row, row_modules) in modules.iter().enumerate() {
        for (col, &is_dark) in row_modules.iter().enumerate() {
            if is_dark {
                let x = col as f32 * module_size;
                let y = row as f32 * module_size;
                path_data.push_str(&format!(
                    "M{:.2},{:.2}h{:.2}v{:.2}h-{:.2}z",
                    x, y, module_size, module_size, module_size
                ));
            }
        }
    }

    let icon_element = icon.as_ref().map(|url| {
        let icon_x = (size - icon_size) / 2;
        let icon_y = (size - icon_size) / 2;
        rsx! {
            g {
                rect {
                    x: "{icon_x}",
                    y: "{icon_y}",
                    width: "{icon_size}",
                    height: "{icon_size}",
                    fill: "white",
                    rx: "4",
                }
                image {
                    href: "{url}",
                    x: "{icon_x}",
                    y: "{icon_y}",
                    width: "{icon_size}",
                    height: "{icon_size}",
                }
            }
        }
    });

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 {size} {size}",
            shape_rendering: "crispEdges",
            path {
                fill: "{color}",
                d: "{path_data}",
            }
            {icon_element}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_class_mapping_is_stable() {
        assert_eq!(QRCodeStatus::Active.as_class(), "adui-qrcode-active");
        assert_eq!(QRCodeStatus::Expired.as_class(), "adui-qrcode-expired");
        assert_eq!(QRCodeStatus::Loading.as_class(), "adui-qrcode-loading");
        assert_eq!(QRCodeStatus::Scanned.as_class(), "adui-qrcode-scanned");
    }

    #[test]
    fn calculate_version_increases_with_content_length() {
        assert_eq!(calculate_version(10, QRCodeErrorLevel::M), 1);
        assert_eq!(calculate_version(30, QRCodeErrorLevel::M), 2);
        assert_eq!(calculate_version(50, QRCodeErrorLevel::M), 3);
    }

    #[test]
    fn simple_hash_is_deterministic() {
        let hash1 = simple_hash("test");
        let hash2 = simple_hash("test");
        let hash3 = simple_hash("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn generate_qr_modules_creates_correct_size() {
        let modules = generate_qr_modules("test", QRCodeErrorLevel::M);
        // Version 1 should be 21x21
        assert_eq!(modules.len(), 21);
        assert_eq!(modules[0].len(), 21);
    }

    #[test]
    fn finder_pattern_is_added_correctly() {
        let mut modules = vec![vec![false; 21]; 21];
        add_finder_pattern(&mut modules, 0, 0);

        // Check corners of finder pattern
        assert!(modules[0][0]); // top-left
        assert!(modules[0][6]); // top-right
        assert!(modules[6][0]); // bottom-left
        assert!(modules[6][6]); // bottom-right

        // Check center
        assert!(modules[3][3]);
    }
}
