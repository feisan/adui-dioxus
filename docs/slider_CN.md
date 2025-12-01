# Slider 滑动输入条

## 概述

Slider 组件允许用户从连续或离散集合中选择一个值或值范围。它支持单值和范围模式、垂直方向和自定义标记。

## API 参考

### SliderProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<SliderValue>` | `None` | 受控值（当 `range = true` 时使用 `Range(_, _)`） |
| `default_value` | `Option<SliderValue>` | `None` | 非受控模式下的默认值 |
| `range` | `bool` | `false` | 是否渲染两个手柄 |
| `min` | `f64` | `0.0` | 最小值 |
| `max` | `f64` | `100.0` | 最大值 |
| `step` | `Option<f64>` | `None` | 步长粒度（为 None 时，滑块是连续的） |
| `precision` | `Option<u32>` | `None` | 用于对齐的小数精度 |
| `reverse` | `bool` | `false` | 反转视觉方向 |
| `vertical` | `bool` | `false` | 垂直方向 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `dots` | `bool` | `false` | 为标记渲染刻度点 |
| `marks` | `Option<Vec<SliderMark>>` | `None` | 沿轨道的可选标记标签 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<SliderValue>>` | `None` | 每次值改变时触发 |
| `on_change_complete` | `Option<EventHandler<SliderValue>>` | `None` | 用户完成交互时触发 |

### SliderValue

- `Single(f64)` - 单值
- `Range(f64, f64)` - 范围值（开始，结束）

### SliderMark

| 字段 | 类型 | 说明 |
|------|------|------|
| `value` | `f64` | 标记值 |
| `label` | `String` | 标记标签 |

## 使用示例

### 基础滑动条

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 范围滑动条

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Range(20.0, 80.0));

rsx! {
    Slider {
        range: true,
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带标记

```rust
use adui_dioxus::{Slider, SliderValue, SliderMark};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        marks: Some(vec![
            SliderMark { value: 0.0, label: "0".to_string() },
            SliderMark { value: 50.0, label: "50".to_string() },
            SliderMark { value: 100.0, label: "100".to_string() },
        ]),
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 垂直滑动条

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        vertical: true,
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

### 带步长

```rust
use adui_dioxus::{Slider, SliderValue};
use dioxus::prelude::*;

let value = use_signal(|| SliderValue::Single(50.0));

rsx! {
    Slider {
        step: Some(10.0),
        value: Some(*value.read()),
        on_change: Some(move |v| {
            value.set(v);
        }),
    }
}
```

## 使用场景

- **音量控制**：控制音量级别
- **价格范围**：选择价格范围
- **时间选择**：选择时间范围
- **评分**：选择评分或分数

## 与 Ant Design 6.0.0 的差异

- ✅ 单值和范围模式
- ✅ 垂直方向
- ✅ 自定义标记
- ✅ 步长控制
- ✅ 键盘导航
- ⚠️ 某些高级功能可能有所不同

