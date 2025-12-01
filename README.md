# adui-dioxus

## Introduction

adui-dioxus is a UI component library for Dioxus that provides rich components and styles, helping developers quickly build cross-platform web and mobile applications.

This is an **experimental project** that ports Ant Design 6.0.0 to Dioxus using **Vibe Coding**. It extracts components from the Ant Design UI library (https://github.com/ant-design/ant-design) and adapts them for the Rust/Dioxus ecosystem. The library inherits Ant Design's design philosophy and component styles while leveraging Dioxus's performance and flexibility to provide developers with an efficient and convenient development experience.

## Project Status

This is an experimental port of Ant Design 6.0.0 to Dioxus. The library is built on **Dioxus 0.7+** and includes a comprehensive set of components:

### Core Features
- **Theme System**: Ant Design 6.x style tokens and theme context (light/dark presets, CSS variable export)
- **Config Provider**: Global configuration and theme management

### Layout Components
- **Layout**: Page layout container with Header, Footer, Sider, and Content
- **Grid**: 24-column grid system for responsive layouts
- **Flex**: Flexible box layout component with gap presets and wrap support
- **Space**: Spacing component for arranging elements
- **Divider**: Divider line for separating content
- **Splitter**: Resizable split panes with draggable handles
- **Masonry**: Responsive masonry layout for cards and items

### General Components
- **Button**: Supports type/size/shape/danger/ghost/loading/block/icon/href
- **FloatButton**: Floating button with round/square, primary/default, danger, tooltip, and configurable position
- **Icon**: Built-in common icon set (plus/minus/check/close/info/question/search/arrow/loading) with rotation, size, and color support
- **Typography**: Title/Text/Paragraph with tone (default/secondary/success/warning/danger/disabled), strong/italic/underline/delete/code/mark, ellipsis (single/multi-line + expand), copyable, editable, and disabled state semantics
- **Affix**: Sticky positioning component
- **Breadcrumb**: Breadcrumb navigation
- **Dropdown**: Dropdown menu component
- **Menu**: Navigation menu with horizontal and vertical modes
- **Pagination**: Pagination component
- **Steps**: Step indicator for processes
- **Tabs**: Tab navigation component
- **Anchor**: Anchor navigation for long pages

### Data Entry
- **Form**: `Form`/`FormItem`/`use_form_item_control` with required/min/max/pattern/custom rule support, layout control, required mark, and context hooks
- **Input**: Text input with various variants (Password, Search, OTP)
- **InputNumber**: Number input with stepper controls
- **TextArea**: Multi-line text input
- **Select**: Dropdown selector with search and multiple selection
- **TreeSelect**: Tree-structured selector
- **Cascader**: Cascading selection component
- **AutoComplete**: Auto-complete input component
- **Checkbox**: Checkbox and checkbox group
- **Radio**: Radio button and radio group
- **Switch**: Toggle switch component
- **Slider**: Range slider component
- **Rate**: Rating component
- **Upload**: Click to select/drag and drop upload, lists (text/picture/picture-card), `before_upload`, XHR upload progress/abort, controlled/uncontrolled lists
- **DatePicker**: Date picker with range selection
- **TimePicker**: Time picker component
- **Calendar**: Calendar component
- **ColorPicker**: Color picker component
- **Mentions**: Mentions input component
- **Segmented**: Segmented control component

### Data Display
- **Table**: Advanced data table with sorting, filtering, pagination, and selection
- **Tag**: Tag component with various colors
- **Badge**: Badge and ribbon components
- **Card**: Card container component
- **Carousel**: Carousel/slider component
- **Collapse**: Collapsible panel component
- **Timeline**: Timeline component
- **Tree**: Tree component with directory tree support
- **Transfer**: Transfer list component
- **Descriptions**: Description list component
- **Empty**: Empty state component
- **List**: List component
- **Statistic**: Statistic display component
- **QRCode**: QR code generator component
- **Avatar**: Avatar component with group support
- **Image**: Image component with preview
- **Skeleton**: Skeleton loading component
- **Progress**: Progress bar component
- **Result**: Result page component
- **Watermark**: Watermark component

### Feedback
- **Alert**: Alert message component
- **Message**: Global message notification
- **Notification**: Notification component
- **Modal**: Modal dialog component
- **Drawer**: Drawer component
- **Popconfirm**: Popconfirm component
- **Popover**: Popover component
- **Tooltip**: Tooltip component
- **Spin**: Loading spinner component
- **Progress**: Progress indicator component
- **Skeleton**: Skeleton loading placeholder

### Other
- **App**: App-level context provider for message, modal, and notification
- **Tour**: Tour guide component
- **BackTop**: Back to top button

## Installation

Install adui-dioxus using Cargo:

```bash
cargo add adui-dioxus
```

Or manually add the dependency to your `Cargo.toml` file:

```toml
[dependencies]
adui-dioxus = "0.1.1"
```

## Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[English Documentation Index](docs/README.md)** - Complete component documentation in English
- **[中文文档索引](docs/README_CN.md)** - 完整的中文组件文档

Each component has detailed documentation including:
- API reference with all props, events, and methods
- Usage examples
- Use cases
- Differences from Ant Design 6.0.0

## Local Development

### Requirements

- Rust toolchain (latest stable recommended)
- Dioxus CLI (`cargo install dioxus-cli` or use `dx` command)
- `wasm32-unknown-unknown` target for WASM builds (`rustup target add wasm32-unknown-unknown`)

### Build and Check

```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

### Run Examples

Run examples in the browser using Dioxus CLI:

```bash
dx serve --example <example_name>
```

Available examples include:

- `button_demo` - Button component with theme switching
- `float_button_demo` - Floating button examples
- `icon_demo` - Icon showcase
- `typography_demo` - Typography components
- `layout_demo` - Layout components (Layout, Divider, Flex, Space, Grid, Masonry, Splitter)
- `flex_space_demo` - Flex and Space components
- `grid_demo` - Grid system examples
- `form_demo` - Form validation and controls
- `upload_demo` - File upload examples
- `table_demo` - Data table examples
- `menu_demo` - Navigation menu
- `tabs_demo` - Tab navigation
- `modal_demo` - Modal dialogs
- `drawer_demo` - Drawer component
- `select_demo` - Select component
- `date_picker_demo` - Date picker
- `input_demo` - Input variants
- `card_demo` - Card component
- `badge_demo` - Badge component
- `avatar_demo` - Avatar component
- `alert_demo` - Alert component
- `message_demo` - Message notifications
- `notification_demo` - Notification component
- `tooltip_demo` - Tooltip component
- `popover_demo` - Popover component
- `progress_demo` - Progress indicators
- `spin_demo` - Loading spinner
- `skeleton_demo` - Skeleton loading
- `steps_demo` - Steps component
- `timeline_demo` - Timeline component
- `tree_demo` - Tree component
- `tree_select_demo` - Tree select
- `transfer_demo` - Transfer list
- `pagination_demo` - Pagination
- `breadcrumb_demo` - Breadcrumb navigation
- `anchor_demo` - Anchor navigation
- `affix_demo` - Affix component
- `dropdown_demo` - Dropdown menu
- `checkbox_demo` - Checkbox component
- `radio_demo` - Radio component
- `switch_demo` - Switch component
- `slider_demo` - Slider component
- `rate_demo` - Rate component
- `input_number_demo` - Input number
- `cascader_demo` - Cascader component
- `auto_complete_demo` - Auto complete
- `color_picker_demo` - Color picker
- `mentions_demo` - Mentions input
- `segmented_demo` - Segmented control
- `descriptions_demo` - Descriptions component
- `empty_demo` - Empty state
- `list_demo` - List component
- `statistic_demo` - Statistic display
- `qrcode_demo` - QR code generator
- `image_demo` - Image component
- `carousel_demo` - Carousel component
- `collapse_demo` - Collapse component
- `tag_demo` - Tag component
- `result_demo` - Result page
- `watermark_demo` - Watermark component
- `tour_demo` - Tour guide
- `config_provider_demo` - Config provider
- `app_demo` - App context
- `dashboard_demo` - Dashboard example
- `landing_page_demo` - Landing page example
- `login_demo` - Login page example
- `register_demo` - Registration page example
- `settings_demo` - Settings page example

## Contributing

We welcome contributions! Please read our [Repository Guidelines](AGENTS.md) for development workflows, coding standards, and contribution requirements.

Key points:
- Follow Rust naming conventions and code style
- Add tests for new components
- Update documentation (both English and Chinese)
- Create examples for new components
- Ensure all checks pass: `cargo fmt && cargo clippy --all-targets --all-features && cargo test`

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Note

This is an experimental project. Some features may differ from the original Ant Design implementation, and the API may evolve as the project matures. Please refer to the component documentation for specific differences and limitations.
