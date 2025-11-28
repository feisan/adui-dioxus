# Menu：导航菜单（MVP）

> 实现位置：`src/components/menu.rs`
>
> 示例：`examples/menu_demo.rs`

## 1. 设计目标

Menu 是中后台应用中最常见的导航组件之一，通常嵌在 Layout 的 Sider 或 Header 中，用来承载站点信息结构和当前选中态。

本版 Menu 采用 **MVP 子集**，重点覆盖：

- Sider 场景下的 `inline` 菜单（左侧导航）；
- Header 场景下的 `horizontal` 菜单（顶部导航）；
- 受控/非受控选中态（`selected_keys` / `default_selected_keys`）；
- 简化版 SubMenu 展开/折叠（仅 `inline` 模式）。

暂不追求 AntD 中所有能力（如多级嵌套、分组、折叠收起时复杂图标布局、内置滚动/溢出处理等），这些会在后续计划中逐步扩展。

---

## 2. 数据结构：MenuItemNode

> 类型定义：`src/components/menu.rs`

Menu 的数据源通过一个简单的树结构 `MenuItemNode` 描述：

```rust
#[derive(Clone, PartialEq)]
pub struct MenuItemNode {
    pub id: String,
    pub label: String,
    pub icon: Option<Element>,
    pub disabled: bool,
    pub children: Option<Vec<MenuItemNode>>,
}

impl MenuItemNode {
    pub fn leaf(id: impl Into<String>, label: impl Into<String>) -> Self { .. }
}
```

说明：

- `id`: 节点的唯一标识，MVP 中也作为「选中 key」使用；
- `label`: 展示文案；
- `icon`: 可选图标节点，通常传入 `Icon` 或自定义 `Element`；
- `disabled`: 是否禁用该菜单项；
- `children`: 子菜单列表（MVP 建议最多两级嵌套）。

> 注意：出于与 Dioxus `key` 属性的兼容性，这里字段名使用 `id` 而不是 `key`。

---

## 3. MenuProps：组件属性

> 类型定义：`src/components/menu.rs:36` 起

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MenuProps {
    pub items: Vec<MenuItemNode>,
    #[props(default)]
    pub mode: MenuMode,
    #[props(optional)]
    pub selected_keys: Option<Vec<String>>,
    #[props(optional)]
    pub default_selected_keys: Option<Vec<String>>,
    #[props(optional)]
    pub open_keys: Option<Vec<String>>,
    #[props(optional)]
    pub default_open_keys: Option<Vec<String>>,
    #[props(optional)]
    pub on_select: Option<EventHandler<String>>,
    #[props(optional)]
    pub on_open_change: Option<EventHandler<Vec<String>>>,
    #[props(default)]
    pub inline_collapsed: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
}
```

- `items`: 菜单项列表，使用 `MenuItemNode` 描述树结构；
- `mode: MenuMode`：菜单模式：
  - `Inline`: 适用于 Sider（左侧导航）；
  - `Horizontal`: 适用于 Header（顶部导航）。
- `selected_keys` / `default_selected_keys`：
  - `selected_keys` 存在时，Menu 处于 **受控模式**，组件仅根据该字段渲染选中态；
  - 否则使用内部 `Signal<Vec<String>>` 存储选中 key，初始值来自 `default_selected_keys`；
  - MVP 中选中策略 **简化为单选**：内部总是将选中 key 设为长度为 1 的向量。
- `open_keys` / `default_open_keys`：
  - 仅在 `Inline` 模式下使用，用于控制/记录 SubMenu 展开状态；
  - `open_keys` 存在时为受控模式，否则内部使用 `Signal<Vec<String>>` 管理。
- `on_select`: 当用户点击叶子菜单项时触发，参数为该项 `id`；
- `on_open_change`: 当 `Inline` 模式下 SubMenu 展开/收起时触发，参数为当前所有展开 `id` 的集合；
- `inline_collapsed`: 与 Layout.Sider 的折叠状态配合使用，MVP 中主要影响样式与 SubMenu 显隐；
- `class` / `style`: 作用在最外层 `nav` 上的附加类名与内联样式。

---

## 4. 行为细节

### 4.1 选中逻辑

- 非受控模式：
  - 初始选中：来自 `default_selected_keys`（为 `None` 时表示无选中项）；
  - 点击叶子菜单项：
    - 将内部 `selected_internal` 更新为 `vec![id.clone()]`；
    - 若存在 `on_select`，在更新后调用 `on_select(id)`。
- 受控模式：
  - Menu 不会修改内部 `selected_internal`，选中态完全由 `selected_keys` 决定；
  - 点击叶子菜单项时，仅调用 `on_select(id)`，由上层根据回调更新 `selected_keys`。

### 4.2 SubMenu 展开/折叠（Inline 模式）

- SubMenu 节点：`children` 非空的 `MenuItemNode`；
- 点击 SubMenu 标题时：
  - 非受控：
    - 若当前 `open_keys` 中含有该 `id`，则移除；否则追加；
    - 更新内部 `open_internal`；
  - 受控：
    - 不修改内部状态，只构造新的 `Vec<String>` 并通过 `on_open_change(next)` 通知上层；
  - 无论受控/非受控，若存在 `on_open_change`，都会在计算出 `next` 后调用一次。
- `inline_collapsed = true` 时，SubMenu 下的 `ul` 会被整体隐藏，仅展示顶层 item。

### 4.3 水平菜单（Horizontal 模式）

- 当前实现中，`Horizontal` 模式仅处理「一层菜单」：
  - 顶层 `MenuItemNode` 均视为叶子项；
  - `children` 字段不会被渲染；
  - 点击行为等同于点击叶子菜单项：更新选中态并触发 `on_select`。

---

## 5. 样式与类名

> 样式定义：`src/theme.rs` 中 `adui_layout_style!` 内的 `.adui-menu-*` 段

主要类名：

- 根容器：
  - `.adui-menu`：基本字体、颜色；
  - `.adui-menu-inline`：内联纵向菜单；
  - `.adui-menu-horizontal`：横向菜单（Header）。
- 列表：
  - `.adui-menu-list`：最外层菜单列表；
  - `.adui-menu-submenu-list`：SubMenu 下的子列表。
- 项：
  - `.adui-menu-item`：普通菜单项；
  - `.adui-menu-submenu`：有子菜单的项；
  - `.adui-menu-submenu-item`：SubMenu 内部的子项；
  - `.adui-menu-item-selected`：当前选中项；
  - `.adui-menu-item-disabled`：禁用项。
- 结构：
  - `.adui-menu-item-title`：承载图标和文本的容器；
  - `.adui-menu-item-icon`：图标区域；
  - `.adui-menu-item-label`：文本区域（带 `ellipsis` 效果）。
- 折叠：
  - `.adui-menu-inline-collapsed .adui-menu-submenu-list`：折叠状态下隐藏子菜单。

整体视觉风格与 Layout.Sider/Header 保持一致，使用 ThemeTokens 中的颜色和圆角。

---

## 6. 示例：Sider + Header 导航

摘自 `examples/menu_demo.rs`，展示 Sider 内 `inline` 菜单和 Header 内 `horizontal` 菜单的组合使用：

```rust
fn sider_items() -> Vec<MenuItemNode> {
    vec![
        MenuItemNode::leaf("dashboard", "仪表盘"),
        MenuItemNode {
            id: "list".into(),
            label: "列表".into(),
            icon: None,
            disabled: false,
            children: Some(vec![
                MenuItemNode::leaf("list-basic", "基础列表"),
                MenuItemNode::leaf("list-advanced", "高级列表"),
            ]),
        },
        MenuItemNode::leaf("settings", "设置"),
    ]
}

#[component]
fn MenuDemoShell() -> Element {
    let mut sider_selected = use_signal(|| vec!["dashboard".to_string()]);
    let mut header_selected = use_signal(|| vec!["home".to_string()]);

    rsx! {
        Layout {
            has_sider: Some(true),
            class: None,
            style: None,
            Sider {
                width: Some(220.0),
                // ... 省略 Sider 其他 props
                Menu {
                    mode: MenuMode::Inline,
                    items: sider_items(),
                    selected_keys: Some(sider_selected.read().clone()),
                    on_select: move |key: String| {
                        sider_selected.set(vec![key]);
                    },
                }
            }
            adui_dioxus::Content {
                class: None,
                style: None,
                has_sider: None,
                children: rsx! {
                    adui_dioxus::Header {
                        class: None,
                        style: None,
                        has_sider: None,
                        children: rsx! {
                            div {
                                style: "display: flex; align-items: center; justify-content: space-between;",
                                span { style: "font-weight: 600;", "顶部菜单示例" }
                                Menu {
                                    mode: MenuMode::Horizontal,
                                    items: header_items(),
                                    selected_keys: Some(header_selected.read().clone()),
                                    on_select: move |key: String| {
                                        header_selected.set(vec![key]);
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

## 7. 综合示例与组合建议

- `layout_navigation_demo`：展示 Menu 与 Layout/Sider/Header/Breadcrumb/Pagination 的基础组合，用于典型列表页导航。
- `dashboard_demo`：在上述基础上进一步引入 Tabs + Card + Tag + Badge + Avatar + List/Pagination，构成完整的仪表盘/用户中心布局，适合作为中后台首页或概览页模板。

---

## 8. 与 Ant Design 的差异

与 AntD 6.x 的 `Menu` 相比，当前实现的差异主要在于：

- 功能裁剪：
  - 仅支持两级结构（顶层 + 一层 SubMenu），不支持多层嵌套、ItemGroup/Divider 等高级结构；
  - 不支持 `multiple` 选中模式，MVP 仅实现单选；
  - 不支持 `inlineIndent` 自定义缩进等外观细节；
  - 不支持溢出/滚动的复杂处理（大量菜单项时需要外部容器配合）。
- API 裁剪：
  - 未暴露 classNames/styles 等语义化定制入口（当前仅支持 `class` / `style`）；
  - 不支持 rc-menu 的全部交互（如键盘导航、焦点管理）——后续可按需要逐步补齐。

> 建议：在需要更复杂导航结构或与路由深度集成时，可以在当前 Menu 之上封装一层「路由感知菜单」，将路由树映射为 `MenuItemNode` 列表，并在文档中记录映射策略。
