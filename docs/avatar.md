# Avatar

## Overview

The Avatar component displays user profile pictures, icons, or text initials. It supports images, icons, and text content, with different shapes and sizes.

## API Reference

### AvatarProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `src` | `Option<String>` | `None` | Image source URL |
| `alt` | `Option<String>` | `None` | Alt text for the image |
| `shape` | `Option<AvatarShape>` | `None` (defaults to `Circle`) | Shape of the avatar |
| `size` | `Option<AvatarSize>` | `None` (defaults to `Default`) | Size variant |
| `icon` | `Option<Element>` | `None` | Optional icon content when no image src |
| `class` | `Option<String>` | `None` | Extra class for root element |
| `style` | `Option<String>` | `None` | Inline style for root element |
| `children` | `Option<Element>` | `None` | Text content for text avatar (typically initials) |

### AvatarGroupProps

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `Option<String>` | `None` | Extra class name for the group |
| `style` | `Option<String>` | `None` | Inline style for the group |
| `children` | `Element` | - | Avatars inside the group (required) |

### AvatarShape

- `Circle` - Circular avatar (default)
- `Square` - Square avatar

### AvatarSize

- `Small` - Small size
- `Default` - Default size
- `Large` - Large size

## Usage Examples

### Image Avatar

```rust
use adui_dioxus::Avatar;

rsx! {
    Avatar {
        src: Some("https://example.com/avatar.jpg".to_string()),
        alt: Some("User avatar".to_string()),
    }
}
```

### Icon Avatar

```rust
use adui_dioxus::{Avatar, Icon, IconKind};

rsx! {
    Avatar {
        icon: Some(rsx!(Icon { kind: IconKind::User, size: 20.0 })),
    }
}
```

### Text Avatar

```rust
use adui_dioxus::Avatar;

rsx! {
    Avatar {
        children: Some(rsx!("JD")),
    }
}
```

### Square Avatar

```rust
use adui_dioxus::{Avatar, AvatarShape};

rsx! {
    Avatar {
        shape: Some(AvatarShape::Square),
        children: Some(rsx!("AB")),
    }
}
```

### Avatar Group

```rust
use adui_dioxus::{Avatar, AvatarGroup};

rsx! {
    AvatarGroup {
        Avatar { children: Some(rsx!("A")) }
        Avatar { children: Some(rsx!("B")) }
        Avatar { children: Some(rsx!("C")) }
    }
}
```

## Use Cases

- **User Profiles**: Display user profile pictures
- **User Lists**: Show avatars in user lists
- **Comments**: Display commenter avatars
- **Teams**: Show team member avatars
- **Status Indicators**: Combine with badges for status

## Differences from Ant Design 6.0.0

- ✅ Image, icon, and text avatars supported
- ✅ Circle and square shapes
- ✅ Size variants
- ✅ Avatar groups
- ⚠️ Image error fallback not explicitly handled (browser default)
- ⚠️ Some advanced styling options may differ

