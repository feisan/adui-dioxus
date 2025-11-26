use dioxus::prelude::*;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

static MASONRY_ID: AtomicUsize = AtomicUsize::new(0);

const BREAKPOINT_RULES: [(&str, u32); 6] = [
    ("xs", 0),
    ("sm", 576),
    ("md", 768),
    ("lg", 992),
    ("xl", 1200),
    ("xxl", 1600),
];

/// Responsive column counts per breakpoint（沿用 antd 栅格语义）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MasonryResponsive {
    pub xs: Option<u16>,
    pub sm: Option<u16>,
    pub md: Option<u16>,
    pub lg: Option<u16>,
    pub xl: Option<u16>,
    pub xxl: Option<u16>,
}

impl MasonryResponsive {
    fn iter(&self) -> Vec<(&'static str, u16)> {
        let mut list = Vec::new();
        if let Some(v) = self.xs {
            list.push(("xs", v));
        }
        if let Some(v) = self.sm {
            list.push(("sm", v));
        }
        if let Some(v) = self.md {
            list.push(("md", v));
        }
        if let Some(v) = self.lg {
            list.push(("lg", v));
        }
        if let Some(v) = self.xl {
            list.push(("xl", v));
        }
        if let Some(v) = self.xxl {
            list.push(("xxl", v));
        }
        list
    }
}

/// Properties for a responsive masonry layout.
#[derive(Props, Clone, PartialEq)]
pub struct MasonryProps {
    #[props(default = 3)]
    pub columns: u16,
    #[props(optional)]
    pub responsive: Option<MasonryResponsive>,
    #[props(optional)]
    pub gap: Option<f32>,
    #[props(optional)]
    pub row_gap: Option<f32>,
    #[props(optional)]
    pub min_column_width: Option<f32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Masonry layout using CSS columns with responsive overrides.
#[component]
pub fn Masonry(props: MasonryProps) -> Element {
    let MasonryProps {
        columns,
        responsive,
        gap,
        row_gap,
        min_column_width,
        class,
        style,
        children,
    } = props;

    let id = MASONRY_ID.fetch_add(1, Ordering::Relaxed);
    let mut class_list = vec!["adui-masonry".to_string(), format!("adui-masonry-{}", id)];
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let column_gap = gap.unwrap_or(16.0);
    let row_gap_value = row_gap.unwrap_or(column_gap);

    let mut style_attr = format!(
        "column-count:{};column-gap:{}px;--adui-masonry-gap:{}px;--adui-masonry-row-gap:{}px;",
        columns, column_gap, column_gap, row_gap_value
    );
    if let Some(width) = min_column_width {
        let _ = write!(style_attr, "column-width:{}px;", width);
    }
    if let Some(extra) = style.as_ref() {
        style_attr.push_str(extra);
    }

    let responsive_rules = responsive
        .as_ref()
        .and_then(|cfg| responsive_columns_rules(id, cfg));

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            if let Some(rules) = responsive_rules {
                style { {rules} }
            }
            {children}
        }
    }
}

fn responsive_columns_rules(id: usize, responsive: &MasonryResponsive) -> Option<String> {
    let entries = responsive.iter();
    if entries.is_empty() {
        return None;
    }

    let mut buffer = String::new();
    for (name, min_width) in BREAKPOINT_RULES {
        if let Some((_, columns)) = entries.iter().find(|(bp, _)| *bp == name) {
            if min_width == 0 {
                let _ = writeln!(buffer, ".adui-masonry-{id} {{ column-count:{columns}; }}");
            } else {
                let _ = writeln!(
                    buffer,
                    "@media (min-width: {min_width}px) {{ .adui-masonry-{id} {{ column-count:{columns}; }} }}"
                );
            }
        }
    }
    Some(buffer)
}
