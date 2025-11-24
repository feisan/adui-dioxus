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
fn tokens_to_css_vars_contains_core_fields() {
    let tokens = ThemeTokens::light();
    let css = format!(
        "--adui-color-primary:{};--adui-color-bg-container:{};",
        tokens.color_primary, tokens.color_bg_container
    );
    assert!(css.contains("--adui-color-primary"));
    assert!(css.contains("--adui-color-bg-container"));
}
