use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CssEditorProps {
    pub content: String,
    pub on_change: Callback<String>,
}

#[function_component(CssEditor)]
pub fn css_editor(props: &CssEditorProps) -> Html {
    let content = props.content.clone();
    let on_change = props.on_change.clone();

    let handle_input = Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
        let value = input.value();
        on_change.emit(value);
    });

    html! {
        <div class="css-editor">
            <h1>{"CSS Editor"}</h1>
            <textarea
                value={content}
                oninput={handle_input}
                cols=30
                rows=10
            />
        </div>
    }
}
