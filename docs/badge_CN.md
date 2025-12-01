# Badge 徽章数

## 概述

Badge 组件显示徽章或通知指示器，通常用于在按钮、头像或菜单项等其他组件上显示计数、状态或通知。

## API 参考

### BadgeProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `count` | `Option<Element>` | `None` | 徽章中显示的数字或自定义元素 |
| `count_number` | `Option<u32>` | `None` | 数字计数（用于向后兼容） |
| `overflow_count` | `u32` | `99` | 显示 "overflow+" 前的最大计数 |
| `dot` | `bool` | `false` | 是否显示无数字的红点 |
| `show_zero` | `bool` | `false` | 计数为零时是否显示徽章 |
| `status` | `Option<BadgeStatus>` | `None` | 可选语义状态 |
| `color` | `Option<BadgeColor>` | `None` | 徽章颜色（预设或自定义） |
| `text` | `Option<String>` | `None` | 状态指示器旁边显示的文本 |
| `size` | `BadgeSize` | `BadgeSize::Default` | 徽章尺寸 |
| `offset` | `Option<(f32, f32)>` | `None` | 徽章位置的偏移量 [x, y] |
| `title` | `Option<String>` | `None` | 徽章的标题属性（工具提示） |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |
| `children` | `Option<Element>` | `None` | 要显示徽章的包装元素 |

### BadgeStatus

- `Default` - 默认状态
- `Success` - 成功状态（绿色）
- `Warning` - 警告状态（橙色）
- `Error` - 错误状态（红色）

### BadgeColor

- `Preset(String)` - 预设颜色名称
- `Custom(String)` - 自定义颜色（hex、rgb 等）

### BadgeSize

- `Default` - 默认尺寸
- `Small` - 小尺寸

## 使用示例

### 带计数的基础徽章

```rust
use adui_dioxus::{Badge, Button, ButtonType};

rsx! {
    Badge {
        count_number: Some(5),
        children: Some(rsx! {
            Button {
                r#type: ButtonType::Primary,
                "消息"
            }
        }),
    }
}
```

### 点状徽章

```rust
use adui_dioxus::{Badge, Avatar};

rsx! {
    Badge {
        dot: true,
        children: Some(rsx! {
            Avatar { children: Some(rsx!("U")) }
        }),
    }
}
```

### 状态徽章

```rust
use adui_dioxus::{Badge, BadgeStatus};

rsx! {
    Badge {
        status: Some(BadgeStatus::Success),
        text: Some("在线".to_string()),
    }
}
```

### 自定义颜色徽章

```rust
use adui_dioxus::{Badge, BadgeColor};

rsx! {
    Badge {
        count_number: Some(99),
        color: Some(BadgeColor::Custom("#ff4d4f".to_string())),
        children: Some(rsx!(div { "通知" })),
    }
}
```

## 使用场景

- **通知**：显示未读消息计数
- **状态指示器**：显示在线/离线状态
- **计数器**：在按钮或图标上显示项目计数
- **提醒**：指示新项目或更新

## 与 Ant Design 6.0.0 的差异

- ✅ 带溢出处理的计数徽章
- ✅ 点状徽章
- ✅ 状态徽章
- ✅ 自定义颜色
- ✅ 偏移定位
- ⚠️ 某些高级样式选项可能有所不同

