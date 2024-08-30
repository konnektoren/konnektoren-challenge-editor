use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HtmlViewProps {
    pub html: String,
    pub css: String,
}

#[function_component(HtmlView)]
pub fn html_view(props: &HtmlViewProps) -> Html {
    let parsed = Html::from_html_unchecked(AttrValue::from(props.html.clone()));
    html! {
        <div class="html-view">
            <style>
                {props.css.clone()}
            </style>
            {parsed}
        </div>
    }
}
