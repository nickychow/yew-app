use std::ops::Deref;

// use gloo::console::log;
use yew::prelude::*;

// use crate::components::atoms::HelloP;
use crate::components::atoms::SubmitButton;
use crate::components::atoms::TextInput;

#[derive(Default, Clone)]
struct Data {
    pub name: String,
    pub count: u32,
}

#[function_component(HelloForm)]
pub fn hello_form() -> Html {
    let state = use_state(Data::default);

    let name_changed = {
        let state = state.clone();
        Callback::from(move |name| {
            let mut data = state.deref().clone();
            data.name = name;
            state.set(data);
        })
    };

    let button_clicked = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut data = state.deref().clone();
            data.count += 1;
            state.set(data);
        })
    };

    let input_classes = {
        classes!(vec![
            "shadow",
            "appearance-none",
            "border",
            "rounded",
            "w-full",
            "py-2",
            "px-3",
            "mb-4",
            "text-gray-700",
            "leading-tight",
            "focus:outline-none",
            "focus:shadow-outline"
        ])
    };

    let button_classes = {
        classes!(vec![
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
        ])
    };

    html! {
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="nameInput">
                { "Name" }
            </label>
            <TextInput name="name" class={input_classes} handle_onchange={name_changed} />
            <SubmitButton label="Say Hello" class={button_classes} onclick={button_clicked}/>

            // <HelloP/>
            <p>{ format!("Hello {}! 👋", &state.name) }</p>
            <p>{ format!("Button has been clicked {} times", &state.count) }</p>
        </div>
    }
}
