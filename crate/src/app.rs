use crate::generated::css_classes::C;
use crate::ts_apis;
use seed::*;
use seed::{events::Listener, prelude::*};
use serde::Deserialize;

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub clicks: i32,
    pub search_text: String,
    pub random_number: i32,
    pub clock_time: Option<String>,
    pub should_render_next_frame: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            clicks: 0,
            search_text: Default::default(),
            random_number: ts_apis::helpers::get_random_number(0, 100),
            clock_time: None,
            should_render_next_frame: false,
        }
    }
}

pub fn button(name: &'static str) -> Node<Msg> {
    button![
        attrs![At::Href => "#"],
        class![
            C.mx_1,
            C.py_2,
            C.px_4,
            C.text_white,
            C.font_bold,
            C.border_b_2,
            C.bg_green_500,
            C.border_green_700,
            C.rounded_lg,
            C.hover__bg_green_400,
            C.hover__border_green_400
        ],
        simple_ev(Ev::Click, Msg::Increment),
        name
    ]
}

pub fn card(title: &'static str, price: u32, description: Option<Vec<Node<Msg>>>) -> Node<Msg> {
    div![
        class![ "w-full flex flex-col justify-center m-2 px-4 border rounded rounded-l-lg border-l-4 border-gray-400 shadow-lg border-blue-300" ],
        div![ class![C.py_2],
            span![ class! [C.font_bold, C.text_lg], title ],
        ],
        div![
            class![ C.flex_grow, C.py_2 ],
            description.unwrap_or_else(|| vec![p!["No description"]]),
        ],
        div![
            class![C.py_1],
            span![ class![C.text_sm, C.text_gray_500, C.font_semibold], format!("Price: {price}", price = price) ]
        ],
        div![
            class![C.py_2],
            button("Purchase"),
            button("Download"),
        ]
    ]
}

pub fn side_bar() -> Node<Msg> {
    aside![
        class![
            C.flex_none,
            C.w_full,
            C.h_full,
            C.border_r,
            C.shadow_lg,
            C.bg_gray_800,
            C.h_screen
        ],
        h1![
            class!["h-24 text-white tracking-4 uppercase font-bold p-4 text-left text-middle leading-lg text-lg"],"Rust Academy"],
            side_bar_section("Courses", "book", false),
            side_bar_section("Lists", "address-book", false),
            side_bar_section("Repositories", "address-book", false)
        ]
}

fn side_bar_section_item(name: &'static str) -> Node<Msg> {
    li![a![
        class![
            C.block,
            C.py_2,
            C.text_sm,
            C.pl_5,
            C.text_gray_500,
            C.select_none,
            C.hover__bg_gray_900,
            C.hover__shadow_lg,
            C.hover__text_gray_300,
        ],
        attrs![At::Href => "#"],
        i![class![C.w_4]],
        name
    ]]
}

pub fn side_bar_section(title: &'static str, icon: &'static str, is_collapsed: bool) -> Node<Msg> {
    let icon_class = format!("fa fa-{icon}", icon = icon);
    let post_icon = format!(
        "fa fa-chevron-{icon}",
        icon = if is_collapsed { "right" } else { "down" }
    );
    div![
        class![C.m_2, C.my_3],
        div![
            class!["flex items-center font-bold text-gray-500 uppercase pb-2 px-2 tracking-wide cursor-pointer hover:text-white"], 
            i![ class![C.flex_none, &icon_class] ],
            span![ class!["flex-grow mx-1 select-none"], title], 
            i![ class![C.flex_none, &post_icon] ]
        ],
        //items goes here
        ul![ class![C.list_none],
            side_bar_section_item("Item 1"),
            side_bar_section_item("Item 2"),
        ]
    ]
}

pub fn nav_bar_button(name: &'static str) -> Node<Msg> {
    a![
        attrs![At::Href => "#"],
        class![
            C.flex_none,
            C.ml_2,
            C.p_3,
            //C.font_large,
            C.no_underline,
            C.rounded_lg,
            C.bg_gray_200,
            C.hover__shadow
        ],
        name
    ]
}

pub fn nav_bar() -> Node<Msg> {
    nav![
        class![
            C.flex,
            C.justify_between,
            C.items_center,
            C.bg_white,
            C.m_0,
            C.p_2,
            C.shadow_lg
        ],
        vec![
            div![
                class![C.flex_initial, C.w_1of2, C.mx_auto],
                input![
                    class![C.justify_center, C.w_full, C.border, C.p_2, C.rounded],
                    attrs![At::Placeholder => "Search courses"],
                    input_ev(Ev::Input, Msg::SearchTyped)
                ]
            ],
            div![
                class![C.flex, C.items_center],
                nav_bar_button("Sign In"),
                nav_bar_button("Sign Out"),
            ]
        ]
    ]
}

// ------ ------
//     Init
// ------ ------

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

pub fn window_events(_model: &Model) -> Vec<Listener<Msg>> {
    vec![
        keyboard_ev("keydown", |ev| Msg::KeyPressed(ev.key())),
        // `trigger_update_handler` processes JS event
        // and forwards it to `update` function.
        trigger_update_handler(),
    ]
}

// ------ ------
//    Update
// ------ ------

// We trigger `OnClockTick` only from JS land
#[allow(dead_code)]
// `Deserialize` is required for receiving messages from JS land
// (see `trigger_update_handler`)
#[derive(Clone, Deserialize)]
pub enum Msg {
    Increment,
    SearchTyped(String),
    NewRandomNumber,
    KeyPressed(String),
    OnClockTick(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.clicks += 1,
        Msg::SearchTyped(input) => model.search_text = input,
        Msg::NewRandomNumber => model.random_number = ts_apis::helpers::get_random_number(0, 100),
        Msg::KeyPressed(key) => {
            log!(key);
            orders.skip();
        }
        Msg::OnClockTick(time) => {
            model.clock_time = Some(time);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
    let courses = vec! [
        ("O101 - Meet Onat", 100, "In this course, you will have the chance to talk with Onat and get familiar with his way of thinking. Lunch with Onat is included."),
        ("O102 - Introduction to Gitdb", 100, "We will walk through towards creating a git clone. Lunch with Onat is included."),
        ("O201 - Fundamentals of Heimdall", 255,  "Onat will take you to a journey through his thought process while he was building his signature project Heimdall." ),
        ("O301 - NAND to Kubernetes", 500,   "In this course, Onat will talk about NANDs and Kubernetes. How they are used together to build highly performing distributed systems."),
        ("O401 - Career planning", 750,   "This couse includes one-to-one sessions with Onat for understanding your needs and help you with planning the rest of your career. was building his signature project Heimdall."),
    ] .into_iter().filter(|item| item.0.contains(&model.search_text) || item.2.contains(&model.search_text) ).collect::<Vec<_>>();

    vec![div![
        // dots at the top
        class![C.w_full],
        div![
            class![C.flex],
            div![class![C.flex_none, C.w_64], side_bar()],
            div![
                class![C.flex_grow],
                nav_bar(),
                div![
                    class![C.pt_8, C.px_8],
                    div![
                        class![C.flex, C.justify_between],
                        courses
                            .iter()
                            .map(|c| card(c.0, c.1, Some(vec![p![c.2]])))
                            .collect::<Vec<_>>()
                    ]
                ]
            ]
        ],
    ]]
}
