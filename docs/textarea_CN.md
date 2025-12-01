# TextArea 文本域

## 概述

TextArea 组件提供多行文本输入字段。它支持字符计数、最大长度和各种视觉变体。

## API 参考

### TextAreaProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 受控值 |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `rows` | `Option<u16>` | `None` | 行数（默认：3） |
| `disabled` | `bool` | `false` | 禁用交互 |
| `size` | `Option<InputSize>` | `None` | 组件尺寸 |
| `variant` | `Option<Variant>` | `None` | 视觉变体 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `max_length` | `Option<usize>` | `None` | 最大长度 |
| `show_count` | `bool` | `false` | 显示字符计数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<InputClassNames>` | `None` | 语义类名 |
| `styles` | `Option<InputStyles>` | `None` | 语义样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |

## 使用示例

### 基础文本域

```rust
use adui_dioxus::TextArea;
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    TextArea {
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("输入文本".to_string()),
    }
}
```

### 带字符计数

```rust
use adui_dioxus::TextArea;

rsx! {
    TextArea {
        max_length: Some(100),
        show_count: true,
        placeholder: Some("输入文本".to_string()),
    }
}
```

### 带自定义行数

```rust
use adui_dioxus::TextArea;

rsx! {
    TextArea {
        rows: Some(5),
        placeholder: Some("输入文本".to_string()),
    }
}
```

## 使用场景

- **表单**：表单中的多行文本输入
- **评论**：评论输入字段
- **描述**：描述输入字段
- **笔记**：笔记输入字段

## 与 Ant Design 6.0.0 的差异

- ✅ 多行文本输入
- ✅ 字符计数
- ✅ 最大长度支持
- ✅ 视觉变体
- ⚠️ 某些高级功能可能有所不同

