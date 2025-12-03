# adui-dioxus

## 介绍

adui-dioxus 是一个基于 Dioxus 的 UI 组件库，提供了丰富的组件和样式，帮助开发者快速构建跨平台的 Web 和移动应用。

这是一个**实验性项目**，使用 **Vibe Coding** 将 Ant Design 6.0.0 移植到 Dioxus。它从 Ant Design UI 库（https://github.com/ant-design/ant-design）中提取组件，并适配到 Rust/Dioxus 生态系统。该库继承了 Ant Design 的设计理念和组件风格，同时结合了 Dioxus 的性能和灵活性，为开发者提供了更加高效和便捷的开发体验。

## 项目状态

这是 Ant Design 6.0.0 到 Dioxus 的实验性移植。该库基于 **Dioxus 0.7+** 构建，包含完整的组件集合：

### 核心功能
- **主题系统**：Ant Design 6.x 风格的令牌与主题上下文（明/暗预设，CSS 变量导出）
- **配置提供者**：全局配置和主题管理

### 布局组件
- **Layout**：页面布局容器，包含 Header、Footer、Sider 和 Content
- **Grid**：24 列网格系统，用于响应式布局
- **Flex**：弹性盒子布局组件，支持 gap 预设和 wrap
- **Space**：间距组件，用于排列元素
- **Divider**：分割线，用于分隔内容
- **Splitter**：可调整大小的分割面板，带可拖拽手柄
- **Masonry**：响应式瀑布流布局，用于卡片和项目

### 通用组件
- **Button**：支持 type/size/shape/danger/ghost/loading/block/icon/href
- **FloatButton**：悬浮按钮，支持圆/方形、primary/default、danger、tooltip、可配置位置
- **Icon**：内置常用图标集（plus/minus/check/close/info/question/search/arrow/loading），支持旋转、大小、颜色
- **Typography**：Title/Text/Paragraph，支持 tone（default/secondary/success/warning/danger/disabled）、strong/italic/underline/delete/code/mark、ellipsis（单/多行 + 展开）、copyable、editable、禁用态语义
- **Affix**：固钉组件
- **Breadcrumb**：面包屑导航
- **Dropdown**：下拉菜单组件
- **Menu**：导航菜单，支持水平和垂直模式
- **Pagination**：分页组件
- **Steps**：步骤指示器，用于流程
- **Tabs**：标签页导航组件
- **Anchor**：锚点导航，用于长页面

### 数据录入
- **Form**：`Form`/`FormItem`/`use_form_item_control`，支持 required/min/max/pattern/custom rule、布局控制、必填标记、上下文 Hook
- **Input**：文本输入，支持多种变体（Password、Search、OTP）
- **InputNumber**：数字输入，带步进器控制
- **TextArea**：多行文本输入
- **Select**：下拉选择器，支持搜索和多选
- **TreeSelect**：树形选择器
- **Cascader**：级联选择组件
- **AutoComplete**：自动完成输入组件
- **Checkbox**：复选框和复选框组
- **Radio**：单选框和单选框组
- **Switch**：开关组件
- **Slider**：滑动输入条组件
- **Rate**：评分组件
- **Upload**：点击选择/拖拽上传、列表（text/picture/picture-card）、`before_upload`、XHR 上传进度/abort、受控/非受控列表
- **DatePicker**：日期选择器，支持范围选择
- **TimePicker**：时间选择器组件
- **Calendar**：日历组件
- **ColorPicker**：颜色选择器组件
- **Mentions**：提及输入组件
- **Segmented**：分段控制器组件

### 数据展示
- **Table**：高级数据表格，支持排序、筛选、分页和选择
- **Tag**：标签组件，支持多种颜色
- **Badge**：徽标和缎带组件
- **Card**：卡片容器组件
- **Carousel**：走马灯/轮播组件
- **Collapse**：折叠面板组件
- **Timeline**：时间轴组件
- **Tree**：树形组件，支持目录树
- **Transfer**：穿梭框组件
- **Descriptions**：描述列表组件
- **Empty**：空状态组件
- **List**：列表组件
- **Statistic**：统计数值展示组件
- **QRCode**：二维码生成组件
- **Avatar**：头像组件，支持组
- **Image**：图片组件，支持预览
- **Skeleton**：骨架屏加载组件
- **Progress**：进度条组件
- **Result**：结果页组件
- **Watermark**：水印组件

### 反馈
- **Alert**：警告提示组件
- **Message**：全局消息提示
- **Notification**：通知提醒组件
- **Modal**：模态对话框组件
- **Drawer**：抽屉组件
- **Popconfirm**：气泡确认框组件
- **Popover**：气泡卡片组件
- **Tooltip**：文字提示组件
- **Spin**：加载中组件
- **Progress**：进度指示器组件
- **Skeleton**：骨架屏占位组件

### 其他
- **App**：应用级上下文提供者，用于 message、modal 和 notification
- **Tour**：漫游式引导组件
- **BackTop**：回到顶部按钮

## 安装

使用 Cargo 安装 adui-dioxus：

```bash
cargo add adui-dioxus
```

或者在你的 `Cargo.toml` 文件中手动添加依赖：

```toml
[dependencies]
adui-dioxus = "0.1.2"
```

## 文档

完整的文档位于 `docs/` 目录：

- **[English Documentation Index](docs/README.md)** - 完整的英文组件文档
- **[中文文档索引](docs/README_CN.md)** - 完整的中文组件文档

每个组件都有详细的文档，包括：
- API 参考（所有属性、事件和方法）
- 使用示例
- 使用场景
- 与 Ant Design 6.0.0 的差异

## 本地开发

### 环境要求

在开始开发之前，你需要设置开发环境。本节将指导你安装必要的工具。

#### 前置条件

首先，确保你已经安装了 Rust 工具链。如果还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 并按照你平台的安装说明进行操作。

验证你的 Rust 安装：

```bash
rustc --version
cargo --version
```

#### 安装 Rust WASM Target

Dioxus 应用程序需要编译为 WebAssembly (WASM) 以便在 Web 上部署。你需要将 `wasm32-unknown-unknown` target 添加到你的 Rust 工具链中。

安装 WASM target：

```bash
rustup target add wasm32-unknown-unknown
```

此命令会下载并安装将 Rust 代码编译为 WebAssembly 所需的工具链组件。

验证安装：

```bash
rustup target list --installed | grep wasm32-unknown-unknown
```

如果命令输出了 `wasm32-unknown-unknown`，说明 target 已成功安装。

#### 安装 Dioxus CLI

Dioxus CLI (`dx`) 提供了构建、运行和打包 Dioxus 应用程序的命令。

**推荐方式（更快）**：使用 `cargo-binstall` 安装，可以从预编译的二进制文件快速安装：

首先，安装 `cargo-binstall`：

```bash
cargo install cargo-binstall
```

然后安装 Dioxus CLI：

```bash
cargo binstall dioxus-cli
```

**备选方式**：使用 Cargo 安装（较慢，从源代码编译）：

```bash
cargo install dioxus-cli
```

**注意**：推荐使用 `cargo-binstall`，因为它安装预编译的二进制文件，比从源代码编译快得多。传统的 `cargo install` 方法可能需要几分钟时间，因为它会从源代码编译 CLI。请确保你有稳定的网络连接。

安装完成后，验证 CLI 是否可用：

```bash
dx --version
```

你应该能看到 Dioxus CLI 的版本号。如果找不到命令，请确保 Cargo 的二进制目录在你的系统 PATH 中：

- **Linux/macOS**: `~/.cargo/bin`
- **Windows**: `%USERPROFILE%\.cargo\bin`

#### 故障排除

**问题**：安装后找不到 `dx` 命令
- **解决方案**：将 `~/.cargo/bin`（Windows 上为 `%USERPROFILE%\.cargo\bin`）添加到你的 PATH 环境变量中，然后重启终端。

**问题**：WASM target 安装失败
- **解决方案**：确保你有稳定的网络连接，先运行 `rustup update`，然后重试 target 安装。

**问题**：Dioxus CLI 安装时间过长或失败
- **解决方案**：使用 `cargo-binstall` 从预编译的二进制文件快速安装。首先使用 `cargo install cargo-binstall` 安装 `cargo-binstall`，然后使用 `cargo binstall dioxus-cli` 安装 Dioxus CLI。或者，确保你有足够的磁盘空间和稳定的网络连接，并尝试使用 `--locked` 标志安装：`cargo install dioxus-cli --locked`

### 构建与检查

```bash
cargo fmt && cargo clippy --all-targets --all-features && cargo test
```

### 运行示例

使用 Dioxus CLI 在浏览器中运行示例：

```bash
dx serve --example <示例名称>
```

可用的示例包括：

- `button_demo` - 按钮组件，带主题切换
- `float_button_demo` - 悬浮按钮示例
- `icon_demo` - 图标展示
- `typography_demo` - 排版组件
- `layout_demo` - 布局组件（Layout、Divider、Flex、Space、Grid、Masonry、Splitter）
- `flex_space_demo` - Flex 和 Space 组件
- `grid_demo` - 网格系统示例
- `form_demo` - 表单验证和控件
- `upload_demo` - 文件上传示例
- `table_demo` - 数据表格示例
- `menu_demo` - 导航菜单
- `tabs_demo` - 标签页导航
- `modal_demo` - 模态对话框
- `drawer_demo` - 抽屉组件
- `select_demo` - 选择器组件
- `date_picker_demo` - 日期选择器
- `input_demo` - 输入框变体
- `card_demo` - 卡片组件
- `badge_demo` - 徽标组件
- `avatar_demo` - 头像组件
- `alert_demo` - 警告提示组件
- `message_demo` - 消息提示
- `notification_demo` - 通知组件
- `tooltip_demo` - 文字提示组件
- `popover_demo` - 气泡卡片组件
- `progress_demo` - 进度指示器
- `spin_demo` - 加载中组件
- `skeleton_demo` - 骨架屏加载
- `steps_demo` - 步骤条组件
- `timeline_demo` - 时间轴组件
- `tree_demo` - 树形组件
- `tree_select_demo` - 树形选择器
- `transfer_demo` - 穿梭框
- `pagination_demo` - 分页
- `breadcrumb_demo` - 面包屑导航
- `anchor_demo` - 锚点导航
- `affix_demo` - 固钉组件
- `dropdown_demo` - 下拉菜单
- `checkbox_demo` - 复选框组件
- `radio_demo` - 单选框组件
- `switch_demo` - 开关组件
- `slider_demo` - 滑动输入条组件
- `rate_demo` - 评分组件
- `input_number_demo` - 数字输入框
- `cascader_demo` - 级联选择组件
- `auto_complete_demo` - 自动完成
- `color_picker_demo` - 颜色选择器
- `mentions_demo` - 提及输入
- `segmented_demo` - 分段控制器
- `descriptions_demo` - 描述列表组件
- `empty_demo` - 空状态
- `list_demo` - 列表组件
- `statistic_demo` - 统计数值展示
- `qrcode_demo` - 二维码生成器
- `image_demo` - 图片组件
- `carousel_demo` - 走马灯组件
- `collapse_demo` - 折叠面板组件
- `tag_demo` - 标签组件
- `result_demo` - 结果页
- `watermark_demo` - 水印组件
- `tour_demo` - 漫游式引导
- `config_provider_demo` - 配置提供者
- `app_demo` - 应用上下文
- `dashboard_demo` - 仪表盘示例
- `landing_page_demo` - 落地页示例
- `login_demo` - 登录页示例
- `register_demo` - 注册页示例
- `settings_demo` - 设置页示例

## 贡献

我们欢迎贡献！请阅读我们的[仓库指南](AGENTS.md)了解开发工作流、代码规范和贡献要求。

关键要点：
- 遵循 Rust 命名规范和代码风格
- 为新组件添加测试
- 更新文档（英文和中文）
- 为新组件创建示例
- 确保所有检查通过：`cargo fmt && cargo clippy --all-targets --all-features && cargo test`

## 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 注意

这是一个实验性项目。某些功能可能与原始 Ant Design 实现有所不同，API 可能会随着项目的成熟而演变。请参考组件文档了解具体的差异和限制。

