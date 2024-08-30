use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct YamlEditorProps {
    pub yaml: String,
    pub on_change: Callback<String>,
}

#[function_component(YamlEditor)]
pub fn yaml_editor(props: &YamlEditorProps) -> Html {
    let on_change = props.on_change.clone();

    let handle_input = Callback::from(move |e: InputEvent| {
        let input = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
        let value = input.value();
        on_change.emit(value);
    });

    html! {
        <div class="yaml-editor">
            <h1>{"YAML Editor"}</h1>
            <textarea
                value={props.yaml.clone()}
                oninput={handle_input}
                cols=30
                rows=10
            />
        </div>
    }
}
