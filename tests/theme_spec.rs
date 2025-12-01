use adui_dioxus::{Theme, ThemeMode, theme::ThemeTokens};
use adui_dioxus::components::config_provider::{ComponentSize, ConfigContextValue};

#[test]
fn default_theme_modes_resolve() {
    let light = Theme::light();
    assert_eq!(light.mode, ThemeMode::Light);
    assert_eq!(light.tokens.color_primary, "#1677ff");

    let dark = Theme::dark();
    assert_eq!(dark.mode, ThemeMode::Dark);
    assert_eq!(dark.tokens.color_bg_base, "#141414");
}

#[test]
fn custom_mode_defaults_to_light_tokens() {
    let custom = Theme::for_mode(ThemeMode::Custom);
    assert_eq!(custom.mode, ThemeMode::Custom);
    assert_eq!(
        custom.tokens.color_primary,
        ThemeTokens::light().color_primary
    );
}

#[test]
fn theme_mode_variants() {
    assert_ne!(ThemeMode::Light, ThemeMode::Dark);
    assert_ne!(ThemeMode::Dark, ThemeMode::Custom);
}

#[test]
fn theme_tokens_light_defaults() {
    let tokens = ThemeTokens::light();
    assert_eq!(tokens.color_primary, "#1677ff");
    assert!(!tokens.color_bg_base.is_empty());
    assert!(!tokens.color_text.is_empty());
}

#[test]
fn theme_tokens_dark_defaults() {
    let tokens = ThemeTokens::dark();
    assert_eq!(tokens.color_bg_base, "#141414");
    assert!(!tokens.color_text.is_empty());
}

#[test]
fn theme_customization() {
    let mut theme = Theme::light();
    theme.tokens.color_primary = "#ff0000".to_string();
    assert_eq!(theme.tokens.color_primary, "#ff0000");
}

#[test]
fn theme_and_config_integration() {
    // Test that Theme and ConfigProvider can work together
    let theme = Theme::light();
    let config = ConfigContextValue {
        size: ComponentSize::Large,
        ..ConfigContextValue::default()
    };
    assert_eq!(theme.mode, ThemeMode::Light);
    assert_eq!(config.size, ComponentSize::Large);
}

#[test]
fn theme_tokens_clone() {
    let tokens1 = ThemeTokens::light();
    let tokens2 = tokens1.clone();
    assert_eq!(tokens1.color_primary, tokens2.color_primary);
}

#[test]
fn theme_clone() {
    let theme1 = Theme::light();
    let theme2 = theme1.clone();
    assert_eq!(theme1.mode, theme2.mode);
    assert_eq!(theme1.tokens.color_primary, theme2.tokens.color_primary);
}
