use crate::components::{HtmlEditor, HtmlView};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let content = use_state(|| "".to_string());

    let update_content = {
        let content = content.clone();
        Callback::from(move |new_content: String| {
            content.set(new_content);
        })
    };

    html! {
    <div>
        <h1>{"Konnektoren Challenge Editor"}</h1>
        <HtmlEditor content={(*content).clone()} on_change={update_content} />
        <HtmlView content={(*content).clone()} />
    </div>
    }
}
