use yew::prelude::*;

#[function_component(HelloP)]
pub fn hellp_p() -> Html {
    html! {
        <p>
            // { format!("Hello {}! 👋", &self.name) }
            {"Hello {}! 👋"}
        </p>
    }
}
