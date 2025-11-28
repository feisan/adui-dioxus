# Tabs：页签导航（MVP）

> 实现位置：`src/components/tabs.rs`
>
> 示例：`examples/tabs_demo.rs`

## 1. 设计目标

Tabs 用于在单个页面中切换不同的视图区域，例如「基础信息 / 安全设置 / 通知偏好」等。当前实现为基础版，仅支持顶部线型样式与简单的 items 模式，不包含可编辑卡片、多行滚动、额外 more 菜单等高级功能。

---

## 2. 数据模型与 Props

### 2.1 TabItem

```rust
#[derive(Clone, PartialEq)]
pub struct TabItem {
    pub key: String,
    pub label: String,
    pub disabled: bool,
    pub content: Option<Element>,
}

impl TabItem {
    pub fn new(key: impl Into<String>, label: impl Into<String>, content: Option<Element>) -> Self { .. }
    pub fn disabled(key: impl Into<String>, label: impl Into<String>) -> Self { .. }
}
```

- `key`：唯一标识，Tabs 的受控/非受控状态都依赖该字段；
- `label`：页签标题，展示在页签导航区域；
- `disabled`：是否禁用当前页签，禁用时不会响应点击；
- `content`：当前页签对应的内容区域；若为 `None`，则 Tabs 只渲染导航，内容由外部自行组织。

### 2.2 TabsProps

```rust
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    pub items: Vec<TabItem>,
    pub active_key: Option<String>,
    pub default_active_key: Option<String>,
    pub on_change: Option<EventHandler<String>>,
    pub size: Option<ComponentSize>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

字段说明：

- `items`：页签列表，每个 `TabItem` 包含 `key`/`label`/`disabled`/`content`；
- `active_key`：
  - 存在时 Tabs 处于 **受控模式**，内部不会管理当前激活 key；
  - 由调用方配合 `on_change` 手动更新；
- `default_active_key`：
  - 仅在 `active_key` 为 `None` 时生效，用作非受控模式下的初始激活页签；
  - 未显式指定时，会使用 `items.first()` 的 key 作为初始值（若存在）；
- `on_change(key)`：
  - 当用户点击页签、激活 key 发生变化时触发；
  - 受控/非受控模式下都会触发此回调；
- `size`：
  - 控制 Tabs 导航高度与字体大小，复用 `ComponentSize`（`Small/Middle/Large`）；
  - 对应样式类：`.adui-tabs-sm` / `.adui-tabs-lg`；
- `class` / `style`：作用于根 `div` 的附加类名与内联样式。

---

## 3. 渲染结构与样式类

UI 结构（简化）：

```html
<div class="adui-tabs adui-tabs-sm/lg?">
  <div class="adui-tabs-nav">
    <div class="adui-tabs-nav-list">
      <button class="adui-tabs-tab adui-tabs-tab-active? adui-tabs-tab-disabled?">基础信息</button>
      <button class="adui-tabs-tab">安全设置</button>
      ...
    </div>
  </div>
  <div class="adui-tabs-content">
    <div class="adui-tabs-tabpane">当前激活页签的内容</div>
  </div>
</div>
```

主要类名（定义在 `src/theme.rs` 的 `adui_tabs_style!` 中）：

- `.adui-tabs`：Tabs 根容器；
- `.adui-tabs-nav`：页签导航区域，底部带一条分割线；
- `.adui-tabs-nav-list`：水平排列的页签按钮容器；
- `.adui-tabs-tab`：单个页签按钮；
- `.adui-tabs-tab-active`：当前激活页签，文字高亮并在底部渲染主色下划线；
- `.adui-tabs-tab-disabled`：禁用页签，文字置灰且不可点击；
- `.adui-tabs-sm` / `.adui-tabs-lg`：控制不同尺寸下的 padding 和字体大小；
- `.adui-tabs-content`：内容区域外层；
- `.adui-tabs-tabpane`：具体内容面板容器。

当前实现仅支持顶部线型样式，不支持 TabPosition/TabPlacement 的多方向布局，也未实现 more/overflow 等高级特性。

---

## 4. 示例：非受控与受控 Tabs

摘自 `examples/tabs_demo.rs`：

```rust
#[component]
fn TabsDemoShell() -> Element {
    let items = vec![
        TabItem::new(
            "basic",
            "基础信息",
            Some(rsx!(div { "这里是基础信息内容" })),
        ),
        TabItem::new(
            "security",
            "安全设置",
            Some(rsx!(div { "这里是安全设置内容" })),
        ),
        TabItem::new(
            "notification",
            "通知偏好",
            Some(rsx!(div { "这里是通知偏好内容" })),
        ),
    ];

    let mut active = use_signal(|| "basic".to_string());

    rsx! {
        // 非受控模式：不传 active_key，Tabs 自行管理当前激活 key
        Tabs { items: items.clone() }

        // 受控模式：由外部 Signal 管理 active_key
        Tabs {
            items: items,
            active_key: Some((*active.read()).clone()),
            on_change: move |key: String| {
                active.set(key);
            },
        }
    }
}
```

---

## 5. 与其他组件的协同

- 与 Layout / Breadcrumb：
  - 建议将 Tabs 放在页面内容区顶部，用于在同一路由下切换不同子视图；
  - Breadcrumb 负责页面级层级导航，Tabs 负责页面内部视图切换；
- 与 Card：
  - 常见场景是 Tabs + Card 组合，在每个 TabPane 中放置一组 Card/表单/列表；
- 与 List / Table / Form：
  - 不同 Tab 中渲染不同的数据视图或表单区域，例如「基本信息表单」/「安全日志表格」等；
- 与 ConfigProvider：
  - `size` 受 `ComponentSize` 控制，建议与 Button/Input 保持一致，统一页面视觉密度。

---

## 6. 与 Ant Design 的差异与后续规划

与 Ant Design 6.x 的 Tabs 相比，当前实现为裁剪版：

- 暂未支持：
  - `type="card"`/`type="editable-card"` 卡片式/可编辑页签；
  - `tabPosition` / `tabPlacement` 多方向布局（右侧/底部等）；
  - more/overflow Tabs 下拉、拖拽排序等高级交互；
  - 自定义 `renderTabBar` / `items` 中的复杂结构；
- 受控模式：
  - 当前仅提供简单的 `active_key` + `on_change`，未直接暴露动画/过渡配置；

后续扩展方向：

- 按需支持 `type="card"`，与 Card 视觉风格进一步统一；
- 引入简单的 overflow 处理（Tab 较多时折叠为下拉）；
- 与路由集成的示例（Tab 与 URL 同步），以及与 DataView/List/Table 的更丰富组合 demo。
