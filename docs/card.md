# Card：卡片（MVP）

> 实现位置：`src/components/card.rs`
>
> 示例：`examples/card_demo.rs`

## 1. 设计目标

Card 用于承载中等粒度的内容块，例如统计信息、配置表单、列表片段等，是中后台界面中非常常见的容器组件。

当前实现为基础版：

- 支持标题 `title`、右上角操作区 `extra`、边框/hoverable 状态；
- 支持简单 `loading` 占位（内部使用 Skeleton 渲染骨架）；
- 不包含 AntD 中的 Grid/Meta 子组件，暂不处理复杂卡片组合布局。

---

## 2. CardProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    pub title: Option<Element>,
    pub extra: Option<Element>,
    pub bordered: bool,
    pub size: Option<ComponentSize>,
    pub loading: bool,
    pub hoverable: bool,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Element,
}
```

字段说明：

- `title`：
  - 可选标题内容，渲染在卡片头部左侧；
  - 通常是简短文本，也可以是任意 Element；
- `extra`：
  - 可选的右上角操作区域（如按钮、链接）；
  - 与 `title` 一起构成卡片头部；
- `bordered`：
  - 是否展示卡片边框，默认 `true`；
- `size`：
  - 控制卡片内边距和文字大小，复用 `ComponentSize`（Small/Middle/Large）；
  - 对应样式类：`.adui-card-sm` / `.adui-card-lg`；
- `loading`：
  - 是否处于加载态；
  - `true` 时，Card 会在 body 内部渲染一个简单 `Skeleton` 骨架，占位真实内容；
- `hoverable`：
  - 是否启用 hover 效果（阴影 + 轻微上浮）；
  - 多用于可点击的卡片元素；
- `class` / `style`：
  - 作用于根 `.adui-card` 元素的附加类名与行内样式；
- `children`：
  - 卡片正文内容，在 `loading=false` 时渲染在 body 内部。

---

## 3. 渲染结构与样式类

UI 结构（简化）：

```html
<div class="adui-card adui-card-bordered adui-card-hoverable? adui-card-sm/lg?">
  <div class="adui-card-head">
    <div class="adui-card-head-title">标题</div>
    <div class="adui-card-head-extra">操作</div>
  </div>
  <div class="adui-card-body">
    <!-- loading=true 时渲染 Skeleton，占位真实内容 -->
  </div>
</div>
```

主要样式类（定义在 `src/theme.rs` 的 `adui_card_style!` 中）：

- `.adui-card`：卡片根容器，控制背景色、圆角、基础过渡；
- `.adui-card-bordered`：启用边框，使用 theme 中的 `color_border`；
- `.adui-card-hoverable`：启用 hover 效果时添加的标记类；
- `.adui-card-hoverable:hover`：鼠标悬停时增加轻微阴影和上浮；
- `.adui-card-head`：头部区域容器，左右两侧分别是标题与 extra；
- `.adui-card-head-title`：标题文本；
- `.adui-card-head-extra`：右侧操作区域，通常放按钮或链接；
- `.adui-card-body`：卡片正文区域；
- `.adui-card-sm / .adui-card-lg`：不同尺寸下的头部与 body 内边距。

---

## 4. 示例：基础卡片与加载态

摘自 `examples/card_demo.rs`：

```rust
#[component]
fn CardDemoShell() -> Element {
    rsx! {
        // 基础卡片
        Card {
            title: Some(rsx!("基础信息")),
            children: rsx! {
                p { "这里是基础卡片的正文内容，可以放任意组件。" }
            }
        }

        // 带 extra 与 hoverable 的操作卡片
        Card {
            title: Some(rsx!("项目概览")),
            extra: Some(rsx!(
                Button { r#type: ButtonType::Link, "查看详情" }
            )),
            hoverable: true,
            children: rsx! {
                p { "卡片右上角可以放操作按钮或链接，例如“查看详情”。" }
            }
        }

        // 加载态卡片
        Card {
            title: Some(rsx!("加载中的数据块")),
            loading: true,
            children: rsx! {
                p { "加载完成后将显示的内容。" }
            }
        }
    }
}
```

---

## 5. 与其他组件的协同

- 与 Layout / Grid / Masonry：
  - 多个 Card 通过 Grid/Masonry 组合，可构建典型仪表盘布局（统计卡片、快捷操作块等）；
- 与 Tabs：
  - 常见模式是 Tabs + Card 组合，不同 Tab 下展示不同类型的 Card 内容（例如“基础信息卡片 / 安全设置卡片”等）；
- 与 List / Table：
  - Card 可作为 List 中的项容器，形成「卡片列表」，也可以在 Card 中嵌入 Table 作为子区域；
- 与 Skeleton / Spin：
  - 当前 Card 内部简单使用 Skeleton 处理 `loading` 状态；
  - 对于整个页面的加载，可结合 Spin/Skeleton 与 Card 一起使用，形成更统一的加载体验。

---

## 6. 与 Ant Design 的差异与后续规划

与 Ant Design 6.x 的 Card 相比，当前实现为裁剪版：

- 暂未支持：
  - `Card.Grid` / `Card.Meta` 子组件；
  - cover 图片、actions 行等高级排版能力；
  - 更细粒度的尺寸/间距控制；
- loading：
  - 目前仅在 `loading=true` 时渲染一个简单 `Skeleton` 骨架，未对齐 AntD 的所有 loading 视觉细节；

后续扩展方向：

- 视业务需要补充 Meta/Grid 能力，用于复杂卡片列表；
- 增加 cover/actions 等区域的布局支持；
- 与 Tabs/列表页示例更紧密结合，提供完整的「仪表盘 / 项目卡片列表」官方 demo。
