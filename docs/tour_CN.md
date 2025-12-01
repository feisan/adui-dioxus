# Tour 漫游式引导

## 概述

Tour 组件提供引导式用户入门体验。它显示一系列步骤，高亮 UI 元素并提供描述，帮助用户学习如何使用应用程序。

## API 参考

### TourProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `open` | `bool` | - | 漫游是否可见（必需） |
| `steps` | `Vec<TourStep>` | - | 要显示的漫游步骤（必需） |
| `current` | `Option<usize>` | `None` | 受控的当前步骤索引 |
| `on_close` | `Option<EventHandler<()>>` | `None` | 漫游关闭时调用 |
| `on_change` | `Option<EventHandler<usize>>` | `None` | 当前步骤改变时调用 |
| `on_finish` | `Option<EventHandler<()>>` | `None` | 用户完成漫游时调用 |
| `r#type` | `TourType` | `TourType::Default` | 漫游的视觉类型 |
| `mask_closable` | `bool` | `true` | 点击遮罩是否关闭漫游 |
| `closable` | `bool` | `true` | 是否显示关闭按钮 |
| `show_indicators` | `bool` | `true` | 是否显示步骤指示器 |
| `next_button_text` | `Option<String>` | `None` | "下一步"按钮文本 |
| `prev_button_text` | `Option<String>` | `None` | "上一步"按钮文本 |
| `finish_button_text` | `Option<String>` | `None` | "完成"按钮文本 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### TourStep

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 步骤的唯一键 |
| `title` | `Option<String>` | 在漫游面板中显示的标题 |
| `description` | `Option<Element>` | 描述文本或元素 |
| `cover` | `Option<Element>` | 封面图片或元素 |
| `placement` | `Option<TooltipPlacement>` | 漫游面板的位置 |
| `target` | `Option<String>` | 目标元素的 CSS 选择器 |
| `next_button_text` | `Option<String>` | 自定义"下一步"按钮文本 |
| `prev_button_text` | `Option<String>` | 自定义"上一步"按钮文本 |

### TourType

- `Default` - 默认样式，浅色背景
- `Primary` - 主要样式，彩色背景

## 使用示例

### 基础漫游

```rust
use adui_dioxus::{Tour, TourStep};
use dioxus::prelude::*;

let open = use_signal(|| true);

rsx! {
    Tour {
        open: *open.read(),
        steps: vec![
            TourStep::new("step1", "欢迎", "这是第一步"),
            TourStep::new("step2", "功能", "探索此功能"),
        ],
        on_close: Some(move |_| {
            open.set(false);
        }),
        on_finish: Some(move |_| {
            open.set(false);
        }),
    }
}
```

### 带目标元素

```rust
use adui_dioxus::{Tour, TourStep, TooltipPlacement};

rsx! {
    Tour {
        open: true,
        steps: vec![
            TourStep::new("step1", "按钮", "点击此按钮")
                .target("#my-button")
                .placement(TooltipPlacement::Bottom),
        ],
        on_close: Some(move |_| {}),
    }
}
```

### 主要样式

```rust
use adui_dioxus::{Tour, TourStep, TourType};

rsx! {
    Tour {
        open: true,
        r#type: TourType::Primary,
        steps: vec![
            TourStep::new("step1", "欢迎", "欢迎使用应用"),
        ],
        on_close: Some(move |_| {}),
    }
}
```

## 使用场景

- **用户入门**：引导新用户使用应用程序
- **功能介绍**：介绍新功能
- **教程**：创建分步教程
- **帮助系统**：提供上下文帮助

## 与 Ant Design 6.0.0 的差异

- ✅ 分步引导
- ✅ 元素高亮
- ✅ 自定义位置
- ✅ 键盘导航
- ⚠️ 某些高级功能可能有所不同

