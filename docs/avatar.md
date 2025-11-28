# Avatar：头像（MVP）

> 实现位置：`src/components/avatar.rs`
>
> 示例：`examples/avatar_demo.rs`

## 1. 设计目标

Avatar 用于展示用户或应用的头像，常出现在顶部导航、列表项或卡片中。当前实现提供基础的图片头像、文字头像和图标头像能力，并支持简单的 AvatarGroup 组合，不追求 AntD 的全部变体（如自适应尺寸、自动缩放文字等）。

---

## 2. AvatarShape / AvatarSize 与 AvatarProps

### 2.1 AvatarShape

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarShape {
    Circle,
    Square,
}
```

- `Circle`：圆形头像（默认）；
- `Square`：方形头像，圆角由主题 radius 控制。

### 2.2 AvatarSize

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    Small,
    Default,
    Large,
}
```

- `Small` / `Default` / `Large`：控制宽高与字体大小，对应 `.adui-avatar-sm` / `.adui-avatar-md` / `.adui-avatar-lg`。

### 2.3 AvatarProps

```rust
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    pub src: Option<String>,
    pub alt: Option<String>,
    pub shape: Option<AvatarShape>,
    pub size: Option<AvatarSize>,
    pub icon: Option<Element>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
}
```

字段说明：

- `src`：
  - 图片地址，存在时优先渲染 `img`；
- `alt`：
  - 图片的 alt 文本；
- `shape`：
  - 头像形状，默认 `Circle`；
- `size`：
  - 尺寸枚举，默认 `Default`；
- `icon`：
  - 当 `src` 为 `None` 时可提供图标 Element 作为头像内容；
- `class` / `style`：
  - 附加类名与内联样式；
- `children`：
  - 文本头像内容（通常是简短文字，如姓名首字母）。

当前渲染优先级：`src` > `icon` > `children`。

---

## 3. AvatarGroup

```rust
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Element,
}

#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element { .. }
```

- `AvatarGroup` 用于水平排列多个头像，形成用户列表或参与者列表；
- 当前实现为简单的 inline-flex 布局，并应用负 margin 形成轻微重叠效果。

---

## 4. 渲染结构与样式类

UI 结构示例：

```html
<span class="adui-avatar adui-avatar-circle adui-avatar-md">
  <img class="adui-avatar-img" src="..." alt="..." />
</span>
```

主要类名（`src/theme.rs` 的 `adui_avatar_style!`）：

- `.adui-avatar`：根容器，inline-flex 居中显示内容；
- `.adui-avatar-circle` / `.adui-avatar-square`：控制圆角；
- `.adui-avatar-sm` / `.adui-avatar-md` / `.adui-avatar-lg`：控制宽高与字体大小；
- `.adui-avatar-img`：图片填充整个容器，`object-fit: cover`；
- `.adui-avatar-icon` / `.adui-avatar-text`：用于图标或文字内容的内层容器；
- `.adui-avatar-group`：AvatarGroup 容器，水平排列头像；
- `.adui-avatar-group .adui-avatar`：在 group 中的头像添加轻微重叠与边框，以增强区分度。

---

## 5. 示例：图片头像、文字头像与 AvatarGroup

摘自 `examples/avatar_demo.rs`：

```rust
#[component]
fn AvatarDemoShell() -> Element {
    rsx! {
        // 图片头像
        Avatar { src: Some("https://via.placeholder.com/64".to_string()), }
        Avatar { src: Some("https://via.placeholder.com/64".to_string()), size: Some(AvatarSize::Large) }
        Avatar { src: Some("https://via.placeholder.com/40".to_string()), size: Some(AvatarSize::Small) }

        // 文字头像
        Avatar { children: Some(rsx!("AD")), }
        Avatar { shape: Some(AvatarShape::Square), children: Some(rsx!("U")), }

        // Group
        AvatarGroup {
            children: rsx! {
                Avatar { children: Some(rsx!("A")), }
                Avatar { children: Some(rsx!("B")), }
                Avatar { children: Some(rsx!("C")), }
            }
        }
    }
}
```

---

## 6. 与其他组件的协同

- 与 Layout/Header/Menu：
  - 将 Avatar 放在 Header 右上角，并结合 Dropdown/Menu 构建用户信息入口；
- 与 Badge：
  - 使用 Badge 包裹 Avatar，在头像右上角展示小红点或消息数量；
- 与 Card/List：
  - 在 Card 或 List 中展示用户头像与名称，构成用户列表或评论列表；

---

## 7. 与 Ant Design 的差异与后续规划

与 Ant Design 6.x 的 Avatar 相比，当前实现为裁剪版：

- 暂未支持：
  - 数值尺寸（直接指定像素大小）；
  - 自动计算文字缩放、src 错误回退到文字/图标的完整策略；
  - 带 tooltip、点击行为的封装等；
- AvatarGroup：
  - 当前仅支持简单水平排列和重叠效果，不支持最大显示数量与溢出计数。

后续扩展方向：

- 增加数值尺寸支持，使 Avatar 可适配更灵活的设计需求；
- 完善图片加载失败的回退逻辑（优先 icon，然后 children 文本）；
- 与 Dropdown/Menu/Badge 组合出“用户信息区”示例，方便在顶部导航中复用。
