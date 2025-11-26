# 测试记录（Plan 0001）

## 自动化
- `cargo fmt --all`：完成，无额外 diff。
- `cargo clippy --all-targets --all-features`：持续提示 Form/Upload 旧逻辑与 demo 中的 `collapsible_if` / `clone_on_copy` / `derivable_impls` / `too_many_arguments` 等警告，共 34 项，均记录在此，等待对应模块下一轮重构时集中清理。
- `cargo test`：库内 20 个单测 + `tests/theme_spec.rs` 2 个集成测全部通过，新覆盖 theme tokens CSS 导出、Button/Floating Button metrics/visuals、Typography ellipsis 判定与键盘激活逻辑。

```
running 20 tests
test components::button::tests::detects_two_cjk_characters ... ok
test components::button::tests::metrics_respect_size_and_shape ... ok
test components::button::tests::visuals_follow_variant_and_tone_rules ... ok
test components::float_button::tests::metrics_reflect_shape_rules ... ok
test components::float_button::tests::visuals_switch_between_danger_and_default ... ok
test components::grid::tests::responsive_col_rules_emits_breakpoints ... ok
test components::grid::tests::responsive_row_rules_emits_media_queries ... ok
test components::grid::tests::row_component_renders_expected_class ... ok
test components::layout_utils::tests::compose_gap_style_merges_values ... ok
test components::layout_utils::tests::push_gap_preset_adds_suffix ... ok
test components::space::tests::collect_children_extracts_dynamic_fragment ... ok
test components::space::tests::collect_children_handles_single_node ... ok
test components::space::tests::collect_children_handles_static_roots_with_fallback ... ok
test components::typography::tests::ellipsis_flags_follow_expand_state ... ok
test components::typography::tests::key_activation_matches_enter_and_space ... ok
test components::typography::tests::level_index_maps_levels ... ok
test components::typography::tests::resolve_color_respects_tone_and_disabled_state ... ok
test components::typography::tests::text_decoration_combines_flags ... ok
test theme::tests::tokens_to_css_vars_emits_expected_variables ... ok
test theme::tests::tokens_to_css_vars_reflect_custom_updates ... ok

running 2 tests
test custom_mode_defaults_to_light_tokens ... ok
test default_theme_modes_resolve ... ok
```

## 手动验证
- Button/Flex/Grid/Layout/FloatButton/Space/Typography 示例持续运行核对：color/variant、ButtonGroup 继承、loading delay、FloatButton Badge/BackTop/Square 文案、Flex gap/Space split、Grid 响应式 gutter、Typography copyable/ellipsis/Editable、Layout Sider 折叠 + Zero Width Trigger。
- Form/Upload demo：检查 required/min/max/pattern 规则、`reset_fields`、拖拽/多文件/`before_upload`/XHR 进度（与上一轮验证保持一致）。
