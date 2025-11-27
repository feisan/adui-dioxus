use crate::theme::{Theme, ThemeProvider, ThemeTokens};
use dioxus::prelude::*;

/// Global size for components when they do not specify an explicit size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSize {
    Small,
    Middle,
    Large,
}

impl Default for ComponentSize {
    fn default() -> Self {
        ComponentSize::Middle
    }
}

/// Global configuration shared by components.
///
/// This is intentionally much smaller than Ant Design's ConfigProvider. We only
/// keep fields that are immediately useful for the current component set.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigContextValue {
    pub size: ComponentSize,
    pub disabled: bool,
    pub prefix_cls: String,
}

impl Default for ConfigContextValue {
    fn default() -> Self {
        Self {
            size: ComponentSize::Middle,
            disabled: false,
            prefix_cls: "adui".to_string(),
        }
    }
}

/// Props for `ConfigProvider`.
#[derive(Props, Clone, PartialEq)]
pub struct ConfigProviderProps {
    /// Global default size for components.
    #[props(optional)]
    pub size: Option<ComponentSize>,
    /// Global disabled flag. When `true`, interactive components should treat
    /// themselves as disabled unless explicitly overridden.
    #[props(optional)]
    pub disabled: Option<bool>,
    /// Global CSS class name prefix. Defaults to `"adui"`.
    #[props(optional)]
    pub prefix_cls: Option<String>,
    /// Optional initial theme. If omitted, the current ThemeProvider behaviour
    /// is preserved.
    #[props(optional)]
    pub theme: Option<Theme>,
    pub children: Element,
}

/// Provide ConfigContext for descendant components. ConfigProvider wraps
/// ThemeProvider so that size / disabled / prefix can live alongside tokens.
#[component]
pub fn ConfigProvider(props: ConfigProviderProps) -> Element {
    let parent = try_use_context::<ConfigContextValue>().unwrap_or_default();

    let value = ConfigContextValue {
        size: props.size.unwrap_or(parent.size),
        disabled: props.disabled.unwrap_or(parent.disabled),
        prefix_cls: props.prefix_cls.clone().unwrap_or(parent.prefix_cls),
    };

    // We still rely on ThemeProvider for concrete tokens & CSS variables, so
    // we forward the optional theme prop. If there is already a ThemeProvider
    // above, it will simply override this one.
    use_context_provider(|| value.clone());

    rsx! {
        ThemeProvider { theme: props.theme.clone(), {props.children} }
    }
}

/// Hook for components to read global config.
pub fn use_config() -> ConfigContextValue {
    try_use_context::<ConfigContextValue>().unwrap_or_default()
}

/// Lightweight helper to compute control dimensions from global config and
/// theme tokens. This does not live in the context to keep it testable and
/// stateless.
pub fn control_heights(config: &ConfigContextValue, tokens: &ThemeTokens) -> (f32, f32, f32) {
    let base = tokens.control_height;
    let small = tokens.control_height_small;
    let large = tokens.control_height_large;
    match config.size {
        ComponentSize::Small => (small, small, base),
        ComponentSize::Large => (large, large, base),
        ComponentSize::Middle => (base, small, large),
    }
}
