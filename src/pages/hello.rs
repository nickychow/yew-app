use yew::prelude::*;

use crate::components::molecules::HelloForm;

#[function_component(Hello)]
pub fn hello() -> Html {
    html! {
        <>
            <HelloForm />
        </>
    }
}
