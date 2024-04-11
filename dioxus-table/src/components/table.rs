use dioxus::prelude::*;

pub type Columns = Vec<Column>;

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub title: &'static str,
    pub dataIndex: &'static str,
}

pub trait RenderCell {
    fn render(&self, dataIndex: &'static str, index: usize) -> Element;
}

#[derive(Clone, PartialEq, Props)]
pub struct TableProps<T: RenderCell + PartialEq + Clone + 'static> {
    columns: Columns,
    dataSource: Vec<T>,
    on_selected_change: Option<EventHandler<T>>,
    on_selected_all: Option<EventHandler<bool>>,
}

#[component]
pub fn Table<T: RenderCell + PartialEq + Clone>(props: TableProps<T>) -> Element {
    let columns = props.columns;
    let dataSource = props.dataSource;
    let mut selectedAll = use_signal(|| false);

    let headers = columns
        .iter()
        .map(|column| column.title)
        .collect::<Vec<&str>>();

    use_effect(move || {
        if props.on_selected_all.is_some() {
            props.on_selected_all.unwrap()(*selectedAll.read())
        }
    });

    rsx! {
      div {
        div {
          class: "text-right text-xl font-bold px-2 py-4",
          "共有 {dataSource.len()} 条数据"
        }
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
                      class: "bg-gray-50 text-xs text-gray-400 font-semibold px-2",
                      tr {
                        th {
                          class: "whitespace-normal p-2",
                          div {
                            class: "flex justify-center items-center",
                            input {
                              r#type: "checkbox",
                              checked: selectedAll,
                              onclick: move |_| {
                                selectedAll.toggle()
                              }
                            }
                          }
                        }
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
                      for (index, data) in dataSource.into_iter().enumerate() {
                        tr {
                          td {
                            class: "whitespace-nowrap p-2",
                            div {
                              class: "flex justify-center items-center",
                              input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                  if props.on_selected_change.is_some() {
                                    props.on_selected_change.unwrap()(data.clone());
                                  }
                                }
                              }
                            }
                          }
                          for column in columns.iter() {
                            td {
                              class: "whitespace-nowrap p-2",
                              div {
                                class: "flex justify-center items-center",
                                div {
                                  class: "text-gray-800 font-medium max-w-[200px] whitespace-nowrap overflow-ellipsis overflow-hidden",
                                  {data.render(column.dataIndex, index)}
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
    }
}
