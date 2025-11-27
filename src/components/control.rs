/// Common status for input-like controls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ControlStatus {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

impl ControlStatus {
    /// Optional CSS class name associated with this status.
    pub fn class(self) -> Option<&'static str> {
        match self {
            ControlStatus::Default => None,
            ControlStatus::Success => Some("adui-control-status-success"),
            ControlStatus::Warning => Some("adui-control-status-warning"),
            ControlStatus::Error => Some("adui-control-status-error"),
        }
    }
}

/// Helper to push a status class into an existing class list.
pub fn push_status_class(classes: &mut Vec<String>, status: Option<ControlStatus>) {
    if let Some(name) = status.and_then(ControlStatus::class) {
        classes.push(name.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_status_class_mapping() {
        assert_eq!(ControlStatus::Default.class(), None);
        assert_eq!(
            ControlStatus::Success.class(),
            Some("adui-control-status-success")
        );
        assert_eq!(
            ControlStatus::Warning.class(),
            Some("adui-control-status-warning")
        );
        assert_eq!(
            ControlStatus::Error.class(),
            Some("adui-control-status-error")
        );
    }

    #[test]
    fn push_status_class_appends_when_present() {
        let mut classes = vec!["base".to_string()];
        push_status_class(&mut classes, Some(ControlStatus::Error));
        assert!(classes.contains(&"base".to_string()));
        assert!(classes.contains(&"adui-control-status-error".to_string()));
    }
}
