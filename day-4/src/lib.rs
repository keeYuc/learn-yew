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
    let my_class = "my_title";
    //let message = Some("I'm a message");
    let message: Option<&str> = None; //None编译器推导不出类型
                                      //let todos = vec![
                                      //    html!(<li>{"A"}</li>),
                                      //    html!(<li>{"B"}</li>),
                                      //    html!(<li>{"C"}</li>),
                                      //];
    let todos = vec!["yi", "er", "san"];
    html!(
        <>
            <h1 class={my_class}>{"hello world"}</h1>
            if my_class=="my_title"{
                <p>{"Hi there!"}</p>
            }else{
                <p>{"I'm not a title!"}</p>
            }
            if let Some(msg)=message{
                <p>{msg}</p>
            }else{
                <p>{"No message to see today"}</p>
            }
            <ul>
                //{todos}

                //{todos.iter().map(|item|html!(<li>{item}</li>)).collect::<Html>()}//需要手动指定返回类型
                {list_to_html(todos)}
            </ul>
        </>
    )
}

fn list_to_html(items: Vec<&str>) -> Vec<Html> {
    //已经标注返回值collect会自动推导类型
    items.iter().map(|item| html!(<li>{item}</li>)).collect()
}
