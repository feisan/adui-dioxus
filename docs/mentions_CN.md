# Mentions 提及

## 概述

Mentions 组件提供类似文本区域的输入，当用户输入触发字符（默认 `@`）时显示下拉列表，允许从选项列表中选择。通常用于文本输入中的 @ 样式提及。

## API 参考

### MentionsProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 文本区域的受控值 |
| `default_value` | `Option<String>` | `None` | 非受控使用的默认值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `options` | `Vec<MentionOption>` | `vec![]` | 可用的提及选项 |
| `prefix` | `Vec<char>` | `vec!['@']` | 打开下拉列表的触发字符 |
| `split` | `char` | `' '` | 将提及与其他文本分开的字符 |
| `disabled` | `bool` | `false` | 组件是否禁用 |
| `read_only` | `bool` | `false` | 组件是否只读 |
| `loading` | `bool` | `false` | 是否显示加载指示器 |
| `status` | `Option<ControlStatus>` | `None` | 验证样式状态 |
| `size` | `Option<ComponentSize>` | `None` | 尺寸变体 |
| `placement` | `MentionPlacement` | `MentionPlacement::Bottom` | 下拉位置 |
| `auto_focus` | `bool` | `false` | 是否自动聚焦输入 |
| `rows` | `u32` | `1` | 文本区域的行数 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 值改变时调用 |
| `on_select` | `Option<EventHandler<MentionOption>>` | `None` | 选项被选中时调用 |
| `on_search` | `Option<EventHandler<(String, char)>>` | `None` | 搜索文本改变时调用 |
| `on_focus` | `Option<EventHandler<()>>` | `None` | 焦点改变时调用 |
| `on_blur` | `Option<EventHandler<()>>` | `None` | 失焦时调用 |
| `filter_option` | `Option<fn(&str, &MentionOption) -> bool>` | `None` | 自定义过滤函数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### MentionOption

| 字段 | 类型 | 说明 |
|------|------|------|
| `value` | `String` | 选中时插入的值 |
| `label` | `String` | 在下拉列表中显示的标签 |
| `disabled` | `bool` | 此选项是否禁用 |

### MentionPlacement

- `Top` - 下拉列表出现在输入上方
- `Bottom` - 下拉列表出现在输入下方（默认）

## 使用示例

### 基础提及

```rust
use adui_dioxus::{Mentions, MentionOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    Mentions {
        options: vec![
            MentionOption::new("user1", "用户 1"),
            MentionOption::new("user2", "用户 2"),
            MentionOption::new("user3", "用户 3"),
        ],
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("输入 @ 来提及".to_string()),
    }
}
```

### 带自定义触发字符

```rust
use adui_dioxus::{Mentions, MentionOption};

rsx! {
    Mentions {
        prefix: vec!['#'],
        options: vec![
            MentionOption::new("tag1", "标签 1"),
            MentionOption::new("tag2", "标签 2"),
        ],
        placeholder: Some("输入 # 来标记".to_string()),
    }
}
```

### 带搜索处理器

```rust
use adui_dioxus::{Mentions, MentionOption};

rsx! {
    Mentions {
        options: vec![
            MentionOption::new("user1", "用户 1"),
        ],
        on_search: Some(move |(query, trigger)| {
            println!("搜索: {} (触发: {})", query, trigger);
        }),
        placeholder: Some("输入 @ 来提及".to_string()),
    }
}
```

## 使用场景

- **用户提及**：在评论或消息中 @ 提及用户
- **标记**：使用 # 样式提及标记项目
- **自动完成**：文本输入中的自动完成
- **社交功能**：社交媒体样式的提及

## 与 Ant Design 6.0.0 的差异

- ✅ 触发字符支持
- ✅ 下拉选择
- ✅ 搜索功能
- ✅ 自定义过滤
- ⚠️ 某些高级功能可能有所不同

