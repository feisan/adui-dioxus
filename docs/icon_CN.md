# Icon 图标

## 概述

Icon 组件提供了一组内置的 SVG 图标，常用于 UI 界面。它支持旋转、尺寸调整、自定义颜色和旋转动画。

## API 参考

### IconProps

| 属性 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `kind` | `IconKind` | `IconKind::Info` | 内置图标类型 |
| `size` | `f32` | `20.0` | 图标尺寸（像素） |
| `color` | `Option<String>` | `None` | 自定义颜色（默认为 "currentColor"） |
| `rotate` | `Option<f32>` | `None` | 旋转角度（度） |
| `spin` | `bool` | `false` | 是否显示旋转动画 |
| `class` | `Option<String>` | `None` | 额外类名 |
| `aria_label` | `Option<String>` | `None` | 无障碍标签 |
| `view_box` | `Option<String>` | `None` | 自定义 SVG viewBox |
| `custom` | `Option<Element>` | `None` | 自定义 SVG 内容（覆盖 `kind`） |

### IconKind

内置图标类型：
- `Plus` - 加号图标
- `Minus` - 减号图标
- `Check` - 对勾
- `Close` - 关闭/X 图标
- `Info` - 信息图标（默认）
- `Question` - 问号
- `ArrowRight` - 右箭头
- `ArrowLeft` - 左箭头
- `ArrowUp` - 上箭头
- `ArrowDown` - 下箭头
- `Search` - 搜索图标
- `Copy` - 复制图标
- `Edit` - 编辑图标
- `Loading` - 加载旋转器（自动旋转）
- `Eye` - 眼睛图标
- `EyeInvisible` - 带斜线的眼睛图标

## 使用示例

### 基础用法

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Search,
    }
}
```

### 自定义尺寸和颜色

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Check,
        size: 24.0,
        color: Some("#52c41a".to_string()),
    }
}
```

### 旋转图标

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::Loading,
        spin: true,
    }
}
```

### 旋转角度

```rust
use adui_dioxus::{Icon, IconKind};

rsx! {
    Icon {
        kind: IconKind::ArrowRight,
        rotate: Some(45.0),
    }
}
```

### 自定义 SVG 图标

```rust
use adui_dioxus::Icon;

rsx! {
    Icon {
        custom: Some(rsx! {
            path { d: "M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z" }
        }),
        size: 24.0,
    }
}
```

## 使用场景

- **UI 指示器**：显示状态、操作或信息
- **按钮**：在按钮中添加图标以改善用户体验
- **导航**：使用箭头图标进行导航
- **加载状态**：使用加载旋转器图标
- **表单控件**：输入框、复选框等的图标

## 与 Ant Design 6.0.0 的差异

- ✅ 内置常用图标集
- ✅ 自定义 SVG 支持
- ✅ 旋转和旋转动画
- ✅ 尺寸和颜色自定义
- ⚠️ 内置图标集有限，相比 Ant Design 的丰富图标库较少
- ⚠️ 图标字体支持未实现（仅 SVG）

