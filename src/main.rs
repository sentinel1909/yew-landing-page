// main.rs

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("container", "d-flex", "align-items-center", "justify-content-center", "h-100")}>
            <div class={classes!("row")}>
                <header class={classes!("text-center", "col-12")}>
                    <h1 class={classes!("text-uppercase")}><strong>{ "The biggest startup event of the year" }</strong></h1>
                </header>
                <div class={classes!("buffer", "col-12")}></div>
                    <section class={classes!("text-center", "col-12")}>
                        <hr />
                    <a href={"https://mailchi.mp/8c5731448ce5/zero-to-mastery"}><button class={classes!("btn", "btn-primary", "btn-x1")}>{ "Find out more" }</button></a>
                    </section>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}