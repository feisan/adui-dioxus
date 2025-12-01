# Affix 固钉

## 概述

Affix 组件包装内容，并在滚动超过阈值时将其固定到视口。通常用于粘性导航、工具栏或操作按钮。

## API 参考

### AffixProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `offset_top` | `Option<f64>` | `None` | 开始固定时距离顶部的偏移量（像素，如果未设置任何偏移量则默认为 0） |
| `offset_bottom` | `Option<f64>` | `None` | 开始固定时距离底部的偏移量（像素） |
| `on_change` | `Option<EventHandler<bool>>` | `None` | 固定状态改变时触发的回调 |
| `class` | `Option<String>` | `None` | 包装器的额外 CSS 类 |
| `style` | `Option<String>` | `None` | 包装器的额外内联样式 |
| `children` | `Element` | - | 要固定的内容（必需） |

## 使用示例

### 基础固钉

```rust
use adui_dioxus::Affix;

rsx! {
    Affix {
        offset_top: Some(10.0),
        div {
            "滚动时此内容将固定在顶部"
        }
    }
}
```

### 固定到底部

```rust
use adui_dioxus::Affix;

rsx! {
    Affix {
        offset_bottom: Some(20.0),
        Button {
            r#type: ButtonType::Primary,
            "粘性按钮"
        }
    }
}
```

### 带改变处理器

```rust
use adui_dioxus::Affix;
use dioxus::prelude::*;

let is_affixed = use_signal(|| false);

rsx! {
    Affix {
        offset_top: Some(0.0),
        on_change: Some(move |affixed| {
            is_affixed.set(affixed);
        }),
        div {
            if *is_affixed.read() {
                "已固定！"
            } else {
                "未固定"
            }
        }
    }
}
```

## 使用场景

- **粘性导航**：创建粘性导航栏
- **操作按钮**：将操作按钮固定到视口
- **工具栏**：创建粘性工具栏
- **侧边栏**：滚动时固定侧边栏

## 与 Ant Design 6.0.0 的差异

- ✅ 顶部和底部固定
- ✅ 自定义偏移量
- ✅ 改变回调
- ⚠️ 容器目标尚未实现
- ⚠️ 某些高级功能可能有所不同

