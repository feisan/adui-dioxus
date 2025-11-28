# Spin：加载指示器（MVP）

> 实现位置：`src/components/spin.rs`
>
> 示例：`examples/spin_demo.rs`

## 1. 设计目标

Spin 用于展示加载状态，适合在以下场景中使用：

- 作为独立指示器，提示正在加载数据；
- 包裹局部内容，在内容上方显示半透明遮罩和 loading 图标；
- 配合 Empty / Skeleton 一起构成加载与空状态方案。

当前实现为简化版：提供基础的三种尺寸、文本提示和嵌套加载能力，暂不支持延迟、百分比进度或自定义指示器。

---

## 2. SpinProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct SpinProps {
    pub spinning: Option<bool>,
    pub size: Option<SpinSize>,
    pub tip: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub fullscreen: bool,
    pub children: Element,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpinSize {
    Small,
    Default,
    Large,
}
```

字段说明：

- `spinning`：
  - 是否显示加载状态；`None` 等价于 `Some(true)`；
  - 当 `children` 为空且 `spinning = false` 时，Spin 不渲染任何内容；
  - 当 `children` 存在且 `spinning = false` 时，仅渲染子内容，不显示遮罩与指示器。
- `size`：
  - 指示器尺寸：`Small` / `Default` / `Large`；
  - 通过类名 `.adui-spin-sm` / `.adui-spin-lg` 调整字体和布局比例。
- `tip`：
  - 指示器下方的文本提示，如“加载中...” 等；
- `class` / `style`：
  - 附加到根元素的类名和内联样式；
- `fullscreen`：
  - 为 Spin 添加 `.adui-spin-fullscreen` 类，用于铺满全屏的加载遮罩；
  - 当前实现仅提供样式钩子，不强制应用于所有嵌套场景。
- `children`：
  - 嵌套模式下的内容；
  - 当存在 children 时，Spin 会始终渲染内容，并在 `spinning = true` 时叠加半透明遮罩和指示器。

---

## 3. 渲染结构与样式类

### 独立模式（无 children）

```html
<div class="adui-spin adui-spin-sm? adui-spin-lg?">
  <div class="adui-spin-indicator">
    <span class="adui-spin-dot"></span>
  </div>
  <div class="adui-spin-text">加载中...</div> <!-- 可选 -->
</div>
```

- `.adui-spin`：根容器，垂直排列指示器和文本；
- `.adui-spin-dot`：圆形指示器，使用 `border` + `animation: adui-spin` 旋转；
- `.adui-spin-text`：文本提示，使用次要文本色；
- `.adui-spin-sm` / `.adui-spin-lg`：调整整体尺寸。

### 嵌套模式（有 children）

```html
<div class="adui-spin adui-spin-nested">
  <div class="adui-spin-nested-container">
    <!-- 子内容 -->
  </div>
  <div class="adui-spin-nested-mask">
    <div class="adui-spin-indicator">...</div>
    <div class="adui-spin-text">加载中...</div>
  </div>
</div>
```

- `.adui-spin-nested`：标记为嵌套加载容器；
- `.adui-spin-nested-container`：真实内容区域，始终渲染；
- `.adui-spin-nested-mask`：半透明遮罩层，`spinning = true` 时渲染并居中显示指示器与文本；
- `.adui-spin-fullscreen`：可用于全屏加载场景（作为根容器类）。

> CSS 定义集中在 `src/theme.rs` 的 `adui_spin_style!` 片段中，使用主题 token 控制颜色与透明度。

---

## 4. 示例：基础与嵌套用法

摘自 `examples/spin_demo.rs`：

```rust
#[component]
fn SpinDemoShell() -> Element {
    let mut loading = use_signal(|| true);

    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Spin demo" }
            p { "展示基础 Spin 指示器以及嵌套加载状态。" }

            h3 { style: "margin-top: 16px;", "基础指示器" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Spin { size: Some(SpinSize::Small), tip: Some("小号".to_string()) }
                Spin { tip: Some("默认".to_string()) }
                Spin { size: Some(SpinSize::Large), tip: Some("大号".to_string()) }
            }

            h3 { style: "margin-top: 24px;", "嵌套加载" }
            div { style: "margin-bottom: 8px;",
                Button {
                    r#type: ButtonType::Primary,
                    onclick: move |_| loading.set(!*loading.read()),
                    if *loading.read() { "停止加载" } else { "开始加载" }
                }
            }
            Spin {
                spinning: Some(*loading.read()),
                tip: Some("加载中...".to_string()),
                children: Some(rsx! {
                    div {
                        style: "padding: 16px; border-radius: var(--adui-radius); border: 1px solid var(--adui-color-border); background: var(--adui-color-bg-container); max-width: 320px;",
                        h4 { "内容卡片" }
                        p { style: "margin: 0; color: var(--adui-color-text-secondary);",
                            "当 Spin 处于加载状态时，会在内容上方显示半透明遮罩和指示器。"
                        }
                    }
                }),
            }
        }
    }
}
```

---

## 5. 与 Skeleton / Empty 的关系

- 与 Skeleton：
  - Spin 更适合“短时、不定结构”的加载，例如按钮点击后快速加载；
  - Skeleton 则更适合页面骨架式加载，在结构感较强的列表/详情视图中使用；
- 与 Empty：
  - 当数据仍在加载时，用 Spin/Skeleton；
  - 当数据加载完成但为空时，用 Empty 展示统一空态。

后续在 List / Table 组件中，会推荐：`loading = true` 时使用 Spin 或 Skeleton 包裹内容，`data.is_empty()` 时使用 Empty。