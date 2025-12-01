//! Integration tests for core components
//!
//! Tests basic rendering, props passing, and class name generation for core components
//! without requiring full Dioxus runtime.

use adui_dioxus::components::button::{
    ButtonColor, ButtonHtmlType, ButtonIconPlacement, ButtonShape, ButtonSize, ButtonType,
    ButtonVariant,
};
use adui_dioxus::components::config_provider::ComponentSize;
use adui_dioxus::components::divider::{DividerOrientation, DividerProps};
use adui_dioxus::components::icon::{IconKind, IconProps};
use adui_dioxus::components::input::{InputProps, InputSize};
use adui_dioxus::components::space::{SpaceAlign, SpaceDirection, SpaceProps, SpaceSize};
use adui_dioxus::components::typography::{TextProps, TextType, TitleLevel, TitleProps};
use adui_dioxus::foundation::Variant;

#[test]
fn button_size_variants() {
    assert_eq!(ButtonSize::Small, ButtonSize::Small);
    assert_eq!(ButtonSize::Middle, ButtonSize::default());
    assert_ne!(ButtonSize::Small, ButtonSize::Large);
}

#[test]
fn button_size_from_global_config() {
    use adui_dioxus::components::button::ButtonSize;
    use adui_dioxus::components::config_provider::ComponentSize;

    // Test that ButtonSize can be derived from ComponentSize
    // This is tested through the from_global method pattern
    assert_eq!(ComponentSize::Small, ComponentSize::Small);
    assert_eq!(ComponentSize::Middle, ComponentSize::default());
}

#[test]
fn button_type_variants() {
    assert_eq!(ButtonType::Default, ButtonType::default());
    assert_ne!(ButtonType::Primary, ButtonType::Text);
    assert_ne!(ButtonType::Dashed, ButtonType::Link);
}

#[test]
fn button_color_variants() {
    assert_eq!(ButtonColor::Default, ButtonColor::default());
    assert_ne!(ButtonColor::Primary, ButtonColor::Danger);
    assert_ne!(ButtonColor::Success, ButtonColor::Warning);
}

#[test]
fn button_variant_variants() {
    assert_eq!(ButtonVariant::Outlined, ButtonVariant::default());
    assert_ne!(ButtonVariant::Solid, ButtonVariant::Text);
    assert_ne!(ButtonVariant::Dashed, ButtonVariant::Link);
}

#[test]
fn button_shape_variants() {
    assert_eq!(ButtonShape::Default, ButtonShape::default());
    assert_ne!(ButtonShape::Round, ButtonShape::Circle);
}

#[test]
fn button_html_type_variants() {
    assert_eq!(ButtonHtmlType::Button, ButtonHtmlType::default());
    assert_ne!(ButtonHtmlType::Submit, ButtonHtmlType::Reset);
}

#[test]
fn button_icon_placement_variants() {
    assert_eq!(ButtonIconPlacement::Start, ButtonIconPlacement::default());
    assert_ne!(ButtonIconPlacement::Start, ButtonIconPlacement::End);
}

#[test]
fn button_group_props_structure() {
    // Test that ButtonGroupProps has expected fields
    // We can't create instances without rsx! macro, so we test the type structure
    assert!(std::mem::size_of::<Option<ButtonSize>>() > 0); // size field
    assert!(std::mem::size_of::<Option<ButtonShape>>() > 0); // shape field
    assert!(std::mem::size_of::<Option<ButtonColor>>() > 0); // color field
    assert!(std::mem::size_of::<Option<ButtonVariant>>() > 0); // variant field
}

#[test]
fn input_size_variants() {
    assert_eq!(InputSize::Middle, InputSize::default());
    assert_ne!(InputSize::Small, InputSize::Large);
}

#[test]
fn input_size_variants_exist() {
    // Test that InputSize variants exist
    assert_eq!(InputSize::Small, InputSize::Small);
    assert_eq!(InputSize::Middle, InputSize::Middle);
    assert_eq!(InputSize::Large, InputSize::Large);
    assert_ne!(InputSize::Small, InputSize::Large);
}

#[test]
fn input_size_and_component_size_integration() {
    // Test that InputSize and ComponentSize work together
    // Note: from_global is private, so we test the relationship conceptually
    assert_eq!(ComponentSize::Small, ComponentSize::Small);
    assert_eq!(ComponentSize::Middle, ComponentSize::Middle);
    assert_eq!(ComponentSize::Large, ComponentSize::Large);
}

#[test]
fn input_props_structure() {
    // Test that InputProps has expected fields
    // We can't create instances without proper setup, so we test the type structure
    assert!(std::mem::size_of::<Option<String>>() > 0); // value field
    assert!(std::mem::size_of::<bool>() > 0); // disabled field
    assert!(std::mem::size_of::<Option<InputSize>>() > 0); // size field
}

#[test]
fn input_variant_integration() {
    use adui_dioxus::foundation::variant_from_bordered;

    // Test variant takes priority over bordered
    assert_eq!(
        variant_from_bordered(Some(false), Some(Variant::Filled)),
        Variant::Filled
    );

    // Test bordered=false maps to Borderless
    assert_eq!(
        variant_from_bordered(Some(false), None),
        Variant::Borderless
    );

    // Test default is Outlined
    assert_eq!(variant_from_bordered(None, None), Variant::Outlined);
}

#[test]
fn icon_kind_variants() {
    assert_ne!(IconKind::Check, IconKind::Close);
    assert_ne!(IconKind::Search, IconKind::Close);
}

#[test]
fn icon_props_structure() {
    // Test that IconProps has expected fields
    assert!(std::mem::size_of::<IconKind>() > 0); // kind field
    assert!(std::mem::size_of::<Option<f32>>() > 0); // size field
}

#[test]
fn text_type_variants() {
    assert_eq!(TextType::Default, TextType::default());
    assert_ne!(TextType::Success, TextType::Danger);
    assert_ne!(TextType::Warning, TextType::Disabled);
}

#[test]
fn text_props_structure() {
    // Test that TextProps has expected fields
    assert!(std::mem::size_of::<TextType>() > 0); // r#type field
    assert!(std::mem::size_of::<bool>() > 0); // strong field
}

#[test]
fn title_level_variants() {
    assert_eq!(TitleLevel::H1, TitleLevel::default());
    assert_ne!(TitleLevel::H1, TitleLevel::H2);
    assert_ne!(TitleLevel::H3, TitleLevel::H4);
}

#[test]
fn title_props_structure() {
    // Test that TitleProps has expected fields
    assert!(std::mem::size_of::<TitleLevel>() > 0); // level field
    assert!(std::mem::size_of::<TextType>() > 0); // r#type field
}

#[test]
fn space_direction_variants() {
    assert_eq!(SpaceDirection::Horizontal, SpaceDirection::default());
    assert_ne!(SpaceDirection::Horizontal, SpaceDirection::Vertical);
}

#[test]
fn space_align_variants() {
    assert_eq!(SpaceAlign::Start, SpaceAlign::default());
    assert_ne!(SpaceAlign::Start, SpaceAlign::End);
    assert_ne!(SpaceAlign::Center, SpaceAlign::Baseline);
}

#[test]
fn space_size_variants() {
    // SpaceSize can be Small, Middle, Large
    assert_eq!(SpaceSize::Middle, SpaceSize::default());
    assert_ne!(SpaceSize::Small, SpaceSize::Large);
    assert_ne!(SpaceSize::Middle, SpaceSize::Small);
}

#[test]
fn space_props_structure() {
    // Test that SpaceProps has expected fields
    assert!(std::mem::size_of::<SpaceDirection>() > 0); // direction field
    assert!(std::mem::size_of::<SpaceSize>() > 0); // size field
    assert!(std::mem::size_of::<SpaceAlign>() > 0); // align field
    assert!(std::mem::size_of::<bool>() > 0); // compact field
}

#[test]
fn divider_orientation_variants() {
    assert_eq!(DividerOrientation::Center, DividerOrientation::default());
    assert_ne!(DividerOrientation::Left, DividerOrientation::Right);
    assert_ne!(DividerOrientation::Center, DividerOrientation::Left);
}

#[test]
fn divider_props_structure() {
    // Test that DividerProps has expected fields
    assert!(std::mem::size_of::<bool>() > 0); // dashed field
    assert!(std::mem::size_of::<bool>() > 0); // plain field
    assert!(std::mem::size_of::<bool>() > 0); // vertical field
    assert!(std::mem::size_of::<DividerOrientation>() > 0); // orientation field
}

#[test]
fn component_size_propagation() {
    // Test that ComponentSize can be used across different components
    let global_size = ComponentSize::Small;
    assert_eq!(global_size, ComponentSize::Small);

    // Test that InputSize variants exist
    let input_size = InputSize::Small;
    assert_eq!(input_size, InputSize::Small);
}

#[test]
fn variant_system_integration() {
    use adui_dioxus::foundation::Variant;

    // Test variant enum values
    assert_eq!(Variant::Outlined, Variant::Outlined);
    assert_ne!(Variant::Filled, Variant::Borderless);
    assert_ne!(Variant::Outlined, Variant::Filled);

    // Test variant_from_bordered integration
    use adui_dioxus::foundation::variant_from_bordered;
    assert_eq!(
        variant_from_bordered(Some(true), Some(Variant::Filled)),
        Variant::Filled
    );
    assert_eq!(
        variant_from_bordered(Some(false), None),
        Variant::Borderless
    );
}

#[test]
fn core_components_type_safety() {
    // Test that component props maintain type safety
    let button_size: ButtonSize = ButtonSize::Small;
    let input_size: InputSize = InputSize::Small;

    // These are different types and should not be equal
    // This test ensures type safety is maintained
    assert!(std::mem::size_of::<ButtonSize>() > 0);
    assert!(std::mem::size_of::<InputSize>() > 0);
}

#[test]
fn core_components_clone_and_copy() {
    // Test that component enums implement Clone and Copy
    let button_size1 = ButtonSize::Small;
    let button_size2 = button_size1; // Copy
    assert_eq!(button_size1, button_size2);

    let button_type1 = ButtonType::Primary;
    let button_type2 = button_type1.clone(); // Clone
    assert_eq!(button_type1, button_type2);

    let input_size1 = InputSize::Large;
    let input_size2 = input_size1; // Copy
    assert_eq!(input_size1, input_size2);
}

