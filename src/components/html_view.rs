use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HtmlViewProps {
    pub content: String,
}

#[function_component(HtmlView)]
pub fn html_view(props: &HtmlViewProps) -> Html {
    let parsed = Html::from_html_unchecked(AttrValue::from(props.content.clone()));
    html! {
        <div class="html-view">
            {parsed}
        </div>
    }
}
