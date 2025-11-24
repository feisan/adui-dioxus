use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Shared base styles for the theme scope and the button component.
pub const THEME_BASE_STYLE: &str = r#"
.adui-theme-scope {
    color: var(--adui-color-text);
    background-color: var(--adui-color-bg-base);
    font-family: "Segoe UI", "SF Pro Text", system-ui, -apple-system, sans-serif;
}

.adui-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    border: 1px solid var(--adui-btn-border);
    border-style: var(--adui-btn-border-style, solid);
    background: var(--adui-btn-bg);
    color: var(--adui-btn-color);
    padding: var(--adui-btn-padding-block) var(--adui-btn-padding-inline);
    height: var(--adui-btn-height);
    min-width: 0;
    border-radius: var(--adui-btn-radius);
    font-size: var(--adui-btn-font-size);
    line-height: 1.2;
    cursor: pointer;
    transition: all 0.18s ease;
    box-shadow: var(--adui-btn-shadow, none);
    text-decoration: none;
    user-select: none;
}

.adui-btn:hover:not(.adui-btn-disabled) {
    background: var(--adui-btn-bg-hover);
    color: var(--adui-btn-color-hover);
    border-color: var(--adui-btn-border-hover);
}

.adui-btn:active:not(.adui-btn-disabled) {
    background: var(--adui-btn-bg-active);
    color: var(--adui-btn-color-active);
    border-color: var(--adui-btn-border-active);
    box-shadow: none;
}

.adui-btn:focus-visible {
    outline: none;
    box-shadow: var(--adui-btn-focus-shadow, 0 0 0 2px rgba(22, 119, 255, 0.3));
}

.adui-btn-disabled {
    cursor: not-allowed;
    opacity: 0.65;
}

.adui-btn-block {
    width: 100%;
}

.adui-btn-active {
    border-color: var(--adui-color-primary);
    color: var(--adui-color-primary);
}

.adui-btn-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

.adui-btn-spinner {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid currentColor;
    border-right-color: transparent;
    animation: adui-spin 0.9s linear infinite;
}

@keyframes adui-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

.adui-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    vertical-align: middle;
    transition: transform 0.16s ease;
}

.adui-icon-spin {
    animation: adui-spin 1s linear infinite;
}

.adui-text {
    font-size: inherit;
    line-height: 1.6;
    word-break: break-word;
}

.adui-text-code {
    padding: 1px 4px;
    background: rgba(0, 0, 0, 0.04);
    border: 1px solid rgba(0, 0, 0, 0.06);
    border-radius: 6px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
    font-size: 0.95em;
}

.adui-text-mark {
    background: rgba(255, 229, 143, 0.8);
    padding: 1px 2px;
}

.adui-text-strong {
    font-weight: 600;
}

.adui-text-italic {
    font-style: italic;
}

.adui-text-nowrap {
    white-space: nowrap;
}

.adui-text-ellipsis {
    display: inline-block;
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    vertical-align: bottom;
}

.adui-paragraph {
    margin: 0 0 0.6em 0;
}

.adui-title {
    margin: 0 0 0.4em 0;
    font-weight: 600;
    color: var(--adui-color-text);
    line-height: 1.25;
}

.adui-title-1 { font-size: 32px; }
.adui-title-2 { font-size: 28px; }
.adui-title-3 { font-size: 24px; }
.adui-title-4 { font-size: 20px; }
.adui-title-5 { font-size: 16px; }

.adui-float-btn {
    position: fixed;
    z-index: 99;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    min-width: 56px;
    height: 56px;
    padding: 0 16px;
    border: 1px solid var(--adui-fb-border);
    background: var(--adui-fb-bg);
    color: var(--adui-fb-color);
    border-radius: var(--adui-fb-radius);
    box-shadow: var(--adui-fb-shadow);
    cursor: pointer;
    transition: all 0.18s ease;
    text-decoration: none;
}

.adui-float-btn:hover {
    background: var(--adui-fb-bg-hover);
    color: var(--adui-fb-color-hover);
    border-color: var(--adui-fb-border-hover);
    transform: translateY(-2px);
}

.adui-float-btn:active {
    background: var(--adui-fb-bg-active);
    color: var(--adui-fb-color-active);
    border-color: var(--adui-fb-border-active);
    transform: translateY(0);
}

.adui-float-btn-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
}

.adui-float-btn-desc {
    font-size: 12px;
    line-height: 1.2;
    max-width: 120px;
    white-space: nowrap;
}
"#;

/// Theme mode tracks the seed variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    Custom,
}

/// Core design tokens needed by early components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThemeTokens {
    pub color_primary: String,
    pub color_primary_hover: String,
    pub color_primary_active: String,
    pub color_error: String,
    pub color_error_hover: String,
    pub color_error_active: String,
    pub color_link: String,
    pub color_link_hover: String,
    pub color_link_active: String,
    pub color_text: String,
    pub color_text_muted: String,
    pub color_text_disabled: String,
    pub color_bg_base: String,
    pub color_bg_container: String,
    pub color_border: String,
    pub color_border_hover: String,
    pub border_radius: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub control_height_large: f32,
    pub padding_inline: f32,
    pub padding_block: f32,
    pub font_size: f32,
    pub shadow: String,
}

impl ThemeTokens {
    /// Ant Design 6.x inspired light tokens.
    pub fn light() -> Self {
        Self {
            color_primary: "#1677ff".into(),
            color_primary_hover: "#4096ff".into(),
            color_primary_active: "#0958d9".into(),
            color_error: "#ff4d4f".into(),
            color_error_hover: "#ff7875".into(),
            color_error_active: "#d9363e".into(),
            color_link: "#1677ff".into(),
            color_link_hover: "#4096ff".into(),
            color_link_active: "#0958d9".into(),
            color_text: "#1f1f1f".into(),
            color_text_muted: "#595959".into(),
            color_text_disabled: "rgba(0,0,0,0.25)".into(),
            color_bg_base: "#f5f5f5".into(),
            color_bg_container: "#ffffff".into(),
            color_border: "#d9d9d9".into(),
            color_border_hover: "#91caff".into(),
            border_radius: 6.0,
            control_height: 32.0,
            control_height_small: 24.0,
            control_height_large: 40.0,
            padding_inline: 15.0,
            padding_block: 6.0,
            font_size: 14.0,
            shadow: "0 2px 0 rgba(5, 145, 255, 0.1)".into(),
        }
    }

    /// Ant Design 6.x inspired dark tokens.
    pub fn dark() -> Self {
        Self {
            color_primary: "#177ddc".into(),
            color_primary_hover: "#3c9ae8".into(),
            color_primary_active: "#1668b2".into(),
            color_error: "#f16364".into(),
            color_error_hover: "#ff7875".into(),
            color_error_active: "#d84a45".into(),
            color_link: "#3c9ae8".into(),
            color_link_hover: "#65b7f3".into(),
            color_link_active: "#2b74b1".into(),
            color_text: "#f0f0f0".into(),
            color_text_muted: "#bfbfbf".into(),
            color_text_disabled: "rgba(255,255,255,0.35)".into(),
            color_bg_base: "#141414".into(),
            color_bg_container: "#1f1f1f".into(),
            color_border: "#2a2a2a".into(),
            color_border_hover: "#3a3a3a".into(),
            border_radius: 6.0,
            control_height: 32.0,
            control_height_small: 24.0,
            control_height_large: 40.0,
            padding_inline: 15.0,
            padding_block: 6.0,
            font_size: 14.0,
            shadow: "0 2px 0 rgba(23, 125, 220, 0.25)".into(),
        }
    }
}

/// Theme object bundling mode and resolved tokens.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub mode: ThemeMode,
    pub tokens: ThemeTokens,
}

impl Theme {
    /// Build a theme from a given mode using defaults.
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Custom => Self {
                mode,
                tokens: ThemeTokens::light(),
            },
        }
    }

    /// Light preset.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            tokens: ThemeTokens::light(),
        }
    }

    /// Dark preset.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            tokens: ThemeTokens::dark(),
        }
    }
}

/// Handle for reading or mutating the active theme from components.
#[derive(Clone, Copy)]
pub struct ThemeHandle {
    signal: Signal<Theme>,
}

impl ThemeHandle {
    pub fn theme(&self) -> Theme {
        self.signal.read().clone()
    }

    pub fn tokens(&self) -> ThemeTokens {
        self.signal.read().tokens.clone()
    }

    pub fn set_theme(&self, theme: Theme) {
        let mut signal = self.signal;
        signal.set(theme);
    }

    pub fn set_mode(&self, mode: ThemeMode) {
        let mut signal = self.signal;
        signal.set(Theme::for_mode(mode));
    }

    pub fn update_tokens(&self, mode: Option<ThemeMode>, update: impl FnOnce(&mut ThemeTokens)) {
        let mut signal = self.signal;
        signal.with_mut(|theme| {
            if let Some(next_mode) = mode {
                theme.mode = next_mode;
                theme.tokens = match next_mode {
                    ThemeMode::Light => ThemeTokens::light(),
                    ThemeMode::Dark => ThemeTokens::dark(),
                    ThemeMode::Custom => theme.tokens.clone(),
                };
            }
            update(&mut theme.tokens);
        });
    }
}

/// Hook to access the current theme signal.
pub fn use_theme() -> ThemeHandle {
    let signal: Signal<Theme> = use_context();
    ThemeHandle { signal }
}

/// Props for [`ThemeProvider`].
#[derive(Props, Clone, PartialEq)]
pub struct ThemeProviderProps {
    #[props(optional)]
    pub theme: Option<Theme>,
    pub children: Element,
}

/// Provide theme context and CSS variables for descendant components.
#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let initial = props.theme.clone().unwrap_or_else(Theme::light);
    let signal = use_context_provider(|| Signal::new(initial));
    let handle = ThemeHandle { signal };
    let tokens = handle.tokens();
    let css_vars = tokens_to_css_vars(&tokens);

    rsx! {
        style { {THEME_BASE_STYLE} }
        div {
            class: "adui-theme-scope",
            style: css_vars,
            {props.children}
        }
    }
}

fn tokens_to_css_vars(tokens: &ThemeTokens) -> String {
    format!(
        "--adui-color-primary:{};\
        --adui-color-primary-hover:{};\
        --adui-color-primary-active:{};\
        --adui-color-error:{};\
        --adui-color-error-hover:{};\
        --adui-color-error-active:{};\
        --adui-color-link:{};\
        --adui-color-link-hover:{};\
        --adui-color-link-active:{};\
        --adui-color-text:{};\
        --adui-color-text-muted:{};\
        --adui-color-text-disabled:{};\
        --adui-color-bg-base:{};\
        --adui-color-bg-container:{};\
        --adui-color-border:{};\
        --adui-color-border-hover:{};\
        --adui-radius:{}px;\
        --adui-control-height:{}px;\
        --adui-control-height-sm:{}px;\
        --adui-control-height-lg:{}px;\
        --adui-padding-inline:{}px;\
        --adui-padding-block:{}px;\
        --adui-font-size:{}px;\
        --adui-shadow:{};",
        tokens.color_primary,
        tokens.color_primary_hover,
        tokens.color_primary_active,
        tokens.color_error,
        tokens.color_error_hover,
        tokens.color_error_active,
        tokens.color_link,
        tokens.color_link_hover,
        tokens.color_link_active,
        tokens.color_text,
        tokens.color_text_muted,
        tokens.color_text_disabled,
        tokens.color_bg_base,
        tokens.color_bg_container,
        tokens.color_border,
        tokens.color_border_hover,
        tokens.border_radius,
        tokens.control_height,
        tokens.control_height_small,
        tokens.control_height_large,
        tokens.padding_inline,
        tokens.padding_block,
        tokens.font_size,
        tokens.shadow,
    )
}
