# Rate 评分

## 概述

Rate 组件允许用户使用星星进行评分。它支持半星评分、自定义字符、工具提示和键盘导航。

## API 参考

### RateProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<f64>` | `None` | 受控的数值 |
| `default_value` | `Option<f64>` | `None` | 非受控的初始值 |
| `count` | `usize` | `5` | 项目总数（星星数） |
| `allow_half` | `bool` | `false` | 允许选择半星（0.5 增量） |
| `allow_clear` | `bool` | `true` | 再次点击相同值时允许清除 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `character` | `Option<Element>` | `None` | 每个项目的可选自定义字符 |
| `tooltips` | `Option<Vec<String>>` | `None` | 每个项目的可选工具提示（按索引对齐） |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `on_change` | `Option<EventHandler<Option<f64>>>` | `None` | 状态改变回调（None 表示已清除） |
| `on_hover_change` | `Option<EventHandler<Option<f64>>>` | `None` | 悬停值回调 |
| `on_focus` | `Option<EventHandler<()>>` | `None` | 焦点事件处理器 |
| `on_blur` | `Option<EventHandler<()>>` | `None` | 失焦事件处理器 |

## 使用示例

### 基础用法

```rust
use adui_dioxus::Rate;
use dioxus::prelude::*;

let rating = use_signal(|| Some(3.0));

rsx! {
    Rate {
        value: *rating.read(),
        on_change: Some(move |new_rating| {
            rating.set(new_rating);
        }),
    }
}
```

### 半星评分

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        allow_half: true,
        default_value: Some(3.5),
    }
}
```

### 带工具提示

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        tooltips: Some(vec![
            "极差".to_string(),
            "差".to_string(),
            "一般".to_string(),
            "好".to_string(),
            "极好".to_string(),
        ]),
    }
}
```

### 自定义字符

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        character: Some(rsx!(span { "❤" })),
        count: 5,
    }
}
```

### 禁用

```rust
use adui_dioxus::Rate;

rsx! {
    Rate {
        value: Some(4.0),
        disabled: true,
    }
}
```

## 使用场景

- **产品评分**：对产品或服务进行评分
- **评论**：收集用户评分
- **反馈**：收集用户反馈
- **调查**：调查中的评分问题

## 与 Ant Design 6.0.0 的差异

- ✅ 支持半星的星星评分
- ✅ 自定义字符
- ✅ 工具提示
- ✅ 键盘导航
- ✅ 悬停反馈
- ⚠️ 某些高级样式选项可能有所不同

