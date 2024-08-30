use crate::components::{CssEditor, HtmlEditor, HtmlView};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let html_content = use_state(|| "<div class=\"greeting\">Hello World</div>".to_string());
    let css_content = use_state(|| {
        ".greeting {
color: green;
padding: 20px;
}"
        .to_string()
    });

    let update_html_content = {
        let html_content = html_content.clone();
        Callback::from(move |new_content: String| {
            html_content.set(new_content);
        })
    };

    let update_content = {
        let css_content = css_content.clone();
        Callback::from(move |new_content: String| {
            css_content.set(new_content);
        })
    };

    html! {
    <div>
        <h1>{"Konnektoren Challenge Editor"}</h1>
        <HtmlEditor content={(*html_content).clone()} on_change={update_html_content} />
        <CssEditor content={(*css_content).clone()} on_change={update_content} />
        <HtmlView html={(*html_content).clone()} css={(*css_content).clone()} />
    </div>
    }
}
