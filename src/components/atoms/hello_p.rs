use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name: String,
}

#[function_component(HelloP)]
pub fn hellp_p(props: &Props) -> Html {
    let Props { name } = props;
    html! {
        <p>
            { format!("Hello {}! 👋", name) }
        </p>
    }
}
