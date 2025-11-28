# Breadcrumb：面包屑导航（MVP）

> 实现位置：`src/components/breadcrumb.rs`
>
> 示例：`examples/breadcrumb_demo.rs`

## 1. 设计目标

Breadcrumb 用于展示当前页面在站点层级结构中的位置，通常配合导航菜单与路由使用。

本版 Breadcrumb 采用 **显式 items 模型**：调用方通过一个有序列表描述路径，而 Breadcrumb 自身不感知路由系统，也不持久化状态。

MVP 目标：

- 支持通过 `items: Vec<BreadcrumbItem>` 描述简单路径（如「首页 / 列表 / 详情」）；
- 支持自定义分隔符 `separator`（默认 `/`）；
- 支持点击非末尾节点触发回调（用于路由跳转或状态更新）；
- 最后一个节点视为当前页面，始终以纯文本展示。

---

## 2. 数据结构：BreadcrumbItem

> 类型定义：`src/components/breadcrumb.rs`

```rust
#[derive(Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub id: String,
    pub label: String,
    pub href: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self { .. }

    pub fn with_href(id: impl Into<String>, label: impl Into<String>, href: impl Into<String>) -> Self { .. }
}
```

字段说明：

- `id`: 唯一标识，用于 `on_item_click` 回调中区分节点；
- `label`: 展示文案；
- `href`: 可选链接地址，非末尾节点且有 `href` 时会渲染为 `<a>`。

> 注意：Breadcrumb 本身并不负责路由跳转，它只提供点击事件，实际跳转由调用方处理。

---

## 3. BreadcrumbProps：组件属性

> 类型定义：`src/components/breadcrumb.rs:24` 起

```rust
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    pub items: Vec<BreadcrumbItem>,
    #[props(optional)]
    pub separator: Option<String>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(optional)]
    pub on_item_click: Option<EventHandler<String>>,
}
```

- `items`: 由左到右的节点序列；
- `separator`: 分隔符字符串，默认为 `/`；
- `class` / `style`: 应用于根 `nav` 的附加类名与样式；
- `on_item_click`：当用户点击非末尾节点时触发，参数为该节点 `id`。

> 最后一个节点视为当前页面：即使 `href` 存在，也不会渲染为链接，也不会触发 `on_item_click`。

---

## 4. 行为与渲染规则

渲染结构（简化）：

```html
<nav class="adui-breadcrumb" role="navigation" aria-label="Breadcrumb">
  <ol class="adui-breadcrumb-list">
    <li class="adui-breadcrumb-item">
      <a class="adui-breadcrumb-link">首页</a>
      <span class="adui-breadcrumb-separator"> / </span>
    </li>
    <li class="adui-breadcrumb-item">
      <a class="adui-breadcrumb-link">列表</a>
      <span class="adui-breadcrumb-separator"> / </span>
    </li>
    <li class="adui-breadcrumb-item">
      <span class="adui-breadcrumb-text adui-breadcrumb-text-current">详情</span>
    </li>
  </ol>
</nav>
```

逻辑规则：

- 若 `items` 为空，则不渲染任何内容；
- 对 `items` 按顺序渲染：
  - 非末尾节点：
    - 若 `href` 存在：渲染 `<a>`，点击时：
      - 保留默认链接行为（浏览器处理 `href`）；
      - 同时如有 `on_item_click`，调用 `on_item_click(id)`；
    - 若 `href` 为 `None`：渲染 `<span>`，点击时仅触发 `on_item_click(id)`；
    - 在节点后方渲染分隔符 `<span class="adui-breadcrumb-separator">{separator}</span>`；
  - 末尾节点：渲染 `<span class="adui-breadcrumb-text adui-breadcrumb-text-current">`，不添加分隔符，不触发点击回调。

---

## 5. 样式与类名

> 样式定义：`src/theme.rs` 中 `adui_layout_style!` 内的 `.adui-breadcrumb-*` 片段

主要类名：

- `.adui-breadcrumb`: 根容器，控制字体大小与基础颜色；
- `.adui-breadcrumb-list`: `ol` 列表，行内布局；
- `.adui-breadcrumb-item`: 单个节点容器；
- `.adui-breadcrumb-link`: 链接，使用主题 link 色，hover 下显示下划线；
- `.adui-breadcrumb-text`: 普通文本节点；
- `.adui-breadcrumb-text-current`: 当前节点，高亮字体颜色、加粗；
- `.adui-breadcrumb-separator`: 分隔符节点，使用次要文字颜色。

默认视觉：

- 文本使用 `--adui-color-text-secondary`，当前节点使用 `--adui-color-text`；
- 分隔符左右各有一个小间距；
- 不对布局做过多约束，方便与 Layout/Header 等组合使用。

---

## 6. 示例：基础用法

摘自 `examples/breadcrumb_demo.rs`：

```rust
fn demo_items() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::with_href("home", "首页", "/"),
        BreadcrumbItem::with_href("list", "列表", "/list"),
        BreadcrumbItem::new("detail", "详情"),
    ]
}

#[component]
fn BreadcrumbDemoShell() -> Element {
    let api = use_message();

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Breadcrumb demo" }
            p { "展示一个典型路径：首页 / 列表 / 详情，点击前两级会触发 message 提示。" }

            Breadcrumb {
                items: demo_items(),
                separator: Some(" > ".to_string()),
                on_item_click: move |id: String| {
                    if let Some(msg) = api.clone() {
                        msg.info(format!("点击了节点: {id}"));
                    }
                },
            }
        }
    }
}
```

---

## 7. 与 Ant Design 的差异

与 Ant Design 6.x 的 Breadcrumb 相比，当前实现的主要差异：

- 暂不支持：
  - `routes` / `itemRender` 等高阶 API；
  - 与 Dropdown 结合的面包屑下拉菜单；
  - 通过 `path` 自动拼接路由路径的行为。
- 能力范围：
  - 仅支持简单的 `items: Vec<BreadcrumbItem>`；
  - `separator` 为简单字符串（不支持 ReactNode 分隔符）；
  - 点击行为统一通过 `on_item_click(id)` 暴露，由上层处理路由或业务逻辑。

> 后续如需要，可以在 this 组件基础上封装一层“路由感知 Breadcrumb”，负责从路由配置中生成 `BreadcrumbItem` 列表，并在 `on_item_click` 中调用路由跳转。