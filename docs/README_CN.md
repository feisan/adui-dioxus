# 组件库文档

## 概述

这是一个实验性组件库，使用 Vibe Coding 将 Ant Design 6.0.0 移植到 Dioxus。该库提供了一套完整的 UI 组件，采用 Ant Design 的设计语言和模式，适配了 Rust 和 Dioxus 框架。

## 关于本项目

本组件库是 Ant Design 6.0.0 到 Dioxus 的实验性移植，使用 Vibe Coding 创建。它旨在将 Ant Design 丰富的组件生态系统和设计原则引入 Rust 生态系统，使开发者能够使用类型安全、高性能的 Rust 代码构建现代 Web 应用程序。

**注意**：这是一个实验性项目。某些功能可能与原始 Ant Design 实现有所不同，API 可能会随着项目的成熟而演变。

## 组件分类

### 布局

用于构建页面布局和组织内容的组件。

- [Layout](layout.md) - 页面布局容器，包含 Header、Footer、Sider 和 Content
- [Grid](grid.md) - 24 列网格系统，用于响应式布局
- [Flex](flex.md) - 弹性盒子布局组件
- [Space](space.md) - 间距组件，用于排列元素
- [Divider](divider.md) - 分割线，用于分隔内容
- [Splitter](splitter.md) - 可调整大小的分割面板
- [Masonry](masonry.md) - 瀑布流布局，用于卡片和项目

### 导航

用于导航和路径查找的组件。

- [Menu](menu.md) - 导航菜单，支持水平和垂直模式
- [Breadcrumb](breadcrumb.md) - 面包屑导航
- [Tabs](tabs.md) - 标签页导航组件
- [Anchor](anchor.md) - 锚点导航，用于长页面
- [Steps](steps.md) - 步骤指示器，用于流程
- [Pagination](pagination.md) - 分页组件

### 数据录入

用于数据输入和表单控件的组件。

- [Input](input.md) - 文本输入组件
- [TextArea](textarea.md) - 多行文本输入
- [Password](input.md#password) - 密码输入，带可见性切换
- [Search](input.md#search) - 带搜索按钮的输入
- [OTP](input.md#otp) - 一次性密码输入
- [InputNumber](input_number.md) - 数字输入，带步进器
- [Select](select.md) - 下拉选择器
- [TreeSelect](tree_select.md) - 树形选择器
- [Cascader](cascader.md) - 级联选择器
- [AutoComplete](auto_complete.md) - 自动完成输入
- [Mentions](mentions.md) - @提及输入
- [DatePicker](date_picker.md) - 日期选择器
- [TimePicker](time_picker.md) - 时间选择器
- [Calendar](calendar.md) - 日历组件
- [ColorPicker](color_picker.md) - 颜色选择器
- [Upload](upload.md) - 文件上传组件
- [Rate](rate.md) - 评分组件
- [Switch](switch.md) - 开关切换
- [Checkbox](checkbox.md) - 复选框输入
- [Radio](radio.md) - 单选按钮
- [Slider](slider.md) - 滑块输入

### 数据展示

用于显示数据和内容的组件。

- [Table](table.md) - 数据表格，支持排序、筛选和分页
- [List](list.md) - 列表组件，用于显示数据
- [Descriptions](descriptions.md) - 描述列表，用于键值对
- [Card](card.md) - 卡片容器组件
- [Collapse](collapse.md) - 可折叠内容面板
- [Timeline](timeline.md) - 时间轴组件
- [Tree](tree.md) - 树形组件
- [Tag](tag.md) - 标签组件
- [Badge](badge.md) - 徽章，用于通知和计数
- [Avatar](avatar.md) - 头像组件
- [Empty](empty.md) - 空状态组件
- [Statistic](statistic.md) - 统计展示组件
- [Progress](progress.md) - 进度指示器
- [QRCode](qrcode.md) - 二维码生成器
- [Image](image.md) - 图片组件，带预览

### 反馈

用于用户反馈和通知的组件。

- [Alert](alert.md) - 警告提示组件
- [Message](message.md) - 全局消息通知
- [Notification](notification.md) - 通知组件
- [Modal](modal.md) - 模态对话框
- [Drawer](drawer.md) - 抽屉组件
- [Popconfirm](popconfirm.md) - 气泡确认框
- [Popover](popover.md) - 气泡卡片组件
- [Tooltip](tooltip.md) - 文字提示组件
- [Spin](spin.md) - 加载中组件
- [Skeleton](skeleton.md) - 骨架屏加载
- [Result](result.md) - 结果页组件
- [Progress](progress.md) - 进度指示器

### 其他

其他实用和专用组件。

- [Button](button.md) - 按钮组件
- [FloatButton](float_button.md) - 悬浮按钮
- [Icon](icon.md) - 图标组件
- [Typography](typography.md) - 排版组件（Title、Text、Paragraph）
- [Form](form.md) - 表单组件，带验证
- [ConfigProvider](config_provider.md) - 全局配置提供者
- [App](app.md) - 应用级组件和钩子
- [Affix](affix.md) - 固钉组件，用于固定定位
- [Watermark](watermark.md) - 水印组件
- [Tour](tour.md) - 漫游式引导组件
- [Transfer](transfer.md) - 穿梭框组件
- [Carousel](carousel.md) - 走马灯组件

## 快速开始

要在 Dioxus 项目中使用此组件库，请将其添加到 `Cargo.toml`：

```toml
[dependencies]
adui-dioxus = "0.1.0"
```

然后导入并使用组件：

```rust
use adui_dioxus::{Button, ButtonType, ThemeProvider};

fn app() -> Element {
    rsx! {
        ThemeProvider {
            Button {
                r#type: ButtonType::Primary,
                "点击我"
            }
        }
    }
}
```

## 文档结构

每个组件都有自己的文档页面，包含：

- **概述**：组件的用途和功能
- **API 参考**：完整的属性、事件和方法文档
- **使用示例**：常见用例的代码示例
- **使用场景**：使用组件的典型场景
- **与 Ant Design 6.0.0 的差异**：主要差异和限制

## 语言支持

文档提供英文（默认）和中文两种版本。中文文档文件以 `_CN.md` 结尾。

## 贡献

这是一个实验性项目。欢迎贡献和反馈。请参考主项目仓库了解贡献指南。

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](../LICENSE) 文件。

