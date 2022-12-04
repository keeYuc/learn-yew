use gloo::console::log;
use stylist::{Style, yew::styled_component};
use yew::prelude::*;

const STYLE_FATH:&str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let stysheet = Style::new(STYLE_FATH).unwrap();
    html!(
        <div class={stysheet}>
            <h1 >{"hello world"}</h1>
            <p>{"more words"}</p>
        </div>
    )
}
