use crate::components::{CssEditor, HtmlEditor, HtmlView, JsEditor, YamlEditor};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let html_content = use_state(|| include_str!("assets/html.html").to_string());
    let css_content = use_state(|| include_str!("assets/css.css").to_string());
    let yaml_content = use_state(|| include_str!("assets/articles.yml").to_string());
    let js_content = use_state(|| include_str!("assets/js.js").to_string());

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

    let update_yaml_content = {
        let yaml_content = yaml_content.clone();
        Callback::from(move |new_content: String| {
            yaml_content.set(new_content);
        })
    };

    let update_js_content = {
        let js_content = js_content.clone();
        Callback::from(move |new_content: String| {
            js_content.set(new_content);
        })
    };

    html! {
    <div class="app">
        <h1>{"Konnektoren Challenge Editor"}</h1>
        <HtmlEditor content={(*html_content).clone()} on_change={update_html_content} />
        <CssEditor content={(*css_content).clone()} on_change={update_content} />
        <HtmlView html={(*html_content).clone()} css={(*css_content).clone()} js={(*js_content).clone()} />
        <YamlEditor yaml={(*yaml_content).clone()} on_change={update_yaml_content} />
        <JsEditor js={(*js_content).clone()} on_change={update_js_content} />
    </div>
    }
}
