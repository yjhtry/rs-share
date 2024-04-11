#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod components;
mod data;
pub use components::*;
use data::DATA;
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
    id: String,
    name: String,
    description: String,
    address: String,
    phone: String,
}

impl RenderCell for Data {
    fn render(&self, dataIndex: &'static str, _: usize) -> Element {
        match dataIndex {
            "name" => rsx! {  "{self.name}" },
            "description" => rsx! { "{self.description}" },
            "address" => rsx! { "{self.address}" },
            "phone" => rsx! { "{self.phone}" },
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
            title: "Phone",
            dataIndex: "phone",
        },
    ];

    let data: Vec<Data> = serde_json::from_str(DATA).unwrap();

    let data: Vec<Data> = data.into_iter().take(1000).collect();

    rsx! {
        div {
            class: "p-6",
            Table {
                columns: columns,
                dataSource: data,
                on_selected_all: |selectedAll: bool|  {
                    log::info!("selectedAll: {}", selectedAll)
                },
                on_selected_change: |selected: Data|  {
                    log::info!("selected data: {:?}", selected)
                }
            }
        }
    }
}
