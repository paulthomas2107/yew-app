use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello Paul !" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}