use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}
#[function_component(MainTitle)]
pub fn demo(item: &Props) -> Html {
    html!(
        <h1>{&item.title}</h1>
    )
}
