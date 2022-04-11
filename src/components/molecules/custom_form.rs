// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::atoms::LabelInput;

#[derive(PartialEq, Clone, Default)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(Data::default);
    let Props { onsubmit } = props;

    let username_changed = {
        let state = state.clone();

        Callback::from(move |username| {
            let mut data = (*state).clone();
            data.username = username;
            state.set(data);
        })
    };

    let language_changed = {
        let state = state.clone();

        Callback::from(move |favorite_language| {
            let mut data = (*state).clone();
            data.favorite_language = favorite_language;
            state.set(data);
        })
    };

    let onsubmit = {
        let state = state;
        let onsubmit = onsubmit.clone();

        Callback::from(move |event: FocusEvent| {
            // Prevent the form from submitting
            event.prevent_default();

            let data = (*state).clone();
            onsubmit.emit(data);
        })
    };

    let button_classes = {
        classes!(
            "mx-auto",
            "bg-blue-500",
            "hover:bg-blue-400",
            "text-white",
            "font-bold",
            "py-2",
            "px-4",
            "border-b-4",
            "border-blue-700",
            "hover:border-blue-500",
            "rounded"
        )
    };

    html! {
        <form onsubmit={onsubmit} class="container mx-auto max-w-sm gap-4 py-4" >
            <LabelInput name="username" onchange={username_changed} />
            <LabelInput name="favorite_language" placeholder="Rust" onchange={language_changed} />
            <button class={button_classes} >{"Submit"}</button>
        </form>
    }
}
