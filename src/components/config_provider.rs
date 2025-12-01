use crate::theme::{Theme, ThemeProvider, ThemeTokens};
use dioxus::prelude::*;

/// Global size for components when they do not specify an explicit size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ComponentSize {
    Small,
    #[default]
    Middle,
    Large,
}

/// Simple locale flag for components that need basic language switching.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Locale {
    /// Simplified Chinese.
    #[default]
    ZhCN,
    /// English (US).
    EnUS,
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
    pub locale: Locale,
}

impl Default for ConfigContextValue {
    fn default() -> Self {
        Self {
            size: ComponentSize::Middle,
            disabled: false,
            prefix_cls: "adui".to_string(),
            locale: Locale::ZhCN,
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
    /// Global locale flag to control basic UI language for components that
    /// integrate with date/time or other text-heavy features.
    #[props(optional)]
    pub locale: Option<Locale>,
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
        locale: props.locale.unwrap_or(parent.locale),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ThemeTokens;

    #[test]
    fn component_size_default() {
        assert_eq!(ComponentSize::default(), ComponentSize::Middle);
    }

    #[test]
    fn component_size_all_variants() {
        assert_eq!(ComponentSize::Small, ComponentSize::Small);
        assert_eq!(ComponentSize::Middle, ComponentSize::Middle);
        assert_eq!(ComponentSize::Large, ComponentSize::Large);
        assert_ne!(ComponentSize::Small, ComponentSize::Middle);
        assert_ne!(ComponentSize::Small, ComponentSize::Large);
        assert_ne!(ComponentSize::Middle, ComponentSize::Large);
    }

    #[test]
    fn component_size_clone() {
        let original = ComponentSize::Small;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn locale_default() {
        assert_eq!(Locale::default(), Locale::ZhCN);
    }

    #[test]
    fn locale_all_variants() {
        assert_eq!(Locale::ZhCN, Locale::ZhCN);
        assert_eq!(Locale::EnUS, Locale::EnUS);
        assert_ne!(Locale::ZhCN, Locale::EnUS);
    }

    #[test]
    fn locale_clone() {
        let original = Locale::ZhCN;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn config_context_value_default() {
        let config = ConfigContextValue::default();
        assert_eq!(config.size, ComponentSize::Middle);
        assert_eq!(config.disabled, false);
        assert_eq!(config.prefix_cls, "adui");
        assert_eq!(config.locale, Locale::ZhCN);
    }

    #[test]
    fn config_context_value_clone() {
        let original = ConfigContextValue {
            size: ComponentSize::Large,
            disabled: true,
            prefix_cls: "custom".to_string(),
            locale: Locale::EnUS,
        };
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn control_heights_small() {
        let tokens = ThemeTokens::light();
        let config = ConfigContextValue {
            size: ComponentSize::Small,
            disabled: false,
            prefix_cls: "adui".to_string(),
            locale: Locale::ZhCN,
        };
        let (small, middle, large) = control_heights(&config, &tokens);
        assert_eq!(small, tokens.control_height_small);
        assert_eq!(middle, tokens.control_height_small);
        assert_eq!(large, tokens.control_height);
    }

    #[test]
    fn control_heights_middle() {
        let tokens = ThemeTokens::light();
        let config = ConfigContextValue {
            size: ComponentSize::Middle,
            disabled: false,
            prefix_cls: "adui".to_string(),
            locale: Locale::ZhCN,
        };
        let (small, middle, large) = control_heights(&config, &tokens);
        assert_eq!(small, tokens.control_height);
        assert_eq!(middle, tokens.control_height_small);
        assert_eq!(large, tokens.control_height_large);
    }

    #[test]
    fn control_heights_large() {
        let tokens = ThemeTokens::light();
        let config = ConfigContextValue {
            size: ComponentSize::Large,
            disabled: false,
            prefix_cls: "adui".to_string(),
            locale: Locale::ZhCN,
        };
        let (small, middle, large) = control_heights(&config, &tokens);
        assert_eq!(small, tokens.control_height_large);
        assert_eq!(middle, tokens.control_height_large);
        assert_eq!(large, tokens.control_height);
    }
}
