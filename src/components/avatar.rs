use dioxus::prelude::*;

/// Shape of the Avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarShape {
    Circle,
    Square,
}

impl AvatarShape {
    fn as_class(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "adui-avatar-circle",
            AvatarShape::Square => "adui-avatar-square",
        }
    }
}

/// Size of the Avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarSize {
    Small,
    Default,
    Large,
}

impl AvatarSize {
    fn as_class(&self) -> &'static str {
        match self {
            AvatarSize::Small => "adui-avatar-sm",
            AvatarSize::Default => "adui-avatar-md",
            AvatarSize::Large => "adui-avatar-lg",
        }
    }
}

/// Props for the Avatar component (MVP subset).
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Image source URL. When present and load succeeds, the image will be
    /// used as the avatar content.
    #[props(optional)]
    pub src: Option<String>,
    /// Alt text for the image.
    #[props(optional)]
    pub alt: Option<String>,
    /// Shape of the avatar (circle or square).
    #[props(optional)]
    pub shape: Option<AvatarShape>,
    /// Size variant for the avatar.
    #[props(optional)]
    pub size: Option<AvatarSize>,
    /// Optional icon content when no image src is provided.
    pub icon: Option<Element>,
    /// Extra class for the root element.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the root element.
    #[props(optional)]
    pub style: Option<String>,
    /// Text content for text avatar. Used when `src` is None; typically a
    /// short string such as initials.
    pub children: Option<Element>,
}

/// Simple Avatar component supporting image, icon and text content.
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let AvatarProps {
        src,
        alt,
        shape,
        size,
        icon,
        class,
        style,
        children,
    } = props;

    let shape_cls = shape.unwrap_or(AvatarShape::Circle).as_class();
    let size_cls = size.unwrap_or(AvatarSize::Default).as_class();

    let mut class_list = vec![
        "adui-avatar".to_string(),
        shape_cls.to_string(),
        size_cls.to_string(),
    ];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    // For MVP we do not handle image load error state explicitly; the browser
    // will render a broken image icon if the src fails. Callers can choose to
    // omit src and rely on icon/text instead.

    rsx! {
        span { class: "{class_attr}", style: "{style_attr}",
            if let Some(url) = src {
                img {
                    class: "adui-avatar-img",
                    src: "{url}",
                    alt: "{alt.clone().unwrap_or_default()}",
                }
            } else if let Some(node) = icon {
                span { class: "adui-avatar-icon", {node} }
            } else if let Some(node) = children {
                span { class: "adui-avatar-text", {node} }
            }
        }
    }
}

/// Props for AvatarGroup.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// Extra class name for the group.
    #[props(optional)]
    pub class: Option<String>,
    /// Inline style for the group.
    #[props(optional)]
    pub style: Option<String>,
    /// Avatars inside the group.
    pub children: Element,
}

/// Simple horizontal Avatar group with overlapping avatars.
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let AvatarGroupProps {
        class,
        style,
        children,
    } = props;

    let mut class_list = vec!["adui-avatar-group".to_string()];
    if let Some(extra) = class {
        class_list.push(extra);
    }
    let class_attr = class_list.join(" ");
    let style_attr = style.unwrap_or_default();

    rsx! {
        div { class: "{class_attr}", style: "{style_attr}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_shape_and_size_class_mapping_is_stable() {
        assert_eq!(AvatarShape::Circle.as_class(), "adui-avatar-circle");
        assert_eq!(AvatarShape::Square.as_class(), "adui-avatar-square");

        assert_eq!(AvatarSize::Small.as_class(), "adui-avatar-sm");
        assert_eq!(AvatarSize::Default.as_class(), "adui-avatar-md");
        assert_eq!(AvatarSize::Large.as_class(), "adui-avatar-lg");
    }
}
