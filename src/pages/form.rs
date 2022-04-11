use yew::prelude::*;
use yew::ContextProvider;

use crate::components::molecules::CustomForm;

#[derive(Default, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[function_component(Form)]
pub fn form() -> Html {
    let state = use_state(User::default);
    let user = (*state).clone();

    let custom_form_submit = {
        let user_state = state;
        Callback::from(move |data: User| {
            let mut user = (*user_state).clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };

    html! {
        <ContextProvider<User> context={user}>
            <CustomForm onsubmit={custom_form_submit} />
        </ContextProvider<User>>
    }
}
