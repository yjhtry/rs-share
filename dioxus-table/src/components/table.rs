use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub type Columns = Vec<Column>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub title: &'static str,
    pub dataIndex: &'static str,
}

pub trait RenderCell {
    fn render(&self, dataIndex: &'static str) -> Element;
}

#[component]
pub fn Table(
    columns: ReadOnlySignal<Columns>,
    dataSource: ReadOnlySignal<Vec<Box<dyn RenderCell>>>,
) -> Element {
    let headers = columns
        .read()
        .iter()
        .map(|column| column.title)
        .collect::<Vec<&str>>();

    rsx! {
      section {
        class: "bg-gray-100 text-gray-600 antialiased",
        div {
          class: "h-full flex flex-col justify-center",
          div {
            class: "w-full border border-gray-200 rounded-sm bg-white shadow-lg",
            div {
              class: "p-3",
              div {
                class: "overflow-x-auto",
                table {
                  class: "w-full table-auto",
                  thead {
                    class: "bg-gray-50 text-xs text-gray-400 font-semibold",
                    tr {
                      for header in headers {
                        th {
                          class: "whitespace-normal p-2",
                          div { "{header}" }
                        }
                      }
                    }
                  }
                  tbody {
                    class: "text-sm divide-y divide-gray-100",
                    for (index, data) in dataSource.read().iter().enumerate() {
                      tr {
                        class: if index % 2 == 0 { "bg-gray-50" } else { "" },
                        for column in columns.read().iter() {
                          td {
                            class: "whitespace-nowrap p-2",
                            div {
                              class: "text-gray-800 font-medium text-center",
                              {data.render(column.dataIndex)}
                             }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
}
