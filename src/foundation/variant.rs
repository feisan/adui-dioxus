//! Variant system for form controls, aligned with Ant Design 6.0.
//!
//! The variant determines the visual style of input-like components.

/// Visual variant for form controls (Input, Select, etc.).
///
/// This aligns with Ant Design 6.0's variant system introduced in v5.13.0.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    /// Default variant with outline border.
    #[default]
    Outlined,
    /// Filled background variant.
    Filled,
    /// No border variant.
    Borderless,
}

impl Variant {
    /// Get the CSS class suffix for this variant.
    pub fn as_class_suffix(&self) -> &'static str {
        match self {
            Variant::Outlined => "outlined",
            Variant::Filled => "filled",
            Variant::Borderless => "borderless",
        }
    }

    /// Get the full CSS class for a component prefix.
    pub fn class_for(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, self.as_class_suffix())
    }
}

/// Bordered prop support for backwards compatibility.
///
/// In Ant Design 6.0, `bordered` is deprecated in favor of `variant`.
/// This helper converts the legacy `bordered` prop to a `Variant`.
pub fn variant_from_bordered(bordered: Option<bool>, variant: Option<Variant>) -> Variant {
    // If variant is explicitly set, use it
    if let Some(v) = variant {
        return v;
    }

    // Otherwise, derive from bordered (legacy prop)
    match bordered {
        Some(false) => Variant::Borderless,
        _ => Variant::Outlined, // Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_class_suffix() {
        assert_eq!(Variant::Outlined.as_class_suffix(), "outlined");
        assert_eq!(Variant::Filled.as_class_suffix(), "filled");
        assert_eq!(Variant::Borderless.as_class_suffix(), "borderless");
    }

    #[test]
    fn variant_class_for_prefix() {
        assert_eq!(
            Variant::Outlined.class_for("adui-input"),
            "adui-input-outlined"
        );
        assert_eq!(
            Variant::Filled.class_for("adui-select"),
            "adui-select-filled"
        );
    }

    #[test]
    fn variant_from_bordered_priority() {
        // variant takes priority
        assert_eq!(
            variant_from_bordered(Some(false), Some(Variant::Filled)),
            Variant::Filled
        );

        // bordered=false -> Borderless
        assert_eq!(
            variant_from_bordered(Some(false), None),
            Variant::Borderless
        );

        // bordered=true or None -> Outlined
        assert_eq!(variant_from_bordered(Some(true), None), Variant::Outlined);
        assert_eq!(variant_from_bordered(None, None), Variant::Outlined);
    }
}
