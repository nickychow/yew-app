use gloo::console::log;
use yew::prelude::*;

use crate::components::molecules::CustomForm;
use crate::components::molecules::Data;

#[function_component(Form)]
pub fn form() -> Html {
    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is", data.username);
        log!("favorite_language is", data.favorite_language);
    });
    html! {
        <>
            <CustomForm onsubmit={custom_form_submit} />
        </>
    }
}
