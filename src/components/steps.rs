use crate::components::config_provider::ComponentSize;
use dioxus::prelude::*;

/// Visual status of an individual step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepStatus {
    Wait,
    Process,
    Finish,
    Error,
}

impl StepStatus {
    fn as_class(&self) -> &'static str {
        match self {
            StepStatus::Wait => "adui-steps-status-wait",
            StepStatus::Process => "adui-steps-status-process",
            StepStatus::Finish => "adui-steps-status-finish",
            StepStatus::Error => "adui-steps-status-error",
        }
    }
}

/// Direction of the steps bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepsDirection {
    Horizontal,
    Vertical,
}

impl StepsDirection {
    fn as_class(&self) -> &'static str {
        match self {
            StepsDirection::Horizontal => "adui-steps-horizontal",
            StepsDirection::Vertical => "adui-steps-vertical",
        }
    }
}

/// Data model for a single step item.
#[derive(Clone, PartialEq)]
pub struct StepItem {
    pub key: String,
    pub title: Element,
    pub description: Option<Element>,
    pub status: Option<StepStatus>,
    pub disabled: bool,
}

impl StepItem {
    pub fn new(key: impl Into<String>, title: Element) -> Self {
        Self {
            key: key.into(),
            title,
            description: None,
            status: None,
            disabled: false,
        }
    }
}

/// Props for the Steps component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct StepsProps {
    pub items: Vec<StepItem>,
    #[props(optional)]
    pub current: Option<usize>,
    #[props(optional)]
    pub default_current: Option<usize>,
    #[props(optional)]
    pub on_change: Option<EventHandler<usize>>,
    #[props(optional)]
    pub direction: Option<StepsDirection>,
    #[props(optional)]
    pub size: Option<ComponentSize>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
}

fn effective_status(index: usize, current: usize, explicit: Option<StepStatus>) -> StepStatus {
    if let Some(st) = explicit {
        return st;
    }
    if index < current {
        StepStatus::Finish
    } else if index == current {
        StepStatus::Process
    } else {
        StepStatus::Wait
    }
}

/// Ant Design flavored Steps (MVP: horizontal line steps with basic status).
#[component]
pub fn Steps(props: StepsProps) -> Element {
    let StepsProps {
        items,
        current,
        default_current,
        on_change,
        direction,
        size,
        class,
        style,
    } = props;

    let initial_current = default_current.unwrap_or(0);
    let current_internal: Signal<usize> = use_signal(|| initial_current);
    let is_controlled = current.is_some();
    let current_index = current.unwrap_or_else(|| *current_internal.read());

    let dir = direction.unwrap_or(StepsDirection::Horizontal);

    let mut class_list = vec!["adui-steps".to_string(), dir.as_class().to_string()];
    if let Some(sz) = size {
        match sz {
            ComponentSize::Small => class_list.push("adui-steps-sm".into()),
            ComponentSize::Middle => {}
            ComponentSize::Large => class_list.push("adui-steps-lg".into()),
        }
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    let on_change_cb = on_change;

    rsx! {
        ol { class: "{class_attr}", style: "{style_attr}",
            {items.iter().enumerate().map(|(idx, item)| {
                let status = effective_status(idx, current_index, item.status);
                let mut item_class = vec!["adui-steps-item".to_string(), status.as_class().to_string()];
                if item.disabled {
                    item_class.push("adui-steps-item-disabled".into());
                }
                if idx == current_index {
                    item_class.push("adui-steps-item-current".into());
                }
                let item_class_attr = item_class.join(" ");

                let current_internal_for_click = current_internal;
                let on_change_for_click = on_change_cb;
                let disabled = item.disabled;

                let title = item.title.clone();
                let description = item.description.clone();
                let display_index = idx + 1;

                rsx! {
                    li {
                        key: "step-{idx}",
                        class: "{item_class_attr}",
                        onclick: move |_| {
                            if disabled {
                                return;
                            }
                            if !is_controlled {
                                let mut sig = current_internal_for_click;
                                sig.set(idx);
                            }
                            if let Some(cb) = on_change_for_click {
                                cb.call(idx);
                            }
                        },
                        div { class: "adui-steps-item-icon",
                            span { class: "adui-steps-item-index", "{display_index}" }
                        }
                        div { class: "adui-steps-item-content",
                            div { class: "adui-steps-item-title", {title} }
                            if let Some(desc) = description {
                                div { class: "adui-steps-item-description", {desc} }
                            }
                        }
                    }
                }
            })}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_status_defaults_by_index() {
        assert_eq!(effective_status(0, 1, None), StepStatus::Finish);
        assert_eq!(effective_status(1, 1, None), StepStatus::Process);
        assert_eq!(effective_status(2, 1, None), StepStatus::Wait);
    }

    #[test]
    fn effective_status_respects_explicit_status() {
        assert_eq!(
            effective_status(0, 1, Some(StepStatus::Error)),
            StepStatus::Error
        );
        assert_eq!(
            effective_status(1, 1, Some(StepStatus::Wait)),
            StepStatus::Wait
        );
        assert_eq!(
            effective_status(2, 1, Some(StepStatus::Finish)),
            StepStatus::Finish
        );
    }

    #[test]
    fn effective_status_before_current() {
        assert_eq!(effective_status(0, 2, None), StepStatus::Finish);
        assert_eq!(effective_status(1, 2, None), StepStatus::Finish);
    }

    #[test]
    fn effective_status_at_current() {
        assert_eq!(effective_status(0, 0, None), StepStatus::Process);
        assert_eq!(effective_status(5, 5, None), StepStatus::Process);
    }

    #[test]
    fn effective_status_after_current() {
        assert_eq!(effective_status(3, 1, None), StepStatus::Wait);
        assert_eq!(effective_status(10, 5, None), StepStatus::Wait);
    }

    #[test]
    fn step_status_class_mapping() {
        assert_eq!(StepStatus::Wait.as_class(), "adui-steps-status-wait");
        assert_eq!(StepStatus::Process.as_class(), "adui-steps-status-process");
        assert_eq!(StepStatus::Finish.as_class(), "adui-steps-status-finish");
        assert_eq!(StepStatus::Error.as_class(), "adui-steps-status-error");
    }

    #[test]
    fn step_status_all_variants() {
        let variants = [
            StepStatus::Wait,
            StepStatus::Process,
            StepStatus::Finish,
            StepStatus::Error,
        ];
        for variant in variants.iter() {
            let class = variant.as_class();
            assert!(!class.is_empty());
            assert!(class.starts_with("adui-steps-status-"));
        }
    }

    #[test]
    fn steps_direction_class_mapping() {
        assert_eq!(
            StepsDirection::Horizontal.as_class(),
            "adui-steps-horizontal"
        );
        assert_eq!(StepsDirection::Vertical.as_class(), "adui-steps-vertical");
    }

    #[test]
    fn steps_direction_equality() {
        assert_eq!(StepsDirection::Horizontal, StepsDirection::Horizontal);
        assert_eq!(StepsDirection::Vertical, StepsDirection::Vertical);
        assert_ne!(StepsDirection::Horizontal, StepsDirection::Vertical);
    }

    #[test]
    fn step_item_new() {
        let item = StepItem::new("key1", rsx!(div { "Title" }));
        assert_eq!(item.key, "key1");
        assert_eq!(item.description, None);
        assert_eq!(item.status, None);
        assert_eq!(item.disabled, false);
    }

    #[test]
    fn step_item_clone() {
        let item = StepItem::new("key1", rsx!(div { "Title" }));
        let cloned = item.clone();
        assert_eq!(item.key, cloned.key);
        assert_eq!(item.disabled, cloned.disabled);
    }

    #[test]
    fn steps_props_defaults() {
        // StepsProps requires items, so we can't create a fully default instance
        // But we can verify:
        // current is optional
        // default_current is optional
        // direction is optional (defaults to Horizontal)
        // size is optional
    }

    #[test]
    fn effective_status_all_status_variants() {
        // Test all status variants with explicit status
        assert_eq!(
            effective_status(0, 0, Some(StepStatus::Wait)),
            StepStatus::Wait
        );
        assert_eq!(
            effective_status(0, 0, Some(StepStatus::Process)),
            StepStatus::Process
        );
        assert_eq!(
            effective_status(0, 0, Some(StepStatus::Finish)),
            StepStatus::Finish
        );
        assert_eq!(
            effective_status(0, 0, Some(StepStatus::Error)),
            StepStatus::Error
        );
    }

    #[test]
    fn effective_status_explicit_overrides_index() {
        // Explicit status should override index-based logic
        assert_eq!(
            effective_status(10, 5, Some(StepStatus::Finish)),
            StepStatus::Finish
        );
        assert_eq!(
            effective_status(0, 10, Some(StepStatus::Wait)),
            StepStatus::Wait
        );
    }

    #[test]
    fn effective_status_boundary_index_zero() {
        assert_eq!(effective_status(0, 0, None), StepStatus::Process);
        assert_eq!(effective_status(0, 1, None), StepStatus::Finish);
    }

    #[test]
    fn effective_status_large_indices() {
        assert_eq!(effective_status(100, 50, None), StepStatus::Wait);
        assert_eq!(effective_status(50, 100, None), StepStatus::Finish);
        assert_eq!(effective_status(100, 100, None), StepStatus::Process);
    }

    #[test]
    fn effective_status_index_equals_current() {
        // When index equals current, should be Process
        assert_eq!(effective_status(0, 0, None), StepStatus::Process);
        assert_eq!(effective_status(1, 1, None), StepStatus::Process);
        assert_eq!(effective_status(99, 99, None), StepStatus::Process);
    }

    #[test]
    fn effective_status_index_less_than_current() {
        // When index < current, should be Finish
        assert_eq!(effective_status(0, 1, None), StepStatus::Finish);
        assert_eq!(effective_status(5, 10, None), StepStatus::Finish);
        assert_eq!(effective_status(98, 99, None), StepStatus::Finish);
    }

    #[test]
    fn effective_status_index_greater_than_current() {
        // When index > current, should be Wait
        assert_eq!(effective_status(1, 0, None), StepStatus::Wait);
        assert_eq!(effective_status(10, 5, None), StepStatus::Wait);
        assert_eq!(effective_status(99, 98, None), StepStatus::Wait);
    }

    #[test]
    fn step_status_equality() {
        assert_eq!(StepStatus::Wait, StepStatus::Wait);
        assert_eq!(StepStatus::Process, StepStatus::Process);
        assert_eq!(StepStatus::Finish, StepStatus::Finish);
        assert_eq!(StepStatus::Error, StepStatus::Error);
        assert_ne!(StepStatus::Wait, StepStatus::Process);
        assert_ne!(StepStatus::Finish, StepStatus::Error);
    }

    #[test]
    fn step_status_clone() {
        let original = StepStatus::Error;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(original.as_class(), cloned.as_class());
    }

    #[test]
    fn step_status_debug() {
        let wait = StepStatus::Wait;
        let process = StepStatus::Process;
        let finish = StepStatus::Finish;
        let error = StepStatus::Error;

        let wait_str = format!("{:?}", wait);
        let process_str = format!("{:?}", process);
        let finish_str = format!("{:?}", finish);
        let error_str = format!("{:?}", error);

        assert!(wait_str.contains("Wait"));
        assert!(process_str.contains("Process"));
        assert!(finish_str.contains("Finish"));
        assert!(error_str.contains("Error"));
    }

    #[test]
    fn steps_direction_clone() {
        let original = StepsDirection::Vertical;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(original.as_class(), cloned.as_class());
    }

    #[test]
    fn steps_direction_debug() {
        let horizontal = StepsDirection::Horizontal;
        let vertical = StepsDirection::Vertical;

        let h_str = format!("{:?}", horizontal);
        let v_str = format!("{:?}", vertical);

        assert!(h_str.contains("Horizontal"));
        assert!(v_str.contains("Vertical"));
    }

    #[test]
    fn step_item_equality() {
        let item1 = StepItem::new("key1", rsx!(div { "Title" }));
        let item2 = StepItem::new("key1", rsx!(div { "Title" }));
        let item3 = StepItem::new("key2", rsx!(div { "Title" }));

        // Note: Element comparison might not work as expected in tests
        // But we can test other fields
        assert_eq!(item1.key, item2.key);
        assert_ne!(item1.key, item3.key);
    }

    #[test]
    fn step_item_with_all_fields() {
        let mut item = StepItem::new("key1", rsx!(div { "Title" }));
        item.description = Some(rsx!(div { "Description" }));
        item.status = Some(StepStatus::Error);
        item.disabled = true;

        assert_eq!(item.key, "key1");
        assert!(item.description.is_some());
        assert_eq!(item.status, Some(StepStatus::Error));
        assert_eq!(item.disabled, true);
    }
}
