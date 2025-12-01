# Descriptions 描述列表

## 概述

Descriptions 组件以结构化格式显示键值对。通常用于显示对象或实体的详细信息。

## API 参考

### DescriptionsProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `items` | `Vec<DescriptionsItem>` | - | 要显示的描述项（必需） |
| `title` | `Option<Element>` | `None` | 描述的可选标题 |
| `extra` | `Option<Element>` | `None` | 头部中的可选额外内容 |
| `bordered` | `bool` | `false` | 是否显示边框 |
| `layout` | `DescriptionsLayout` | `DescriptionsLayout::Horizontal` | 布局方向 |
| `column` | `ColumnConfig` | `ColumnConfig::Simple(3)` | 列配置 |
| `size` | `Option<DescriptionsSize>` | `None` | 尺寸变体 |
| `colon` | `bool` | `true` | 标签后是否显示冒号 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `style` | `Option<String>` | `None` | 内联样式 |

### DescriptionsItem

| 字段 | 类型 | 说明 |
|------|------|------|
| `key` | `String` | 项的唯一键 |
| `label` | `Element` | 标签元素 |
| `content` | `Element` | 内容元素 |
| `span` | `usize` | 此项跨越的列数 |

### DescriptionsLayout

- `Horizontal` - 水平布局（默认）
- `Vertical` - 垂直布局

### DescriptionsSize

- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

### ColumnConfig

- `Simple(usize)` - 简单列数
- `Responsive(ResponsiveColumn)` - 响应式列配置

## 使用示例

### 基础描述列表

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        items: vec![
            DescriptionsItem::new("name", rsx!("姓名"), rsx!("张三")),
            DescriptionsItem::new("email", rsx!("邮箱"), rsx!("zhang@example.com")),
            DescriptionsItem::new("age", rsx!("年龄"), rsx!("30")),
        ],
    }
}
```

### 带边框描述列表

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        bordered: true,
        items: vec![
            DescriptionsItem::new("name", rsx!("姓名"), rsx!("张三")),
            DescriptionsItem::new("email", rsx!("邮箱"), rsx!("zhang@example.com")),
        ],
    }
}
```

### 带标题

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        title: Some(rsx!("用户信息")),
        items: vec![
            DescriptionsItem::new("name", rsx!("姓名"), rsx!("张三")),
            DescriptionsItem::new("email", rsx!("邮箱"), rsx!("zhang@example.com")),
        ],
    }
}
```

### 自定义列跨度

```rust
use adui_dioxus::{Descriptions, DescriptionsItem};

rsx! {
    Descriptions {
        items: vec![
            DescriptionsItem::new("name", rsx!("姓名"), rsx!("张三")).span(2),
            DescriptionsItem::new("email", rsx!("邮箱"), rsx!("zhang@example.com")),
        ],
    }
}
```

## 使用场景

- **详情视图**：显示对象的详细信息
- **用户资料**：显示用户资料信息
- **产品详情**：显示产品规格
- **表单审查**：提交前审查表单数据

## 与 Ant Design 6.0.0 的差异

- ✅ 基础描述列表功能
- ✅ 带边框和无边框模式
- ✅ 水平和垂直布局
- ✅ 响应式列
- ✅ 自定义列跨度
- ⚠️ 某些高级功能可能有所不同

