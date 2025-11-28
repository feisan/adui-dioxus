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
}
