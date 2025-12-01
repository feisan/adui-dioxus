//! Semantic classNames and styles system aligned with Ant Design 6.0.
//!
//! This module provides a type-safe way to apply custom classes and styles
//! to specific semantic parts of components.

use std::collections::HashMap;
use std::hash::Hash;

/// A collection of CSS class names keyed by semantic slot name.
///
/// Used to customize the styling of specific parts of a component.
///
/// # Example
///
/// ```rust,ignore
/// use adui_dioxus::foundation::SemanticClassNames;
///
/// #[derive(Hash, Eq, PartialEq, Clone, Copy)]
/// enum ButtonSemantic {
///     Root,
///     Icon,
///     Content,
/// }
///
/// let mut class_names = SemanticClassNames::new();
/// class_names.set(ButtonSemantic::Root, "my-custom-button");
/// class_names.set(ButtonSemantic::Icon, "my-custom-icon");
/// ```
#[derive(Clone, Debug, Default)]
pub struct SemanticClassNames<S: Hash + Eq> {
    values: HashMap<S, String>,
}

impl<S: Hash + Eq> SemanticClassNames<S> {
    /// Create an empty semantic class names collection.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Set the class name for a specific semantic slot.
    pub fn set(&mut self, slot: S, class_name: impl Into<String>) {
        self.values.insert(slot, class_name.into());
    }

    /// Get the class name for a specific semantic slot, if set.
    pub fn get(&self, slot: &S) -> Option<&str> {
        self.values.get(slot).map(|s| s.as_str())
    }

    /// Get the class name for a slot or return an empty string.
    pub fn get_or_empty(&self, slot: &S) -> &str {
        self.values.get(slot).map(|s| s.as_str()).unwrap_or("")
    }

    /// Check if a semantic slot has a class name.
    pub fn contains(&self, slot: &S) -> bool {
        self.values.contains_key(slot)
    }

    /// Create from an iterator of (slot, class_name) pairs.
    pub fn from_iter<I, N>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, N)>,
        N: Into<String>,
    {
        Self {
            values: iter.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl<S: Hash + Eq> PartialEq for SemanticClassNames<S> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

/// A collection of inline CSS styles keyed by semantic slot name.
///
/// Used to customize the styling of specific parts of a component.
///
/// # Example
///
/// ```rust,ignore
/// use adui_dioxus::foundation::SemanticStyles;
///
/// #[derive(Hash, Eq, PartialEq, Clone, Copy)]
/// enum ModalSemantic {
///     Root,
///     Header,
///     Body,
///     Footer,
/// }
///
/// let mut styles = SemanticStyles::new();
/// styles.set(ModalSemantic::Body, "padding: 24px; background: #f0f0f0;");
/// ```
#[derive(Clone, Debug, Default)]
pub struct SemanticStyles<S: Hash + Eq> {
    values: HashMap<S, String>,
}

impl<S: Hash + Eq> SemanticStyles<S> {
    /// Create an empty semantic styles collection.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Set the inline style for a specific semantic slot.
    pub fn set(&mut self, slot: S, style: impl Into<String>) {
        self.values.insert(slot, style.into());
    }

    /// Get the inline style for a specific semantic slot, if set.
    pub fn get(&self, slot: &S) -> Option<&str> {
        self.values.get(slot).map(|s| s.as_str())
    }

    /// Get the style for a slot or return an empty string.
    pub fn get_or_empty(&self, slot: &S) -> &str {
        self.values.get(slot).map(|s| s.as_str()).unwrap_or("")
    }

    /// Check if a semantic slot has a style.
    pub fn contains(&self, slot: &S) -> bool {
        self.values.contains_key(slot)
    }

    /// Create from an iterator of (slot, style) pairs.
    pub fn from_iter<I, N>(iter: I) -> Self
    where
        I: IntoIterator<Item = (S, N)>,
        N: Into<String>,
    {
        Self {
            values: iter.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl<S: Hash + Eq> PartialEq for SemanticStyles<S> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

// ============================================================================
// Pre-defined semantic slot enums for common components
// ============================================================================

/// Semantic slots for Button component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ButtonSemantic {
    Root,
    Icon,
    Content,
}

/// Semantic slots for Input component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum InputSemantic {
    Root,
    Prefix,
    Suffix,
    Input,
    Count,
}

/// Semantic slots for Select component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SelectSemantic {
    Root,
    Prefix,
    Suffix,
}

/// Semantic slots for Select popup.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SelectPopupSemantic {
    Root,
    List,
    ListItem,
}

/// Semantic slots for Modal component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ModalSemantic {
    Root,
    Header,
    Body,
    Footer,
    Container,
    Title,
    Wrapper,
    Mask,
}

/// Semantic slots for Table component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TableSemantic {
    Section,
    Title,
    Footer,
    Content,
    Root,
}

/// Semantic slots for Table body/header.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TablePartSemantic {
    Wrapper,
    Cell,
    Row,
}

/// Semantic slots for Tabs component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TabsSemantic {
    Root,
    Item,
    Indicator,
    Content,
    Header,
}

/// Semantic slots for Collapse component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum CollapseSemantic {
    Root,
    Header,
    Title,
    Body,
    Icon,
}

/// Semantic slots for Form component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum FormSemantic {
    Root,
    Label,
    Content,
}

/// Semantic slots for Descriptions component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DescriptionsSemantic {
    Root,
    Header,
    Title,
    Extra,
    Label,
    Content,
}

/// Semantic slots for Timeline component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TimelineSemantic {
    Root,
    Item,
    ItemTitle,
    ItemContent,
    Indicator,
}

/// Semantic slots for Anchor component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum AnchorSemantic {
    Root,
    Item,
    ItemTitle,
    Indicator,
}

/// Semantic slots for Message component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum MessageSemantic {
    Root,
    Content,
    Icon,
}

/// Semantic slots for Notification component.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum NotificationSemantic {
    Root,
    Title,
    Description,
    Icon,
    CloseButton,
}

// ============================================================================
// Type aliases for convenience
// ============================================================================

pub type ButtonClassNames = SemanticClassNames<ButtonSemantic>;
pub type ButtonStyles = SemanticStyles<ButtonSemantic>;

pub type InputClassNames = SemanticClassNames<InputSemantic>;
pub type InputStyles = SemanticStyles<InputSemantic>;

pub type SelectClassNames = SemanticClassNames<SelectSemantic>;
pub type SelectStyles = SemanticStyles<SelectSemantic>;

pub type ModalClassNames = SemanticClassNames<ModalSemantic>;
pub type ModalStyles = SemanticStyles<ModalSemantic>;

pub type TableClassNames = SemanticClassNames<TableSemantic>;
pub type TableStyles = SemanticStyles<TableSemantic>;

pub type TabsClassNames = SemanticClassNames<TabsSemantic>;
pub type TabsStyles = SemanticStyles<TabsSemantic>;

pub type CollapseClassNames = SemanticClassNames<CollapseSemantic>;
pub type CollapseStyles = SemanticStyles<CollapseSemantic>;

pub type FormClassNames = SemanticClassNames<FormSemantic>;
pub type FormStyles = SemanticStyles<FormSemantic>;

// ============================================================================
// Helper trait for merging class names
// ============================================================================

/// Extension trait for building class lists with semantic classes.
pub trait ClassListExt {
    /// Push a semantic class if it exists.
    fn push_semantic<S: Hash + Eq>(&mut self, class_names: &Option<SemanticClassNames<S>>, slot: S);
}

impl ClassListExt for Vec<String> {
    fn push_semantic<S: Hash + Eq>(
        &mut self,
        class_names: &Option<SemanticClassNames<S>>,
        slot: S,
    ) {
        if let Some(cn) = class_names {
            if let Some(class) = cn.get(&slot) {
                if !class.is_empty() {
                    self.push(class.to_string());
                }
            }
        }
    }
}

/// Extension trait for building style strings with semantic styles.
pub trait StyleStringExt {
    /// Append a semantic style if it exists.
    fn append_semantic<S: Hash + Eq>(&mut self, styles: &Option<SemanticStyles<S>>, slot: S);
}

impl StyleStringExt for String {
    fn append_semantic<S: Hash + Eq>(&mut self, styles: &Option<SemanticStyles<S>>, slot: S) {
        if let Some(s) = styles {
            if let Some(style) = s.get(&slot) {
                if !style.is_empty() {
                    if !self.is_empty() && !self.ends_with(';') {
                        self.push(';');
                    }
                    self.push_str(style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_class_names_basic_operations() {
        let mut cn = SemanticClassNames::<ButtonSemantic>::new();
        assert!(cn.get(&ButtonSemantic::Root).is_none());

        cn.set(ButtonSemantic::Root, "custom-root");
        assert_eq!(cn.get(&ButtonSemantic::Root), Some("custom-root"));
        assert!(cn.contains(&ButtonSemantic::Root));
        assert!(!cn.contains(&ButtonSemantic::Icon));
    }

    #[test]
    fn semantic_styles_basic_operations() {
        let mut styles = SemanticStyles::<ModalSemantic>::new();
        assert!(styles.get(&ModalSemantic::Body).is_none());

        styles.set(ModalSemantic::Body, "padding: 24px;");
        assert_eq!(styles.get(&ModalSemantic::Body), Some("padding: 24px;"));
    }

    #[test]
    fn class_list_ext_works() {
        let mut cn = SemanticClassNames::<ButtonSemantic>::new();
        cn.set(ButtonSemantic::Root, "my-button");

        let mut classes = vec!["adui-btn".to_string()];
        classes.push_semantic(&Some(cn), ButtonSemantic::Root);
        assert_eq!(classes, vec!["adui-btn", "my-button"]);
    }

    #[test]
    fn style_string_ext_works() {
        let mut styles = SemanticStyles::<ModalSemantic>::new();
        styles.set(ModalSemantic::Body, "padding: 24px");

        let mut style_str = "background: white".to_string();
        style_str.append_semantic(&Some(styles), ModalSemantic::Body);
        assert_eq!(style_str, "background: white;padding: 24px");
    }
}


