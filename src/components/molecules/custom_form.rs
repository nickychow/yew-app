// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::atoms::LabelInput;
use crate::pages::User;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<User>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(User::default);
    let context = use_context::<User>();
    let user_context = context.unwrap_or_default();
    let Props { onsubmit } = props;

    let username_changed = {
        let state = state.clone();

        Callback::from(move |username| {
            let mut user = (*state).clone();
            user.username = username;
            state.set(user);
        })
    };

    let language_changed = {
        let state = state.clone();

        Callback::from(move |favorite_language| {
            let mut user = (*state).clone();
            user.favorite_language = favorite_language;
            state.set(user);
        })
    };

    let onsubmit = {
        let state = state;
        let onsubmit = onsubmit.clone();

        Callback::from(move |event: FocusEvent| {
            // Prevent the form from submitting
            event.prevent_default();

            let user = (*state).clone();
            onsubmit.emit(user);
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
        <form onsubmit={onsubmit} class="container flex flex-col mx-auto max-w-sm gap-4 py-4 " >
            <LabelInput name="username" onchange={username_changed} />
            <LabelInput name="favorite_language" placeholder="Rust" onchange={language_changed} />
            <button class={button_classes} >{"Submit"}</button>
            <p>{"Username: "}{user_context.username}</p>
            <p>{"Language: "}{user_context.favorite_language}</p>
        </form>
    }
}
