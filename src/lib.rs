// use gloo::console::log;
// use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;

use crate::components::molecules::HelloForm;

#[function_component(App)]
pub fn app() -> Html {
    // let name = "Nick";
    // log!(name);

    // let hello = Hello {
    //     name: name.to_string(),
    // };

    // log!(serde_json::to_string_pretty(&hello).unwrap());

    html! {
       <main class="bg-gray-100 h-screen flex flex-col justify-center items-center">
            <HelloForm/>
       </main>
    }
}
