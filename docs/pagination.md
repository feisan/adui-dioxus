# Pagination：分页器（MVP）

> 实现位置：`src/components/pagination.rs`
>
> 示例：`examples/pagination_demo.rs`

## 1. 设计目标

Pagination 用于控制列表类数据的分页行为，负责「当前页」与「每页条数」状态，实际数据加载由列表组件或业务逻辑完成。

本版 Pagination 采用 **受控优先 + 简化省略策略**：

- 支持 `current` / `page_size` / `total` 的受控使用；
- 提供基础的 “上一页 / 下一页 + 页码列表 + 简单省略号” UI；
- 可选展示总条数文案与 pageSize 切换器；
- 暂不实现 AntD 中 `showQuickJumper` 等高级跳页能力。

---

## 2. PaginationProps：组件属性

> 类型定义：`src/components/pagination.rs`

```rust
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    pub current: Option<u32>,
    pub default_current: Option<u32>,
    pub page_size: Option<u32>,
    pub default_page_size: Option<u32>,
    pub total: u32,
    pub page_size_options: Option<Vec<u32>>,
    pub show_size_changer: bool,
    pub show_total: bool,
    pub on_change: Option<EventHandler<(u32, u32)>>,
    pub on_page_size_change: Option<EventHandler<(u32, u32)>>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

字段说明：

- `current` / `default_current`：
  - `current` 存在时，Pagination 处于 **受控模式**，内部不会持久化当前页；
  - 否则使用内部 `Signal<u32>` 存储当前页，初始值来自 `default_current`（默认为 1）。
- `page_size` / `default_page_size`：
  - `page_size` 存在时，页大小受控；
  - 否则使用内部 `Signal<u32>` 存储页大小，初始值来自 `default_page_size`（默认为 10，且至少为 1）。
- `total`：总条数，用于计算总页数：`total_pages = ceil(total / page_size)`，至少为 1。
- `page_size_options`：pageSize 下拉中可选的页大小列表（默认 `[10, 20, 50]`）。
- `show_size_changer`：是否展示 pageSize 下拉；
- `show_total`：是否展示类似 “共 X 条” 的总数文案；
- `on_change(page, page_size)`：当当前页或 pageSize 发生变化时触发；
- `on_page_size_change(page, page_size)`：仅在 pageSize 变化时额外触发一次，便于业务单独监听；
- `class` / `style`：作用于根 `div` 的附加类名与内联样式。

---

## 3. 行为与页码计算

内部状态管理：

- 使用 `Signal<u32>` 维护 `current_internal` / `page_size_internal`，仅在对应字段未受控时使用；
- 每次交互后通过统一的 `update_state(page, size, emit_size_change)` 函数：
  - 将 page clamp 到 `[1, total_pages]`；
  - 在非受控模式下更新内部 `Signal`；
  - 触发 `on_change`；
  - 如 `emit_size_change = true`，额外触发 `on_page_size_change`。

页码展示策略（简化版）：

- 当 `total_pages <= 7`：展示 `1..=total_pages` 全部页码；
- 当 `total_pages > 7`：
  - 永远显示第一页与最后一页；
  - 当前页前后各展示最多一页；
  - 在需要的地方渲染 `"..."` 作为省略号；
  - 省略号不会响应点击，仅用于视觉提示。

UI 结构：

```html
<div class="adui-pagination">
  <span class="adui-pagination-total">共 X 条</span> <!-- 可选 -->
  <ul class="adui-pagination-list">
    <li class="adui-pagination-item">上一页</li>
    <li class="adui-pagination-item adui-pagination-item-active">1</li>
    <li class="adui-pagination-item">2</li>
    <li class="adui-pagination-item adui-pagination-item-ellipsis">...</li>
    <li class="adui-pagination-item">N</li>
    <li class="adui-pagination-item">下一页</li>
  </ul>
  <select class="adui-pagination-size-changer"> <!-- 可选 -->
    <option>10 / 页</option>
    <option>20 / 页</option>
    ...
  </select>
</div>
```

交互规则：

- 上一页：当前页大于 1 时，点击将 `page` 减 1 并调用 `update_state(page-1, size, false)`；
- 下一页：当前页小于 `total_pages` 时，点击将 `page` 加 1；
- 页码项：点击非当前页的页码将切换到该页；
- 省略号项：不响应点击；
- pageSize 下拉：选中新的 size 时，调用 `update_state(current, new_size, true)`，内部会在合法范围内校正当前页。

---

## 4. 样式与类名

> 样式定义：`src/theme.rs` 中 `adui_layout_style!` 内的 `.adui-pagination-*`

主要类名：

- `.adui-pagination`：根容器，水平排列总数、页码与 size changer；
- `.adui-pagination-total`：总数文案；
- `.adui-pagination-list`：页码列表 `ul`；
- `.adui-pagination-item`：通用分页按钮；
- `.adui-pagination-item-active`：当前页高亮；
- `.adui-pagination-item-disabled`：禁用态（上一页/下一页到达边界时）；
- `.adui-pagination-item-ellipsis`：省略号项；
- `.adui-pagination-size-changer`：pageSize 下拉；

默认视觉：

- 当前页使用主色描边与浅色背景；
- hover 时边框与背景做轻微变化；
- 禁用项降低不透明度并禁止点击。

---

## 5. 示例：基础分页

摘自 `examples/pagination_demo.rs`：

```rust
#[component]
fn PaginationDemoShell() -> Element {
    let mut current = use_signal(|| 1u32);
    let mut page_size = use_signal(|| 10u32);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Pagination demo" }
            p { "展示基础分页用法，包含页码切换与页面大小切换。" }

            Pagination {
                total: 100,
                current: Some(*current.read()),
                page_size: Some(*page_size.read()),
                show_total: true,
                show_size_changer: true,
                on_change: move |(page, size)| {
                    current.set(page);
                    page_size.set(size);
                },
            }

            p { "当前页: {current}，每页条数: {page_size}" }
        }
    }
}
```

在该示例中，Pagination 完全处于受控模式：`current` / `page_size` 由外部 `Signal` 管理，所有交互通过 `on_change` 写回。

此外，`layout_navigation_demo` 与 `data_view_demo` 两个示例分别展示了 Pagination 与 Layout/Menu/Breadcrumb 的组合，以及与 List/Table/Empty/Spin/Skeleton 的典型列表页用法，可用作实际页面的参考模板。

---

## 6. 与 Ant Design 的差异

与 AntD 6.x 的 Pagination 相比，当前实现为裁剪版：

- 暂未支持：
  - `showQuickJumper`（输入页码快速跳转）；
  - `size`（small/default）和 `simple` 模式；
  - `itemRender` 自定义页码内容；
  - `showLessItems` 等高级省略策略。
- 省略策略：
  - 当前版本采用非常简化的 ellipsis 逻辑，仅在页数较多时插入 `...`，未精细对齐 AntD 的所有边界行为；
- pageSize 选择器：
  - 使用原生 `<select>` 实现 MVP，下拉样式与 Select 不完全一致，后续可切换为基于 Select 的版本。

> 后续扩展方向：
> 
> - 根据业务需要补充 `showQuickJumper` 与 `itemRender`；
> - 与 Table/List 组合出一套「受控分页 + 异步数据加载」的完整指南；
> - 将 pageSize 选择器迁移到 Select 组件上，使视觉更统一。
