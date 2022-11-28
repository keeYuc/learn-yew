use gloo::console::log;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    let stysheet = style!(
        r#"
            h1 {
                color:red;
            }
            p {
                color:blue;
            }
        "#
    ) //创建无须转义的字符串 r#""#
    .unwrap();
    html!(
        <div class={stysheet}>
            <h1 >{"hello world"}</h1>
            <p>{"more words"}</p>
        </div>
    )
}
