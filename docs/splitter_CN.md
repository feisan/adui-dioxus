# Splitter 分割面板

## 概述

Splitter 组件显示两个可调整大小的面板，带有一个可拖动的分隔条。通常用于创建可调整大小的布局，如代码编辑器或文件浏览器。

## API 参考

### SplitterProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `orientation` | `SplitterOrientation` | `SplitterOrientation::Horizontal` | 分割面板方向 |
| `split` | `Option<f32>` | `None` | 受控的分割比例（0.0-1.0） |
| `default_split` | `f32` | `0.5` | 非受控模式下的默认分割比例 |
| `on_change` | `Option<EventHandler<f32>>` | `None` | 分割比例改变时调用 |
| `on_moving` | `Option<EventHandler<f32>>` | `None` | 拖动时调用 |
| `on_release` | `Option<EventHandler<f32>>` | `None` | 拖动结束时调用 |
| `min_primary` | `Option<f32>` | `None` | 主面板的最小尺寸（默认为 80px） |
| `min_secondary` | `Option<f32>` | `None` | 次面板的最小尺寸（默认为 80px） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `gutter_aria_label` | `Option<String>` | `None` | 分隔条的 ARIA 标签 |
| `first` | `Element` | - | 第一个面板内容（必需） |
| `second` | `Element` | - | 第二个面板内容（必需） |

### SplitterOrientation

- `Horizontal` - 水平分割面板（默认）
- `Vertical` - 垂直分割面板

## 使用示例

### 基础分割面板

```rust
use adui_dioxus::Splitter;

rsx! {
    Splitter {
        first: rsx! {
            div { "左侧面板" }
        }
        second: rsx! {
            div { "右侧面板" }
        }
    }
}
```

### 垂直分割面板

```rust
use adui_dioxus::{Splitter, SplitterOrientation};

rsx! {
    Splitter {
        orientation: SplitterOrientation::Vertical,
        first: rsx! {
            div { "顶部面板" }
        }
        second: rsx! {
            div { "底部面板" }
        }
    }
}
```

### 受控分割面板

```rust
use adui_dioxus::Splitter;
use dioxus::prelude::*;

let split_ratio = use_signal(|| 0.3);

rsx! {
    Splitter {
        split: Some(*split_ratio.read()),
        on_change: Some(move |ratio| {
            split_ratio.set(ratio);
        }),
        first: rsx! { div { "左侧" } }
        second: rsx! { div { "右侧" } }
    }
}
```

### 带最小尺寸

```rust
use adui_dioxus::Splitter;

rsx! {
    Splitter {
        min_primary: Some(200.0),
        min_secondary: Some(300.0),
        first: rsx! { div { "左侧" } }
        second: rsx! { div { "右侧" } }
    }
}
```

## 使用场景

- **代码编辑器**：代码和预览的分割面板
- **文件浏览器**：可调整大小的文件和内容面板
- **仪表板**：可调整大小的仪表板面板
- **布局**：创建可调整大小的布局

## 与 Ant Design 6.0.0 的差异

- ✅ 水平和垂直方向
- ✅ 受控和非受控模式
- ✅ 最小尺寸约束
- ✅ 拖动处理器
- ⚠️ 某些高级功能可能有所不同

