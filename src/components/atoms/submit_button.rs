use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub class: Classes,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    let Props {
        label,
        class,
        onclick,
    } = props;

    let onclick = onclick.clone();
    let button_onclick = Callback::from(move |e: MouseEvent| {
        onclick.emit(e);
    });

    html! {
        <button class={class.clone()} onclick={button_onclick}>
            { label }
        </button>
    }
}
