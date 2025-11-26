use adui_dioxus::{Theme, ThemeMode, theme::ThemeTokens};

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
