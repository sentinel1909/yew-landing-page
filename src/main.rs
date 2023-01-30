// main.rs

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1><strong>{ "The biggest startup event of the year" }</strong></h1>
            <button>{ "Find out more" }</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}