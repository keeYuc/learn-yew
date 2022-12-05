use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
mod components;
use components::atome::main_title::MainTitle;
const STYLE_FATH: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stysheet = Style::new(STYLE_FATH).unwrap();
    html!(
        <div class={stysheet}>
            <MainTitle title="fuck you bbay"/>
            <h1 >{"hello world"}</h1>
            <p>{"more words"}</p>
        </div>
    )
}
