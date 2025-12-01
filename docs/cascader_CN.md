# Cascader 级联选择器

## 概述

Cascader 组件提供级联选择界面，用于分层数据。它在多列中显示选项，允许用户浏览数据层级。

## API 参考

### CascaderProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `options` | `Vec<CascaderNode>` | - | 级联选项树（必需） |
| `value` | `Option<Vec<String>>` | `None` | 单选的受控路径值 |
| `multiple` | `bool` | `false` | 多选（预留，尚未实现） |
| `placeholder` | `Option<String>` | `None` | 占位符文本 |
| `allow_clear` | `bool` | `false` | 显示清除按钮 |
| `disabled` | `bool` | `false` | 禁用交互 |
| `status` | `Option<ControlStatus>` | `None` | 控制状态 |
| `size` | `Option<ComponentSize>` | `None` | 组件尺寸 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |
| `dropdown_class` | `Option<String>` | `None` | 下拉的额外类 |
| `dropdown_style` | `Option<String>` | `None` | 下拉的额外样式 |
| `on_change` | `Option<EventHandler<Vec<String>>>` | `None` | 选择改变时调用 |

### CascaderNode

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 节点的唯一键 |
| `label` | `String` | 显示标签 |
| `disabled` | `bool` | 节点是否禁用 |
| `children` | `Vec<CascaderNode>` | 子节点 |

## 使用示例

### 基础级联选择器

```rust
use adui_dioxus::{Cascader, CascaderNode};
use dioxus::prelude::*;

let value = use_signal(|| None::<Vec<String>>);

rsx! {
    Cascader {
        options: vec![
            CascaderNode {
                key: "zhejiang".to_string(),
                label: "浙江".to_string(),
                disabled: false,
                children: vec![
                    CascaderNode {
                        key: "hangzhou".to_string(),
                        label: "杭州".to_string(),
                        disabled: false,
                        children: vec![],
                    },
                ],
            },
        ],
        value: value.read().clone(),
        on_change: Some(move |path| {
            value.set(Some(path));
        }),
        placeholder: Some("选择位置".to_string()),
    }
}
```

### 多层级

```rust
use adui_dioxus::{Cascader, CascaderNode};

rsx! {
    Cascader {
        options: vec![
            CascaderNode {
                key: "china".to_string(),
                label: "中国".to_string(),
                disabled: false,
                children: vec![
                    CascaderNode {
                        key: "zhejiang".to_string(),
                        label: "浙江".to_string(),
                        disabled: false,
                        children: vec![
                            CascaderNode {
                                key: "hangzhou".to_string(),
                                label: "杭州".to_string(),
                                disabled: false,
                                children: vec![],
                            },
                        ],
                    },
                ],
            },
        ],
        placeholder: Some("选择位置".to_string()),
    }
}
```

## 使用场景

- **位置选择**：选择具有分层结构的位置
- **类别选择**：在嵌套结构中选择类别
- **组织选择**：选择组织单位
- **分层数据**：从分层数据中选择

## 与 Ant Design 6.0.0 的差异

- ✅ 单选
- ✅ 分层导航
- ✅ 多层级
- ⚠️ 多选尚未实现
- ⚠️ 某些高级功能可能有所不同

