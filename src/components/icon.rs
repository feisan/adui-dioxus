use dioxus::prelude::*;

/// Built-in icon set (minimal subset aligned with Ant Design semantics).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum IconKind {
    Plus,
    Minus,
    Check,
    Close,
    #[default]
    Info,
    Question,
    ArrowRight,
    ArrowLeft,
    ArrowUp,
    ArrowDown,
    Search,
    Copy,
    Edit,
    Loading,
    Eye,
    EyeInvisible,
}

/// Icon props.
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default)]
    pub kind: IconKind,
    #[props(default = 20.0)]
    pub size: f32,
    #[props(optional)]
    pub color: Option<String>,
    #[props(optional)]
    pub rotate: Option<f32>,
    #[props(default)]
    pub spin: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub aria_label: Option<String>,
    #[props(optional)]
    pub view_box: Option<String>,
    /// 自定义 SVG 内容，若提供则忽略内置 `kind`。
    #[props(optional)]
    pub custom: Option<Element>,
}

/// SVG-based icon component with small built-in set.
#[component]
pub fn Icon(props: IconProps) -> Element {
    let IconProps {
        kind,
        size,
        color,
        rotate,
        spin,
        class,
        aria_label,
        view_box,
        custom,
    } = props;

    let def = icon_def(kind);
    let mut class_list = vec!["adui-icon".to_string()];
    if spin || matches!(kind, IconKind::Loading) {
        class_list.push("adui-icon-spin".into());
    }
    if let Some(extra) = class.as_ref() {
        class_list.push(extra.clone());
    }
    let class_attr = class_list.join(" ");

    let style = format!(
        "width:{size}px;height:{size}px;{}",
        rotate
            .map(|deg| format!("transform:rotate({deg}deg);"))
            .unwrap_or_default()
    );

    let stroke_color = color.clone().unwrap_or_else(|| "currentColor".into());

    let aria_text = aria_label.unwrap_or_else(|| format!("{:?}", kind));
    let view_box_attr = view_box.unwrap_or_else(|| def.view_box.to_string());

    rsx! {
        svg {
            class: "{class_attr}",
            style: "{style}",
            width: "{size}",
            height: "{size}",
            view_box: "{view_box_attr}",
            fill: if def.fill { stroke_color.clone() } else { "none".into() },
            stroke: if def.fill { "none" } else { stroke_color.as_str() },
            stroke_width: "1.6",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": aria_text.clone(),
            "aria-hidden": if aria_text.is_empty() { "true" } else { "false" },
            if let Some(content) = custom {
                {content}
            } else {
                {def.paths.iter().map(|d| {
                    let fill = if def.fill { "currentColor" } else { "none" };
                    rsx!(path { d: "{d}", fill: "{fill}" })
                })}
            }
        }
    }
}

struct IconDef {
    view_box: &'static str,
    fill: bool,
    paths: &'static [&'static str],
}

fn icon_def(kind: IconKind) -> IconDef {
    match kind {
        IconKind::Plus => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M12 5v14", "M5 12h14"],
        },
        IconKind::Minus => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M5 12h14"],
        },
        IconKind::Check => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M5 13l4 4 10-10"],
        },
        IconKind::Close => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M6 6l12 12", "M6 18L18 6"],
        },
        IconKind::Info => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M12 4a8 8 0 1 0 0 16 8 8 0 0 0 0-16Z",
                "M12 10v6",
                "M12 8h.01",
            ],
        },
        IconKind::Question => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M12 4a8 8 0 1 0 0 16 8 8 0 0 0 0-16Z",
                "M9.5 9.5a2.5 2.5 0 0 1 5 0c0 1.667-1.5 2-2 3",
                "M12 16h.01",
            ],
        },
        IconKind::ArrowRight => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M5 12h14", "M13 6l6 6-6 6"],
        },
        IconKind::ArrowLeft => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M19 12H5", "M11 6l-6 6 6 6"],
        },
        IconKind::ArrowUp => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M12 19V5", "M6 11l6-6 6 6"],
        },
        IconKind::ArrowDown => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M12 5v14", "M18 13l-6 6-6-6"],
        },
        IconKind::Search => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &["M11 4a7 7 0 1 0 0 14 7 7 0 0 0 0-14Z", "M21 21l-4.35-4.35"],
        },
        IconKind::Copy => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M9 9V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-4",
                "M5 9h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2Z",
            ],
        },
        IconKind::Edit => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M4 20h4l10.5-10.5a2.121 2.121 0 0 0-3-3L5 17v3Z",
                "M14.5 6.5l3 3",
            ],
        },
        IconKind::Loading => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M12 2v4",
                "M12 18v4",
                "M4.93 4.93l2.83 2.83",
                "M16.24 16.24l2.83 2.83",
                "M2 12h4",
                "M18 12h4",
                "M4.93 19.07l2.83-2.83",
                "M16.24 7.76l2.83-2.83",
            ],
        },
        IconKind::Eye => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8Z",
                "M12 9a3 3 0 1 0 0 6 3 3 0 0 0 0-6Z",
            ],
        },
        IconKind::EyeInvisible => IconDef {
            view_box: "0 0 24 24",
            fill: false,
            paths: &[
                "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94",
                "M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19",
                "M14.12 14.12a3 3 0 1 1-4.24-4.24",
                "M1 1l22 22",
            ],
        },
    }
}
