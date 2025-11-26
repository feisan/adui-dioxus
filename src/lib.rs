//! Ant Design flavored components and theme utilities for Dioxus 0.7+.
//! Modules are organized by theme primitives and component implementations.
pub mod components;
pub mod theme;

pub use components::button::{
    Button, ButtonColor, ButtonGroup, ButtonGroupProps, ButtonIconPlacement, ButtonProps,
    ButtonShape, ButtonSize, ButtonType, ButtonVariant,
};
pub use components::divider::{Divider, DividerOrientation, DividerProps};
pub use components::flex::{
    Flex, FlexAlign, FlexComponent, FlexConfigProvider, FlexDirection, FlexGap, FlexJustify,
    FlexOrientation, FlexProps, FlexSharedConfig, FlexWrap,
};
pub use components::float_button::{
    BackTop, BadgeConfig, FloatButton, FloatButtonGroup, FloatButtonProps, FloatButtonShape,
    FloatButtonType,
};
pub use components::grid::{
    Col, ColProps, ColResponsive, ColSize, ResponsiveGutter, ResponsiveValue, Row, RowAlign,
    RowGutter, RowJustify, RowProps,
};
pub use components::icon::{Icon, IconKind, IconProps};
pub use components::layout::{
    Content, Footer, Header, Layout, LayoutProps, Sider, SiderProps, SiderTheme,
};
pub use components::masonry::{Masonry, MasonryProps, MasonryResponsive};
pub use components::space::{Space, SpaceAlign, SpaceDirection, SpaceProps, SpaceSize};
pub use components::splitter::{
    Splitter, SplitterOrientation, SplitterPane, SplitterPaneProps, SplitterProps,
};
pub use components::typography::{
    Paragraph, ParagraphProps, Text, TextProps, TextType, Title, TitleLevel, TitleProps,
    TypographyCopyable, TypographyEditable, TypographyEllipsis,
};
pub use theme::{
    THEME_BASE_STYLE, Theme, ThemeHandle, ThemeMode, ThemeProvider, ThemeTokens, use_theme,
};
