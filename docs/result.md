# Result：操作结果页（MVP）

> 实现位置：`src/components/result.rs`
>
> 示例：`examples/result_demo.rs`

## 1. 设计目标

Result 用于展示一次操作的整体结果，常见于：

- 单独的结果页（如提交成功/失败、权限不足、404/500 页面等）；
- 组合在 Card 或 Modal 中，作为复杂流程的最终反馈区域。

本版 Result 提供一个与 Ant Design 语义对齐的 **MVP 子集**：

- 支持多种状态（成功/信息/警告/错误/403/404/500）；
- 提供标题、副标题、图标、操作区域（extra）以及扩展内容区（children）；
- 主要作为展示组件，与外层路由/布局/表单协作完成完整结果页体验。

---

## 2. ResultStatus / ResultProps：组件属性

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultStatus {
    Success,
    Info,
    Warning,
    Error,
    NotFound,
    Forbidden,
    ServerError,
}

#[derive(Props, Clone, PartialEq)]
pub struct ResultProps {
    pub status: Option<ResultStatus>,
    pub icon: Option<Element>,
    pub title: Option<Element>,
    pub sub_title: Option<Element>,
    pub extra: Option<Element>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>,
}
```

- `status: Option<ResultStatus>`
  - 结果状态，决定默认图标和部分颜色映射：
    - `Success`：成功结果；
    - `Info`：一般信息展示；
    - `Warning`：警告结果（如部分失败、需注意的结果）；
    - `Error`：错误结果；
    - `NotFound`：404 资源未找到；
    - `Forbidden`：403 权限不足；
    - `ServerError`：500 服务器错误；
  - 对应样式类：`.adui-result-success` / `.adui-result-info` / `.adui-result-warning` / `.adui-result-error` / `.adui-result-404` / `.adui-result-403` / `.adui-result-500`；
  - 若为 `None`，默认使用 `Info`。
- `icon: Option<Element>`
  - 自定义图标节点；
  - 若未提供，组件会根据 `status` 渲染一个放大的基础 `Icon`：
    - `Success`：`IconKind::Check`；
    - `Error`/`ServerError`：`IconKind::Close`；
    - 其他状态：`IconKind::Info`。
- `title: Option<Element>`
  - 结果标题，例如 “提交成功”、“服务器错误”；
- `sub_title: Option<Element>`
  - 副标题/描述，用于展示较长的说明文字；
- `extra: Option<Element>`
  - 操作区域，通常为按钮组，例如“返回列表”、“查看详情” 等；
- `class` / `style`
  - 附加类名与内联样式，作用于根 `div`；
- `children: Option<Element>`
  - 扩展内容区，渲染在整个结果块的底部，可以放步骤说明、表格/列表等。

---

## 3. DOM 结构与样式类

典型结构：

```html
<div class="adui-result adui-result-success">
  <div class="adui-result-icon">...</div>      <!-- 可选，自定义或默认 icon -->
  <div class="adui-result-title">提交成功</div>  <!-- 可选 -->
  <div class="adui-result-subtitle">...</div>  <!-- 可选 -->
  <div class="adui-result-extra">...</div>     <!-- 可选，按钮组等操作区 -->
  <div class="adui-result-content">...</div>   <!-- 可选，扩展内容区 -->
</div>
```

样式定义位于 `src/theme.rs` 的 `adui_result_style!` 宏中，主要类名：

- `.adui-result`：根容器，默认居中排版；
- `.adui-result-icon`：图标区域；
- `.adui-result-title`：标题；
- `.adui-result-subtitle`：副标题/描述；
- `.adui-result-extra`：操作区域；
- `.adui-result-content`：扩展内容区域；
- `.adui-result-success/info/warning/error/403/404/500 .adui-result-icon`：根据状态为图标着色。

---

## 4. 示例：成功结果与错误结果

摘自 `examples/result_demo.rs`：

```rust
#[component]
fn ResultDemoShell() -> Element {
    rsx! {
        Layout {
            adui_dioxus::Content {
                style: Some("padding: 24px; background: var(--adui-color-bg-base);".into()),
                children: rsx! {
                    h2 { "Result demo" }
                    p { "展示基础成功/错误结果页，并组合按钮作为操作区域。" }

                    Card {
                        title: Some(rsx!("操作成功")),
                        children: rsx! {
                            Result {
                                status: Some(ResultStatus::Success),
                                title: Some(rsx!("提交成功")),
                                sub_title: Some(rsx!("您的操作已成功提交，可以在列表页查看最新状态。")),
                                extra: Some(rsx!(
                                    div { style: "display: inline-flex; gap: 8px;",
                                        Button { r#type: ButtonType::Primary, "返回列表" }
                                        Button { r#type: ButtonType::Default, "查看详情" }
                                    }
                                )),
                            }
                        },
                    }

                    Card {
                        style: Some("margin-top: 24px;".into()),
                        title: Some(rsx!("错误结果")),
                        children: rsx! {
                            Result {
                                status: Some(ResultStatus::ServerError),
                                title: Some(rsx!("服务器错误")),
                                sub_title: Some(rsx!(
                                    "请求处理过程中发生了未知错误，请稍后重试，或联系管理员。"
                                )),
                                extra: Some(rsx!(
                                    div { style: "display: inline-flex; gap: 8px;",
                                        Button { r#type: ButtonType::Primary, "重新尝试" }
                                        Button { r#type: ButtonType::Default, "返回首页" }
                                    }
                                )),
                            }
                        },
                    }
                },
            }
        }
    }
}
```

---

## 5. 使用建议与组合场景

- 单独结果页：
  - 将 Result 置于单独路由页面的内容区域，配合 Layout/Breadcrumb 呈现结果；
  - `extra` 区域放置“返回列表”、“继续创建”、“查看详情”等按钮。
- 多步表单：
  - 使用 Steps 管理流程步骤，最后一步使用 Result 作为收尾视图；
  - 配合 Alert 展示额外说明（例如风险提示、后续操作要求）。
- 与 Card/List/Table 组合：
  - 在 Card 中嵌入 Result，用于弹层或侧栏中的结果展示；
  - 在结果页下方的 `children` 区域展示与本次操作相关的列表或表格。

与 Ant Design 相比：

- 当前版本未内置精细的插画（如官方 403/404/500 SVG），仅使用基础图标 + 语义色；
- `status` 支持的取值覆盖常见场景，若需要更多自定义外观，可通过 `icon` + `children` 组合扩展；
- 不提供额外静态字段（如 `PRESENTED_IMAGE_404`），推荐在项目层面封装更贴合业务的结果页模板。