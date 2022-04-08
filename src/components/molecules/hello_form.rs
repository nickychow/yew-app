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
    let cloned_state = state.clone();

    // let name_state = use_state(|| "Stranger".to_owned());
    // let cloned_name_state = name_state.clone();

    // let name_changed = Callback::from(move |name| {
    //     cloned_name_state.set(name);
    // });

    // let button_count_state = use_state(|| 0_u32);
    // let cloned_button_count_state = button_count_state.clone();

    // let button_clicked = Callback::from(move |_| {
    //     // cloned_name_state.set(name);
    //     // log!("Button Clicked");
    //     let count = *cloned_button_count_state;
    //     cloned_button_count_state.set(count + 1);
    // });

    let name_changed = Callback::from(move |name| {
        let mut data = cloned_state.deref().clone();
        data.name = name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });

    html! {
        // <form class="mb-4">
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="nameInput">
                { "Name" }
            </label>
            <TextInput name="name" class="shadow appearance-none border rounded w-full py-2 px-3 mb-4 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" handle_onchange={name_changed} />
            <SubmitButton label="Say Hello" class="bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded" onclick={button_clicked}/>

            // <HelloP/>
            <p>{ format!("Hello {}! 👋", &state.name) }</p>
            <p>{ format!("Button has been clicked {} times", &state.count) }</p>
        </div>
        // </form>
    }
}
