# adui-dioxus

## Introduction

adui-dioxus is a UI component library for Dioxus that provides rich components and styles, helping developers quickly build cross-platform web and mobile applications.

This is an **experimental project** that ports Ant Design 6.0.0 to Dioxus using **Vibe Coding**. It extracts components from the Ant Design UI library (https://github.com/ant-design/ant-design) and adapts them for the Rust/Dioxus ecosystem. The library inherits Ant Design's design philosophy and component styles while leveraging Dioxus's performance and flexibility to provide developers with an efficient and convenient development experience.

## Project Status

This is an experimental port of Ant Design 6.0.0 to Dioxus. The library includes:

- **Theme**: Ant Design 6.x style tokens and theme context (light/dark presets, CSS variable export)
- **Button**: Supports type/size/shape/danger/ghost/loading/block/icon/href
- **FloatButton**: Floating button with round/square, primary/default, danger, tooltip, and configurable position
- **Icon**: Built-in common icon set (plus/minus/check/close/info/question/search/arrow/loading) with rotation, size, and color support
- **Typography**: Title/Text/Paragraph with tone (default/secondary/success/warning/danger/disabled), strong/italic/underline/delete/code/mark, ellipsis (single/multi-line + expand), copyable, editable, and disabled state semantics
- **Form**: `Form`/`FormItem`/`use_form_item_control` with required/min/max/pattern/custom rule support, layout control, required mark, and context hooks
- **Upload**: Click to select/drag and drop upload, lists (text/picture/picture-card), `before_upload`, XHR upload progress/abort, controlled/uncontrolled lists
- **Layout**: Divider/Flex/Grid (supports basic breakpoint span)/Layout (Sider supports collapsible/trigger/theme/manual has_sider)/Masonry (responsive column count + gap/row_gap/min width)/Space/Splitter (draggable split panes), covering common layout scenarios

And many more components...

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

Requires Rust + Dioxus 0.7 ecosystem (recommended to install dioxus-cli).

### Build and Check

```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

### Run Examples

- Button example (browser): `dx serve --example button_demo`
- Float button example (browser): `dx serve --example float_button_demo`
- Icon example (browser): `dx serve --example icon_demo`
- Typography example (browser): `dx serve --example typography_demo`
- Layout example (browser): `dx serve --example layout_demo`
- Flex/Space example (browser): `dx serve --example flex_space_demo`
- Grid example (browser): `dx serve --example grid_demo`
- Form example (browser): `dx serve --example form_demo`
- Upload example (browser): `dx serve --example upload_demo`

## Example Features Overview

- `button_demo`: Theme switching (Light/Dark), primary color presets, button type/size/shape and state toggles
- `float_button_demo`: Floating button primary/secondary buttons, theme switching, position and tooltip display
- `icon_demo`: Icon list, theme switching, size adjustment, primary color switching, global rotation toggle
- `typography_demo`: Title/Text/Paragraph, showing tone switching, modifiers, copyable, expandable ellipsis, inline editing
- `layout_demo`: Shows Divider, Flex, Space, Grid, Layout (including Sider collapse/Zero Trigger), Masonry, Splitter (drag to adjust split ratio)
- `grid_demo`: Shows Row horizontal/vertical/responsive gutter and Col's span/offset/order/push/pull/flex responsive configuration
- `flex_space_demo`: Demonstrates `FlexConfigProvider`, gap presets, wrap, Space size/split/compact and other layout capabilities
- `form_demo`: Shows `Form` basic validation, layout, reset/submit callbacks and custom control integration
- `upload_demo`: Shows basic upload, image list, dragger drag area, validates `before_upload`, list removal and upload logs

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Note

This is an experimental project. Some features may differ from the original Ant Design implementation, and the API may evolve as the project matures. Please refer to the component documentation for specific differences and limitations.
