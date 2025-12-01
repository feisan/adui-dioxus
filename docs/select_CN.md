# Select 选择器

## 概述

Select 组件提供下拉选择器，支持单选、多选、标签模式和组合框模式。它包含搜索功能和自定义过滤。

## API 参考

### SelectProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `value` | `Option<String>` | `None` | 单选的受控值 |
| `values` | `Option<Vec<String>>` | `None` | 多选的受控值 |
| `options` | `Vec<SelectOption>` | - | 选项列表（必需） |
| `mode` | `SelectMode` | `SelectMode::Single` | 选择模式 |
| `multiple` | `bool` | `false` | @deprecated 请使用 `mode` |
| `allow_clear` | `bool` | `false` | 有选择时显示清除图标 |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `show_search` | `bool` | `false` | 启用搜索功能 |
| `filter_option` | `Option<Rc<dyn Fn(&str, &SelectOption) -> bool>>` | `None` | 自定义过滤函数 |
| `token_separators` | `Option<Vec<String>>` | `None` | 标签模式的标记分隔符 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `size` | `Option<ComponentSize>` | `None` | 组件尺寸 |
| `variant` | `Option<Variant>` | `None` | 视觉变体 |
| `bordered` | `Option<bool>` | `None` | @deprecated 请使用 `variant` |
| `prefix` | `Option<Element>` | `None` | 前缀元素 |
| `suffix_icon` | `Option<Element>` | `None` | 自定义后缀图标 |
| `placement` | `SelectPlacement` | `SelectPlacement::BottomLeft` | 下拉位置 |
| `popup_match_select_width` | `bool` | `true` | 下拉宽度是否匹配选择器宽度 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `root_class_name` | `Option<String>` | `None` | 根元素的额外类 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<SelectClassNames>` | `None` | 语义类名 |
| `styles` | `Option<SelectStyles>` | `None` | 语义样式 |
| `dropdown_class` | `Option<String>` | `None` | 下拉的额外类 |
| `dropdown_style` | `Option<String>` | `None` | 下拉的额外样式 |
| `popup_render` | `Option<Rc<dyn Fn(Element) -> Element>>` | `None` | 自定义下拉渲染函数 |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | 选择改变时调用 |
| `on_dropdown_visible_change` | `Option<EventHandler<bool>>` | `None` | 下拉可见性改变时调用 |

### SelectMode

- `Single` - 单选（默认）
- `Multiple` - 多选
- `Tags` - 标签模式（允许创建新选项）
- `Combobox` - 组合框模式（允许自由文本输入）

### SelectPlacement

- `BottomLeft` - 底部左侧（默认）
- `BottomRight` - 底部右侧
- `TopLeft` - 顶部左侧
- `TopRight` - 顶部右侧

## 使用示例

### 基础选择器

```rust
use adui_dioxus::{Select, SelectOption};
use dioxus::prelude::*;

let value = use_signal(|| None::<String>);

rsx! {
    Select {
        options: vec![
            SelectOption::new("1", "选项 1"),
            SelectOption::new("2", "选项 2"),
            SelectOption::new("3", "选项 3"),
        ],
        value: *value.read(),
        on_change: Some(move |values| {
            value.set(values.first().cloned());
        }),
        placeholder: Some("选择一个选项".to_string()),
    }
}
```

### 多选

```rust
use adui_dioxus::{Select, SelectMode, SelectOption};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    Select {
        mode: SelectMode::Multiple,
        options: vec![
            SelectOption::new("1", "选项 1"),
            SelectOption::new("2", "选项 2"),
        ],
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
    }
}
```

### 带搜索

```rust
use adui_dioxus::{Select, SelectOption};

rsx! {
    Select {
        show_search: true,
        options: vec![
            SelectOption::new("1", "苹果"),
            SelectOption::new("2", "香蕉"),
            SelectOption::new("3", "樱桃"),
        ],
        placeholder: Some("搜索...".to_string()),
    }
}
```

### 标签模式

```rust
use adui_dioxus::{Select, SelectMode, SelectOption};
use dioxus::prelude::*;

let values = use_signal(|| vec![]);

rsx! {
    Select {
        mode: SelectMode::Tags,
        options: vec![
            SelectOption::new("1", "标签 1"),
            SelectOption::new("2", "标签 2"),
        ],
        values: Some(values.read().clone()),
        on_change: Some(move |v| {
            values.set(v);
        }),
        token_separators: Some(vec![",".to_string(), " ".to_string()]),
    }
}
```

## 使用场景

- **表单选择**：在表单中选择选项
- **数据过滤**：通过选择过滤数据
- **标签管理**：使用标签模式管理标签
- **搜索**：搜索和选择选项

## 与 Ant Design 6.0.0 的差异

- ✅ 单选和多选
- ✅ 标签和组合框模式
- ✅ 搜索功能
- ✅ 自定义过滤
- ⚠️ 某些高级功能可能有所不同

