use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub class: String,
    pub onclick: Callback<()>,
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_: MouseEvent| {
        onclick.emit(());
    });

    html! {
        <button class={&props.class} onclick={button_onclick}>
            { &props.label }
        </button>
    }
}
