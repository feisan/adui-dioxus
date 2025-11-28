use adui_dioxus::{
    Button, ButtonType, Space, SpaceDirection, ThemeProvider, Upload, UploadChangeInfo, UploadFile,
    UploadListConfig, UploadListType,
};
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            UploadDemo {}
        }
    }
}

#[component]
fn UploadDemo() -> Element {
    let change_log = use_signal(|| "尚未上传".to_string());
    let default_pictures = vec![UploadFile::done("示例图片.png", Some(2_048))];

    rsx! {
        div { style: "padding: 24px; display: flex; flex-direction: column; gap: 32px; max-width: 640px;",
            h2 { "基础上传" }
            Upload {
                on_change: {
                    let mut change_log = change_log;
                    move |info: UploadChangeInfo| {
                        change_log.set(format!(
                            "最近上传: {}，当前列表 {} 项",
                            info.file.name,
                            info.file_list.len()
                        ));
                    }
                },
                show_upload_list: Some(UploadListConfig { show_remove_icon: true }),
                description: Some(rsx!(p { class: "adui-upload-description", "单击选择文件，未配置 action 时会立即完成。" })),
                Button {
                    r#type: ButtonType::Primary,
                    "点击上传"
                }
            }
            div { class: "upload-log", style: "font-size:13px;color:#666;", "{change_log.read()}" }

            h2 { "图片列表" }
            Upload {
                list_type: UploadListType::Picture,
                default_file_list: Some(default_pictures.clone()),
                show_upload_list: Some(UploadListConfig { show_remove_icon: true }),
                Button {
                    r#type: ButtonType::Default,
                    "追加图片"
                }
            }

            h2 { "拖拽上传" }
            Upload {
                dragger: true,
                description: Some(rsx!(p { "支持点击或拖拽文件至区域，配合 dragger 样式展示。" })),
                Space {
                    direction: SpaceDirection::Vertical,
                    style: "width:100%;align-items:center;padding:16px 0;",
                    span { "拖拽文件到这里" }
                    span { style: "font-size:12px;color:#888;", "支持单个或多个文件" }
                }
            }
        }
    }
}
