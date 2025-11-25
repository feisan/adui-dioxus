//! Ant Design flavored components and theme utilities for Dioxus 0.7+.
//! Modules are organized by theme primitives and component implementations.
pub mod components;
pub mod theme;

pub use components::button::{Button, ButtonProps, ButtonShape, ButtonSize, ButtonType};
pub use components::divider::{Divider, DividerOrientation, DividerProps};
pub use components::flex::{Flex, FlexAlign, FlexDirection, FlexJustify, FlexProps, FlexWrap};
pub use components::float_button::{
    FloatButton, FloatButtonProps, FloatButtonShape, FloatButtonType,
};
pub use components::grid::{Col, ColProps, Row, RowAlign, RowJustify, RowProps};
pub use components::icon::{Icon, IconKind, IconProps};
pub use components::layout::{Content, Footer, Header, Layout, LayoutProps, Sider, SiderProps};
pub use components::masonry::{Masonry, MasonryProps};
pub use components::space::{Space, SpaceAlign, SpaceDirection, SpaceProps};
pub use components::splitter::{
    Splitter, SplitterOrientation, SplitterPane, SplitterPaneProps, SplitterProps,
};
pub use components::typography::{
    Paragraph, ParagraphProps, Text, TextProps, TextType, Title, TitleLevel, TitleProps,
};
pub use theme::{
    THEME_BASE_STYLE, Theme, ThemeHandle, ThemeMode, ThemeProvider, ThemeTokens, use_theme,
};
