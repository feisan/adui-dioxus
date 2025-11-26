use std::fmt::Write;

/// Shared preset for spacing utilities (small/middle/large).
pub(crate) enum GapPreset {
    Small,
    Middle,
    Large,
}

impl GapPreset {
    fn suffix(&self) -> &'static str {
        match self {
            GapPreset::Small => "small",
            GapPreset::Middle => "middle",
            GapPreset::Large => "large",
        }
    }
}

pub(crate) fn push_gap_preset_class(
    class_list: &mut Vec<String>,
    prefix: &str,
    preset: Option<GapPreset>,
) {
    if let Some(size) = preset {
        class_list.push(format!("{prefix}-{}", size.suffix()));
    }
}

pub(crate) fn compose_gap_style(
    base_style: Option<String>,
    gap: Option<f32>,
    row_gap: Option<f32>,
    column_gap: Option<f32>,
) -> String {
    let mut buffer = String::new();
    if let Some(value) = gap {
        let _ = write!(buffer, "gap:{value}px;");
    }
    if let Some(value) = row_gap {
        let _ = write!(buffer, "row-gap:{value}px;");
    }
    if let Some(value) = column_gap {
        let _ = write!(buffer, "column-gap:{value}px;");
    }
    if let Some(extra) = base_style {
        buffer.push_str(&extra);
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_gap_preset_adds_suffix() {
        let mut classes = vec!["adui-space".into()];
        push_gap_preset_class(&mut classes, "adui-space", Some(GapPreset::Middle));
        assert_eq!(
            classes,
            vec!["adui-space".to_string(), "adui-space-middle".to_string()]
        );
    }

    #[test]
    fn compose_gap_style_merges_values() {
        let style = compose_gap_style(Some("color:red;".into()), Some(8.0), None, Some(4.0));
        assert!(style.contains("gap:8"));
        assert!(style.contains("column-gap:4"));
        assert!(style.ends_with("color:red;"));
    }
}
