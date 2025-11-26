use dioxus::prelude::*;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

static COL_ID: AtomicUsize = AtomicUsize::new(0);
static ROW_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GridBreakpoint {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

const BREAKPOINT_RULES: &[(GridBreakpoint, u32)] = &[
    (GridBreakpoint::Xs, 0),
    (GridBreakpoint::Sm, 576),
    (GridBreakpoint::Md, 768),
    (GridBreakpoint::Lg, 992),
    (GridBreakpoint::Xl, 1200),
    (GridBreakpoint::Xxl, 1600),
];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponsiveValue {
    pub xs: Option<f32>,
    pub sm: Option<f32>,
    pub md: Option<f32>,
    pub lg: Option<f32>,
    pub xl: Option<f32>,
    pub xxl: Option<f32>,
}

impl ResponsiveValue {
    fn iter(&self) -> Vec<(GridBreakpoint, f32)> {
        let mut entries = Vec::new();
        if let Some(v) = self.xs {
            entries.push((GridBreakpoint::Xs, v));
        }
        if let Some(v) = self.sm {
            entries.push((GridBreakpoint::Sm, v));
        }
        if let Some(v) = self.md {
            entries.push((GridBreakpoint::Md, v));
        }
        if let Some(v) = self.lg {
            entries.push((GridBreakpoint::Lg, v));
        }
        if let Some(v) = self.xl {
            entries.push((GridBreakpoint::Xl, v));
        }
        if let Some(v) = self.xxl {
            entries.push((GridBreakpoint::Xxl, v));
        }
        entries
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponsiveGutter {
    pub horizontal: ResponsiveValue,
    pub vertical: Option<ResponsiveValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RowGutter {
    Uniform(f32),
    Pair(f32, f32),
    Responsive(ResponsiveGutter),
}

/// Horizontal justification for a grid row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RowJustify {
    #[default]
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

/// Cross-axis alignment for row items.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RowAlign {
    #[default]
    Top,
    Middle,
    Bottom,
    Stretch,
}

/// Layout props for a row container.
#[derive(Props, Clone, PartialEq)]
pub struct RowProps {
    #[props(optional)]
    pub gutter: Option<f32>,
    #[props(optional)]
    pub gutter_vertical: Option<f32>,
    #[props(optional)]
    pub responsive_gutter: Option<ResponsiveGutter>,
    #[props(optional)]
    pub gutter_spec: Option<RowGutter>,
    #[props(default)]
    pub justify: RowJustify,
    #[props(default)]
    pub align: RowAlign,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Flex-based grid row with 24-column gutter system.
#[component]
pub fn Row(props: RowProps) -> Element {
    let RowProps {
        gutter,
        gutter_vertical,
        responsive_gutter,
        gutter_spec,
        justify,
        align,
        class,
        style,
        children,
    } = props;

    let row_id = ROW_ID.fetch_add(1, Ordering::Relaxed);
    let mut class_list = vec!["adui-row".to_string()];
    class_list.push(format!("adui-row-{row_id}"));
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");
    let mut base_x = gutter.unwrap_or(0.0);
    let mut base_y = gutter_vertical.unwrap_or(0.0);
    let mut responsive_cfg = responsive_gutter.clone();

    if let Some(spec) = gutter_spec {
        match spec {
            RowGutter::Uniform(v) => {
                base_x = v;
                base_y = 0.0;
                responsive_cfg = None;
            }
            RowGutter::Pair(h, v) => {
                base_x = h;
                base_y = v;
                responsive_cfg = None;
            }
            RowGutter::Responsive(cfg) => {
                responsive_cfg = Some(cfg);
            }
        }
    }
    let mut style_buffer = String::new();
    if let Some(extra) = style.as_ref() {
        style_buffer.push_str(extra);
    }
    let style_attr = format!(
        "display:flex;flex-wrap:wrap;margin-left:calc(var(--adui-row-gutter-x,{base_x}px)/-2);margin-right:calc(var(--adui-row-gutter-x,{base_x}px)/-2);row-gap:var(--adui-row-gutter-y,{base_y}px);column-gap:var(--adui-row-gutter-x,{base_x}px);justify-content:{};align-items:{};{}",
        match justify {
            RowJustify::Start => "flex-start",
            RowJustify::End => "flex-end",
            RowJustify::Center => "center",
            RowJustify::SpaceAround => "space-around",
            RowJustify::SpaceBetween => "space-between",
            RowJustify::SpaceEvenly => "space-evenly",
        },
        match align {
            RowAlign::Top => "flex-start",
            RowAlign::Middle => "center",
            RowAlign::Bottom => "flex-end",
            RowAlign::Stretch => "stretch",
        },
        style_buffer
    );
    let responsive_rules = responsive_row_rules(row_id, base_x, base_y, responsive_cfg.as_ref());

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

/// Column sizing options within a row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColSize {
    pub span: Option<u16>,
    pub offset: Option<u16>,
    pub push: Option<i16>,
    pub pull: Option<i16>,
    pub order: Option<i16>,
    pub flex: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColResponsive {
    pub xs: Option<ColSize>,
    pub sm: Option<ColSize>,
    pub md: Option<ColSize>,
    pub lg: Option<ColSize>,
    pub xl: Option<ColSize>,
    pub xxl: Option<ColSize>,
}

impl ColSize {
    pub fn is_empty(&self) -> bool {
        self.span.is_none()
            && self.offset.is_none()
            && self.push.is_none()
            && self.pull.is_none()
            && self.order.is_none()
            && self.flex.is_none()
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ColProps {
    #[props(default = 24)]
    pub span: u16,
    #[props(default)]
    pub offset: u16,
    #[props(optional)]
    pub push: Option<i16>,
    #[props(optional)]
    pub pull: Option<i16>,
    #[props(optional)]
    pub order: Option<i16>,
    #[props(optional)]
    pub flex: Option<String>,
    #[props(optional)]
    pub responsive: Option<ColResponsive>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    pub children: Element,
}

/// Grid column in a 24-part system with optional flex sizing.
#[component]
pub fn Col(props: ColProps) -> Element {
    let ColProps {
        span,
        offset,
        push,
        pull,
        order,
        flex,
        class,
        style,
        children,
        responsive,
    } = props;

    let mut class_list = vec!["adui-col".to_string()];
    let id = COL_ID.fetch_add(1, Ordering::Relaxed);
    class_list.push(format!("adui-col-{id}"));
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let width_percent = (span as f32 / 24.0) * 100.0;
    let offset_percent = (offset as f32 / 24.0) * 100.0;

    let mut style_buf = String::new();
    if let Some(flex_val) = flex {
        let _ = write!(style_buf, "flex:{flex_val};max-width:100%;");
    } else {
        let _ = write!(
            style_buf,
            "flex:0 0 {width_percent}%;max-width:{width_percent}%;"
        );
    }
    if offset > 0 {
        let _ = write!(style_buf, "margin-left:{offset_percent}%;");
    }
    if let Some(val) = push {
        let shift = column_percent(val);
        let _ = write!(style_buf, "position:relative;left:{shift}%;");
    }
    if let Some(val) = pull {
        let shift = column_percent(val);
        let _ = write!(style_buf, "position:relative;right:{shift}%;");
    }
    if let Some(ord) = order {
        let _ = write!(style_buf, "order:{ord};");
    }
    let _ = write!(
        style_buf,
        "padding:0 calc(var(--adui-row-gutter-x, 0px)/2);padding-bottom:var(--adui-row-gutter-y, 0px);box-sizing:border-box;{}",
        style.unwrap_or_default()
    );
    let style_attr = style_buf;

    let responsive_rules = responsive_col_rules(id, responsive.as_ref());

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

fn append_responsive_axis(
    buffer: &mut String,
    row_id: usize,
    axis: &str,
    base: f32,
    values: &ResponsiveValue,
) {
    let _ = writeln!(
        buffer,
        ".adui-row-{row_id} {{ --adui-row-gutter-{axis}:{base}px; }}"
    );
    for (bp, value) in values.iter() {
        let rule = if let Some((_, min_width)) = BREAKPOINT_RULES
            .iter()
            .find(|(breakpoint, _)| *breakpoint == bp)
        {
            if *min_width == 0 {
                format!(".adui-row-{row_id} {{ --adui-row-gutter-{axis}:{value}px; }}")
            } else {
                format!(
                    "@media (min-width: {min_width}px) {{ .adui-row-{row_id} {{ --adui-row-gutter-{axis}:{value}px; }} }}"
                )
            }
        } else {
            format!(".adui-row-{row_id} {{ --adui-row-gutter-{axis}:{value}px; }}")
        };
        let _ = writeln!(buffer, "{rule}");
    }
}

fn responsive_row_rules(
    row_id: usize,
    base_x: f32,
    base_y: f32,
    responsive: Option<&ResponsiveGutter>,
) -> Option<String> {
    let responsive = responsive?;
    let mut buffer = String::new();

    append_responsive_axis(&mut buffer, row_id, "x", base_x, &responsive.horizontal);
    if let Some(vertical) = responsive.vertical.as_ref() {
        append_responsive_axis(&mut buffer, row_id, "y", base_y, vertical);
    } else {
        let _ = writeln!(
            buffer,
            ".adui-row-{row_id} {{ --adui-row-gutter-y:{base_y}px; }}"
        );
    }

    Some(buffer)
}

fn responsive_col_rules(id: usize, responsive: Option<&ColResponsive>) -> Option<String> {
    let responsive = responsive?;
    let mut buffer = String::new();

    for (bp, min_width) in BREAKPOINT_RULES {
        let size = match bp {
            GridBreakpoint::Xs => responsive.xs.as_ref(),
            GridBreakpoint::Sm => responsive.sm.as_ref(),
            GridBreakpoint::Md => responsive.md.as_ref(),
            GridBreakpoint::Lg => responsive.lg.as_ref(),
            GridBreakpoint::Xl => responsive.xl.as_ref(),
            GridBreakpoint::Xxl => responsive.xxl.as_ref(),
        };
        if let Some(size) = size {
            if size.is_empty() {
                continue;
            }
            let mut declarations = String::new();
            if let Some(span) = size.span {
                let pct = (span as f32 / 24.0) * 100.0;
                let _ = write!(declarations, "flex:0 0 {pct}%;max-width:{pct}%;");
            }
            if let Some(offset) = size.offset {
                let pct = (offset as f32 / 24.0) * 100.0;
                let _ = write!(declarations, "margin-left:{pct}%;");
            }
            if let Some(push) = size.push {
                let shift = column_percent(push);
                let _ = write!(declarations, "position:relative;left:{shift}%;");
            }
            if let Some(pull) = size.pull {
                let shift = column_percent(pull);
                let _ = write!(declarations, "position:relative;right:{shift}%;");
            }
            if let Some(order) = size.order {
                let _ = write!(declarations, "order:{order};");
            }
            if let Some(flex_val) = size.flex.as_ref() {
                let _ = write!(declarations, "flex:{flex_val};max-width:100%;");
            }

            if declarations.is_empty() {
                continue;
            }

            if *min_width == 0 {
                let _ = write!(buffer, ".adui-col-{id} {{{declarations}}}");
            } else {
                let _ = write!(
                    buffer,
                    "@media (min-width: {min_width}px) {{ .adui-col-{id} {{{declarations}}} }}"
                );
            }
        }
    }

    if buffer.is_empty() {
        None
    } else {
        Some(buffer)
    }
}

fn column_percent(value: i16) -> f32 {
    (value as f32 / 24.0) * 100.0
}
