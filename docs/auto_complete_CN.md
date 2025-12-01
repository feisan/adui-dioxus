# AutoComplete 自动完成

## 概述

AutoComplete 组件提供带自动完成建议的输入字段。它根据用户输入过滤选项，并允许从下拉列表中选择。

## API 参考

### AutoCompleteProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `options` | `Option<Vec<SelectOption>>` | `None` | 候选选项列表 |
| `value` | `Option<String>` | `None` | 受控输入值 |
| `default_value` | `Option<String>` | `None` | 非受控模式下的初始值 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `allow_clear` | `bool` | `false` | 显示清除按钮 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `size` | `Option<ComponentSize>` | `None` | 组件尺寸 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `dropdown_class` | `Option<String>` | `None` | 下拉的额外类 |
| `dropdown_style` | `Option<String>` | `None` | 下拉的额外样式 |
| `on_change` | `Option<EventHandler<String>>` | `None` | 输入改变时调用 |
| `on_search` | `Option<EventHandler<String>>` | `None` | 搜索文本改变时调用 |
| `on_select` | `Option<EventHandler<String>>` | `None` | 选项被选中时调用 |

## 使用示例

### 基础自动完成

```rust
use adui_dioxus::{AutoComplete, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());

rsx! {
    AutoComplete {
        options: Some(vec![
            SelectOption::new("apple", "苹果"),
            SelectOption::new("banana", "香蕉"),
            SelectOption::new("cherry", "樱桃"),
        ]),
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        placeholder: Some("输入搜索...".to_string()),
    }
}
```

### 带搜索处理器

```rust
use adui_dioxus::{AutoComplete, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| String::new());
let options = use_signal(|| vec![
    SelectOption::new("1", "选项 1"),
]);

rsx! {
    AutoComplete {
        options: Some(options.read().clone()),
        value: Some(value.read().clone()),
        on_change: Some(move |v| {
            value.set(v);
        }),
        on_search: Some(move |query| {
            // 根据搜索查询更新选项
            println!("搜索: {}", query);
        }),
        placeholder: Some("输入搜索...".to_string()),
    }
}
```

### 带选择处理器

```rust
use adui_dioxus::{AutoComplete, SelectOption};

rsx! {
    AutoComplete {
        options: Some(vec![
            SelectOption::new("1", "选项 1"),
            SelectOption::new("2", "选项 2"),
        ]),
        on_select: Some(move |key| {
            println!("已选择: {}", key);
        }),
        placeholder: Some("输入搜索...".to_string()),
    }
}
```

## 使用场景

- **搜索**：自动完成搜索输入
- **表单**：带建议的表单输入
- **数据录入**：带自动完成的数据录入
- **过滤**：用户输入时过滤选项

## 与 Ant Design 6.0.0 的差异

- ✅ 带自动完成的输入
- ✅ 选项过滤
- ✅ 选择处理
- ✅ 搜索回调
- ⚠️ 某些高级功能可能有所不同

