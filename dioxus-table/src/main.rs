#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod components;
pub use components::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    name: String,
    description: String,
    address: String,
    age: i32,
}

impl RenderCell for Data {
    fn render(&self, dataIndex: &'static str) -> Element {
        match dataIndex {
            "name" => rsx! {  "{self.name}" },
            "description" => rsx! { "{self.description}" },
            "address" => rsx! { "{self.address}" },
            "age" => rsx! { "{self.age}" },
            _ => rsx! { "" },
        }
    }
}

#[component]
fn Home() -> Element {
    let columns = vec![
        Column {
            title: "Name",
            dataIndex: "name",
        },
        Column {
            title: "Description",
            dataIndex: "description",
        },
        Column {
            title: "Address",
            dataIndex: "address",
        },
        Column {
            title: "Age",
            dataIndex: "age",
        },
    ];

    let data: Vec<Box<dyn RenderCell>> = vec![
        Box::new(Data {
            name: "John Doe".into(),
            description: "Software Engineer".into(),
            address: "1234 Elm St".into(),
            age: 30,
        }),
        Box::new(Data {
            name: "Jane Doe".into(),
            description: "Software Engineer".into(),
            address: "1234 Elm St".into(),
            age: 30,
        }),
    ];

    rsx! {
        div {
            class: "p-6",
            Table {
                columns,
                dataSource: data
            }
        }
    }
}
