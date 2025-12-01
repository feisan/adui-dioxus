# Component Library Documentation

## Overview

This is an experimental component library that ports Ant Design 6.0.0 to Dioxus using Vibe Coding. The library provides a comprehensive set of UI components with Ant Design's design language and patterns, adapted for Rust and the Dioxus framework.

## About This Project

This component library is an experimental port of Ant Design 6.0.0 to Dioxus, created using Vibe Coding. It aims to bring Ant Design's rich component ecosystem and design principles to the Rust ecosystem, enabling developers to build modern web applications with type-safe, performant Rust code.

**Note**: This is an experimental project. Some features may differ from the original Ant Design implementation, and the API may evolve as the project matures.

## Component Categories

### Layout

Components for structuring page layouts and organizing content.

- [Layout](layout.md) - Page layout container with Header, Footer, Sider, and Content
- [Grid](grid.md) - 24-column grid system for responsive layouts
- [Flex](flex.md) - Flexible box layout component
- [Space](space.md) - Spacing component for arranging elements
- [Divider](divider.md) - Divider line for separating content
- [Splitter](splitter.md) - Resizable split panes
- [Masonry](masonry.md) - Masonry layout for cards and items

### Navigation

Components for navigation and wayfinding.

- [Menu](menu.md) - Navigation menu with horizontal and vertical modes
- [Breadcrumb](breadcrumb.md) - Breadcrumb navigation
- [Tabs](tabs.md) - Tab navigation component
- [Anchor](anchor.md) - Anchor navigation for long pages
- [Steps](steps.md) - Step indicator for processes
- [Pagination](pagination.md) - Pagination component

### Data Entry

Components for data input and form controls.

- [Input](input.md) - Text input component
- [TextArea](textarea.md) - Multi-line text input
- [Password](input.md#password) - Password input with visibility toggle
- [Search](input.md#search) - Input with search button
- [OTP](input.md#otp) - One-time password input
- [InputNumber](input_number.md) - Number input with stepper
- [Select](select.md) - Dropdown selector
- [TreeSelect](tree_select.md) - Tree-structured selector
- [Cascader](cascader.md) - Cascading selector
- [AutoComplete](auto_complete.md) - Autocomplete input
- [Mentions](mentions.md) - Mentions input for @mentions
- [DatePicker](date_picker.md) - Date picker
- [TimePicker](time_picker.md) - Time picker
- [Calendar](calendar.md) - Calendar component
- [ColorPicker](color_picker.md) - Color picker
- [Upload](upload.md) - File upload component
- [Rate](rate.md) - Rating component
- [Switch](switch.md) - Switch toggle
- [Checkbox](checkbox.md) - Checkbox input
- [Radio](radio.md) - Radio button
- [Slider](slider.md) - Slider input

### Data Display

Components for displaying data and content.

- [Table](table.md) - Data table with sorting, filtering, and pagination
- [List](list.md) - List component for displaying data
- [Descriptions](descriptions.md) - Description list for key-value pairs
- [Card](card.md) - Card container component
- [Collapse](collapse.md) - Collapsible content panels
- [Timeline](timeline.md) - Timeline component
- [Tree](tree.md) - Tree component
- [Tag](tag.md) - Tag component for labels
- [Badge](badge.md) - Badge for notifications and counts
- [Avatar](avatar.md) - Avatar component
- [Empty](empty.md) - Empty state component
- [Statistic](statistic.md) - Statistic display component
- [Progress](progress.md) - Progress indicator
- [QRCode](qrcode.md) - QR code generator
- [Image](image.md) - Image component with preview

### Feedback

Components for user feedback and notifications.

- [Alert](alert.md) - Alert message component
- [Message](message.md) - Global message notification
- [Notification](notification.md) - Notification component
- [Modal](modal.md) - Modal dialog
- [Drawer](drawer.md) - Drawer component
- [Popconfirm](popconfirm.md) - Popconfirm dialog
- [Popover](popover.md) - Popover component
- [Tooltip](tooltip.md) - Tooltip component
- [Spin](spin.md) - Loading spinner
- [Skeleton](skeleton.md) - Skeleton screen loading
- [Result](result.md) - Result page component
- [Progress](progress.md) - Progress indicator

### Other

Additional utility and specialized components.

- [Button](button.md) - Button component
- [FloatButton](float_button.md) - Floating action button
- [Icon](icon.md) - Icon component
- [Typography](typography.md) - Typography components (Title, Text, Paragraph)
- [Form](form.md) - Form component with validation
- [ConfigProvider](config_provider.md) - Global configuration provider
- [App](app.md) - App-level component and hooks
- [Affix](affix.md) - Affix component for sticky positioning
- [Watermark](watermark.md) - Watermark component
- [Tour](tour.md) - Tour component for onboarding
- [Transfer](transfer.md) - Transfer component for moving items
- [Carousel](carousel.md) - Carousel component

## Getting Started

To use this component library in your Dioxus project, add it to your `Cargo.toml`:

```toml
[dependencies]
adui-dioxus = "0.1.0"
```

Then import and use components:

```rust
use adui_dioxus::{Button, ButtonType, ThemeProvider};

fn app() -> Element {
    rsx! {
        ThemeProvider {
            Button {
                r#type: ButtonType::Primary,
                "Click Me"
            }
        }
    }
}
```

## Documentation Structure

Each component has its own documentation page with:

- **Overview**: Component purpose and functionality
- **API Reference**: Complete props, events, and methods documentation
- **Usage Examples**: Code examples for common use cases
- **Use Cases**: Typical scenarios where the component is used
- **Differences from Ant Design 6.0.0**: Key differences and limitations

## Language Support

Documentation is available in both English (default) and Chinese. Chinese documentation files are suffixed with `_CN.md`.

## Contributing

This is an experimental project. Contributions and feedback are welcome. Please refer to the main project repository for contribution guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

