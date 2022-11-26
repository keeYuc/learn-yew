use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    foverite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "keey";
    let my_object = MyObject {
        username: name.to_owned(),
        foverite_language: "rust".to_owned(),
    };
    log!("%d hello %s", 1, name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    html!(
        <h1>{"hello world"}</h1>
    )
}
