use crate::components::{CssEditor, HtmlEditor, HtmlView, JsEditor, YamlEditor};
use gloo::net::http::Request;
use serde_yaml::Value;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let html_content = use_state(|| "".to_string());
    let css_content = use_state(|| "".to_string());
    let yaml_content = use_state(|| include_str!("assets/challenge.yml").to_string());
    let js_content = use_state(|| "".to_string());
    let loading = use_state(|| true);

    let update_html_content = {
        let html_content = html_content.clone();
        Callback::from(move |new_content: String| {
            html_content.set(new_content);
        })
    };

    let update_css_content = {
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

    {
        let yaml_content = yaml_content.clone();
        let html_content = html_content.clone();
        let css_content = css_content.clone();
        let js_content = js_content.clone();
        let loading = loading.clone();

        use_effect_with(yaml_content.clone(), move |_| {
            let yaml_data: Value = serde_yaml::from_str(&yaml_content).unwrap();

            let html_path = yaml_data["html"].as_str().unwrap().to_string();
            let css_path = yaml_data["css"].as_str().unwrap().to_string();
            let js_path = yaml_data["js"].as_str().unwrap().to_string();

            wasm_bindgen_futures::spawn_local({
                let html_content = html_content.clone();
                let loading = loading.clone();
                async move {
                    match fetch_file(&html_path).await {
                        Ok(content) => html_content.set(content),
                        Err(error) => log::error!("Failed to fetch HTML file: {}", error),
                    }
                    loading.set(false);
                }
            });

            wasm_bindgen_futures::spawn_local({
                let css_content = css_content.clone();
                let loading = loading.clone();
                async move {
                    match fetch_file(&css_path).await {
                        Ok(content) => css_content.set(content),
                        Err(error) => log::error!("Failed to fetch CSS file: {}", error),
                    }
                    loading.set(false);
                }
            });

            wasm_bindgen_futures::spawn_local({
                let js_content = js_content.clone();
                let loading = loading.clone();
                async move {
                    match fetch_file(&js_path).await {
                        Ok(content) => js_content.set(content),
                        Err(error) => log::error!("Failed to fetch JS file: {}", error),
                    }
                    loading.set(false);
                }
            });

            || ()
        });
    }

    html! {
        <div class="app">
            <h1>{"Konnektoren Challenge Editor"}</h1>
            {
                if *loading {
                    html! {<p>{"Loading..."}</p>}
                } else {
                    html! {
                        <>
                            <HtmlEditor content={(*html_content).clone()} on_change={update_html_content} />
                            <CssEditor content={(*css_content).clone()} on_change={update_css_content} />
                            <HtmlView html={(*html_content).clone()} css={(*css_content).clone()}
                                js={(*js_content).clone()} yaml={(*yaml_content).clone()} />
                            <YamlEditor yaml={(*yaml_content).clone()} on_change={update_yaml_content} />
                            <JsEditor js={(*js_content).clone()} on_change={update_js_content} />
                        </>
                    }
                }
            }
        </div>
    }
}

async fn fetch_file(path: &str) -> Result<String, String> {
    let response = Request::get(path)
        .send()
        .await
        .map_err(|_| format!("Failed to fetch the file {}", path))?;
    if response.status() == 200 {
        response
            .text()
            .await
            .map_err(|_| format!("Failed to read the file content of {}", path))
    } else {
        Err(format!("File not found: {}", path))
    }
}
