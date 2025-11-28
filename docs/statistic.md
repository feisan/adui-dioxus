# Statistic：数字统计（MVP）

> 实现位置：`src/components/statistic.rs`
>
> 示例：`examples/statistic_demo.rs`

## 1. 设计目标

Statistic 用于展示关键指标的数值，是仪表盘、统计卡片中常见的元素，例如：

- 今日访问量、活跃用户数；
- 转化率、错误率、延迟等百分比指标；
- 金额、订单数等业务数字。

本版 Statistic 提供一个轻量的 **MVP 实现**：

- 支持数值/预格式化文本展示；
- 支持简单精度控制、前缀和后缀元素；
- 适合作为 Card/Grid/Tabs 等容器内部的子块使用。

暂不实现复杂的倒计时、动画增减效果等能力。

---

## 2. StatisticProps：组件属性

```rust
#[derive(Props, Clone, PartialEq)]
pub struct StatisticProps {
    pub title: Option<Element>,
    pub value: Option<f64>,
    pub value_text: Option<String>,
    pub precision: Option<u8>,
    pub prefix: Option<Element>,
    pub suffix: Option<Element>,
    pub value_style: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
}
```

字段说明：

- `title: Option<Element>`
  - 可选标题，展示在数值上方，通常用于说明该指标含义；
- `value: Option<f64>`
  - 数值形式的指标值；
  - 当 `value_text` 未提供时，组件会基于 `value` 和 `precision` 生成展示文本；
  - 若为 `None`，在未提供 `value_text` 时视为 `0.0`。
- `value_text: Option<String>`
  - 预格式化的数值文本；
  - 若存在，将优先使用该文本，忽略 `value` 和 `precision` 的格式化逻辑；
- `precision: Option<u8>`
  - 当 `value_text` 为 `None`，且提供 `value` 时生效；
  - 表示小数位数，例如 `precision = Some(2)` 时会展示为 `3.14`；
  - 未指定时使用 Rust 默认格式化，同时会尝试去掉形如 `"10.0"` 的尾随 `.0`；
- `prefix: Option<Element>` / `suffix: Option<Element>`
  - 前缀/后缀元素，例如货币符号、单位、涨跌图标等；
- `value_style: Option<String>`
  - 作用于数值部分的内联样式字符串，例如 `"color: #52c41a;"` 表示绿色数值；
- `class` / `style`
  - 根元素的附加类名与内联样式。

---

## 3. DOM 结构与样式类

结构示例：

```html
<div class="adui-statistic">
  <div class="adui-statistic-title">转化率</div>          <!-- 可选 -->
  <div class="adui-statistic-content">
    <span class="adui-statistic-prefix">≈</span>       <!-- 可选 -->
    <span class="adui-statistic-value">3.14</span>
    <span class="adui-statistic-suffix">%</span>       <!-- 可选 -->
  </div>
</div>
```

样式定义位于 `src/theme.rs` 的 `adui_statistic_style!` 宏中，核心类包括：

- `.adui-statistic`：根容器，列方向布局；
- `.adui-statistic-title`：标题（使用次要文本颜色与较小字号）；
- `.adui-statistic-content`：数值行容器，对齐基线；
- `.adui-statistic-prefix` / `.adui-statistic-suffix`：前缀/后缀样式；
- `.adui-statistic-value`：数值本体，较大字号和较重字体。

---

## 4. 示例：统计卡片

摘自 `examples/statistic_demo.rs`：

```rust
#[component]
fn StatisticDemoShell() -> Element {
    rsx! {
        div { style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Statistic demo" }
            p { "数字统计卡片示例，展示基础数值与前后缀组合。" }

            Grid {
                Row {
                    Col { span: Some(8),
                        Card {
                            title: Some(rsx!("今日访问量")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Visits")),
                                    value: Some(12345.0),
                                    precision: Some(0),
                                    suffix: Some(rsx!("次")),
                                }
                                Tag { children: rsx!("较昨日 +8%") }
                            },
                        }
                    }
                    Col { span: Some(8),
                        Card {
                            title: Some(rsx!("转化率")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Conversion")),
                                    value: Some(3.1415),
                                    precision: Some(2),
                                    suffix: Some(rsx!("%")),
                                }
                                Tag { children: rsx!("较昨日 -1.2%") }
                            },
                        }
                    }
                    Col { span: Some(8),
                        Card {
                            title: Some(rsx!("错误率")),
                            children: rsx! {
                                Statistic {
                                    title: Some(rsx!("Error rate")),
                                    value: Some(0.07),
                                    precision: Some(2),
                                    suffix: Some(rsx!("%")),
                                }
                                Tag { children: rsx!("稳定") }
                            },
                        }
                    }
                }
            }
        }
    }
}
```

以上示例展示了三张典型统计卡片，分别显示访问量、转化率和错误率，并结合 `Tag` 表达趋势或状态。

---

## 5. 使用建议与与其它组件的组合

- 与 Card/Grid 组合：
  - 在 Card 中嵌入 1–3 个 Statistic，用于仪表盘统计块；
  - 与 Grid/Row/Col 搭配，构建多列统计面板。
- 与 Progress/Tag 组合：
  - 使用 Statistic 展示总体数值（如完成率 75%），Progress 展示视觉进度条，Tag 展示环比变化；
- 与 Tabs/Steps/Result 组合：
  - 在 Result 页面中使用 Statistic 展示关键指标（如总耗时、成功条数等），与 Steps/Progress 共同构成完整流程反馈界面。

与 Ant Design 的差异：

- 当前版本只提供基础格式化（简单 precision + `value_text`），未实现复杂的分组符号/本地化格式；
- 未实现倒计时 `Statistic.Countdown` 能力；
- 若项目需要更复杂格式，可在业务层使用现有格式化库生成 `value_text`，交由 Statistic 直接展示。
