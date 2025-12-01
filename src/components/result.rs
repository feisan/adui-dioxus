use crate::components::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Status of a Result view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultStatus {
    Success,
    Info,
    Warning,
    Error,
    NotFound,
    Forbidden,
    ServerError,
}

impl ResultStatus {
    fn as_class(&self) -> &'static str {
        match self {
            ResultStatus::Success => "adui-result-success",
            ResultStatus::Info => "adui-result-info",
            ResultStatus::Warning => "adui-result-warning",
            ResultStatus::Error => "adui-result-error",
            ResultStatus::NotFound => "adui-result-404",
            ResultStatus::Forbidden => "adui-result-403",
            ResultStatus::ServerError => "adui-result-500",
        }
    }

    fn icon_kind(&self) -> IconKind {
        match self {
            ResultStatus::Success => IconKind::Check,
            ResultStatus::Error | ResultStatus::ServerError => IconKind::Close,
            ResultStatus::Warning | ResultStatus::Forbidden | ResultStatus::NotFound => {
                IconKind::Info
            }
            ResultStatus::Info => IconKind::Info,
        }
    }
}

/// Props for the Result component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct ResultProps {
    /// Overall status of the result.
    #[props(optional)]
    pub status: Option<ResultStatus>,
    /// Optional custom icon.
    #[props(optional)]
    pub icon: Option<Element>,
    /// Title of the result page.
    #[props(optional)]
    pub title: Option<Element>,
    /// Optional subtitle/description text.
    #[props(optional)]
    pub sub_title: Option<Element>,
    /// Extra action area, typically buttons.
    #[props(optional)]
    pub extra: Option<Element>,
    /// Extra class on the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style on the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Optional content section rendered below extra.
    pub children: Option<Element>,
}

/// Ant Design flavored Result (MVP: status + icon + title/subtitle/extra/content).
#[component]
pub fn Result(props: ResultProps) -> Element {
    let ResultProps {
        status,
        icon,
        title,
        sub_title,
        extra,
        class,
        style,
        children,
    } = props;

    let status_value = status.unwrap_or(ResultStatus::Info);

    let mut class_list = vec![
        "adui-result".to_string(),
        status_value.as_class().to_string(),
    ];
    if let Some(extra_class) = class {
        class_list.push(extra_class);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let icon_node = icon.map(Some).unwrap_or_else(|| {
        Some(rsx!(Icon {
            kind: status_value.icon_kind(),
            size: 40.0,
        }))
    });

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            if let Some(node) = icon_node {
                div { class: "adui-result-icon", {node} }
            }
            if let Some(t) = title {
                div { class: "adui-result-title", {t} }
            }
            if let Some(st) = sub_title {
                div { class: "adui-result-subtitle", {st} }
            }
            if let Some(extra_node) = extra {
                div { class: "adui-result-extra", {extra_node} }
            }
            if let Some(content) = children {
                div { class: "adui-result-content", {content} }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_status_class_mapping_is_stable() {
        assert_eq!(ResultStatus::Success.as_class(), "adui-result-success");
        assert_eq!(ResultStatus::Info.as_class(), "adui-result-info");
        assert_eq!(ResultStatus::Warning.as_class(), "adui-result-warning");
        assert_eq!(ResultStatus::Error.as_class(), "adui-result-error");
        assert_eq!(ResultStatus::NotFound.as_class(), "adui-result-404");
        assert_eq!(ResultStatus::Forbidden.as_class(), "adui-result-403");
        assert_eq!(ResultStatus::ServerError.as_class(), "adui-result-500");
    }

    #[test]
    fn result_status_icon_mapping() {
        assert_eq!(ResultStatus::Success.icon_kind(), IconKind::Check);
        assert_eq!(ResultStatus::Info.icon_kind(), IconKind::Info);
        assert_eq!(ResultStatus::Warning.icon_kind(), IconKind::Info);
        assert_eq!(ResultStatus::Error.icon_kind(), IconKind::Close);
        assert_eq!(ResultStatus::NotFound.icon_kind(), IconKind::Info);
        assert_eq!(ResultStatus::Forbidden.icon_kind(), IconKind::Info);
        assert_eq!(ResultStatus::ServerError.icon_kind(), IconKind::Close);
    }

    #[test]
    fn result_status_all_variants() {
        let variants = [
            ResultStatus::Success,
            ResultStatus::Info,
            ResultStatus::Warning,
            ResultStatus::Error,
            ResultStatus::NotFound,
            ResultStatus::Forbidden,
            ResultStatus::ServerError,
        ];
        for variant in variants.iter() {
            let class = variant.as_class();
            assert!(!class.is_empty());
            assert!(class.starts_with("adui-result-"));
            let icon = variant.icon_kind();
            // Just verify icon_kind doesn't panic
            let _ = format!("{:?}", icon);
        }
    }

    #[test]
    fn result_status_equality() {
        assert_eq!(ResultStatus::Success, ResultStatus::Success);
        assert_eq!(ResultStatus::Info, ResultStatus::Info);
        assert_ne!(ResultStatus::Success, ResultStatus::Error);
        assert_ne!(ResultStatus::NotFound, ResultStatus::Forbidden);
    }

    #[test]
    fn result_status_clone() {
        let original = ResultStatus::Warning;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(original.as_class(), cloned.as_class());
        assert_eq!(original.icon_kind(), cloned.icon_kind());
    }

    #[test]
    fn result_props_defaults() {
        // ResultProps doesn't require any fields, but status defaults to Info when None
        // All other fields are optional
    }

    #[test]
    fn result_status_debug() {
        let status = ResultStatus::ServerError;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("ServerError"));
    }
}
