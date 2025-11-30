//! Ant Design flavored components and theme utilities for Dioxus 0.7+.
//! Modules are organized by theme primitives and component implementations.
pub mod components;
pub mod foundation;
pub mod theme;

pub use components::TreeNode;
pub use components::affix::{Affix, AffixProps};
pub use components::alert::{Alert, AlertProps, AlertType};
pub use components::anchor::{
    Anchor, AnchorClickInfo, AnchorDirection, AnchorLinkItem, AnchorProps,
};
pub use components::app::{
    App, AppContextValue, AppProps, ModalApi, use_app, use_message, use_modal, use_notification,
};
pub use components::auto_complete::{AutoComplete, AutoCompleteProps};
pub use components::avatar::{
    Avatar, AvatarGroup, AvatarGroupProps, AvatarProps, AvatarShape, AvatarSize,
};
pub use components::badge::{
    Badge, BadgeColor, BadgeProps, BadgeSize, BadgeStatus, Ribbon, RibbonPlacement, RibbonProps,
};
pub use components::breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbProps};
pub use components::button::{
    Button, ButtonColor, ButtonGroup, ButtonGroupProps, ButtonHtmlType, ButtonIconPlacement,
    ButtonProps, ButtonShape, ButtonSize, ButtonType, ButtonVariant,
};
pub use components::calendar::{Calendar, CalendarDate, CalendarMode, CalendarProps};
pub use components::card::{Card, CardProps};
pub use components::cascader::{Cascader, CascaderProps};
pub use components::checkbox::{Checkbox, CheckboxGroup, CheckboxGroupProps, CheckboxProps};
pub use components::collapse::{
    Collapse, CollapsePanel, CollapseProps, CollapseSize, CollapsibleType, ExpandIconPlacement,
    ExpandIconRenderFn,
};
pub use components::config_provider::{
    ComponentSize, ConfigContextValue, ConfigProvider, ConfigProviderProps, Locale, use_config,
};
pub use components::date_picker::{
    DateGenerateConfig, DatePicker, DatePickerProps, DateRangeValue, DateValue, RangePicker,
    RangePickerProps, ShowTimeConfig,
};
pub use components::descriptions::{
    ColumnConfig, Descriptions, DescriptionsItem, DescriptionsLayout, DescriptionsProps,
    DescriptionsSize, ResponsiveColumn,
};
pub use components::divider::{Divider, DividerOrientation, DividerProps};
pub use components::drawer::{Drawer, DrawerPlacement, DrawerProps};
pub use components::dropdown::{
    Dropdown, DropdownItem, DropdownPlacement, DropdownProps, DropdownTrigger,
};
pub use components::empty::{Empty, EmptyImage, EmptyProps};
pub use components::flex::{
    Flex, FlexAlign, FlexComponent, FlexConfigProvider, FlexDirection, FlexGap, FlexJustify,
    FlexOrientation, FlexProps, FlexSharedConfig, FlexWrap,
};
pub use components::float_button::{
    BackTop, BadgeConfig, FloatButton, FloatButtonGroup, FloatButtonProps, FloatButtonPurePanel,
    FloatButtonPurePanelProps, FloatButtonShape, FloatButtonType,
};
pub use components::form::{
    ControlSize, FeedbackIcons, Form, FormHandle, FormItem, FormItemProps, FormLayout, FormList,
    FormListContext, FormListItemMeta, FormListProps, LabelAlign, RequiredMark,
    ScrollToFirstErrorConfig, use_form, use_form_item_control, use_form_list,
};
pub use components::grid::{
    Col, ColProps, ColResponsive, ColSize, ResponsiveGutter, ResponsiveValue, Row, RowAlign,
    RowGutter, RowJustify, RowProps,
};
pub use components::icon::{Icon, IconKind, IconProps};
pub use components::input::{
    Input, InputProps, InputSize, OTP, OTPProps, Password, PasswordProps, Search, SearchProps,
    TextArea, TextAreaProps,
};
pub use components::layout::{
    Content, Footer, Header, Layout, LayoutProps, Sider, SiderProps, SiderTheme,
};
pub use components::list::List;
pub use components::masonry::{Masonry, MasonryProps, MasonryResponsive};
pub use components::menu::{Menu, MenuItemNode, MenuMode, MenuProps};
pub use components::message::{MessageApi, MessageConfig, MessageType};
pub use components::modal::{
    ClosableConfig, FooterExtra, MaskConfig, Modal, ModalProps, ModalType,
};
pub use components::notification::{
    NotificationApi, NotificationConfig, NotificationPlacement, NotificationType,
};
pub use components::pagination::{Pagination, PaginationProps};
pub use components::popconfirm::{Popconfirm, PopconfirmProps};
pub use components::popover::{Popover, PopoverProps};
pub use components::progress::{Progress, ProgressProps, ProgressStatus, ProgressType};
pub use components::qrcode::{QRCode, QRCodeErrorLevel, QRCodeProps, QRCodeStatus, QRCodeType};
pub use components::radio::{Radio, RadioButton, RadioGroup, RadioGroupProps, RadioProps};
pub use components::result::{Result, ResultProps, ResultStatus};
pub use components::select::{
    PublicSelectOption as SelectOption, Select, SelectMode, SelectPlacement, SelectProps,
};
pub use components::skeleton::Skeleton;
pub use components::space::{Space, SpaceAlign, SpaceDirection, SpaceProps, SpaceSize};
pub use components::spin::{Spin, SpinProps, SpinSize};
pub use components::splitter::{
    Splitter, SplitterOrientation, SplitterPane, SplitterPaneProps, SplitterProps,
};
pub use components::statistic::Statistic;
pub use components::steps::{StepItem, StepStatus, Steps, StepsDirection, StepsProps};
pub use components::switch::{Switch, SwitchProps, SwitchSize};
pub use components::table::{
    ColumnAlign, ColumnFilter, ColumnFixed, ColumnRenderFn, ExpandableConfig, RowClassNameFn,
    RowPropsFn, RowSelection, SelectionType, SortOrder, StickyConfig, SummaryConfig, Table,
    TableChangeEvent, TableColumn, TableLocale, TablePaginationState, TableProps, TableScroll,
    TableSorterState,
};
pub use components::tabs::{
    TabEditAction, TabItem, TabPlacement, Tabs, TabsProps, TabsType,
};
pub use components::tag::{Tag, TagColor, TagProps};
pub use components::time_picker::{TimePicker, TimePickerProps, TimeValue};
pub use components::timeline::{
    Timeline, TimelineColor, TimelineItem, TimelineMode, TimelineOrientation, TimelineProps,
};
pub use components::tooltip::{Tooltip, TooltipPlacement, TooltipProps, TooltipTrigger};
pub use components::tour::{Tour, TourProps, TourStep, TourType};
pub use components::tree::{
    DirectoryTree, DirectoryTreeProps, DraggableConfig, FieldNames, FlatTreeNode, Tree, TreeProps,
};
pub use components::tree_select::{TreeSelect, TreeSelectProps};
pub use components::typography::{
    Paragraph, ParagraphProps, Text, TextProps, TextType, Title, TitleLevel, TitleProps,
    TypographyCopyable, TypographyEditable, TypographyEllipsis,
};
pub use components::upload::{
    AcceptConfig, ItemActions, Upload, UploadChangeInfo, UploadFile, UploadHttpMethod,
    UploadListConfig, UploadListType, UploadLocale, UploadProgressConfig, UploadProps,
    UploadRequestOptions, UploadStatus,
};
pub use components::watermark::{Watermark, WatermarkFont, WatermarkProps};
pub use theme::{
    THEME_BASE_STYLE, Theme, ThemeHandle, ThemeMode, ThemeProvider, ThemeTokens, use_theme,
};

// Foundation exports
pub use foundation::{
    // Semantic system
    ClassListExt, SemanticClassNames, SemanticStyles, StyleStringExt,
    // Semantic slot enums
    AnchorSemantic, ButtonSemantic, CollapseSemantic, DescriptionsSemantic, FormSemantic,
    InputSemantic, MessageSemantic, ModalSemantic, NotificationSemantic, SelectPopupSemantic,
    SelectSemantic, TablePartSemantic, TableSemantic, TabsSemantic, TimelineSemantic,
    // Type aliases
    ButtonClassNames, ButtonStyles, CollapseClassNames, CollapseStyles, FormClassNames,
    FormStyles, InputClassNames, InputStyles, ModalClassNames, ModalStyles, SelectClassNames,
    SelectStyles, TableClassNames, TableStyles, TabsClassNames, TabsStyles,
    // Variant system
    Variant, variant_from_bordered,
};
