// use gloo::console::log;
use yew::prelude::*;

use crate::components::atoms::HelloP;
use crate::components::atoms::SubmitButton;
use crate::components::atoms::TextInput;

#[derive(Clone)]
struct Data {
    pub name: String,
    pub count: u32,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            name: "Stranger".to_owned(),
            count: 0,
        }
    }
}

#[function_component(HelloForm)]
pub fn hello_form() -> Html {
    let state = use_state(Data::default);

    let name_changed = {
        let state = state.clone();

        Callback::from(move |name| {
            let mut data = (*state).clone();
            data.name = name;
            state.set(data);
        })
    };

    let button_clicked = {
        let state = state.clone();

        Callback::from(move |_e: MouseEvent| {
            let mut data = (*state).clone();
            data.count += 1;
            state.set(data);

            // log!("{}", e);
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
        <>
            <section class="mx-auto max-w-sm bg-white shadow-md rounded-lg px-8 pt-6 pb-4 mb-4">

                <h1 class="title font-bold text-6xl text-center text-gray-800">{ "Hello Rust WASM" }</h1>
                <div class="mb-4">
                    <label class="block text-gray-700 text-sm font-bold mb-2">
                        { "Name" }
                    </label>
                    <TextInput name="name" classes={input_classes} handle_onchange={name_changed} />
                    <SubmitButton label="Say Hello" classes={button_classes} onclick={button_clicked}/>
                </div>

            </section>

            <section class="mx-auto max-w-sm bg-white shadow-md rounded-lg px-8 pt-6 pb-4">
                <HelloP name={state.name.clone()} />
                <p>{ format!("Button has been clicked {} times", &state.count) }</p>
            </section>
        </>
    }
}
