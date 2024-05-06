#![allow(non_snake_case)]

use dioxus::prelude::*;

use tracing::Level;

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
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
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
    id: i32,
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

    let data = use_resource(move || get_data());
    let data = &*data.read();

    match data {
        Some(Ok(list)) => {
            rsx! {
                div {
                    class: "p-6",
                    Table {
                        columns: columns,
                        dataSource: list.clone(),
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
        _ => rsx! {
            div {
                "loading..."
            }
        },
    }
}

pub async fn get_data() -> Result<Vec<Data>, reqwest::Error> {
    let story = reqwest::get("http://yjh-anything-oss.com/other/people_data.json")
        .await?
        .json::<Vec<Data>>()
        .await?;

    Ok(story)
}
