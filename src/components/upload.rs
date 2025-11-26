use dioxus::{
    html::events::{DragEvent, FormData},
    prelude::*,
};
use dioxus_html::HasFileData;
use std::{rc::Rc, time::SystemTime};

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, collections::HashMap};

#[cfg(target_arch = "wasm32")]
use {
    js_sys::{Array, Uint8Array},
    wasm_bindgen::{JsCast, closure::Closure},
    wasm_bindgen_futures::spawn_local,
    web_sys::{Blob, FormData as WebFormData, ProgressEvent, XmlHttpRequest},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadStatus {
    Ready,
    Uploading,
    Done,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadListType {
    Text,
    Picture,
    PictureCard,
}

impl Default for UploadListType {
    fn default() -> Self {
        UploadListType::Text
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UploadFile {
    pub uid: String,
    pub name: String,
    pub status: UploadStatus,
    pub size: Option<u64>,
    pub url: Option<String>,
    pub error: Option<String>,
    pub percent: Option<f32>,
    pub response: Option<String>,
}

impl UploadFile {
    pub fn done(name: impl Into<String>, size: Option<u64>) -> Self {
        Self {
            uid: format!("upload-{}", unique_id()),
            name: name.into(),
            status: UploadStatus::Done,
            size,
            url: None,
            error: None,
            percent: Some(100.0),
            response: None,
        }
    }

    pub fn uploading(name: impl Into<String>, size: Option<u64>) -> Self {
        Self {
            uid: format!("upload-{}", unique_id()),
            name: name.into(),
            status: UploadStatus::Uploading,
            size,
            url: None,
            error: None,
            percent: Some(0.0),
            response: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UploadChangeInfo {
    pub file: UploadFile,
    pub file_list: Vec<UploadFile>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UploadListConfig {
    pub show_remove_icon: bool,
}

impl Default for UploadListConfig {
    fn default() -> Self {
        Self {
            show_remove_icon: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UploadFileMeta {
    pub name: String,
    pub size: Option<u64>,
    pub mime: Option<String>,
}

pub type BeforeUploadFn = Rc<dyn Fn(&UploadFileMeta) -> bool>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UploadHttpMethod {
    Post,
    Put,
}

impl UploadHttpMethod {
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    fn as_str(&self) -> &'static str {
        match self {
            UploadHttpMethod::Post => "POST",
            UploadHttpMethod::Put => "PUT",
        }
    }
}

impl Default for UploadHttpMethod {
    fn default() -> Self {
        UploadHttpMethod::Post
    }
}

#[derive(Props, Clone)]
pub struct UploadProps {
    #[props(optional)]
    pub action: Option<String>,
    #[props(default)]
    pub multiple: bool,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub list_type: UploadListType,
    #[props(optional)]
    pub field_name: Option<String>,
    #[props(default)]
    pub method: UploadHttpMethod,
    #[props(default)]
    pub with_credentials: bool,
    #[props(optional)]
    pub headers: Option<Vec<(String, String)>>,
    #[props(optional)]
    pub accept: Option<String>,
    #[props(optional)]
    pub file_list: Option<Vec<UploadFile>>,
    #[props(optional)]
    pub default_file_list: Option<Vec<UploadFile>>,
    #[props(optional)]
    pub before_upload: Option<BeforeUploadFn>,
    #[props(optional)]
    pub on_change: Option<EventHandler<UploadChangeInfo>>,
    #[props(optional)]
    pub on_remove: Option<EventHandler<UploadFile>>,
    #[props(optional)]
    pub show_upload_list: Option<UploadListConfig>,
    #[props(optional)]
    pub description: Option<Element>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    #[props(default)]
    pub dragger: bool,
    pub children: Element,
}

impl PartialEq for UploadProps {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
            && self.multiple == other.multiple
            && self.disabled == other.disabled
            && self.list_type == other.list_type
            && self.field_name == other.field_name
            && self.method == other.method
            && self.with_credentials == other.with_credentials
            && self.headers == other.headers
            && self.accept == other.accept
            && self.file_list == other.file_list
            && self.default_file_list == other.default_file_list
            && self.show_upload_list == other.show_upload_list
            && self.description == other.description
            && self.class == other.class
            && self.style == other.style
            && self.dragger == other.dragger
            && self.children == other.children
    }
}

#[component]
pub fn Upload(props: UploadProps) -> Element {
    let UploadProps {
        action,
        multiple,
        disabled,
        list_type,
        field_name,
        method,
        with_credentials,
        headers,
        accept,
        file_list,
        default_file_list,
        before_upload,
        on_change,
        on_remove,
        show_upload_list,
        description,
        class,
        style,
        children,
        dragger,
    } = props;
    let field_name = field_name.unwrap_or_else(|| "file".to_string());

    let controlled = file_list.is_some();
    let initial_files = file_list
        .clone()
        .or_else(|| default_file_list.clone())
        .unwrap_or_default();
    let mut files_signal = use_signal(|| initial_files.clone());
    if let Some(controlled_list) = file_list {
        files_signal.set(controlled_list);
    }

    #[cfg(target_arch = "wasm32")]
    let upload_requests =
        use_hook(|| Rc::new(RefCell::new(HashMap::<String, XmlHttpRequest>::new())));
    #[cfg(not(target_arch = "wasm32"))]
    let _upload_requests = ();

    let list_config = show_upload_list.unwrap_or_default();
    let abort_upload: Rc<dyn Fn(&str)> = {
        #[cfg(target_arch = "wasm32")]
        {
            let store = upload_requests.clone();
            Rc::new(move |uid: &str| {
                if let Some(xhr) = store.borrow_mut().remove(uid) {
                    let _ = xhr.abort();
                }
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = &_upload_requests;
            Rc::new(|_: &str| {})
        }
    };
    let input_id = format!("adui-upload-input-{}", unique_id());
    let accept_attr = accept.unwrap_or_default();
    let dragging = use_signal(|| false);
    let class_attr = format!(
        "adui-upload adui-upload-type-{} {}",
        match list_type {
            UploadListType::Text => "text",
            UploadListType::Picture => "picture",
            UploadListType::PictureCard => "picture-card",
        },
        class.unwrap_or_default()
    );

    let headers = Rc::new(headers.clone().unwrap_or_default());
    let process_files = {
        let files_signal = files_signal.clone();
        let before_upload = before_upload.clone();
        let on_change = on_change.clone();
        let action = action.clone();
        let field_name = field_name.clone();
        let method = method;
        let with_credentials = with_credentials;
        let headers = headers.clone();
        #[cfg(target_arch = "wasm32")]
        let request_store = upload_requests.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let request_store = _upload_requests;

        Rc::new(move |files: Vec<dioxus_html::FileData>| {
            if disabled || files.is_empty() {
                return;
            }
            for file in files {
                let meta = UploadFileMeta {
                    name: file.name(),
                    size: Some(file.size()),
                    mime: file.content_type(),
                };
                if let Some(filter) = before_upload.as_ref() {
                    if !(filter)(&meta) {
                        continue;
                    }
                }

                let entry = if action.is_some() {
                    UploadFile::uploading(meta.name.clone(), meta.size)
                } else {
                    UploadFile::done(meta.name.clone(), meta.size)
                };
                let uid = entry.uid.clone();

                if let Some((changed, snapshot)) =
                    mutate_files(files_signal.clone(), controlled, |list| {
                        list.push(entry.clone());
                        Some(entry.clone())
                    })
                {
                    if let Some(handler) = on_change.as_ref() {
                        handler.call(UploadChangeInfo {
                            file: changed,
                            file_list: snapshot,
                        });
                    }
                }

                if let Some(action_url) = action.clone() {
                    start_upload_task(
                        file.clone(),
                        meta.clone(),
                        uid,
                        action_url,
                        field_name.clone(),
                        method,
                        with_credentials,
                        (*headers).clone(),
                        files_signal.clone(),
                        controlled,
                        on_change.clone(),
                        request_store.clone(),
                    );
                }
            }
        })
    };

    let onchange = {
        let process_files = process_files.clone();
        move |evt: Event<FormData>| {
            if disabled {
                return;
            }
            process_files(evt.files());
        }
    };

    let mut selector_classes = vec!["adui-upload-selector".to_string()];
    if disabled {
        selector_classes.push("adui-upload-disabled".into());
    }
    if dragger {
        selector_classes.push("adui-upload-dragger".into());
        if *dragging.read() {
            selector_classes.push("adui-upload-dragger-hover".into());
        }
    }
    let selector_class = selector_classes.join(" ");
    let mut dragging_for_over = dragging.clone();
    let mut dragging_for_leave = dragging.clone();
    let mut dragging_for_drop = dragging.clone();
    let process_files_drop = process_files.clone();

    rsx! {
        div { class: "{class_attr}", style: style.unwrap_or_default(),
            label {
                r#for: input_id.clone(),
                class: "{selector_class}",
                ondragover: move |evt: DragEvent| {
                    if !dragger || disabled {
                        return;
                    }
                    evt.prevent_default();
                    dragging_for_over.set(true);
                },
                ondragleave: move |evt: DragEvent| {
                    if !dragger || disabled {
                        return;
                    }
                    evt.prevent_default();
                    dragging_for_leave.set(false);
                },
                ondrop: move |evt: DragEvent| {
                    if !dragger || disabled {
                        return;
                    }
                    evt.prevent_default();
                    dragging_for_drop.set(false);
                    process_files_drop(evt.files());
                },
                {children}
                if let Some(desc) = description {
                    div { class: "adui-upload-description", {desc} }
                }
            }
            input {
                id: input_id,
                r#type: "file",
                multiple: multiple,
                disabled: disabled,
                accept: accept_attr,
                onchange: onchange,
                style: "display:none",
            }
            {render_upload_list(files_signal.read().clone(), list_type, list_config, files_signal.clone(), controlled, disabled, on_remove.clone(), on_change.clone(), abort_upload.clone())}
        }
    }
}

fn render_upload_list(
    files: Vec<UploadFile>,
    list_type: UploadListType,
    config: UploadListConfig,
    files_signal: Signal<Vec<UploadFile>>,
    controlled: bool,
    disabled: bool,
    on_remove: Option<EventHandler<UploadFile>>,
    on_change: Option<EventHandler<UploadChangeInfo>>,
    abort_upload: Rc<dyn Fn(&str)>,
) -> Element {
    if files.is_empty() {
        return rsx! { div {} };
    }
    rsx! {
        ul { class: format!("adui-upload-list adui-upload-list-{}", match list_type {
                UploadListType::Text => "text",
                UploadListType::Picture => "picture",
                UploadListType::PictureCard => "picture-card",
            }),
            {files.into_iter().map(|file| {
                let files_signal = files_signal.clone();
                let on_remove = on_remove.clone();
                let on_change = on_change.clone();
                let file_entry = file.clone();
                let file_for_remove = file.clone();
                let abort_upload = abort_upload.clone();
                rsx!(li { key: "{file_entry.uid}", class: "adui-upload-list-item",
                    span { class: "adui-upload-list-item-name", "{file_entry.name}" }
                    if config.show_remove_icon {
                        button {
                            r#type: "button",
                            class: "adui-upload-list-item-remove",
                            onclick: move |_| {
                                if disabled {
                                    return;
                                }
                                abort_upload(&file_for_remove.uid);
                                if let Some((removed, snapshot)) =
                                    mutate_files(files_signal.clone(), controlled, |list| {
                                        if let Some(pos) =
                                            list.iter().position(|f| f.uid == file_for_remove.uid)
                                        {
                                            Some(list.remove(pos))
                                        } else {
                                            None
                                        }
                                    })
                                {
                                    if let Some(handler) = on_remove.as_ref() {
                                        handler.call(removed.clone());
                                    }
                                    if let Some(handler) = on_change.as_ref() {
                                        handler.call(UploadChangeInfo {
                                            file: removed,
                                            file_list: snapshot,
                                        });
                                    }
                                }
                            },
                            "删除"
                        }
                    }
                    if let Some(err) = file_entry.error.clone() {
                        span { class: "adui-upload-list-item-error", "{err}" }
                    }
                })
            })}
        }
    }
}

fn mutate_files(
    mut files_signal: Signal<Vec<UploadFile>>,
    controlled: bool,
    mutator: impl FnOnce(&mut Vec<UploadFile>) -> Option<UploadFile>,
) -> Option<(UploadFile, Vec<UploadFile>)> {
    let mut list = files_signal.read().clone();
    let changed = mutator(&mut list)?;
    if !controlled {
        files_signal.set(list.clone());
    }
    Some((changed, list))
}

fn update_file_state(
    files_signal: Signal<Vec<UploadFile>>,
    controlled: bool,
    uid: &str,
    on_change: Option<EventHandler<UploadChangeInfo>>,
    mut updater: impl FnMut(&mut UploadFile),
) {
    if let Some((changed, snapshot)) = mutate_files(files_signal, controlled, |list| {
        list.iter_mut().find(|item| item.uid == uid).map(|entry| {
            updater(entry);
            entry.clone()
        })
    }) {
        if let Some(handler) = on_change {
            handler.call(UploadChangeInfo {
                file: changed,
                file_list: snapshot,
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn start_upload_task(
    file: dioxus_html::FileData,
    meta: UploadFileMeta,
    uid: String,
    action: String,
    field_name: String,
    method: UploadHttpMethod,
    with_credentials: bool,
    headers: Vec<(String, String)>,
    files_signal: Signal<Vec<UploadFile>>,
    controlled: bool,
    on_change: Option<EventHandler<UploadChangeInfo>>,
    request_store: Rc<RefCell<HashMap<String, XmlHttpRequest>>>,
) {
    spawn_local(async move {
        let bytes = match file.read_bytes().await {
            Ok(data) => data,
            Err(err) => {
                update_file_state(
                    files_signal.clone(),
                    controlled,
                    &uid,
                    on_change.clone(),
                    |entry| {
                        entry.status = UploadStatus::Error;
                        entry.error = Some(err.to_string());
                    },
                );
                return;
            }
        };

        let xhr = match XmlHttpRequest::new() {
            Ok(req) => req,
            Err(_) => {
                update_file_state(
                    files_signal.clone(),
                    controlled,
                    &uid,
                    on_change.clone(),
                    |entry| {
                        entry.status = UploadStatus::Error;
                        entry.error = Some("无法创建请求".into());
                    },
                );
                return;
            }
        };

        if xhr.open_with_async(method.as_str(), &action, true).is_err() {
            update_file_state(
                files_signal.clone(),
                controlled,
                &uid,
                on_change.clone(),
                |entry| {
                    entry.status = UploadStatus::Error;
                    entry.error = Some("打开上传连接失败".into());
                },
            );
            return;
        }
        xhr.set_with_credentials(with_credentials);
        for (key, value) in headers.iter() {
            let _ = xhr.set_request_header(key, value);
        }
        request_store.borrow_mut().insert(uid.clone(), xhr.clone());

        let progress_signal = files_signal.clone();
        let progress_uid = uid.clone();
        let progress_on_change = on_change.clone();
        let progress_closure =
            Closure::<dyn FnMut(ProgressEvent)>::wrap(Box::new(move |event: ProgressEvent| {
                if event.length_computable() {
                    let total = event.total();
                    if total > 0.0 {
                        let percent = ((event.loaded() / total) * 100.0).clamp(0.0, 100.0) as f32;
                        update_file_state(
                            progress_signal.clone(),
                            controlled,
                            &progress_uid,
                            progress_on_change.clone(),
                            |entry| entry.percent = Some(percent),
                        );
                    }
                }
            }));
        if let Ok(upload) = xhr.upload() {
            upload.set_onprogress(Some(progress_closure.as_ref().unchecked_ref()));
        }
        progress_closure.forget();

        let success_signal = files_signal.clone();
        let success_uid = uid.clone();
        let success_on_change = on_change.clone();
        let success_store = request_store.clone();
        let xhr_clone = xhr.clone();
        let load_closure =
            Closure::<dyn FnMut(_)>::wrap(Box::new(move |_event: web_sys::Event| {
                success_store.borrow_mut().remove(&success_uid);
                let status = xhr_clone.status().unwrap_or(0);
                let response = xhr_clone.response_text().ok().flatten();
                if (200..300).contains(&status) {
                    update_file_state(
                        success_signal.clone(),
                        controlled,
                        &success_uid,
                        success_on_change.clone(),
                        |entry| {
                            entry.status = UploadStatus::Done;
                            entry.percent = Some(100.0);
                            entry.response = response.clone();
                            entry.error = None;
                        },
                    );
                } else {
                    update_file_state(
                        success_signal.clone(),
                        controlled,
                        &success_uid,
                        success_on_change.clone(),
                        |entry| {
                            entry.status = UploadStatus::Error;
                            entry.error = Some(format!("HTTP {}", xhr_clone.status().unwrap_or(0)));
                        },
                    );
                }
            }));
        xhr.set_onload(Some(load_closure.as_ref().unchecked_ref()));
        load_closure.forget();

        let error_signal = files_signal.clone();
        let error_uid = uid.clone();
        let error_on_change = on_change.clone();
        let error_store = request_store.clone();
        let error_closure =
            Closure::<dyn FnMut(_)>::wrap(Box::new(move |_event: web_sys::Event| {
                error_store.borrow_mut().remove(&error_uid);
                update_file_state(
                    error_signal.clone(),
                    controlled,
                    &error_uid,
                    error_on_change.clone(),
                    |entry| {
                        entry.status = UploadStatus::Error;
                        entry.error = Some("上传失败".into());
                    },
                );
            }));
        xhr.set_onerror(Some(error_closure.as_ref().unchecked_ref()));
        xhr.set_onabort(Some(error_closure.as_ref().unchecked_ref()));
        error_closure.forget();

        let mut array = Uint8Array::new_with_length(bytes.len() as u32);
        array.copy_from(bytes.as_ref());
        let buffer = array.buffer();
        let sequence = Array::new();
        sequence.push(&buffer);
        let blob = Blob::new_with_u8_array_sequence(&sequence).unwrap();
        let form = WebFormData::new().unwrap();
        form.append_with_blob_and_filename(&field_name, &blob, &meta.name)
            .unwrap();
        let _ = xhr.send_with_opt_form_data(Some(&form));
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn start_upload_task(
    _file: dioxus_html::FileData,
    _meta: UploadFileMeta,
    uid: String,
    _action: String,
    _field_name: String,
    _method: UploadHttpMethod,
    _with_credentials: bool,
    _headers: Vec<(String, String)>,
    files_signal: Signal<Vec<UploadFile>>,
    controlled: bool,
    on_change: Option<EventHandler<UploadChangeInfo>>,
    _request_store: (),
) {
    update_file_state(files_signal, controlled, &uid, on_change, |entry| {
        entry.status = UploadStatus::Error;
        entry.error = Some("Upload is only supported on web targets".into());
    });
}

fn unique_id() -> u128 {
    #[cfg(target_arch = "wasm32")]
    {
        (js_sys::Date::now() * 1000.0) as u128
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_micros())
            .unwrap_or_default()
    }
}
