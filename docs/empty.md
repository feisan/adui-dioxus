# Empty：空状态组件（MVP）

> 实现位置：`src/components/empty.rs`
>
> 示例：`examples/empty_demo.rs`

## 1. 设计目标

Empty 用于在「无数据」场景下展示统一的空状态视觉，常见于：

- 列表 / 表格没有数据时；
- 搜索结果为空时；
- 详情页尚未初始化内容时。

本版 Empty 采用 **简单插画 + 描述文案 + 可选操作区域** 的形式，不在组件内部引入复杂图片库，仅保留必要的视觉占位。

---

## 2. EmptyProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct EmptyProps {
    pub description: Option<String>,
    pub image: Option<EmptyImage>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub footer: Option<Element>,
}

#[derive(Clone, PartialEq)]
pub enum EmptyImage {
    Default,
    Simple,
    Small,
    Custom(String),
}
```

字段说明：

- `description`：
  - 描述文案，显示在图形下方；
  - 若未提供，则使用一个简短的中文默认文案（当前为 `"暂无数据"`）。
- `image`：
  - `EmptyImage::Default`：默认插画（使用内嵌 SVG 实现）；
  - `EmptyImage::Simple`：简化插画，使用简单矩形占位；
  - `EmptyImage::Small`：小型插画，会额外在根节点添加 `.adui-empty-sm` 类，以便样式调整；
  - `EmptyImage::Custom(String)`：使用自定义图片 URL（`<img src="..." />`）。
- `class` / `style`：
  - 作用于根 `div` 的附加类名与内联样式；
- `footer`：
  - 渲染在描述文案下方的“尾部区域”，常用于放置操作按钮（例如“创建第一个项目”）。

---

## 3. 渲染结构与样式类

UI 结构（简化）：

```html
<div class="adui-empty adui-empty-sm?">
  <div class="adui-empty-image">
    <!-- SVG / 占位块 / 自定义图片 -->
  </div>
  <p class="adui-empty-description">暂无数据</p>
  <div class="adui-empty-footer"> <!-- 可选：footer --> </div>
</div>
```

主要类名：

- `.adui-empty`：根容器，居中、次要文本色；
- `.adui-empty-sm`：小号空态（由 `EmptyImage::Small` 触发）；
- `.adui-empty-image`：图形区域容器；
- `.adui-empty-image-svg`：默认 SVG 插画节点；
- `.adui-empty-image-simple`：简化矩形占位图形；
- `.adui-empty-image-img`：自定义图片 `<img>`；
- `.adui-empty-description`：描述文案；
- `.adui-empty-footer`：尾部操作区域。

> 样式定义集中在 `src/theme.rs` 的 `adui_layout_style!` 片段中，使用现有主题 token 控制颜色和间距。

---

## 4. 示例：基础用法

摘自 `examples/empty_demo.rs`：

```rust
#[component]
fn EmptyDemoShell() -> Element {
    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Empty demo" }
            p { "展示统一的空状态组件 Empty，不同描述与附加操作按钮。" }

            div { style: "margin-top: 16px; display: flex; flex-direction: column; gap: 24px; max-width: 480px;", 
                // 基础 Empty
                div {
                    h3 { "基础 Empty" }
                    Empty {}
                }

                // 自定义描述
                div {
                    h3 { "自定义描述" }
                    Empty { description: Some("当前没有任何记录".to_string()) }
                }

                // 带操作按钮
                div {
                    h3 { "带操作按钮" }
                    Empty {
                        description: Some("还没有创建任何项目".to_string()),
                        footer: Some(rsx! {
                            Button { r#type: ButtonType::Primary, "创建第一个项目" }
                        }),
                    }
                }
            }
        }
    }
}
```

---

## 5. 与 List / Table / Select 的集成建议

- List / Table：
  - 在 `loading = false && data.is_empty()` 时，使用 Empty 作为默认内容：
    - 提供统一“暂无数据”视觉；
    - 可通过 `EmptyProps.children` 提供“创建记录”之类的操作按钮。
- Select / TreeSelect / Cascader / AutoComplete：
  - 当下拉选项列表为空时，可重用 Empty 的 SVG 与描述样式，或直接将下拉内容渲染为简化版 Empty。

在后续实现 List / Table 组件时，会在各自的文档中交叉引用 Empty 的这些用法，形成统一的空状态体验。

---

## 6. 与 Ant Design 的差异

相较于 AntD 6.x 的 Empty，本版为裁剪版：

- 暂未提供：
  - 通过 `Empty.PRESENTED_IMAGE_*` 静态字段访问预置图片的能力；
  - 完整的 `classNames` / `styles` 语义钩子系统；
  - 内建多种复杂插画资源（仅保留一套轻量 SVG）。
- 兼容性与扩展：
  - 已预留 `EmptyImage::Custom(String)` 以支持自定义图片；
  - 后续可以考虑增加 size/locale 相关配置，与 `ConfigProvider` 集成。

当前版本主要目标是统一空态视觉与结构，方便在 List / Table / Select 等组件中复用。