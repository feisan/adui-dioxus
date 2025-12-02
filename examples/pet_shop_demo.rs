//! Pet Shop Demo - 综合展示 UI 组件库的使用
//!
//! 这是一个完整的宠物商店演示应用，综合展示 adui-dioxus 组件库的各种组件。
//! 包含商品列表、详情、购物车、筛选、分类、评价等完整电商功能。
//!
//! 使用的组件包括：
//! - Layout, Header, Content, Footer
//! - Card, Grid, Row, Col
//! - Button, ButtonGroup
//! - Image, ImagePreviewGroup
//! - Typography (Title, Text, Paragraph)
//! - Input, Search, InputNumber
//! - Select, Cascader
//! - Table, List
//! - Tabs, TabItem
//! - Pagination
//! - Rate, Tag, Badge, Statistic
//! - Carousel, Drawer, Modal, Popconfirm
//! - Message, Notification
//! - Form, FormItem
//! - Radio, RadioGroup, Checkbox, CheckboxGroup
//! - Slider, Steps, Timeline, Progress
//! - Empty, Spin, Skeleton
//! - Space, Divider, Avatar, Descriptions
//! - FloatButton, BackTop, Popover, Menu, Segmented

use adui_dioxus::{
    App, Avatar, BackTop, Badge, Button, ButtonType, Card, Checkbox, CheckboxGroup,
    Col, ColResponsive, ColSize, components::carousel::{Carousel, CarouselEffect, CarouselItem},
    components::image::{Image, ImagePreviewGroup, ImagePreviewItem},
    components::input_number::InputNumber,
    components::rate::Rate,
    components::segmented::{Segmented, SegmentedOption},
    components::slider::{Slider, SliderValue},
    Content, Descriptions, DescriptionsItem, Divider, Drawer, DrawerPlacement, Empty, EmptyImage,
    Footer, Header, Icon, IconKind, Layout, Menu, MenuItemNode, MenuMode, Pagination, Paragraph,
    Popconfirm, Progress, Radio, RadioGroup, Row, Search, Space, SpaceDirection, SpaceSize,
    Statistic, Steps, StepItem, StepsDirection, TabItem, Tabs, Tag, TagColor,
    Text, TextType, ThemeProvider, Title, TitleLevel, use_message, use_notification,
};
use dioxus::prelude::*;
use serde_json::json;
use std::collections::HashMap;

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        ThemeProvider {
            App {
                PetShopDemo {}
            }
        }
    }
}

// Mock 数据结构
#[derive(Clone, PartialEq)]
struct Pet {
    id: u32,
    name: String,
    category: String,
    price: f64,
    original_price: Option<f64>,
    rating: f64,
    review_count: u32,
    image: String,
    images: Vec<String>,
    description: String,
    brand: String,
    tags: Vec<String>,
    stock: u32,
    specifications: HashMap<String, String>,
    reviews: Vec<Review>,
}

#[derive(Clone, PartialEq)]
struct Review {
    user: String,
    avatar: String,
    rating: f64,
    comment: String,
    date: String,
}

#[derive(Clone, PartialEq)]
struct CartItem {
    pet: Pet,
    quantity: u32,
}

// 生成 Mock 数据
fn generate_mock_pets() -> Vec<Pet> {
    let mut pets = Vec::new();
    let categories = vec!["狗", "猫", "鸟", "鱼", "兔子"];
    let brands = vec!["PetLove", "HappyPaws", "FurryFriends", "PetParadise", "AnimalKingdom"];
    let tags_pool = vec!["新品", "热销", "折扣", "推荐", "限量"];
    
    for i in 1..=30 {
        let i_u32 = i as u32;
        let category = categories[(i_u32 - 1) as usize % categories.len()];
        let brand = brands[(i_u32 - 1) as usize % brands.len()];
        let mut tags = Vec::new();
        if i_u32 % 3 == 0 {
            tags.push("新品".to_string());
        }
        if i_u32 % 4 == 0 {
            tags.push("热销".to_string());
        }
        if i_u32 % 5 == 0 {
            tags.push("折扣".to_string());
        }
        
        let price = 50.0 + (i as f64 * 25.5);
        let original_price = if i_u32 % 5 == 0 {
            Some(price * 1.3)
        } else {
            None
        };
        
        let mut reviews = Vec::new();
        for j in 1..=5 {
            reviews.push(Review {
                user: format!("用户{}", j),
                avatar: format!("https://api.dicebear.com/7.x/avataaars/svg?seed={}", j),
                rating: 4.0 + ((j as u32 % 2) as f64) * 0.5,
                comment: format!("非常好的{}，质量很好，推荐购买！", category),
                date: format!("2024-01-{}", j),
            });
        }
        
        let mut specifications = HashMap::new();
        specifications.insert("品种".to_string(), format!("{}品种{}", category, i));
        specifications.insert("年龄".to_string(), format!("{}个月", 3 + (i_u32 % 12)));
        specifications.insert("性别".to_string(), if i_u32 % 2 == 0 { "公".to_string() } else { "母".to_string() });
        specifications.insert("颜色".to_string(), vec!["棕色", "白色", "黑色", "灰色", "花色"][(i_u32 - 1) as usize % 5].to_string());
        
        pets.push(Pet {
            id: i_u32,
            name: format!("{}宠物{}", category, i),
            category: category.to_string(),
            price,
            original_price,
            rating: 4.0 + ((i_u32 % 2) as f64) * 0.5,
            review_count: 10 + (i_u32 % 50),
            image: format!("https://picsum.photos/400/300?random={}", i),
            images: vec![
                format!("https://picsum.photos/800/600?random={}", i * 10),
                format!("https://picsum.photos/800/600?random={}", i * 10 + 1),
                format!("https://picsum.photos/800/600?random={}", i * 10 + 2),
                format!("https://picsum.photos/800/600?random={}", i * 10 + 3),
            ],
            description: format!("这是一只非常可爱的{}，性格温顺，适合家庭饲养。具有优良的血统和健康证明。", category),
            brand: brand.to_string(),
            tags,
            stock: 10 + (i_u32 % 20),
            specifications,
            reviews,
        });
    }
    pets
}

#[component]
fn PetShopDemo() -> Element {
    let pets = use_signal(|| generate_mock_pets());
    let cart = use_signal(|| Vec::<CartItem>::new());
    let mut current_page = use_signal(|| "home".to_string());
    let mut selected_category = use_signal(|| None::<String>);
    let mut search_query = use_signal(|| String::new());
    let filter_drawer_open = use_signal(|| false);
    let selected_pet = use_signal(|| None::<Pet>);
    let mut current_page_num = use_signal(|| 1u32);
    let page_size = use_signal(|| 12u32);
    
    let message = use_message();
    let notification = use_notification();
    
    // 筛选后的商品列表
    let filtered_pets = use_memo(move || {
        let pets_list = pets.read();
        let category = selected_category.read();
        let query = search_query.read();
        
        pets_list.iter()
            .filter(|pet| {
                if let Some(ref cat) = *category {
                    if pet.category != *cat {
                        return false;
                    }
                }
                if !query.is_empty() {
                    if !pet.name.contains(query.as_str()) && 
                       !pet.description.contains(query.as_str()) &&
                       !pet.brand.contains(query.as_str()) {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    
    // 分页后的商品
    let paginated_pets = use_memo(move || {
        let filtered = filtered_pets();
        let start = ((*current_page_num.read() - 1) * *page_size.read()) as usize;
        let end = (start + *page_size.read() as usize).min(filtered.len());
        filtered.iter().skip(start).take(end - start).cloned().collect::<Vec<_>>()
    });
    
    // 购物车商品数量
    let cart_count = use_memo(move || {
        cart.read().iter().map(|item| item.quantity).sum::<u32>()
    });
    
    rsx! {
        Layout {
            style: "min-height: 100vh; background: #f7f7f8;",
            Header {
                style: "background: #ffffff; border-bottom: 1px solid rgba(0, 0, 0, 0.06); position: sticky; top: 0; z-index: 100; backdrop-filter: blur(10px);",
                div {
                    style: "max-width: 1400px; margin: 0 auto; padding: 0 32px; display: flex; align-items: center; justify-content: space-between; height: 64px;",
                    // Logo 和标题
                    div {
                        style: "display: flex; align-items: center; gap: 12px; cursor: pointer; transition: opacity 0.2s;",
                        onclick: move |_| {
                            current_page.set("home".to_string());
                            selected_category.set(None);
                            search_query.set(String::new());
                            current_page_num.set(1);
                        },
                        div {
                            style: "font-size: 24px; line-height: 1;",
                            "🐾"
                        }
                        Title {
                            level: TitleLevel::H3,
                            style: "margin: 0; color: #202123; font-weight: 500; letter-spacing: -0.3px; font-size: 18px;",
                            "Pet Shop"
                        }
                    }
                    
                    // 搜索框
                    div {
                        style: "flex: 1; max-width: 600px; margin: 0 24px;",
                        Search {
                            placeholder: Some("搜索宠物、品牌...".to_string()),
                            value: Some(search_query.read().clone()),
                            on_search: {
                                let mut query = search_query;
                                let mut page = current_page_num;
                                move |value: String| {
                                    query.set(value);
                                    page.set(1);
                                }
                            },
                            on_change: {
                                let mut query = search_query;
                                move |value: String| {
                                    query.set(value);
                                }
                            },
                        }
                    }
                    
                    // 导航菜单和购物车
                    div {
                        style: "display: flex; align-items: center; gap: 20px;",
                        Menu {
                            mode: MenuMode::Horizontal,
                            items: vec![
                                MenuItemNode {
                                    id: "home".into(),
                                    label: "首页".into(),
                                    icon: None,
                                    disabled: false,
                                    children: None,
                                },
                                MenuItemNode {
                                    id: "cart".into(),
                                    label: "购物车".into(),
                                    icon: None,
                                    disabled: false,
                                    children: None,
                                },
                            ],
                            selected_keys: Some(vec![current_page.read().clone()]),
                            on_select: {
                                let mut page = current_page;
                                move |key: String| {
                                    page.set(key);
                                }
                            },
                        }
                        Badge {
                            count: Some(rsx!({(*cart_count.read()).to_string()})),
                            div {
                                style: "cursor: pointer; padding: 8px 12px; border-radius: 8px; transition: background 0.2s; display: flex; align-items: center; gap: 6px; color: #565869;",
                                onclick: move |_| current_page.set("cart".to_string()),
                                Icon { kind: IconKind::Info }
                                span { style: "font-weight: 400; font-size: 14px;", "购物车" }
                            }
                        }
                    }
                }
            }
            
            Content {
                style: "min-height: calc(100vh - 64px - 80px); background: #f7f7f8;",
                if *current_page.read() == "home" {
                    HomePage {
                        pets: paginated_pets(),
                        all_pets: filtered_pets(),
                        selected_category: selected_category.read().clone(),
                        current_page_num: *current_page_num.read(),
                        page_size: *page_size.read(),
                        total: filtered_pets().len() as u32,
                        on_category_change: {
                            let mut cat = selected_category;
                            let mut page = current_page_num;
                            move |cat_opt: Option<String>| {
                                cat.set(cat_opt);
                                page.set(1);
                            }
                        },
                        on_page_change: {
                            let mut page = current_page_num;
                            move |p: u32| {
                                page.set(p);
                            }
                        },
                        on_pet_click: {
                            let mut pet = selected_pet;
                            let mut page = current_page;
                            move |p: Pet| {
                                pet.set(Some(p.clone()));
                                page.set("detail".to_string());
                            }
                        },
                        on_add_to_cart: EventHandler::new({
                            let mut cart_sig = cart;
                            let msg = message.clone();
                            move |p: Pet| {
                                let mut cart_items = cart_sig.read().clone();
                                if let Some(item) = cart_items.iter_mut().find(|item| item.pet.id == p.id) {
                                    item.quantity += 1;
                                } else {
                                    cart_items.push(CartItem { pet: p, quantity: 1 });
                                }
                                cart_sig.set(cart_items);
                                if let Some(m) = msg.clone() {
                                    m.success("已添加到购物车");
                                }
                            }
                        }),
                        on_filter_click: {
                            let mut open = filter_drawer_open;
                            move |_| open.set(true)
                        },
                    }
                } else if *current_page.read() == "detail" {
                    if let Some(ref pet) = *selected_pet.read() {
                        ProductDetailPage {
                            pet: pet.clone(),
                            on_back: {
                                let mut page = current_page;
                                move |_| page.set("home".to_string())
                            },
                            on_add_to_cart: EventHandler::new({
                                let mut cart_sig = cart;
                                let msg = message.clone();
                                let pet_clone = pet.clone();
                                move |quantity: u32| {
                                    let mut cart_items = cart_sig.read().clone();
                                    if let Some(item) = cart_items.iter_mut().find(|item| item.pet.id == pet_clone.id) {
                                        item.quantity += quantity;
                                    } else {
                                        cart_items.push(CartItem { pet: pet_clone.clone(), quantity });
                                    }
                                    cart_sig.set(cart_items);
                                    if let Some(m) = msg.clone() {
                                        m.success(format!("已添加 {} 件到购物车", quantity));
                                    }
                                }
                            }),
                        }
                    }
                } else if *current_page.read() == "cart" {
                    ShoppingCartPage {
                        cart: cart.read().clone(),
                            on_update_quantity: EventHandler::new({
                                let mut cart_sig = cart;
                                move |(pet_id, quantity): (u32, u32)| {
                                    let mut items = cart_sig.read().clone();
                                    if quantity == 0 {
                                        items.retain(|item| item.pet.id != pet_id);
                                    } else {
                                        if let Some(item) = items.iter_mut().find(|item| item.pet.id == pet_id) {
                                            item.quantity = quantity;
                                        }
                                    }
                                    cart_sig.set(items);
                                }
                            }),
                        on_remove: {
                            let mut cart_sig = cart;
                            move |pet_id: u32| {
                                let mut items = cart_sig.read().clone();
                                items.retain(|item| item.pet.id != pet_id);
                                cart_sig.set(items);
                            }
                        },
                        on_checkout: {
                            let mut page = current_page;
                            move |_| page.set("checkout".to_string())
                        },
                    }
                } else if *current_page.read() == "checkout" {
                    CheckoutPage {
                        cart: cart.read().clone(),
                        on_back: {
                            let mut page = current_page;
                            move |_| page.set("cart".to_string())
                        },
                            on_complete: EventHandler::new({
                                let mut page = current_page;
                                let mut cart_sig = cart;
                                let notif = notification.clone();
                                move |_| {
                                    cart_sig.set(Vec::new());
                                    page.set("home".to_string());
                                    if let Some(n) = notif.clone() {
                                        n.success("订单提交成功", Some("您的订单已成功提交，我们将在24小时内处理".to_string()));
                                    }
                                }
                            }),
                    }
                }
            }
            
            Footer {
                style: "text-align: center; padding: 32px 24px; background: #ffffff; border-top: 1px solid rgba(0, 0, 0, 0.06); color: #8e8ea0; margin-top: 64px;",
                div {
                    style: "font-size: 13px; font-weight: 400;",
                    "© 2024 Pet Shop Demo. All rights reserved."
                }
            }
        }
        
        // 筛选抽屉
        FilterDrawer {
            open: *filter_drawer_open.read(),
            on_close: {
                let mut open = filter_drawer_open;
                move |_| open.set(false)
            },
        }
        
        // 返回顶部按钮
        BackTop {}
    }
}

// 首页组件
#[derive(Props, Clone, PartialEq)]
struct HomePageProps {
    pets: Vec<Pet>,
    all_pets: Vec<Pet>,
    selected_category: Option<String>,
    current_page_num: u32,
    page_size: u32,
    total: u32,
    on_category_change: EventHandler<Option<String>>,
    on_page_change: EventHandler<u32>,
    on_pet_click: EventHandler<Pet>,
    on_add_to_cart: EventHandler<Pet>,
    on_filter_click: EventHandler<()>,
}

#[component]
fn HomePage(props: HomePageProps) -> Element {
    let categories = vec!["全部", "狗", "猫", "鸟", "鱼", "兔子"];
    let category_options = categories.iter().map(|cat| {
        SegmentedOption {
            label: cat.to_string(),
            value: cat.to_string(),
            icon: None,
            tooltip: None,
            disabled: false,
        }
    }).collect::<Vec<_>>();
    
    let selected_cat_value = props.selected_category.as_ref()
        .map(|s| s.clone())
        .unwrap_or_else(|| "全部".to_string());
    
    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 40px 24px;",
            // 轮播图
            div {
                style: "margin-bottom: 48px; border-radius: 16px; overflow: hidden; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);",
                Carousel {
                    items: vec![
                        CarouselItem::new("热门宠物特惠").with_background("linear-gradient(135deg, #ececf1 0%, #d1d5db 100%)"),
                        CarouselItem::new("新品上市").with_background("linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)"),
                        CarouselItem::new("限时折扣").with_background("linear-gradient(135deg, #f9fafb 0%, #f3f4f6 100%)"),
                    ],
                    autoplay: true,
                    arrows: true,
                    effect: CarouselEffect::Fade,
                }
            }
            
            // 分类导航
            div {
                style: "margin-bottom: 40px; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 16px;",
                div {
                    style: "flex: 1; min-width: 300px;",
                    Segmented {
                        options: category_options,
                        value: Some(selected_cat_value.clone()),
                        on_change: {
                            let handler = props.on_category_change;
                            move |v: String| {
                                if v == "全部" {
                                    handler.call(None);
                                } else {
                                    handler.call(Some(v));
                                }
                            }
                        },
                    }
                }
                Button {
                    r#type: ButtonType::Default,
                    onclick: Some(EventHandler::new(move |_| props.on_filter_click.call(()))),
                    styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869;".to_string()),
                    Icon { kind: IconKind::Search }
                    "筛选"
                }
            }
            
            // 商品网格
            if props.pets.is_empty() {
                Empty {
                    image: EmptyImage::Default,
                    description: Some("暂无商品".to_string()),
                }
            } else {
                Row {
                    gutter: Some(16.0),
                    {
                        props.pets.iter().map(|pet| {
                            let pet_clone = pet.clone();
                            let pet_clone2 = pet.clone();
                            rsx! {
                                Col {
                                    span: 24,
                                    responsive: Some(ColResponsive {
                                        xs: Some(ColSize { span: Some(24), ..Default::default() }),
                                        sm: Some(ColSize { span: Some(12), ..Default::default() }),
                                        md: Some(ColSize { span: Some(8), ..Default::default() }),
                                        lg: Some(ColSize { span: Some(6), ..Default::default() }),
                                        ..Default::default()
                                    }),
                                    style: Some("margin-bottom: 24px;".to_string()),
                                    PetCard {
                                        pet: pet_clone.clone(),
                                        on_click: {
                                            let handler = props.on_pet_click;
                                            let p = pet_clone.clone();
                                            move |_| handler.call(p.clone())
                                        },
                                        on_add_to_cart: {
                                            let handler = props.on_add_to_cart;
                                            let p = pet_clone2.clone();
                                            move |_| handler.call(p.clone())
                                        },
                                    }
                                }
                            }
                        })
                    }
                }
                
                // 分页
                if props.total > props.page_size {
                    div {
                        style: "display: flex; justify-content: center; margin-top: 64px; padding-top: 40px; border-top: 1px solid rgba(0, 0, 0, 0.06);",
                        Pagination {
                            current: Some(props.current_page_num as u32),
                            page_size: Some(props.page_size),
                            total: props.total,
                            show_size_changer: true,
                            on_change: {
                                let handler = props.on_page_change;
                                move |(page, _size): (u32, u32)| {
                                    handler.call(page);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

// 商品卡片组件
#[derive(Props, Clone, PartialEq)]
struct PetCardProps {
    pet: Pet,
    on_click: EventHandler<()>,
    on_add_to_cart: EventHandler<()>,
}

#[component]
fn PetCard(props: PetCardProps) -> Element {
    rsx! {
        Card {
            hoverable: true,
            style: "height: 100%; display: flex; flex-direction: column; background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px; transition: all 0.2s ease; box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);",
            div {
                style: "position: relative; margin-bottom: 16px; border-radius: 12px 12px 0 0; overflow: hidden; background: #f9fafb;",
                div {
                    style: "cursor: pointer; transition: transform 0.3s ease;",
                    onclick: move |_| props.on_click.call(()),
                    Image {
                        src: props.pet.image.clone(),
                        alt: Some(props.pet.name.clone()),
                        width: Some("100%".to_string()),
                        height: Some("220px".to_string()),
                        style: Some("object-fit: cover;".to_string()),
                    }
                }
                if !props.pet.tags.is_empty() {
                    div {
                        style: "position: absolute; top: 10px; left: 10px; display: flex; gap: 6px; flex-wrap: wrap; z-index: 1;",
                        for tag in &props.pet.tags {
                            Tag {
                                color: if tag == "折扣" {
                                    TagColor::Error
                                } else if tag == "热销" {
                                    TagColor::Warning
                                } else if tag == "新品" {
                                    TagColor::Primary
                                } else {
                                    TagColor::Default
                                },
                                {tag.clone()}
                            }
                        }
                    }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column; padding: 16px;",
                div {
                    style: "cursor: pointer; margin-bottom: 10px;",
                    onclick: move |_| props.on_click.call(()),
                    Title {
                        level: TitleLevel::H4,
                        style: "margin: 0 0 8px 0; font-weight: 500; color: #202123; font-size: 16px; line-height: 1.4;",
                        {props.pet.name.clone()}
                    }
                }
                div {
                    style: "display: flex; align-items: center; gap: 6px; margin-bottom: 12px;",
                    Rate {
                        value: Some(props.pet.rating),
                        disabled: true,
                    }
                    Text {
                        r#type: TextType::Secondary,
                        style: "font-size: 12px; color: #8e8ea0;",
                        {format!("({})", props.pet.review_count)}
                    }
                }
                div {
                    style: "display: flex; align-items: baseline; gap: 10px; margin-bottom: 16px;",
                    Statistic {
                        value: props.pet.price,
                        precision: Some(2),
                        prefix: Some(rsx!("¥")),
                        style: "font-size: 20px; font-weight: 600; color: #202123;",
                    }
                    if let Some(original) = props.pet.original_price {
                        Text {
                            r#type: TextType::Secondary,
                            style: "text-decoration: line-through; font-size: 14px; color: #8e8ea0;",
                            {format!("¥{:.2}", original)}
                        }
                    }
                }
                div {
                    style: "display: flex; gap: 8px; margin-top: auto;",
                    div {
                        style: "flex: 1;",
                        Button {
                            r#type: ButtonType::Primary,
                            onclick: Some(EventHandler::new(move |_| props.on_add_to_cart.call(()))),
                            styles_root: Some("background: #10a37f; border: none; border-radius: 8px; color: white; font-weight: 400;".to_string()),
                            "加入购物车"
                        }
                    }
                    Button {
                        r#type: ButtonType::Default,
                        onclick: move |_| props.on_click.call(()),
                        styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869; font-weight: 400;".to_string()),
                        "查看详情"
                    }
                }
            }
        }
    }
}

// 商品详情页组件
#[derive(Props, Clone, PartialEq)]
struct ProductDetailPageProps {
    pet: Pet,
    on_back: EventHandler<()>,
    on_add_to_cart: EventHandler<u32>,
}

#[component]
fn ProductDetailPage(props: ProductDetailPageProps) -> Element {
    let quantity = use_signal(|| 1u32);
    let preview_visible = use_signal(|| false);
    let mut preview_current = use_signal(|| 0usize);
    
    let preview_items: Vec<ImagePreviewItem> = props.pet.images.iter()
        .map(|url| ImagePreviewItem::new(url.clone()).with_alt(props.pet.name.clone()))
        .collect();
    
    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 40px 24px; background: #f7f7f8; min-height: calc(100vh - 64px - 80px);",
            div {
                style: "margin-bottom: 32px;",
                Button {
                    r#type: ButtonType::Text,
                    onclick: Some(EventHandler::new(move |_| props.on_back.call(()))),
                    styles_root: Some("color: #565869; font-weight: 400; padding: 8px 0;".to_string()),
                    Icon { kind: IconKind::ArrowLeft }
                    "返回"
                }
            }
            
            Row {
                gutter: Some(24.0),
                Col {
                    span: 24,
                    responsive: Some(ColResponsive {
                        xs: Some(ColSize { span: Some(24), ..Default::default() }),
                        md: Some(ColSize { span: Some(12), ..Default::default() }),
                        ..Default::default()
                    }),
                    div {
                        style: "position: relative;",
                        div {
                            style: "cursor: pointer;",
                            onclick: {
                                let mut visible = preview_visible;
                                move |_| {
                                    visible.set(true);
                                    preview_current.set(0);
                                }
                            },
                            Image {
                                src: props.pet.images[0].clone(),
                                alt: Some(props.pet.name.clone()),
                                width: Some("100%".to_string()),
                                style: Some("border-radius: 12px; background: #f9fafb;".to_string()),
                            }
                        }
                        div {
                            style: "display: flex; gap: 8px; margin-top: 16px;",
                            for (idx, img_url) in props.pet.images.iter().take(4).enumerate() {
                                div {
                                    style: "cursor: pointer; border-radius: 8px; overflow: hidden; border: 2px solid transparent; transition: border-color 0.2s;",
                                    onclick: {
                                        let mut visible = preview_visible;
                                        let mut current = preview_current;
                                        let idx_val = idx;
                                        move |_| {
                                            visible.set(true);
                                            current.set(idx_val);
                                        }
                                    },
                                    Image {
                                        src: img_url.clone(),
                                        alt: Some(format!("{} {}", props.pet.name, idx)),
                                        width: Some("80px".to_string()),
                                        height: Some("80px".to_string()),
                                        style: Some("object-fit: cover; background: #f9fafb;".to_string()),
                                    }
                                }
                            }
                        }
                    }
                }
                Col {
                    span: 24,
                    responsive: Some(ColResponsive {
                        xs: Some(ColSize { span: Some(24), ..Default::default() }),
                        md: Some(ColSize { span: Some(12), ..Default::default() }),
                        ..Default::default()
                    }),
                    div {
                        style: "display: flex; flex-direction: column; gap: 24px; padding: 0 24px;",
                        Title {
                            level: TitleLevel::H2,
                            style: "margin: 0; font-weight: 500; color: #202123; font-size: 24px; line-height: 1.3;",
                            {props.pet.name.clone()}
                        }
                        div {
                            style: "display: flex; align-items: center; gap: 10px; padding: 8px 0;",
                            Rate {
                                value: Some(props.pet.rating),
                                disabled: true,
                            }
                            Text {
                                r#type: TextType::Secondary,
                                style: "font-size: 14px; color: #8e8ea0;",
                                {format!("{} 条评价", props.pet.review_count)}
                            }
                        }
                        div {
                            style: "display: flex; align-items: baseline; gap: 16px; padding: 20px; background: #ffffff; border-radius: 12px; border: 1px solid rgba(0, 0, 0, 0.06);",
                            Statistic {
                                value: props.pet.price,
                                precision: Some(2),
                                prefix: Some(rsx!("¥")),
                                style: "font-size: 32px; font-weight: 600; color: #202123;",
                            }
                            if let Some(original) = props.pet.original_price {
                                Text {
                                    r#type: TextType::Secondary,
                                    style: "text-decoration: line-through; font-size: 18px; color: #8e8ea0;",
                                    {format!("¥{:.2}", original)}
                                }
                            }
                        }
                        div {
                            style: "display: flex; gap: 8px; flex-wrap: wrap;",
                            for tag in &props.pet.tags {
                                Tag {
                                    color: if tag == "折扣" {
                                        TagColor::Error
                                    } else if tag == "热销" {
                                        TagColor::Warning
                                    } else if tag == "新品" {
                                        TagColor::Primary
                                    } else {
                                        TagColor::Default
                                    },
                                    {tag.clone()}
                                }
                            }
                            Tag {
                                {format!("品牌: {}", props.pet.brand)}
                            }
                            Tag {
                                {format!("库存: {} 件", props.pet.stock)}
                            }
                        }
                        Divider {}
                        div {
                            style: "display: flex; align-items: center; gap: 16px; padding: 16px; background: #ffffff; border-radius: 12px; border: 1px solid rgba(0, 0, 0, 0.06);",
                            Text { style: "font-weight: 400; color: #565869;", "数量：" }
                            InputNumber {
                                value: Some(*quantity.read() as f64),
                                min: Some(1.0),
                                max: Some(props.pet.stock as f64),
                                on_change: Some(EventHandler::new({
                                    let mut qty = quantity;
                                    move |v: Option<f64>| {
                                        if let Some(val) = v {
                                            qty.set(val as u32);
                                        }
                                    }
                                })),
                            }
                        }
                        div {
                            style: "display: flex; gap: 12px;",
                            div {
                                style: "flex: 1;",
                                Button {
                                    r#type: ButtonType::Primary,
                                    size: adui_dioxus::ButtonSize::Large,
                                    onclick: Some(EventHandler::new({
                                        let handler = props.on_add_to_cart;
                                        let qty = quantity;
                                        move |_| {
                                            handler.call(*qty.read());
                                        }
                                    })),
                                    styles_root: Some("background: #10a37f; border: none; border-radius: 8px; color: white; font-weight: 400; width: 100%;".to_string()),
                                    "加入购物车"
                                }
                            }
                            div {
                                style: "flex: 1;",
                                Button {
                                    r#type: ButtonType::Default,
                                    size: adui_dioxus::ButtonSize::Large,
                                    styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869; font-weight: 400; width: 100%;".to_string()),
                                    "立即购买"
                                }
                            }
                        }
                        Divider {}
                        Paragraph {
                            style: "line-height: 1.7; color: #565869; font-size: 15px;",
                            {props.pet.description.clone()}
                        }
                    }
                }
            }
            
            // 商品详情标签页
            div {
                style: "margin-top: 48px;",
                Tabs {
                    items: vec![
                        TabItem::new("desc", "商品描述", Some(rsx!(
                            Paragraph {
                                style: "line-height: 1.7; color: #565869; font-size: 15px; padding: 20px; background: #ffffff; border-radius: 12px; border: 1px solid rgba(0, 0, 0, 0.06);",
                                {props.pet.description.clone()}
                            }
                        ))),
                        TabItem::new("spec", "规格参数", Some(rsx!(
                            Descriptions {
                                items: props.pet.specifications.iter().map(|(key, value)| {
                                    DescriptionsItem::new(
                                        key.clone(),
                                        rsx! { {key.clone()} },
                                        rsx! { {value.clone()} }
                                    )
                                }).collect(),
                            }
                        ))),
                        TabItem::new("reviews", "用户评价", Some(rsx!(
                            div {
                                style: "display: flex; flex-direction: column; gap: 16px;",
                                for review in &props.pet.reviews {
                        div {
                            style: "display: flex; gap: 16px; padding: 20px; background: #ffffff; border-radius: 12px; border: 1px solid rgba(0, 0, 0, 0.06); transition: box-shadow 0.2s;",
                                        Avatar {
                                            src: Some(review.avatar.clone()),
                                            size: Some(adui_dioxus::AvatarSize::Large),
                                        }
                                        div {
                                            style: "flex: 1;",
                                            div {
                                                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                                                Text {
                                                    style: "font-weight: 500; font-size: 15px; color: #202123;",
                                                    {review.user.clone()}
                                                }
                                                Text {
                                                    r#type: TextType::Secondary,
                                                    style: "font-size: 13px; color: #8e8ea0;",
                                                    {review.date.clone()}
                                                }
                                            }
                                            div {
                                                style: "margin-bottom: 10px;",
                                                Rate {
                                                    value: Some(review.rating),
                                                    disabled: true,
                                                }
                                            }
                                            Paragraph {
                                                style: "margin: 0; line-height: 1.7; color: #565869; font-size: 14px;",
                                                {review.comment.clone()}
                                            }
                                        }
                                    }
                                }
                            }
                        ))),
                    ],
                }
            }
            
            // 图片预览组
            ImagePreviewGroup {
                items: preview_items,
                visible: *preview_visible.read(),
                current: *preview_current.read(),
                on_visible_change: Some(EventHandler::new({
                    let mut visible = preview_visible;
                    move |v| visible.set(v)
                })),
                on_change: Some(EventHandler::new({
                    let mut current = preview_current;
                    move |idx| current.set(idx)
                })),
            }
        }
    }
}

// 购物车页面组件
#[derive(Props, Clone, PartialEq)]
struct ShoppingCartPageProps {
    cart: Vec<CartItem>,
    on_update_quantity: EventHandler<(u32, u32)>,
    on_remove: EventHandler<u32>,
    on_checkout: EventHandler<()>,
}

#[component]
fn ShoppingCartPage(props: ShoppingCartPageProps) -> Element {
    let total_price = props.cart.iter()
        .map(|item| item.pet.price * item.quantity as f64)
        .sum::<f64>();
    
    let total_count = props.cart.iter()
        .map(|item| item.quantity)
        .sum::<u32>();
    
    if props.cart.is_empty() {
        return rsx! {
            div {
                style: "max-width: 1200px; margin: 0 auto; padding: 80px 24px; display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 400px; background: #f7f7f8;",
                Empty {
                    image: EmptyImage::Default,
                    description: Some("购物车是空的".to_string()),
                }
            }
        };
    }
    
    rsx! {
        div {
            style: "max-width: 1200px; margin: 0 auto; padding: 40px 24px; background: #f7f7f8; min-height: calc(100vh - 64px - 80px);",
            Title {
                level: TitleLevel::H2,
                style: "margin-bottom: 32px; font-weight: 500; color: #202123; font-size: 24px;",
                "购物车"
            }
            
            Row {
                gutter: Some(24.0),
                Col {
                    span: 24,
                    responsive: Some(ColResponsive {
                        xs: Some(ColSize { span: Some(24), ..Default::default() }),
                        lg: Some(ColSize { span: Some(16), ..Default::default() }),
                        ..Default::default()
                    }),
                    Card {
                        style: "background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        div {
                            style: "display: flex; flex-direction: column; gap: 16px;",
                            for item in &props.cart {
                                div {
                                    style: "display: flex; align-items: center; gap: 20px; padding: 20px; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px; background: #ffffff; transition: box-shadow 0.2s;",
                                    Image {
                                        src: item.pet.image.clone(),
                                        alt: Some(item.pet.name.clone()),
                                        width: Some("100px".to_string()),
                                        height: Some("100px".to_string()),
                                        style: Some("object-fit: cover; border-radius: 8px; flex-shrink: 0;".to_string()),
                                    }
                                    div {
                                        style: "flex: 1; min-width: 0;",
                                        Title {
                                            level: TitleLevel::H5,
                                            style: "margin: 0 0 12px 0; font-weight: 500; color: #202123; font-size: 16px;",
                                            {item.pet.name.clone()}
                                        }
                                        div {
                                            style: "display: flex; align-items: center; gap: 20px; flex-wrap: wrap;",
                                            Text {
                                                r#type: TextType::Secondary,
                                                style: "font-size: 14px; color: #8e8ea0;",
                                                {format!("单价: ¥{:.2}", item.pet.price)}
                                            }
                                            div {
                                                style: "display: flex; align-items: center; gap: 8px;",
                                                Text { style: "font-size: 14px; color: #565869;", "数量: " }
                                                InputNumber {
                                                    value: Some(item.quantity as f64),
                                                    min: Some(1.0),
                                                    on_change: Some(EventHandler::new({
                                                        let handler = props.on_update_quantity;
                                                        let pet_id = item.pet.id;
                                                        move |v: Option<f64>| {
                                                            if let Some(val) = v {
                                                                handler.call((pet_id, val as u32));
                                                            }
                                                        }
                                                    })),
                                                }
                                            }
                                            Text {
                                                style: "font-weight: 600; font-size: 16px; color: #202123;",
                                                {format!("小计: ¥{:.2}", item.pet.price * item.quantity as f64)}
                                            }
                                        }
                                    }
                                    Popconfirm {
                                        title: "确定要删除这个商品吗？",
                                        on_confirm: {
                                            let handler = props.on_remove;
                                            let pet_id = item.pet.id;
                                            move |_| handler.call(pet_id)
                                        },
                                        Button {
                                            r#type: ButtonType::Text,
                                            danger: true,
                                            "删除"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Col {
                    span: 24,
                    responsive: Some(ColResponsive {
                        xs: Some(ColSize { span: Some(24), ..Default::default() }),
                        lg: Some(ColSize { span: Some(8), ..Default::default() }),
                        ..Default::default()
                    }),
                    Card {
                        title: Some(rsx!("结算信息")),
                        style: "position: sticky; top: 88px; background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        div {
                            style: "display: flex; flex-direction: column; gap: 20px;",
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 0;",
                                Text { style: "font-size: 15px; color: #565869;", "商品数量：" }
                                Text {
                                    style: "font-weight: 500; font-size: 15px; color: #202123;",
                                    {format!("{} 件", total_count)}
                                }
                            }
                            Divider {}
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 20px; background: #f7f7f8; border-radius: 12px;",
                                Text {
                                    style: "font-size: 18px; font-weight: 500; color: #202123;",
                                    "合计："
                                }
                                Statistic {
                                    value: total_price,
                                    precision: Some(2),
                                    prefix: Some(rsx!("¥")),
                                    style: "font-size: 28px; font-weight: 600; color: #202123;",
                                }
                            }
                            div {
                                style: "width: 100%;",
                                Button {
                                    r#type: ButtonType::Primary,
                                    size: adui_dioxus::ButtonSize::Large,
                                    onclick: Some(EventHandler::new(move |_| props.on_checkout.call(()))),
                                    styles_root: Some("width: 100%; height: 48px; font-size: 16px; font-weight: 400; background: #10a37f; border: none; border-radius: 8px; color: white;".to_string()),
                                    "去结算"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// 结算页面组件
#[derive(Props, Clone, PartialEq)]
struct CheckoutPageProps {
    cart: Vec<CartItem>,
    on_back: EventHandler<()>,
    on_complete: EventHandler<()>,
}

#[component]
fn CheckoutPage(props: CheckoutPageProps) -> Element {
    let current_step = use_signal(|| 0u32);
    let delivery_method = use_signal(|| Some("standard".to_string()));
    let payment_method = use_signal(|| Some("alipay".to_string()));
    
    let total_price = props.cart.iter()
        .map(|item| item.pet.price * item.quantity as f64)
        .sum::<f64>();
    
    let steps = vec![
        StepItem::new("confirm", rsx!("确认订单")),
        StepItem::new("delivery", rsx!("选择配送")),
        StepItem::new("payment", rsx!("支付方式")),
        StepItem::new("complete", rsx!("完成")),
    ];
    
    rsx! {
        div {
            style: "max-width: 900px; margin: 0 auto; padding: 40px 24px; background: #f7f7f8; min-height: calc(100vh - 64px - 80px);",
            Title {
                level: TitleLevel::H2,
                style: "margin-bottom: 40px; font-weight: 500; color: #202123; font-size: 24px;",
                "结算"
            }
            
            Steps {
                current: Some(*current_step.read() as usize),
                items: steps,
                direction: StepsDirection::Horizontal,
            }
            
            div {
                style: "margin: 32px 0;",
                if *current_step.read() == 0 {
                    Card {
                        title: Some(rsx!("订单确认")),
                        style: "background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        div {
                            style: "display: flex; flex-direction: column; gap: 16px;",
                            div {
                                style: "display: flex; justify-content: space-between; padding: 12px 0;",
                                Text { style: "color: #565869;", "商品总价：" }
                                Statistic {
                                    value: total_price,
                                    precision: Some(2),
                                    prefix: Some(rsx!("¥")),
                                    style: "color: #202123;",
                                }
                            }
                            div {
                                style: "display: flex; justify-content: space-between; padding: 12px 0;",
                                Text { style: "color: #565869;", "运费：" }
                                Text { style: "color: #202123;", "¥0.00" }
                            }
                            Divider {}
                            div {
                                style: "display: flex; justify-content: space-between; align-items: center; padding: 20px; background: #f7f7f8; border-radius: 12px;",
                                Text {
                                    style: "font-size: 18px; font-weight: 500; color: #202123;",
                                    "实付金额："
                                }
                                Statistic {
                                    value: total_price,
                                    precision: Some(2),
                                    prefix: Some(rsx!("¥")),
                                    style: "font-size: 24px; font-weight: 600; color: #202123;",
                                }
                            }
                            div {
                                style: "display: flex; justify-content: flex-end; gap: 12px; margin-top: 32px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: Some(EventHandler::new(move |_| props.on_back.call(()))),
                                styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869; font-weight: 400;".to_string()),
                                "返回"
                            }
                                Button {
                                    r#type: ButtonType::Primary,
                                    onclick: {
                                        let mut step = current_step;
                                        move |_| step.set(1)
                                    },
                                    styles_root: Some("background: #10a37f; border: none; border-radius: 8px; color: white; font-weight: 400;".to_string()),
                                    "下一步"
                                }
                            }
                        }
                    }
                } else if *current_step.read() == 1 {
                    Card {
                        title: Some(rsx!("选择配送方式")),
                        style: "background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        RadioGroup {
                            value: delivery_method.read().clone(),
                            on_change: {
                                let mut method = delivery_method;
                                move |v: String| method.set(Some(v))
                            },
                            Radio {
                                value: "standard".to_string(),
                                "标准配送（3-5个工作日）"
                            }
                            Radio {
                                value: "express".to_string(),
                                "快递配送（1-2个工作日）"
                            }
                            Radio {
                                value: "overnight".to_string(),
                                "次日达"
                            }
                        }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px; margin-top: 32px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: {
                                    let mut step = current_step;
                                    move |_| step.set(0)
                                },
                                styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869; font-weight: 400;".to_string()),
                                "上一步"
                            }
                            Button {
                                r#type: ButtonType::Primary,
                                onclick: {
                                    let mut step = current_step;
                                    move |_| step.set(2)
                                },
                                styles_root: Some("background: #10a37f; border: none; border-radius: 8px; color: white; font-weight: 400;".to_string()),
                                "下一步"
                            }
                        }
                    }
                } else if *current_step.read() == 2 {
                    Card {
                        title: Some(rsx!("选择支付方式")),
                        style: "background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        RadioGroup {
                            value: payment_method.read().clone(),
                            on_change: {
                                let mut method = payment_method;
                                move |v: String| method.set(Some(v))
                            },
                            Radio {
                                value: "alipay".to_string(),
                                "支付宝"
                            }
                            Radio {
                                value: "wechat".to_string(),
                                "微信支付"
                            }
                            Radio {
                                value: "card".to_string(),
                                "银行卡"
                            }
                        }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px; margin-top: 32px;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: {
                                    let mut step = current_step;
                                    move |_| step.set(1)
                                },
                                styles_root: Some("background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.1); border-radius: 8px; color: #565869; font-weight: 400;".to_string()),
                                "上一步"
                            }
                            Button {
                                r#type: ButtonType::Primary,
                                onclick: {
                                    let mut step = current_step;
                                    move |_| step.set(3)
                                },
                                styles_root: Some("background: #10a37f; border: none; border-radius: 8px; color: white; font-weight: 400;".to_string()),
                                "确认支付"
                            }
                        }
                    }
                } else {
                    Card {
                        title: Some(rsx!("订单完成")),
                        style: "background: #ffffff; border: 1px solid rgba(0, 0, 0, 0.06); border-radius: 12px;",
                        div {
                            style: "text-align: center; padding: 64px 32px;",
                            div {
                                style: "font-size: 64px; margin-bottom: 24px;",
                                "✅"
                            }
                            Title {
                                level: TitleLevel::H3,
                                style: "margin-bottom: 16px; font-weight: 500; color: #202123; font-size: 22px;",
                                "订单提交成功！"
                            }
                            Paragraph {
                                style: "margin: 16px 0 40px 0; color: #8e8ea0; font-size: 15px; line-height: 1.7;",
                                "您的订单已成功提交，我们将在24小时内处理并安排发货。"
                            }
                            Progress {
                                percent: 100.0,
                                status: Some(adui_dioxus::ProgressStatus::Success),
                            }
                            div {
                                style: "margin-top: 40px;",
                            Button {
                                r#type: ButtonType::Primary,
                                size: adui_dioxus::ButtonSize::Large,
                                onclick: Some(EventHandler::new(move |_| props.on_complete.call(()))),
                                styles_root: Some("padding: 0 32px; height: 44px; font-size: 16px; font-weight: 400; background: #10a37f; border: none; border-radius: 8px; color: white;".to_string()),
                                "返回首页"
                            }
                            }
                        }
                    }
                }
            }
        }
    }
}

// 筛选抽屉组件
#[derive(Props, Clone, PartialEq)]
struct FilterDrawerProps {
    open: bool,
    on_close: EventHandler<()>,
}

#[component]
fn FilterDrawer(props: FilterDrawerProps) -> Element {
    let price_range = use_signal(|| vec![0.0, 1000.0]);
    let selected_brands = use_signal(|| Vec::<String>::new());
    let min_rating = use_signal(|| None::<f64>);
    
    let brands = vec!["PetLove", "HappyPaws", "FurryFriends", "PetParadise", "AnimalKingdom"];
    
    rsx! {
        Drawer {
            open: props.open,
            title: Some("筛选条件".to_string()),
            placement: DrawerPlacement::Right,
            size: Some(400.0),
            on_close: props.on_close,
            children: rsx! {
                div {
                    style: "display: flex; flex-direction: column; gap: 24px;",
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        Text {
                            style: "font-weight: 600;",
                            "价格范围"
                        }
                        Slider {
                            range: true,
                            min: 0.0,
                            max: 1000.0,
                            value: Some(SliderValue::Range(price_range.read()[0], price_range.read()[1])),
                            on_change: {
                                let mut range = price_range;
                                move |v: SliderValue| {
                                    match v {
                                        SliderValue::Range(start, end) => range.set(vec![start, end]),
                                        SliderValue::Single(val) => range.set(vec![val, val]),
                                    }
                                }
                            },
                        }
                        div {
                            style: "display: flex; justify-content: space-between;",
                            Text {
                                r#type: TextType::Secondary,
                                {format!("¥{:.0} - ¥{:.0}", price_range.read()[0], price_range.read()[1])}
                            }
                        }
                    }
                    
                    Divider {}
                    
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        Text {
                            style: "font-weight: 600;",
                            "品牌"
                        }
                        CheckboxGroup {
                            value: Some(selected_brands.read().clone()),
                            on_change: {
                                let mut brands = selected_brands;
                                move |v: Vec<String>| brands.set(v)
                            },
                            for brand in &brands {
                                Checkbox {
                                    value: Some(brand.to_string()),
                                    {brand.clone()}
                                }
                            }
                        }
                    }
                    
                    Divider {}
                    
                    div {
                        style: "display: flex; flex-direction: column; gap: 12px;",
                        Text {
                            style: "font-weight: 600;",
                            "最低评分"
                        }
                        Rate {
                            value: *min_rating.read(),
                            on_change: {
                                let mut rating = min_rating;
                                move |v| rating.set(v)
                            },
                        }
                    }
                    
                    div {
                        style: "display: flex; gap: 8px; margin-top: 24px;",
                        div {
                            style: "flex: 1;",
                            Button {
                                r#type: ButtonType::Default,
                                onclick: Some(EventHandler::new({
                                    let mut range = price_range;
                                    let mut brands = selected_brands;
                                    let mut rating = min_rating;
                                    move |_| {
                                        range.set(vec![0.0, 1000.0]);
                                        brands.set(Vec::new());
                                        rating.set(None);
                                    }
                                })),
                                "重置"
                            }
                        }
                        div {
                            style: "flex: 1;",
                            Button {
                                r#type: ButtonType::Primary,
                                onclick: Some(EventHandler::new(move |_| props.on_close.call(()))),
                                "应用"
                            }
                        }
                    }
                }
            },
        }
    }
}

