# Card 卡片

## 概述

Card 组件是以卡片格式显示内容的容器。它支持标题、额外内容、加载状态和悬停效果。

## API 参考

### CardProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | `Option<Element>` | `None` | 在头部渲染的可选卡片标题 |
| `extra` | `Option<Element>` | `None` | 在头部右侧区域渲染的可选额外内容 |
| `bordered` | `bool` | `true` | 是否显示卡片边框 |
| `size` | `Option<ComponentSize>` | `None` | 卡片内边距和字体的视觉密度 |
| `loading` | `bool` | `false` | 加载状态。为 true 时，主体渲染骨架屏而非子元素 |
| `hoverable` | `bool` | `false` | 卡片是否应有悬停效果 |
| `class` | `Option<String>` | `None` | 根元素的额外类名 |
| `style` | `Option<String>` | `None` | 根元素的内联样式 |
| `children` | `Element` | - | 卡片主体内容（必需） |

### ComponentSize

- `Small` - 小尺寸
- `Middle` - 中等尺寸（默认）
- `Large` - 大尺寸

## 使用示例

### 基础用法

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        "卡片内容在这里"
    }
}
```

### 带标题

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        title: Some(rsx!("卡片标题")),
        "卡片内容"
    }
}
```

### 带标题和额外内容

```rust
use adui_dioxus::{Card, Button, ButtonType};

rsx! {
    Card {
        title: Some(rsx!("卡片标题")),
        extra: Some(rsx! {
            Button {
                r#type: ButtonType::Link,
                "更多"
            }
        }),
        "卡片内容"
    }
}
```

### 加载状态

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        title: Some(rsx!("加载中的卡片")),
        loading: true,
        "此内容将被骨架屏替换"
    }
}
```

### 可悬停卡片

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        hoverable: true,
        title: Some(rsx!("可悬停卡片")),
        "悬停在此卡片上查看效果"
    }
}
```

### 无边框

```rust
use adui_dioxus::Card;

rsx! {
    Card {
        bordered: false,
        "无边框卡片"
    }
}
```

## 使用场景

- **内容容器**：以卡片格式显示内容
- **仪表板卡片**：显示统计或信息卡片
- **产品卡片**：显示产品或项目
- **设置面板**：在卡片布局中组织设置
- **加载状态**：在卡片中显示骨架屏加载

## 与 Ant Design 6.0.0 的差异

- ✅ 支持所有基础卡片功能
- ✅ 标题和额外内容
- ✅ 带骨架屏的加载状态
- ✅ 悬停效果
- ✅ 尺寸变体
- ⚠️ 卡片操作（页脚按钮）尚未实现
- ⚠️ 某些高级样式选项可能有所不同

