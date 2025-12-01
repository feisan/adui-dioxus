# Form 表单

## 概述

Form 组件提供表单管理功能，包括验证、布局控制和字段协调。它支持多种布局、验证规则，并与表单控件集成。

## API 参考

### FormProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `layout` | `FormLayout` | `FormLayout::Horizontal` | 表单布局 |
| `size` | `ControlSize` | `ControlSize::Middle` | 控件尺寸 |
| `colon` | `bool` | `true` | 标签后显示冒号 |
| `required_mark` | `RequiredMark` | `RequiredMark::Default` | 必填标记配置 |
| `label_align` | `LabelAlign` | `LabelAlign::Right` | 标签对齐 |
| `label_wrap` | `bool` | `false` | 是否换行标签文本 |
| `label_col` | `Option<ColProps>` | `None` | 标签列配置 |
| `wrapper_col` | `Option<ColProps>` | `None` | 包装器列配置 |
| `disabled` | `bool` | `false` | 禁用所有表单控件 |
| `variant` | `Option<Variant>` | `None` | 视觉变体 |
| `scroll_to_first_error` | `bool` | `false` | 提交时滚动到第一个错误 |
| `scroll_to_first_error_config` | `Option<ScrollToFirstErrorConfig>` | `None` | 滚动配置 |
| `feedback_icons` | `Option<FeedbackIcons>` | `None` | 反馈图标配置 |
| `name` | `Option<String>` | `None` | 表单名称 |
| `initial_values` | `Option<FormValues>` | `None` | 初始表单值 |
| `form` | `Option<FormHandle>` | `None` | 表单句柄实例 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `class_names` | `Option<FormClassNames>` | `None` | 语义类名 |
| `styles` | `Option<FormStyles>` | `None` | 语义样式 |
| `on_finish` | `Option<EventHandler<FormFinishEvent>>` | `None` | 成功提交时调用 |
| `on_finish_failed` | `Option<EventHandler<FormFinishFailedEvent>>` | `None` | 验证失败时调用 |
| `on_values_change` | `Option<EventHandler<ValuesChangeEvent>>` | `None` | 值改变时调用 |
| `children` | `Element` | - | 表单内容（必需） |

### FormItemProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `name` | `String` | - | 字段名称（必需） |
| `label` | `Option<Element>` | `None` | 标签元素 |
| `required` | `bool` | `false` | 字段是否必填 |
| `rules` | `Option<Vec<FormRule>>` | `None` | 验证规则 |
| `value_prop_name` | `Option<String>` | `None` | 自定义控件的值属性名 |
| `get_value_from_event` | `Option<GetValueFromEventFn>` | `None` | 从事件中提取值的函数 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `children` | `Element` | - | 表单控件（必需） |

### FormRule

| 字段 | 类型 | 说明 |
|------|------|------|
| `required` | `bool` | 字段是否必填 |
| `min` | `Option<usize>` | 最小长度 |
| `max` | `Option<usize>` | 最大长度 |
| `len` | `Option<usize>` | 精确长度 |
| `pattern` | `Option<String>` | 正则表达式模式 |
| `message` | `Option<String>` | 错误消息 |
| `validator` | `Option<FormValidator>` | 自定义验证器函数 |

### FormLayout

- `Horizontal` - 水平布局（默认）
- `Vertical` - 垂直布局
- `Inline` - 内联布局

### RequiredMark

- `None` - 隐藏必填标记
- `Optional` - 显示"可选"文本
- `Default` - 显示默认必填标记
- `Custom` - 自定义渲染函数

## 使用示例

### 基础表单

```rust
use adui_dioxus::{Form, FormItem, Input, Button, ButtonType};
use dioxus::prelude::*;

rsx! {
    Form {
        on_finish: Some(move |event| {
            println!("表单值: {:?}", event.values);
        }),
        FormItem {
            name: "username".to_string(),
            label: Some(rsx!("用户名")),
            required: true,
            Input {
                placeholder: Some("输入用户名".to_string()),
            }
        }
        FormItem {
            name: "email".to_string(),
            label: Some(rsx!("邮箱")),
            required: true,
            Input {
                placeholder: Some("输入邮箱".to_string()),
            }
        }
        Button {
            r#type: ButtonType::Primary,
            html_type: ButtonHtmlType::Submit,
            "提交"
        }
    }
}
```

### 带验证规则

```rust
use adui_dioxus::{Form, FormItem, FormRule, Input};

rsx! {
    Form {
        FormItem {
            name: "email".to_string(),
            label: Some(rsx!("邮箱")),
            rules: Some(vec![
                FormRule {
                    required: true,
                    message: Some("邮箱是必填的".to_string()),
                    ..Default::default()
                },
                FormRule {
                    pattern: Some(r"^[^\s@]+@[^\s@]+\.[^\s@]+$".to_string()),
                    message: Some("无效的邮箱格式".to_string()),
                    ..Default::default()
                },
            ]),
            Input {
                placeholder: Some("输入邮箱".to_string()),
            }
        }
    }
}
```

### 垂直布局

```rust
use adui_dioxus::{Form, FormLayout, FormItem, Input};

rsx! {
    Form {
        layout: FormLayout::Vertical,
        FormItem {
            name: "username".to_string(),
            label: Some(rsx!("用户名")),
            Input {}
        }
    }
}
```

### 使用 FormHandle

```rust
use adui_dioxus::{Form, FormItem, Input, Button, ButtonType, use_form};
use dioxus::prelude::*;

fn MyForm() -> Element {
    let form = use_form();
    
    rsx! {
        Form {
            form: Some(form.clone()),
            FormItem {
                name: "username".to_string(),
                label: Some(rsx!("用户名")),
                Input {}
            }
            Button {
                onclick: move |_| {
                    let values = form.values();
                    println!("表单值: {:?}", values);
                },
                "获取值"
            }
        }
    }
}
```

## 使用场景

- **用户注册**：注册表单
- **设置**：设置表单
- **数据录入**：数据录入表单
- **搜索**：带验证的搜索表单

## 与 Ant Design 6.0.0 的差异

- ✅ 表单验证
- ✅ 多种布局
- ✅ 表单句柄 API
- ✅ 自定义验证器
- ✅ 表单列表支持
- ⚠️ 某些高级功能可能有所不同

