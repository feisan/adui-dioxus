use adui_dioxus::{
    App, Avatar, AvatarGroup, AvatarProps, AvatarShape, AvatarSize, ComponentSize, ConfigProvider,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ConfigProvider {
            size: Some(ComponentSize::Middle),
            App { AvatarDemoShell {} }
        }
    }
}

#[component]
fn AvatarDemoShell() -> Element {
    rsx! {
        div {
            style: "padding: 16px; min-height: 100vh; background: var(--adui-color-bg-base);",
            h2 { "Avatar demo" }
            p { "展示基础 Avatar 用法，包括图片头像、文字头像、图标头像与简单 AvatarGroup。" }

            // 图片头像
            h3 { style: "margin-top: 16px;", "图片头像" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Avatar { src: Some("https://via.placeholder.com/64".to_string()), }
                Avatar { src: Some("https://via.placeholder.com/64".to_string()), size: Some(AvatarSize::Large) }
                Avatar { src: Some("https://via.placeholder.com/40".to_string()), size: Some(AvatarSize::Small) }
            }

            // 文字头像
            h3 { style: "margin-top: 24px;", "文字头像" }
            div { style: "display: flex; gap: 16px; align-items: center;",
                Avatar { children: Some(rsx!("AD")), }
                Avatar { shape: Some(AvatarShape::Square), children: Some(rsx!("U")), }
            }

            // Group
            h3 { style: "margin-top: 24px;", "AvatarGroup" }
            AvatarGroup {
                children: rsx! {
                    Avatar { children: Some(rsx!("A")), }
                    Avatar { children: Some(rsx!("B")), }
                    Avatar { children: Some(rsx!("C")), }
                }
            }
        }
    }
}
