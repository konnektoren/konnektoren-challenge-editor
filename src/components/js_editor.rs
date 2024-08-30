use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct JsEditorProps {
    pub js: String,
    pub on_change: Callback<String>,
}

#[function_component(JsEditor)]
pub fn js_editor(props: &JsEditorProps) -> Html {
    let on_change = props.on_change.clone();

    let handle_input = Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
        let value = input.value();
        on_change.emit(value);
    });

    html! {
        <div class="js-editor">
            <h1>{"JS Editor"}</h1>
            <textarea
                value={props.js.clone()}
                oninput={handle_input}
                cols=30
                rows=10
            />
        </div>
    }
}
