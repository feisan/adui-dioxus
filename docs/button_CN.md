# Button 按钮

## 概述

Button 组件是一个基础的 UI 元素，提供多种样式、尺寸和状态用于用户交互。它支持多种变体（实心、描边、虚线、文本、链接）、颜色（主要、成功、警告、危险、默认），并且可以在组中使用共享样式。

## API 参考

### ButtonProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `type` | `ButtonType` | `ButtonType::Default` | 视觉类型（旧版 API，映射到 variant/color） |
| `size` | `ButtonSize` | `ButtonSize::Middle` | 按钮尺寸：`Small`、`Middle`、`Large` |
| `shape` | `ButtonShape` | `ButtonShape::Default` | 按钮形状：`Default`、`Round`、`Circle` |
| `danger` | `bool` | `false` | 是否为危险按钮 |
| `ghost` | `bool` | `false` | 是否为幽灵按钮（透明背景） |
| `block` | `bool` | `false` | 按钮是否占满整行 |
| `loading` | `bool` | `false` | 按钮是否处于加载状态 |
| `loading_delay` | `Option<u64>` | `None` | 显示加载指示器前的延迟（毫秒） |
| `loading_icon` | `Option<Element>` | `None` | 自定义加载图标元素 |
| `auto_insert_space` | `bool` | `true` | 是否在中文字符间自动插入空格 |
| `label` | `Option<String>` | `None` | 可选标签文本，用于自动间距/仅图标检测 |
| `icon_only` | `Option<bool>` | `None` | 标记为仅图标按钮（添加类名） |
| `disabled` | `bool` | `false` | 按钮是否禁用 |
| `color` | `Option<ButtonColor>` | `None` | 按钮颜色：`Default`、`Primary`、`Success`、`Warning`、`Danger` |
| `variant` | `Option<ButtonVariant>` | `None` | 按钮变体：`Solid`、`Outlined`、`Dashed`、`Text`、`Link` |
| `icon_placement` | `ButtonIconPlacement` | `ButtonIconPlacement::Start` | 图标位置：`Start`、`End` |
| `icon_position` | `Option<ButtonIconPlacement>` | `None` | **已废弃**：请使用 `icon_placement` |
| `icon` | `Option<Element>` | `None` | 要显示的图标元素 |
| `href` | `Option<String>` | `None` | 如果提供，将渲染为 `<a>` 标签而非 `<button>` |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `class_names_root` | `Option<String>` | `None` | 应用于按钮元素的额外类 |
| `class_names_icon` | `Option<String>` | `None` | 应用于图标 span 的额外类 |
| `class_names_content` | `Option<String>` | `None` | 应用于内容 span 的额外类 |
| `styles_root` | `Option<String>` | `None` | 应用于根的内联样式 |
| `html_type` | `ButtonHtmlType` | `ButtonHtmlType::Button` | 原生按钮类型：`Button`、`Submit`、`Reset` |
| `data_attributes` | `Option<Vec<(String, String)>>` | `None` | 数据属性键值对（不带 "data-" 前缀） |
| `onclick` | `Option<EventHandler<MouseEvent>>` | `None` | 点击事件处理器 |
| `children` | `Element` | - | 按钮内容（必需） |

### ButtonGroupProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `size` | `Option<ButtonSize>` | `None` | 组内所有按钮的共享尺寸 |
| `shape` | `Option<ButtonShape>` | `None` | 组内所有按钮的共享形状 |
| `color` | `Option<ButtonColor>` | `None` | 组内所有按钮的共享颜色 |
| `variant` | `Option<ButtonVariant>` | `None` | 组内所有按钮的共享变体 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 按钮子元素（必需） |

### 枚举类型

#### ButtonType
- `Default` - 默认按钮样式
- `Primary` - 主要按钮样式
- `Dashed` - 虚线边框按钮
- `Text` - 文本按钮（无边框）
- `Link` - 链接样式按钮

#### ButtonColor
- `Default` - 默认颜色
- `Primary` - 主要颜色
- `Success` - 成功颜色
- `Warning` - 警告颜色
- `Danger` - 危险颜色

#### ButtonVariant
- `Solid` - 实心填充按钮
- `Outlined` - 描边按钮（默认）
- `Dashed` - 虚线边框按钮
- `Text` - 文本按钮
- `Link` - 链接按钮

#### ButtonSize
- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

#### ButtonShape
- `Default` - 默认形状
- `Round` - 圆角
- `Circle` - 圆形按钮

#### ButtonIconPlacement
- `Start` - 图标在文本前（默认）
- `End` - 图标在文本后

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Button, ButtonType, ButtonSize};

rsx! {
    Button {
        r#type: ButtonType::Primary,
        size: ButtonSize::Large,
        onclick: move |_| {
            println!("按钮被点击！");
        },
        "点击我"
    }
}
```

### 按钮变体

```rust
use adui_dioxus::{Button, ButtonVariant, ButtonColor};

rsx! {
    div {
        Button {
            variant: Some(ButtonVariant::Solid),
            color: Some(ButtonColor::Primary),
            "实心主要"
        }
        Button {
            variant: Some(ButtonVariant::Outlined),
            color: Some(ButtonColor::Primary),
            "描边主要"
        }
        Button {
            variant: Some(ButtonVariant::Text),
            "文本按钮"
        }
        Button {
            variant: Some(ButtonVariant::Link),
            "链接按钮"
        }
    }
}
```

### 带图标的按钮

```rust
use adui_dioxus::{Button, Icon, IconKind, ButtonIconPlacement};

rsx! {
    Button {
        icon: rsx!(Icon { kind: IconKind::Search, size: 16.0 }),
        icon_placement: ButtonIconPlacement::Start,
        "搜索"
    }
}
```

### 加载状态

```rust
use adui_dioxus::Button;

rsx! {
    Button {
        loading: true,
        loading_delay: Some(300),
        "加载中..."
    }
}
```

### 按钮组

```rust
use adui_dioxus::{Button, ButtonGroup, ButtonSize, ButtonVariant, ButtonColor};

rsx! {
    ButtonGroup {
        size: Some(ButtonSize::Small),
        variant: Some(ButtonVariant::Solid),
        color: Some(ButtonColor::Primary),
        Button { "上一页" }
        Button { "刷新" }
        Button { "下一页" }
    }
}
```

### 块级按钮

```rust
use adui_dioxus::Button;

rsx! {
    Button {
        block: true,
        "全宽按钮"
    }
}
```

## 使用场景

- **表单操作**：表单中的提交、重置或取消按钮
- **导航**：用于导航的链接样式按钮
- **操作**：模态框、抽屉或页面中的主要和次要操作
- **工具栏**：工具栏和操作栏的按钮组
- **图标按钮**：紧凑界面的仅图标按钮

## 与 Ant Design 6.0.0 的差异

### 类型系统差异

1. **Loading 属性**：在 Ant Design TypeScript 中，`loading` 可以是布尔值或对象 `{ delay?: number, icon?: ReactNode }`。在此 Rust 实现中，它被拆分为：
   - `loading: bool` - 加载状态
   - `loading_delay: Option<u64>` - 延迟（毫秒）
   - `loading_icon: Option<Element>` - 自定义加载图标

2. **Props 结构**：Rust 实现使用显式枚举类型（`ButtonColor`、`ButtonVariant`）而不是字符串字面量，提供更好的类型安全性。

3. **图标位置**：已废弃的 `icon_position` 属性为向后兼容而保留，但会映射到 `icon_placement`。

### 功能特性

- ✅ 所有基础按钮类型和变体
- ✅ 具有共享样式的按钮组
- ✅ 带延迟支持的加载状态
- ✅ 带位置控制的图标支持
- ✅ 块级和幽灵模式
- ✅ 通过 `href` 属性进行链接渲染
- ⚠️ 波纹效果尚未实现（可通过自定义动画添加）
- ⚠️ 由于 Rust/Dioxus 约束，某些高级样式功能可能略有不同

