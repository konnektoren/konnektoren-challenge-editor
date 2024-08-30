use serde_json::Value;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HtmlViewProps {
    pub html: String,
    pub css: String,
    pub js: String,
    pub yaml: String,
}

#[function_component(HtmlView)]
pub fn html_view(props: &HtmlViewProps) -> Html {
    let parsed = Html::from_html_unchecked(AttrValue::from(props.html.clone()));

    let parsed_yaml: serde_yaml::Value = serde_yaml::from_str(&props.yaml).unwrap();
    let parsed_json: Value = serde_json::to_value(parsed_yaml).unwrap();
    let challenge = &parsed_json["!custom"];
    let challenge_json = serde_json::to_string_pretty(challenge).unwrap();

    html! {
        <div class="html-view">
            <style>
                {props.css.clone()}
            </style>
            <script>
                {format!("const challenge = {}", challenge_json)}
            </script>
            <script>
                {props.js.clone()}
            </script>
            {parsed}
        </div>
    }
}
