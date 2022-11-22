use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <h1>{"hello world"}</h1>
    )
}
