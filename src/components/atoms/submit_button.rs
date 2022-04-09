use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub classes: Classes,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(SubmitButton)]
pub fn submit_button(props: &Props) -> Html {
    let Props {
        label,
        classes,
        onclick,
    } = props;

    let onclick = onclick.clone();
    let button_onclick = Callback::from(move |e: MouseEvent| {
        onclick.emit(e);
    });

    html! {
        <button class={classes.clone()} onclick={button_onclick}>
            { label }
        </button>
    }
}
