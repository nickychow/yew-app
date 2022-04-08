// use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod components;

use crate::components::atoms::HelloP;
use crate::components::molecules::HelloForm;

enum Msg {
    // Click,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    name: String,
}

impl Component for Hello {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "Stranger".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Msg::Click => {
            //     self.name = "World".to_string();
            //     true
            // }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        // let link = ctx.link();
        html! {
            <>
                <section class="col max-w-sm bg-white shadow-md rounded-lg px-8 pt-6 pb-4 mb-4">
                    <h1 class="title font-bold text-6xl text-center text-gray-800">{ "Hello Rust WASM" }</h1>
                    <HelloForm/>
                </section>
                <section id="messageDisplay" class="col max-w-sm bg-white shadow-md rounded-lg px-8 pt-6 pb-4 mb-4">
                    // <p>{ format!("Hello {}! 👋", &self.name) }</p>
                    <HelloP/>
                </section>
            </>
        }
    }
}

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
            <Hello/>
       </main>
    }
}
