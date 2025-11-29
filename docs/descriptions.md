# Descriptions 描述列表

成组展示多个只读字段。

## 何时使用

常见于详情页的信息展示。

## 代码演示

### 基础用法

简单的展示。

```rust
use adui_dioxus::*;

let items = vec![
    DescriptionsItem::new(
        "name",
        rsx! { "用户名" },
        rsx! { Text { "张三" } },
    ),
    DescriptionsItem::new(
        "phone",
        rsx! { "手机号" },
        rsx! { Text { "138-0000-0000" } },
    ),
    DescriptionsItem::new(
        "email",
        rsx! { "邮箱" },
        rsx! { Text { "zhangsan@example.com" } },
    ),
];

rsx! {
    Descriptions {
        items: items,
    }
}
```

### 带边框

加上边框的描述列表。

```rust
rsx! {
    Descriptions {
        items: items,
        bordered: true,
    }
}
```

### 垂直布局

垂直的列表。

```rust
rsx! {
    Descriptions {
        items: items,
        layout: DescriptionsLayout::Vertical,
        bordered: true,
    }
}
```

### 自定义列数

一行显示几列，可以自定义。

```rust
rsx! {
    Descriptions {
        items: items,
        column: ColumnConfig::Simple(4),
    }
}
```

### 响应式列数

响应式的描述列表。

```rust
let responsive = ResponsiveColumn {
    default: 3,
    xs: Some(1),
    sm: Some(2),
    md: Some(3),
    lg: Some(3),
    xl: Some(4),
    xxl: Some(4),
};

rsx! {
    Descriptions {
        items: items,
        column: ColumnConfig::Responsive(responsive),
    }
}
```

### 跨列展示

某些字段需要跨多列显示。

```rust
let items = vec![
    DescriptionsItem::new("name", rsx! { "姓名" }, rsx! { "李四" }),
    DescriptionsItem::new("phone", rsx! { "电话" }, rsx! { "139-0000-0000" }),
    DescriptionsItem::new("address", rsx! { "地址" }, rsx! { "上海市..." })
        .span(2),  // 跨两列
];
```

### 带标题和操作

带标题和额外操作的描述列表。

```rust
rsx! {
    Descriptions {
        items: items,
        title: Some(rsx! { Title { level: TitleLevel::H4, "用户信息" } }),
        extra: Some(rsx! {
            Space {
                Button { size: ButtonSize::Small, "编辑" }
                Button { size: ButtonSize::Small, "删除" }
            }
        }),
        bordered: true,
    }
}
```

### 不同尺寸

提供三种尺寸。

```rust
// Small
rsx! { Descriptions { items: items, size: Some(DescriptionsSize::Small) } }

// Middle (默认)
rsx! { Descriptions { items: items, size: Some(DescriptionsSize::Middle) } }

// Large
rsx! { Descriptions { items: items, size: Some(DescriptionsSize::Large) } }
```

## API

### Descriptions Props

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| items | 描述项数据 | `Vec<DescriptionsItem>` | - |
| title | 标题 | `Option<Element>` | - |
| extra | 额外内容 | `Option<Element>` | - |
| bordered | 是否显示边框 | `bool` | `false` |
| layout | 布局方向 | `DescriptionsLayout` | `Horizontal` |
| column | 列配置 | `ColumnConfig` | `Simple(3)` |
| size | 尺寸 | `Option<DescriptionsSize>` | `None` |
| colon | 是否显示冒号 | `bool` | `true` |
| class | 自定义类名 | `Option<String>` | - |
| style | 自定义样式 | `Option<String>` | - |

### DescriptionsItem

| 属性 | 说明 | 类型 | 默认值 |
| --- | --- | --- | --- |
| key | 唯一标识 | `String` | - |
| label | 标签文本 | `Element` | - |
| content | 内容 | `Element` | - |
| span | 跨列数 | `usize` | `1` |

### DescriptionsLayout

```rust
pub enum DescriptionsLayout {
    Horizontal,  // 默认，水平布局
    Vertical,    // 垂直布局
}
```

### ColumnConfig

```rust
pub enum ColumnConfig {
    Simple(usize),                    // 简单列数配置
    Responsive(ResponsiveColumn),     // 响应式列数配置
}
```

### ResponsiveColumn

```rust
pub struct ResponsiveColumn {
    pub default: usize,        // 默认列数
    pub xs: Option<usize>,     // < 576px
    pub sm: Option<usize>,     // ≥ 576px
    pub md: Option<usize>,     // ≥ 768px
    pub lg: Option<usize>,     // ≥ 992px
    pub xl: Option<usize>,     // ≥ 1200px
    pub xxl: Option<usize>,    // ≥ 1600px
}
```

### DescriptionsSize

```rust
pub enum DescriptionsSize {
    Small,
    Middle,  // 默认
    Large,
}
```

## 设计指引

- 描述列表常用于详情页展示结构化信息
- 带边框的样式更适合表格式展示
- 垂直布局适合标签较长的场景
- 可以根据屏幕宽度响应式调整列数
- 某些字段可以跨多列显示以适应内容长度

