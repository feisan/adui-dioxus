# List：列表视图（MVP）

> 实现位置：`src/components/list.rs`
>
> 示例：`examples/list_demo.rs`

## 1. 设计目标

List 用于展示一组结构相近的条目，是中后台中最常见的视图之一。

本版 List 采用 **“容器 + 子项”** 的 MVP 模式：

- 由调用方负责渲染每一项内容（使用 `.adui-list-item` 类名）；
- List 负责头尾区域、边框、加载态、空态和分页区域的布局与样式；
- 分页逻辑由外部控件（如状态 + Pagination）受控管理，List 只渲染分页器并将交互向外抛出。

---

## 2. ListProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct ListProps {
    pub header: Option<Element>,
    pub footer: Option<Element>,
    pub bordered: bool,
    pub size: Option<ComponentSize>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub loading: bool,
    pub is_empty: Option<bool>,
    pub empty: Option<Element>,
    pub pagination_total: Option<u32>,
    pub pagination_current: Option<u32>,
    pub pagination_page_size: Option<u32>,
    pub pagination_on_change: Option<EventHandler<(u32, u32)>>,
    pub children: Element,
}
```

字段说明：

- `header` / `footer`：
  - 头部和尾部区域的内容，一般为标题/说明和额外信息（例如“共 N 条”）；
- `bordered`：
  - 是否显示外边框，启用后根容器使用 `.adui-list-bordered` 风格；
- `size`：
  - 使用 `ComponentSize::Small/Middle/Large` 控制列表紧凑程度（对应 `.adui-list-sm` / 默认 / `.adui-list-lg` 类名）；
- `class` / `style`：
  - 作用于根 `div` 的附加类名与内联样式；
- `loading`：
  - 是否处于加载状态；
  - 为 true 时，List 会使用 `Spin` 包裹 body 区域，并显示“加载中...” 提示；
- `is_empty`：
  - 是否为空列表（应由调用方基于数据长度计算）；
  - 仅在 `loading = false` 时生效；
- `empty`：
  - 自定义空态内容；
  - 若为空且 `is_empty = Some(true)`，则使用内置 `Empty` 组件展示统一空态；
- `pagination_total` / `pagination_current` / `pagination_page_size`：
  - 分页配置，全部存在时会在列表下方渲染 Pagination；
  - `total`：数据总条数；
  - `current`：当前页（1 基）；
  - `page_size`：每页条数；
- `pagination_on_change`：
  - 当用户点击分页器时触发，参数为 `(page, page_size)`；
  - 调用方应在回调中更新外部状态（如 `Signal`），并重新计算 children 对应的片段；
- `children`：
  - 列表项内容；
  - 通常在内部渲染多个块元素，并为每一行添加 `.adui-list-item` 类名以获得默认行样式。

---

## 3. DOM 结构与样式类

典型结构：

```html
<div class="adui-list adui-list-bordered adui-list-sm? adui-list-lg?">
  <div class="adui-list-header">...</div>        <!-- 可选 -->
  <div class="adui-list-body">
    <div class="adui-list-items">
      <div class="adui-list-item">...</div>
      <div class="adui-list-item">...</div>
      ...
    </div>
  </div>
  <div class="adui-list-footer">...</div>       <!-- 可选 -->
  <div class="adui-list-pagination">...</div>   <!-- 有分页时 -->
</div>
```

主要类名：

- `.adui-list`：根容器；
- `.adui-list-bordered`：带边框样式；
- `.adui-list-sm` / `.adui-list-lg`：紧凑/宽松间距；
- `.adui-list-header` / `.adui-list-footer`：头尾区域；
- `.adui-list-body`：主体区域；
- `.adui-list-items`：列表项容器；
- `.adui-list-item`：单条列表项默认行样式；
- `.adui-list-empty`：空态容器，用于承载 Empty 或自定义空内容；
- `.adui-list-pagination`：分页器容器，通常右对齐。

> 样式定义集中在 `src/theme.rs` 的 `adui_list_style!` 片段，使用主题 tokens 控制背景、边框与间距。

---

## 4. 示例：基础列表 / 空列表 / 分页列表

摘自 `examples/list_demo.rs`：

```rust
const ITEMS: &[&str] = &[
    "列表项 1", "列表项 2", "列表项 3", "列表项 4", "列表项 5",
    "列表项 6", "列表项 7", "列表项 8", "列表项 9", "列表项 10",
];

#[component]
fn ListDemoShell() -> Element {
    let mut current_page = use_signal(|| 1u32);
    let page_size: u32 = 4;
    let total = ITEMS.len() as u32;

    let page = *current_page.read();
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(ITEMS.len());

    rsx! {
        // 基础无分页列表
        List {
            bordered: true,
            header: Some(rsx!("最近更新")),
            footer: Some(rsx!("共 3 条")),
            children: rsx! {
                div { class: "adui-list-item", "列表项 A" }
                div { class: "adui-list-item", "列表项 B" }
                div { class: "adui-list-item", "列表项 C" }
            }
        }

        // 空列表
        List {
            bordered: true,
            is_empty: Some(true),
            empty: Some(rsx!(Empty { description: Some("当前没有任何记录".to_string()) })),
            children: rsx! { }
        }

        // 分页列表
        List {
            bordered: true,
            header: Some(rsx!("分页示例")),
            is_empty: Some(start >= end),
            pagination_total: Some(total),
            pagination_current: Some(page),
            pagination_page_size: Some(page_size),
            pagination_on_change: {
                let mut page_sig = current_page;
                move |(next, _size)| page_sig.set(next)
            },
            children: rsx! {
                {(start..end).map(|idx| {
                    let text = ITEMS[idx];
                    rsx!(
                        div { class: "adui-list-item", "{text}" }
                    )
                })}
            }
        }
    }
}
```

---

## 5. 与 Ant Design 的差异

相较于 AntD 6.x 的 `List`，本版为裁剪版：

- 未实现：
  - `dataSource` / `renderItem` 泛型 API（当前通过 `children` 由调用方自行渲染）；
  - `grid` 模式（栅格卡片列表）；
  - `loadMore` / `infinite scroll` 等高级能力；
  - 复杂布局（如 `List.Item.Meta` 等封装组件）。
- 分页策略：
  - List 不持有内部数据，仅通过 `pagination_*` props 渲染 Pagination；
  - 数据切片逻辑由外部根据 `page` / `page_size` 自行计算；
- 空态与加载：
  - 空态通过 `is_empty` + `empty` 控制；
  - 加载态统一复用 `Spin`，骨架屏可以结合 `Skeleton` 在 `children` 内部使用。

后续若需要，可以进一步靠拢 AntD：引入 `data` + `render_item` 模式，增加 grid 布局和 `loadMore` 等功能。

此外，综合示例 `dashboard_demo` 展示了 List 与 Layout + Tabs + Card + Tag + Badge + Avatar + Pagination 的组合，用于构建用户列表/概览型仪表盘页面，可作为实际项目的模板参考。