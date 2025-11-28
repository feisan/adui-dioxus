# Skeleton：骨架屏（MVP）

> 实现位置：`src/components/skeleton.rs`
>
> 示例：`examples/skeleton_demo.rs`

## 1. 设计目标

Skeleton 用于在内容尚未加载完成时提供“页面骨架”，让用户对布局结构有预期，比简单的 Spin 更适合结构化内容（列表、详情卡片等）。

当前实现为基础版：提供标题 + 多行段落骨架，支持 active 动画标记和“加载完成后渲染 children”的模式，暂不实现 Avatar/Button/Input 等子变体。

---

## 2. SkeletonProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    pub loading: Option<bool>,
    pub active: bool,
    pub title: bool,
    pub paragraph_rows: Option<u8>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub content: Option<Element>,
}
```

字段说明：

- `loading`：
  - `None` 或 `Some(true)`：渲染骨架；
  - `Some(false)`：渲染 `children`（若存在），不再显示骨架；
- `active`：
  - 是否添加 `.adui-skeleton-active` 类，用于启用骨架动画；当前样式中仅作为类钩子，后续可加渐变效果；
- `title`：
  - 是否渲染标题骨架块，默认 `true`；
- `paragraph_rows`：
  - 段落行数，默认为 3 行，最少 1 行；
- `class` / `style`：
  - 作用于根 `div` 的附加类名与内联样式；
- `content`：
  - 当 `loading = Some(false)` 时渲染的真实内容；
  - 当 `loading` 为 `None` 或 `Some(true)` 时被忽略，仅显示骨架。

---

## 3. 渲染结构与样式类

UI 结构（加载中）：

```html
<div class="adui-skeleton adui-skeleton-active?">
  <div class="adui-skeleton-title"></div> <!-- 可选 -->
  <div class="adui-skeleton-paragraph">
    <div class="adui-skeleton-paragraph-line"></div>
    <div class="adui-skeleton-paragraph-line"></div>
    <div class="adui-skeleton-paragraph-line adui-skeleton-paragraph-line-last"></div>
  </div>
</div>
```

主要类名：

- `.adui-skeleton`：根容器；
- `.adui-skeleton-active`：表示有动画的骨架，可用于添加渐变效果；
- `.adui-skeleton-title`：标题行骨架；
- `.adui-skeleton-paragraph`：段落骨架容器；
- `.adui-skeleton-paragraph-line`：单行段落骨架；
- `.adui-skeleton-paragraph-line-last`：最后一行，用于适当缩短宽度模仿真实文本。

> 样式定义集中在 `src/theme.rs` 的 `adui_skeleton_style!` 片段（当前为简化版，可按需补充动画效果）。

---

## 4. 示例：基础与包裹内容

摘自 `examples/skeleton_demo.rs`：

```rust
#[component]
fn SkeletonDemoShell() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Skeleton demo" }
            p { "展示基础骨架屏用法，以及加载完成后渲染真实内容。" }

            h3 { style: "margin-top: 16px;", "纯骨架" }
            Skeleton {}

            h3 { style: "margin-top: 24px;", "包裹真实内容" }
            div { style: "margin-bottom: 8px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: {
                        let mut flag = loading;
                        move |_| {
                            let next = !*flag.read();
                            flag.set(next);
                        }
                    },
                    if *loading.read() { "加载完成" } else { "重新加载" }
                }
            }
            Skeleton {
                loading: Some(*loading.read()),
                active: true,
                paragraph_rows: Some(3),
                content: Some(rsx! {
                    div {
                        style: "padding: 16px; border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border); background: var(--adui-color-bg-container); max-width: 360px;",
                        h4 { "真实内容" }
                        p { style: "margin: 0; color: var(--adui-color-text-secondary);",
                            "当 loading=false 时，Skeleton 渲染子内容而不再显示骨架。"
                        }
                    }
                }),
            }
        }
    }
}
```

---

## 5. 与 Spin / Empty / List / Table 的关系

- 与 Spin：
  - Spin 更适合按钮点击、局部加载等“短时、结构不确定”的场景；
  - Skeleton 更适合列表/详情页等“结构明确”的内容，用骨架模拟最终布局；
- 与 Empty：
  - Skeleton 处理“正在加载”的阶段；
  - Empty 处理“加载完成但无数据”的阶段；
- 与 List / Table：
  - 建议在后续 List/Table 实现中，将 `loading = true` 时的渲染策略统一为 Skeleton（或 Skeleton+Spin），并在 `data.is_empty()` 时使用 Empty。