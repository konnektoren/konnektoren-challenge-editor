use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Konnektoren Challenge Editor"}</h1>
        </div>
        }
}

fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    yew::Renderer::<App>::new().render();
}
