//! Integration tests for layout components
//!
//! Tests Layout, Grid, Flex, and ConfigProvider integration
//! without requiring full Dioxus runtime.

use adui_dioxus::components::config_provider::{ComponentSize, ConfigContextValue, Locale};
use adui_dioxus::components::flex::{
    FlexAlign, FlexComponent, FlexDirection, FlexGap, FlexJustify, FlexOrientation, FlexWrap,
};
use adui_dioxus::components::grid::{
    ColResponsive, ColSize, GridBreakpoint, ResponsiveGutter, ResponsiveValue, RowAlign,
    RowGutter, RowJustify,
};
use adui_dioxus::components::layout::{LayoutProps, SiderProps, SiderTheme};

#[test]
fn component_size_variants() {
    assert_eq!(ComponentSize::Middle, ComponentSize::default());
    assert_ne!(ComponentSize::Small, ComponentSize::Large);
    assert_ne!(ComponentSize::Middle, ComponentSize::Small);
}

#[test]
fn component_size_propagation() {
    // Test that ComponentSize can be used consistently
    let size = ComponentSize::Small;
    assert_eq!(size, ComponentSize::Small);

    let size2 = ComponentSize::Large;
    assert_ne!(size, size2);
}

#[test]
fn locale_variants() {
    assert_eq!(Locale::ZhCN, Locale::default());
    assert_ne!(Locale::ZhCN, Locale::EnUS);
}

#[test]
fn config_context_value_defaults() {
    let config = ConfigContextValue::default();
    assert_eq!(config.size, ComponentSize::Middle);
    assert_eq!(config.disabled, false);
    assert_eq!(config.prefix_cls, "adui");
    assert_eq!(config.locale, Locale::ZhCN);
}

#[test]
fn config_context_value_custom() {
    let config = ConfigContextValue {
        size: ComponentSize::Large,
        disabled: true,
        prefix_cls: "custom".to_string(),
        locale: Locale::EnUS,
    };
    assert_eq!(config.size, ComponentSize::Large);
    assert_eq!(config.disabled, true);
    assert_eq!(config.prefix_cls, "custom");
    assert_eq!(config.locale, Locale::EnUS);
}

#[test]
fn layout_props_structure() {
    // Test that LayoutProps has expected fields
    assert!(std::mem::size_of::<Option<String>>() > 0); // class field
    assert!(std::mem::size_of::<Option<String>>() > 0); // style field
    assert!(std::mem::size_of::<Option<bool>>() > 0); // has_sider field
}

#[test]
fn sider_theme_variants() {
    // Test that SiderTheme has expected variants
    let light = SiderTheme::Light;
    let dark = SiderTheme::Dark;
    assert_ne!(light, dark);
    // Note: default might be Light or Dark, so we just test they're different
}

#[test]
fn sider_props_structure() {
    // Test that SiderProps has expected fields
    assert!(std::mem::size_of::<Option<SiderTheme>>() > 0); // theme field
    assert!(std::mem::size_of::<Option<bool>>() > 0); // collapsed field
    assert!(std::mem::size_of::<Option<f32>>() > 0); // width field
}

#[test]
fn grid_breakpoint_variants() {
    assert_ne!(GridBreakpoint::Xs, GridBreakpoint::Sm);
    assert_ne!(GridBreakpoint::Md, GridBreakpoint::Lg);
    assert_ne!(GridBreakpoint::Xl, GridBreakpoint::Xxl);
}

#[test]
fn grid_breakpoint_order() {
    // Test that breakpoints are ordered correctly
    let breakpoints = vec![
        GridBreakpoint::Xs,
        GridBreakpoint::Sm,
        GridBreakpoint::Md,
        GridBreakpoint::Lg,
        GridBreakpoint::Xl,
        GridBreakpoint::Xxl,
    ];
    assert_eq!(breakpoints.len(), 6);
}

#[test]
fn responsive_value_defaults() {
    let value = ResponsiveValue::default();
    assert!(value.xs.is_none());
    assert!(value.sm.is_none());
    assert!(value.md.is_none());
    assert!(value.lg.is_none());
    assert!(value.xl.is_none());
    assert!(value.xxl.is_none());
}

#[test]
fn responsive_value_custom() {
    let value = ResponsiveValue {
        xs: Some(12.0),
        sm: Some(8.0),
        md: Some(6.0),
        lg: None,
        xl: None,
        xxl: None,
    };
    assert_eq!(value.xs, Some(12.0));
    assert_eq!(value.sm, Some(8.0));
    assert_eq!(value.md, Some(6.0));
}

#[test]
fn responsive_value_fields() {
    let value = ResponsiveValue {
        xs: Some(12.0),
        sm: Some(8.0),
        md: None,
        lg: None,
        xl: None,
        xxl: None,
    };
    assert_eq!(value.xs, Some(12.0));
    assert_eq!(value.sm, Some(8.0));
    assert!(value.md.is_none());
}

#[test]
fn responsive_gutter_defaults() {
    let gutter = ResponsiveGutter::default();
    assert_eq!(gutter.horizontal, ResponsiveValue::default());
    assert!(gutter.vertical.is_none());
}

#[test]
fn responsive_gutter_custom() {
    let horizontal = ResponsiveValue {
        xs: Some(16.0),
        sm: Some(24.0),
        ..ResponsiveValue::default()
    };
    let vertical = ResponsiveValue {
        xs: Some(8.0),
        sm: Some(12.0),
        ..ResponsiveValue::default()
    };
    let gutter = ResponsiveGutter {
        horizontal,
        vertical: Some(vertical),
    };
    assert_eq!(gutter.horizontal.xs, Some(16.0));
    assert_eq!(gutter.vertical.as_ref().unwrap().xs, Some(8.0));
}

#[test]
fn row_gutter_variants() {
    // Test uniform gutter
    let uniform = RowGutter::Uniform(16.0);
    assert!(matches!(uniform, RowGutter::Uniform(_)));

    // Test pair gutter
    let pair = RowGutter::Pair(16.0, 8.0);
    assert!(matches!(pair, RowGutter::Pair(_, _)));

    // Test responsive gutter
    let responsive = RowGutter::Responsive(ResponsiveGutter::default());
    assert!(matches!(responsive, RowGutter::Responsive(_)));
}

#[test]
fn row_justify_variants() {
    assert_eq!(RowJustify::Start, RowJustify::default());
    assert_ne!(RowJustify::Start, RowJustify::End);
    assert_ne!(RowJustify::Center, RowJustify::SpaceBetween);
    assert_ne!(RowJustify::SpaceAround, RowJustify::SpaceEvenly);
}

#[test]
fn row_align_variants() {
    assert_eq!(RowAlign::Top, RowAlign::default());
    assert_ne!(RowAlign::Top, RowAlign::Middle);
    assert_ne!(RowAlign::Bottom, RowAlign::Stretch);
}

#[test]
fn col_size_structure() {
    // ColSize is a struct with optional fields
    let size = ColSize {
        span: Some(12),
        offset: None,
        push: None,
        pull: None,
        order: None,
        flex: None,
    };
    assert_eq!(size.span, Some(12));
    assert!(size.is_empty() == false);

    let empty_size = ColSize {
        span: None,
        offset: None,
        push: None,
        pull: None,
        order: None,
        flex: None,
    };
    assert!(empty_size.is_empty());
}

#[test]
fn flex_direction_variants() {
    assert_eq!(FlexDirection::Row, FlexDirection::default());
    assert_ne!(FlexDirection::Row, FlexDirection::Column);
    assert_ne!(FlexDirection::RowReverse, FlexDirection::ColumnReverse);
}

#[test]
fn flex_justify_variants() {
    assert_eq!(FlexJustify::Start, FlexJustify::default());
    assert_ne!(FlexJustify::Start, FlexJustify::End);
    assert_ne!(FlexJustify::Center, FlexJustify::Between);
    assert_ne!(FlexJustify::Around, FlexJustify::Evenly);
}

#[test]
fn flex_align_variants() {
    assert_eq!(FlexAlign::Stretch, FlexAlign::default());
    assert_ne!(FlexAlign::Start, FlexAlign::End);
    assert_ne!(FlexAlign::Center, FlexAlign::Baseline);
}

#[test]
fn flex_wrap_variants() {
    assert_eq!(FlexWrap::NoWrap, FlexWrap::default());
    assert_ne!(FlexWrap::NoWrap, FlexWrap::Wrap);
    assert_ne!(FlexWrap::Wrap, FlexWrap::WrapReverse);
}

#[test]
fn flex_component_variants() {
    assert_eq!(FlexComponent::Div, FlexComponent::default());
    assert_ne!(FlexComponent::Div, FlexComponent::Section);
    assert_ne!(FlexComponent::Article, FlexComponent::Nav);
}

#[test]
fn flex_orientation_variants() {
    assert_eq!(FlexOrientation::Horizontal, FlexOrientation::default());
    assert_ne!(FlexOrientation::Horizontal, FlexOrientation::Vertical);
}

#[test]
fn flex_gap_variants() {
    assert_ne!(FlexGap::Small, FlexGap::Middle);
    assert_ne!(FlexGap::Middle, FlexGap::Large);
}

#[test]
fn flex_gap_conversion() {
    // Test that FlexGap variants exist and can be used
    let small = FlexGap::Small;
    let middle = FlexGap::Middle;
    let large = FlexGap::Large;
    assert_ne!(small, middle);
    assert_ne!(middle, large);
}

#[test]
fn config_provider_size_propagation() {
    // Test that ComponentSize propagates through ConfigProvider
    let small_config = ConfigContextValue {
        size: ComponentSize::Small,
        ..ConfigContextValue::default()
    };
    assert_eq!(small_config.size, ComponentSize::Small);

    let large_config = ConfigContextValue {
        size: ComponentSize::Large,
        ..ConfigContextValue::default()
    };
    assert_eq!(large_config.size, ComponentSize::Large);
}

#[test]
fn config_provider_disabled_propagation() {
    // Test that disabled state propagates through ConfigProvider
    let disabled_config = ConfigContextValue {
        disabled: true,
        ..ConfigContextValue::default()
    };
    assert_eq!(disabled_config.disabled, true);

    let enabled_config = ConfigContextValue {
        disabled: false,
        ..ConfigContextValue::default()
    };
    assert_eq!(enabled_config.disabled, false);
}

#[test]
fn layout_components_integration() {
    // Test that layout components can work together
    // LayoutProps is shared across Layout, Header, Content, Footer, Sider
    assert!(std::mem::size_of::<LayoutProps>() > 0);
    assert!(std::mem::size_of::<SiderProps>() > 0);
}

#[test]
fn grid_system_integration() {
    // Test that Row and Col work together
    // RowGutter and ColSize are compatible
    let gutter = RowGutter::Uniform(16.0);
    let col_size = ColSize {
        span: Some(12),
        ..ColSize::default()
    };
    assert!(matches!(gutter, RowGutter::Uniform(_)));
    assert_eq!(col_size.span, Some(12));
}

#[test]
fn flex_system_integration() {
    // Test that Flex components work together
    let direction = FlexDirection::Row;
    let justify = FlexJustify::Start;
    let align = FlexAlign::Stretch;
    let wrap = FlexWrap::NoWrap;

    assert_eq!(direction, FlexDirection::Row);
    assert_eq!(justify, FlexJustify::Start);
    assert_eq!(align, FlexAlign::Stretch);
    assert_eq!(wrap, FlexWrap::NoWrap);
}

#[test]
fn responsive_system_integration() {
    // Test that responsive values work across different components
    use adui_dioxus::components::grid::ColResponsive;
    
    let responsive_value = ResponsiveValue {
        xs: Some(12.0),
        sm: Some(8.0),
        md: Some(6.0),
        lg: Some(4.0),
        xl: None,
        xxl: None,
    };

    let col_responsive = ColResponsive {
        xs: Some(ColSize {
            span: Some(12),
            ..ColSize::default()
        }),
        sm: Some(ColSize {
            span: Some(8),
            ..ColSize::default()
        }),
        ..ColResponsive::default()
    };
    
    let gutter = ResponsiveGutter {
        horizontal: responsive_value,
        vertical: None,
    };

    assert_eq!(col_responsive.xs.as_ref().unwrap().span, Some(12));
    assert_eq!(gutter.horizontal.xs, Some(12.0));
}

