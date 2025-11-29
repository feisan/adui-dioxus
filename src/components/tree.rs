//! Tree component for displaying hierarchical data.
//!
//! # Features
//! - Expand/collapse node control
//! - Single and multiple selection
//! - Checkable mode with checkbox indicators
//! - Keyboard navigation
//! - Optional show line mode
//!
//! # Example
//! ```rust,ignore
//! use adui_dioxus::components::tree::{Tree, TreeProps};
//! use adui_dioxus::components::select_base::TreeNode;
//!
//! let data = vec![
//!     TreeNode {
//!         key: "parent".into(),
//!         label: "Parent".into(),
//!         disabled: false,
//!         children: vec![
//!             TreeNode { key: "child1".into(), label: "Child 1".into(), disabled: false, children: vec![] },
//!             TreeNode { key: "child2".into(), label: "Child 2".into(), disabled: false, children: vec![] },
//!         ],
//!     },
//! ];
//!
//! rsx! {
//!     Tree {
//!         tree_data: data,
//!         checkable: true,
//!         on_check: move |keys| { /* handle checked keys */ },
//!     }
//! }
//! ```

use crate::components::config_provider::use_config;
use crate::components::select_base::{OptionKey, TreeNode};
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Internal flattened representation of a tree node for rendering.
#[derive(Clone, Debug)]
pub struct FlatTreeNode {
    pub key: OptionKey,
    pub label: String,
    pub disabled: bool,
    pub depth: usize,
    pub has_children: bool,
    pub parent_key: Option<OptionKey>,
    /// Whether this node is the last child at its level.
    pub is_last: bool,
    /// For each ancestor level (0..depth), whether that ancestor was the last child.
    /// Used to determine whether to draw vertical lines at each indent level.
    pub ancestor_is_last: Vec<bool>,
}

/// Flatten tree nodes into a linear list with depth information.
///
/// This is used for rendering and keyboard navigation while preserving
/// hierarchy through depth levels.
pub fn flatten_tree(
    nodes: &[TreeNode],
    depth: usize,
    parent_key: Option<&str>,
    out: &mut Vec<FlatTreeNode>,
) {
    flatten_tree_with_last(nodes, depth, parent_key, out, &[]);
}

fn flatten_tree_with_last(
    nodes: &[TreeNode],
    depth: usize,
    parent_key: Option<&str>,
    out: &mut Vec<FlatTreeNode>,
    ancestor_is_last: &[bool],
) {
    let len = nodes.len();
    for (idx, node) in nodes.iter().enumerate() {
        let is_last = idx == len - 1;
        out.push(FlatTreeNode {
            key: node.key.clone(),
            label: node.label.clone(),
            disabled: node.disabled,
            depth,
            has_children: !node.children.is_empty(),
            parent_key: parent_key.map(|s| s.to_string()),
            is_last,
            ancestor_is_last: ancestor_is_last.to_vec(),
        });
        if !node.children.is_empty() {
            let mut next_ancestor_is_last = ancestor_is_last.to_vec();
            next_ancestor_is_last.push(is_last);
            flatten_tree_with_last(
                &node.children,
                depth + 1,
                Some(&node.key),
                out,
                &next_ancestor_is_last,
            );
        }
    }
}

/// Collect all descendant keys of a node (including the node itself).
fn collect_descendant_keys(nodes: &[TreeNode], target_key: &str) -> Vec<OptionKey> {
    let mut result = Vec::new();
    collect_descendant_keys_recursive(nodes, target_key, &mut result, false);
    result
}

fn collect_descendant_keys_recursive(
    nodes: &[TreeNode],
    target_key: &str,
    out: &mut Vec<OptionKey>,
    collecting: bool,
) -> bool {
    for node in nodes {
        let is_target = node.key == target_key;
        let should_collect = collecting || is_target;

        if should_collect {
            out.push(node.key.clone());
        }

        if !node.children.is_empty() {
            let found =
                collect_descendant_keys_recursive(&node.children, target_key, out, should_collect);
            if found && !collecting {
                return true;
            }
        }

        if is_target {
            return true;
        }
    }
    false
}

/// Collect all parent keys of a node.
fn collect_parent_keys(flat_nodes: &[FlatTreeNode], target_key: &str) -> Vec<OptionKey> {
    let mut result = Vec::new();
    let mut current_key = Some(target_key.to_string());

    while let Some(key) = current_key {
        if let Some(node) = flat_nodes.iter().find(|n| n.key == key) {
            if let Some(parent) = &node.parent_key {
                result.push(parent.clone());
                current_key = Some(parent.clone());
            } else {
                current_key = None;
            }
        } else {
            current_key = None;
        }
    }

    result
}

/// Props for the Tree component.
#[derive(Props, Clone, PartialEq)]
pub struct TreeProps {
    /// Tree data source.
    #[props(optional)]
    pub tree_data: Option<Vec<TreeNode>>,

    // --- Expand control ---
    /// Controlled expanded keys.
    #[props(optional)]
    pub expanded_keys: Option<Vec<String>>,
    /// Default expanded keys (uncontrolled mode).
    #[props(optional)]
    pub default_expanded_keys: Option<Vec<String>>,
    /// Expand all nodes by default.
    #[props(default)]
    pub default_expand_all: bool,
    /// Auto expand parent nodes when children are expanded.
    #[props(default = true)]
    pub auto_expand_parent: bool,
    /// Callback when expand keys change.
    #[props(optional)]
    pub on_expand: Option<EventHandler<Vec<String>>>,

    // --- Selection control ---
    /// Controlled selected keys.
    #[props(optional)]
    pub selected_keys: Option<Vec<String>>,
    /// Default selected keys (uncontrolled mode).
    #[props(optional)]
    pub default_selected_keys: Option<Vec<String>>,
    /// Whether nodes are selectable.
    #[props(default = true)]
    pub selectable: bool,
    /// Allow multiple selection.
    #[props(default)]
    pub multiple: bool,
    /// Callback when selected keys change.
    #[props(optional)]
    pub on_select: Option<EventHandler<Vec<String>>>,

    // --- Check control (checkable mode) ---
    /// Show checkbox next to nodes.
    #[props(default)]
    pub checkable: bool,
    /// Controlled checked keys.
    #[props(optional)]
    pub checked_keys: Option<Vec<String>>,
    /// Default checked keys (uncontrolled mode).
    #[props(optional)]
    pub default_checked_keys: Option<Vec<String>>,
    /// Check strictly (parent and child are independent).
    #[props(default)]
    pub check_strictly: bool,
    /// Callback when checked keys change.
    #[props(optional)]
    pub on_check: Option<EventHandler<Vec<String>>>,

    // --- Visual options ---
    /// Show connecting lines between nodes.
    #[props(default)]
    pub show_line: bool,
    /// Show icon next to nodes.
    #[props(default)]
    pub show_icon: bool,
    /// Block node (full-width clickable area).
    #[props(default)]
    pub block_node: bool,
    /// Disable the entire tree.
    #[props(default)]
    pub disabled: bool,

    // --- Styling ---
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
}

/// Ant Design flavored Tree component.
#[component]
pub fn Tree(props: TreeProps) -> Element {
    let TreeProps {
        tree_data,
        expanded_keys,
        default_expanded_keys,
        default_expand_all,
        auto_expand_parent,
        on_expand,
        selected_keys,
        default_selected_keys,
        selectable,
        multiple,
        on_select,
        checkable,
        checked_keys,
        default_checked_keys,
        check_strictly,
        on_check,
        show_line,
        show_icon,
        block_node,
        disabled,
        class,
        style,
    } = props;

    let config = use_config();
    let theme = use_theme();
    let tokens = theme.tokens();

    let is_disabled = disabled || config.disabled;

    // Prepare tree data
    let nodes: Vec<TreeNode> = tree_data.unwrap_or_default();

    // Flatten tree for rendering
    let mut flat_nodes: Vec<FlatTreeNode> = Vec::new();
    flatten_tree(&nodes, 0, None, &mut flat_nodes);

    // Collect all keys for default_expand_all
    let all_parent_keys: Vec<String> = flat_nodes
        .iter()
        .filter(|n| n.has_children)
        .map(|n| n.key.clone())
        .collect();

    // --- Expand state ---
    let initial_expanded = if default_expand_all {
        all_parent_keys.clone()
    } else {
        default_expanded_keys.unwrap_or_default()
    };
    let internal_expanded: Signal<Vec<String>> = use_signal(|| initial_expanded);

    let is_expand_controlled = expanded_keys.is_some();
    let current_expanded = if is_expand_controlled {
        expanded_keys.clone().unwrap_or_default()
    } else {
        internal_expanded.read().clone()
    };

    // --- Selection state ---
    let initial_selected = default_selected_keys.unwrap_or_default();
    let internal_selected: Signal<Vec<String>> = use_signal(|| initial_selected);

    let is_select_controlled = selected_keys.is_some();
    let current_selected = if is_select_controlled {
        selected_keys.clone().unwrap_or_default()
    } else {
        internal_selected.read().clone()
    };

    // --- Checked state ---
    let initial_checked = default_checked_keys.unwrap_or_default();
    let internal_checked: Signal<Vec<String>> = use_signal(|| initial_checked);

    let is_check_controlled = checked_keys.is_some();
    let current_checked = if is_check_controlled {
        checked_keys.clone().unwrap_or_default()
    } else {
        internal_checked.read().clone()
    };

    // Active index for keyboard navigation
    let active_index: Signal<Option<usize>> = use_signal(|| None);

    // Filter visible nodes based on expanded state
    let visible_nodes: Vec<FlatTreeNode> = {
        let mut result = Vec::new();
        let mut skip_depth: Option<usize> = None;

        for node in &flat_nodes {
            // If we're skipping collapsed subtree
            if let Some(sd) = skip_depth {
                if node.depth > sd {
                    continue;
                } else {
                    skip_depth = None;
                }
            }

            result.push(node.clone());

            // If this node has children but is not expanded, skip its subtree
            if node.has_children && !current_expanded.contains(&node.key) {
                skip_depth = Some(node.depth);
            }
        }

        result
    };

    // Build root classes
    let mut class_list = vec!["adui-tree".to_string()];
    if show_line {
        class_list.push("adui-tree-show-line".into());
    }
    if show_icon {
        class_list.push("adui-tree-show-icon".into());
    }
    if block_node {
        class_list.push("adui-tree-block-node".into());
    }
    if is_disabled {
        class_list.push("adui-tree-disabled".into());
    }
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");

    // Use primary color with low opacity for selected background
    let selected_bg = format!("{}1a", &tokens.color_primary[..7]); // Add 10% opacity
    let style_attr = format!(
        "--adui-tree-node-hover-bg: {}; --adui-tree-node-selected-bg: {}; {}",
        tokens.color_bg_base,
        selected_bg,
        style.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{class_attr}",
            style: "{style_attr}",
            role: "tree",
            tabindex: 0,
            onkeydown: {
                let visible_for_keydown = visible_nodes.clone();
                let nodes_for_keydown = nodes.clone();
                let flat_for_keydown = flat_nodes.clone();
                let on_expand_cb = on_expand;
                let on_select_cb = on_select;
                let on_check_cb = on_check;
                let current_expanded_for_keydown = current_expanded.clone();
                let current_selected_for_keydown = current_selected.clone();
                let current_checked_for_keydown = current_checked.clone();
                move |evt: KeyboardEvent| {
                    if is_disabled {
                        return;
                    }
                    use dioxus::prelude::Key;

                    let nodes_len = visible_for_keydown.len();
                    if nodes_len == 0 {
                        return;
                    }

                    let mut active = active_index;

                    match evt.key() {
                        Key::ArrowDown => {
                            evt.prevent_default();
                            let current = *active.read();
                            let next = match current {
                                None => Some(0),
                                Some(idx) => Some((idx + 1) % nodes_len),
                            };
                            active.set(next);
                        }
                        Key::ArrowUp => {
                            evt.prevent_default();
                            let current = *active.read();
                            let next = match current {
                                None => Some(nodes_len.saturating_sub(1)),
                                Some(idx) => Some((idx + nodes_len - 1) % nodes_len),
                            };
                            active.set(next);
                        }
                        Key::ArrowRight => {
                            evt.prevent_default();
                            if let Some(idx) = *active.read() {
                                if idx < visible_for_keydown.len() {
                                    let node = &visible_for_keydown[idx];
                                    if node.has_children && !current_expanded_for_keydown.contains(&node.key) {
                                        // Expand node
                                        let mut next_expanded = current_expanded_for_keydown.clone();
                                        next_expanded.push(node.key.clone());
                                        if let Some(cb) = on_expand_cb {
                                            cb.call(next_expanded.clone());
                                        }
                                        if !is_expand_controlled {
                                            let mut signal = internal_expanded;
                                            signal.set(next_expanded);
                                        }
                                    }
                                }
                            }
                        }
                        Key::ArrowLeft => {
                            evt.prevent_default();
                            if let Some(idx) = *active.read() {
                                if idx < visible_for_keydown.len() {
                                    let node = &visible_for_keydown[idx];
                                    if node.has_children && current_expanded_for_keydown.contains(&node.key) {
                                        // Collapse node
                                        let next_expanded: Vec<String> = current_expanded_for_keydown
                                            .iter()
                                            .filter(|k| *k != &node.key)
                                            .cloned()
                                            .collect();
                                        if let Some(cb) = on_expand_cb {
                                            cb.call(next_expanded.clone());
                                        }
                                        if !is_expand_controlled {
                                            let mut signal = internal_expanded;
                                            signal.set(next_expanded);
                                        }
                                    }
                                }
                            }
                        }
                        Key::Enter => {
                            evt.prevent_default();
                            if let Some(idx) = *active.read() {
                                if idx < visible_for_keydown.len() {
                                    let node = &visible_for_keydown[idx];
                                    if node.disabled {
                                        return;
                                    }

                                    if checkable {
                                        // Toggle check
                                        let next_checked = toggle_check(
                                            &current_checked_for_keydown,
                                            &node.key,
                                            check_strictly,
                                            &nodes_for_keydown,
                                            &flat_for_keydown,
                                        );
                                        if let Some(cb) = on_check_cb {
                                            cb.call(next_checked.clone());
                                        }
                                        if !is_check_controlled {
                                            let mut signal = internal_checked;
                                            signal.set(next_checked);
                                        }
                                    } else if selectable {
                                        // Toggle selection
                                        let next_selected = toggle_selection(
                                            &current_selected_for_keydown,
                                            &node.key,
                                            multiple,
                                        );
                                        if let Some(cb) = on_select_cb {
                                            cb.call(next_selected.clone());
                                        }
                                        if !is_select_controlled {
                                            let mut signal = internal_selected;
                                            signal.set(next_selected);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            },
            ul { class: "adui-tree-list",
                {visible_nodes.iter().enumerate().map(|(index, node)| {
                    let key = node.key.clone();
                    let label = node.label.clone();
                    let depth = node.depth;
                    let has_children = node.has_children;
                    let node_disabled = node.disabled || is_disabled;
                    let is_last = node.is_last;
                    let ancestor_is_last = node.ancestor_is_last.clone();

                    let is_expanded = current_expanded.contains(&key);
                    let is_selected = current_selected.contains(&key);
                    let is_checked = current_checked.contains(&key);
                    let is_active = (*active_index.read()).map(|i| i == index).unwrap_or(false);

                    // Check indeterminate state (some but not all children checked)
                    let is_indeterminate = if checkable && !check_strictly && has_children {
                        let descendants = collect_descendant_keys(&nodes, &key);
                        let checked_count = descendants.iter().filter(|k| current_checked.contains(*k)).count();
                        checked_count > 0 && checked_count < descendants.len()
                    } else {
                        false
                    };

                    let on_expand_for_node = on_expand;
                    let on_select_for_node = on_select;
                    let on_check_for_node = on_check;
                    let current_expanded_for_node = current_expanded.clone();
                    let current_selected_for_node = current_selected.clone();
                    let current_checked_for_node = current_checked.clone();
                    let nodes_for_node = nodes.clone();
                    let flat_for_node = flat_nodes.clone();

                    rsx! {
                        li {
                            key: "{key}",
                            class: {
                                let mut classes = vec!["adui-tree-treenode".to_string()];
                                if is_selected {
                                    classes.push("adui-tree-treenode-selected".into());
                                }
                                if node_disabled {
                                    classes.push("adui-tree-treenode-disabled".into());
                                }
                                if is_active {
                                    classes.push("adui-tree-treenode-active".into());
                                }
                                classes.join(" ")
                            },
                            role: "treeitem",
                            "aria-selected": is_selected,
                            "aria-expanded": if has_children { is_expanded.to_string() } else { String::new() },
                            // Indent with optional lines
                            if show_line {
                                {(0..depth).map(|i| {
                                    // Show vertical line if ancestor at this level was NOT the last child
                                    let ancestor_was_last = ancestor_is_last.get(i).copied().unwrap_or(false);
                                    let show_vertical = !ancestor_was_last;
                                    rsx! {
                                        span {
                                            key: "{i}",
                                            class: "adui-tree-indent-unit",
                                            style: "display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 28px; position: relative;",
                                            if show_vertical {
                                                span {
                                                    style: "position: absolute; left: 11px; top: 0; bottom: 0; width: 1px; background: var(--adui-color-border, #d9d9d9);"
                                                }
                                            }
                                        }
                                    }
                                })}
                            } else {
                                {(0..depth).map(|i| {
                                    rsx! {
                                        span {
                                            key: "{i}",
                                            class: "adui-tree-indent-unit",
                                            style: "display: inline-block; width: 24px;"
                                        }
                                    }
                                })}
                            }
                            // Switcher (expand/collapse icon)
                            span {
                                class: {
                                    let mut classes = vec!["adui-tree-switcher".to_string()];
                                    if has_children {
                                        if is_expanded {
                                            classes.push("adui-tree-switcher-open".into());
                                        } else {
                                            classes.push("adui-tree-switcher-close".into());
                                        }
                                    } else {
                                        classes.push("adui-tree-switcher-leaf".into());
                                    }
                                    classes.join(" ")
                                },
                                style: if show_line {
                                    "display: inline-flex; align-items: center; justify-content: center; width: 24px; height: 28px; position: relative;"
                                } else {
                                    ""
                                },
                                onclick: {
                                    let key_for_expand = key.clone();
                                    let current_expanded_for_expand = current_expanded_for_node.clone();
                                    move |evt: MouseEvent| {
                                        evt.stop_propagation();
                                        if !has_children {
                                            return;
                                        }

                                        let next_expanded = if current_expanded_for_expand.contains(&key_for_expand) {
                                            current_expanded_for_expand
                                                .iter()
                                                .filter(|k| *k != &key_for_expand)
                                                .cloned()
                                                .collect()
                                        } else {
                                            let mut next = current_expanded_for_expand.clone();
                                            next.push(key_for_expand.clone());
                                            next
                                        };

                                        if let Some(cb) = on_expand_for_node {
                                            cb.call(next_expanded.clone());
                                        }
                                        if !is_expand_controlled {
                                            let mut signal = internal_expanded;
                                            signal.set(next_expanded);
                                        }
                                    }
                                },
                                if show_line {
                                    // Vertical line (top half for non-first items, full for non-last items)
                                    if depth > 0 {
                                        // Top half of vertical line
                                        span {
                                            style: "position: absolute; left: 11px; top: 0; height: calc(50% - 5px); width: 1px; background: var(--adui-color-border, #d9d9d9);"
                                        }
                                        // Bottom half (only if not last child)
                                        if !is_last {
                                            span {
                                                style: "position: absolute; left: 11px; top: calc(50% + 5px); bottom: 0; width: 1px; background: var(--adui-color-border, #d9d9d9);"
                                            }
                                        }
                                        // Horizontal connector line
                                        span {
                                            style: "position: absolute; left: 11px; top: 50%; width: 6px; height: 1px; background: var(--adui-color-border, #d9d9d9); transform: translateY(-50%);"
                                        }
                                    }
                                    // Show line style icons - bordered box
                                    if has_children {
                                        span {
                                            style: "position: relative; z-index: 1; display: inline-flex; align-items: center; justify-content: center; width: 10px; height: 10px; border: 1px solid var(--adui-color-border, #d9d9d9); border-radius: 2px; background: var(--adui-color-bg-container, #fff); font-size: 10px; line-height: 1; cursor: pointer; color: var(--adui-color-text-secondary, rgba(0,0,0,0.65));",
                                            if is_expanded { "−" } else { "+" }
                                        }
                                    } else if depth > 0 {
                                        // Leaf node - just extend horizontal line
                                        span {
                                            style: "position: absolute; left: 17px; top: 50%; width: 6px; height: 1px; background: var(--adui-color-border, #d9d9d9); transform: translateY(-50%);"
                                        }
                                    }
                                } else if has_children {
                                    {
                                        let rotate_deg = if is_expanded { 90 } else { 0 };
                                        rsx! {
                                            span {
                                                class: "adui-tree-switcher-icon",
                                                style: "display: inline-block; transition: transform 0.2s; transform: rotate({rotate_deg}deg);",
                                                "▶"
                                            }
                                        }
                                    }
                                }
                            }
                            // Checkbox (if checkable)
                            if checkable {
                                span {
                                    class: {
                                        let mut classes = vec!["adui-tree-checkbox".to_string()];
                                        if is_checked {
                                            classes.push("adui-tree-checkbox-checked".into());
                                        }
                                        if is_indeterminate {
                                            classes.push("adui-tree-checkbox-indeterminate".into());
                                        }
                                        if node_disabled {
                                            classes.push("adui-tree-checkbox-disabled".into());
                                        }
                                        classes.join(" ")
                                    },
                                    onclick: {
                                        let key_for_check = key.clone();
                                        let current_checked_for_check = current_checked_for_node.clone();
                                        let nodes_for_check = nodes_for_node.clone();
                                        let flat_for_check = flat_for_node.clone();
                                        move |evt: MouseEvent| {
                                            evt.stop_propagation();
                                            if node_disabled {
                                                return;
                                            }

                                            let next_checked = toggle_check(
                                                &current_checked_for_check,
                                                &key_for_check,
                                                check_strictly,
                                                &nodes_for_check,
                                                &flat_for_check,
                                            );

                                            if let Some(cb) = on_check_for_node {
                                                cb.call(next_checked.clone());
                                            }
                                            if !is_check_controlled {
                                                let mut signal = internal_checked;
                                                signal.set(next_checked);
                                            }
                                        }
                                    },
                                    span { class: "adui-tree-checkbox-inner" }
                                }
                            }
                            // Title
                            span {
                                class: {
                                    let mut classes = vec!["adui-tree-node-content-wrapper".to_string()];
                                    if is_selected {
                                        classes.push("adui-tree-node-selected".into());
                                    }
                                    classes.join(" ")
                                },
                                onclick: {
                                    let key_for_select = key.clone();
                                    let current_selected_for_select = current_selected_for_node.clone();
                                    move |_| {
                                        if node_disabled || !selectable {
                                            return;
                                        }

                                        let next_selected = toggle_selection(
                                            &current_selected_for_select,
                                            &key_for_select,
                                            multiple,
                                        );

                                        if let Some(cb) = on_select_for_node {
                                            cb.call(next_selected.clone());
                                        }
                                        if !is_select_controlled {
                                            let mut signal = internal_selected;
                                            signal.set(next_selected);
                                        }
                                    }
                                },
                                if show_icon {
                                    span {
                                        class: "adui-tree-iconEle",
                                        style: "margin-right: 4px;",
                                        if has_children {
                                            if is_expanded { "📂" } else { "📁" }
                                        } else {
                                            "📄"
                                        }
                                    }
                                }
                                span { class: "adui-tree-title", "{label}" }
                            }
                        }
                    }
                })}
            }
        }
    }
}

/// Toggle selection for a key.
fn toggle_selection(current: &[String], key: &str, multiple: bool) -> Vec<String> {
    if multiple {
        if current.contains(&key.to_string()) {
            current.iter().filter(|k| *k != key).cloned().collect()
        } else {
            let mut next = current.to_vec();
            next.push(key.to_string());
            next
        }
    } else {
        if current.contains(&key.to_string()) {
            Vec::new()
        } else {
            vec![key.to_string()]
        }
    }
}

/// Toggle check for a key, handling cascading behavior if not check_strictly.
fn toggle_check(
    current: &[String],
    key: &str,
    check_strictly: bool,
    nodes: &[TreeNode],
    flat_nodes: &[FlatTreeNode],
) -> Vec<String> {
    let is_checked = current.contains(&key.to_string());

    if check_strictly {
        // Simple toggle without cascading
        if is_checked {
            current.iter().filter(|k| *k != key).cloned().collect()
        } else {
            let mut next = current.to_vec();
            next.push(key.to_string());
            next
        }
    } else {
        // Cascading check: toggle all descendants
        let descendants = collect_descendant_keys(nodes, key);

        if is_checked {
            // Uncheck: remove this node and all descendants
            current
                .iter()
                .filter(|k| !descendants.contains(k))
                .cloned()
                .collect()
        } else {
            // Check: add this node and all descendants
            let mut next: Vec<String> = current.to_vec();
            for dk in descendants {
                if !next.contains(&dk) {
                    next.push(dk);
                }
            }

            // Also check parent nodes if all siblings are checked
            let parents = collect_parent_keys(flat_nodes, key);
            for parent_key in parents {
                let siblings = collect_descendant_keys(nodes, &parent_key);
                let all_checked = siblings
                    .iter()
                    .all(|sk| next.contains(sk) || sk == &parent_key);
                if all_checked && !next.contains(&parent_key) {
                    next.push(parent_key);
                }
            }

            next
        }
    }
}

/// DirectoryTree variant - Tree with directory icons and expand on click.
#[derive(Props, Clone, PartialEq)]
pub struct DirectoryTreeProps {
    /// Tree data source.
    #[props(optional)]
    pub tree_data: Option<Vec<TreeNode>>,

    // --- Expand control ---
    #[props(optional)]
    pub expanded_keys: Option<Vec<String>>,
    #[props(optional)]
    pub default_expanded_keys: Option<Vec<String>>,
    #[props(default)]
    pub default_expand_all: bool,
    #[props(optional)]
    pub on_expand: Option<EventHandler<Vec<String>>>,

    // --- Selection control ---
    #[props(optional)]
    pub selected_keys: Option<Vec<String>>,
    #[props(optional)]
    pub default_selected_keys: Option<Vec<String>>,
    #[props(default)]
    pub multiple: bool,
    #[props(optional)]
    pub on_select: Option<EventHandler<Vec<String>>>,

    // --- Styling ---
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
}

/// Directory-style tree with folder icons and expand-on-click behavior.
#[component]
pub fn DirectoryTree(props: DirectoryTreeProps) -> Element {
    let DirectoryTreeProps {
        tree_data,
        expanded_keys,
        default_expanded_keys,
        default_expand_all,
        on_expand,
        selected_keys,
        default_selected_keys,
        multiple,
        on_select,
        class,
        style,
    } = props;

    let mut class_list = vec!["adui-tree-directory".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }

    rsx! {
        Tree {
            tree_data,
            expanded_keys,
            default_expanded_keys,
            default_expand_all,
            on_expand,
            selected_keys,
            default_selected_keys,
            multiple,
            on_select,
            show_icon: true,
            block_node: true,
            class: class_list.join(" "),
            style,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_tree_produces_correct_depth() {
        let nodes = vec![TreeNode {
            key: "parent".into(),
            label: "Parent".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "child1".into(),
                    label: "Child 1".into(),
                    disabled: false,
                    children: vec![],
                },
                TreeNode {
                    key: "child2".into(),
                    label: "Child 2".into(),
                    disabled: false,
                    children: vec![TreeNode {
                        key: "grandchild".into(),
                        label: "Grandchild".into(),
                        disabled: false,
                        children: vec![],
                    }],
                },
            ],
        }];

        let mut flat = Vec::new();
        flatten_tree(&nodes, 0, None, &mut flat);

        assert_eq!(flat.len(), 4);
        assert_eq!(flat[0].key, "parent");
        assert_eq!(flat[0].depth, 0);
        assert!(flat[0].has_children);
        assert_eq!(flat[1].key, "child1");
        assert_eq!(flat[1].depth, 1);
        assert!(!flat[1].has_children);
        assert_eq!(flat[2].key, "child2");
        assert_eq!(flat[2].depth, 1);
        assert!(flat[2].has_children);
        assert_eq!(flat[3].key, "grandchild");
        assert_eq!(flat[3].depth, 2);
    }

    #[test]
    fn toggle_selection_single_mode() {
        let current: Vec<String> = vec![];
        let next = toggle_selection(&current, "a", false);
        assert_eq!(next, vec!["a".to_string()]);

        let next2 = toggle_selection(&next, "b", false);
        assert_eq!(next2, vec!["b".to_string()]);

        let next3 = toggle_selection(&next2, "b", false);
        assert!(next3.is_empty());
    }

    #[test]
    fn toggle_selection_multiple_mode() {
        let current: Vec<String> = vec![];
        let next = toggle_selection(&current, "a", true);
        assert_eq!(next, vec!["a".to_string()]);

        let next2 = toggle_selection(&next, "b", true);
        assert_eq!(next2, vec!["a".to_string(), "b".to_string()]);

        let next3 = toggle_selection(&next2, "a", true);
        assert_eq!(next3, vec!["b".to_string()]);
    }

    #[test]
    fn collect_descendant_keys_finds_all_children() {
        let nodes = vec![TreeNode {
            key: "root".into(),
            label: "Root".into(),
            disabled: false,
            children: vec![
                TreeNode {
                    key: "a".into(),
                    label: "A".into(),
                    disabled: false,
                    children: vec![TreeNode {
                        key: "a1".into(),
                        label: "A1".into(),
                        disabled: false,
                        children: vec![],
                    }],
                },
                TreeNode {
                    key: "b".into(),
                    label: "B".into(),
                    disabled: false,
                    children: vec![],
                },
            ],
        }];

        let descendants = collect_descendant_keys(&nodes, "a");
        assert!(descendants.contains(&"a".to_string()));
        assert!(descendants.contains(&"a1".to_string()));
        assert!(!descendants.contains(&"b".to_string()));
        assert!(!descendants.contains(&"root".to_string()));
    }
}
