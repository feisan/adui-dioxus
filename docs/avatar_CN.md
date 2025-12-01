# Avatar 头像

## 概述

Avatar 组件显示用户头像图片、图标或文本首字母。它支持图片、图标和文本内容，具有不同的形状和尺寸。

## API 参考

### AvatarProps

| 属性       | 类型                  | 默认值                     | 说明                               |
| ---------- | --------------------- | -------------------------- | ---------------------------------- |
| `src`      | `Option<String>`      | `None`                     | 图片源 URL                         |
| `alt`      | `Option<String>`      | `None`                     | 图片的替代文本                     |
| `shape`    | `Option<AvatarShape>` | `None`（默认为 `Circle`）  | 头像形状                           |
| `size`     | `Option<AvatarSize>`  | `None`（默认为 `Default`） | 尺寸变体                           |
| `icon`     | `Option<Element>`     | `None`                     | 无图片源时的可选图标内容           |
| `class`    | `Option<String>`      | `None`                     | 根元素的额外类名                   |
| `style`    | `Option<String>`      | `None`                     | 根元素的内联样式                   |
| `children` | `Option<Element>`     | `None`                     | 文本头像的文本内容（通常是首字母） |

### AvatarGroupProps

| 属性       | 类型             | 默认值 | 说明               |
| ---------- | ---------------- | ------ | ------------------ |
| `class`    | `Option<String>` | `None` | 组的额外类名       |
| `style`    | `Option<String>` | `None` | 组的内联样式       |
| `children` | `Element`        | -      | 组内的头像（必需） |

### AvatarShape

- `Circle` - 圆形头像（默认）
- `Square` - 方形头像

### AvatarSize

- `Small` - 小尺寸
- `Default` - 默认尺寸
- `Large` - 大尺寸

## 使用示例

### 图片头像

```rust
use adui_dioxus::Avatar;

rsx! {
    Avatar {
        src: Some("https://example.com/avatar.jpg".to_string()),
        alt: Some("用户头像".to_string()),
    }
}
```

### 图标头像

```rust
use adui_dioxus::{Avatar, Icon, IconKind};

rsx! {
    Avatar {
        icon: Some(rsx!(Icon { kind: IconKind::User, size: 20.0 })),
    }
}
```

### 文本头像

```rust
use adui_dioxus::Avatar;

rsx! {
    Avatar {
        children: Some(rsx!("张三")),
    }
}
```

### 方形头像

```rust
use adui_dioxus::{Avatar, AvatarShape};

rsx! {
    Avatar {
        shape: Some(AvatarShape::Square),
        children: Some(rsx!("AB")),
    }
}
```

### 头像组

```rust
use adui_dioxus::{Avatar, AvatarGroup};

rsx! {
    AvatarGroup {
        Avatar { children: Some(rsx!("A")) }
        Avatar { children: Some(rsx!("B")) }
        Avatar { children: Some(rsx!("C")) }
    }
}
```

## 使用场景

- **用户资料**：显示用户头像
- **用户列表**：在用户列表中显示头像
- **评论**：显示评论者头像
- **团队**：显示团队成员头像
- **状态指示器**：与徽章结合显示状态

## 与 Ant Design 6.0.0 的差异

- ✅ 支持图片、图标和文本头像
- ✅ 圆形和方形形状
- ✅ 尺寸变体
- ✅ 头像组
- ⚠️ 图片错误回退未显式处理（浏览器默认）
- ⚠️ 某些高级样式选项可能有所不同

