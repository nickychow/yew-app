// use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

use crate::components::molecules::HelloForm;

#[function_component(Hello)]
pub fn hello() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(Route::Home);
    });
    html! {
        <>
            <HelloForm />

            <button {onclick}>{"Go Home"}</button>
        </>
    }
}
