use yew::prelude::*;

use crate::components::organisms::{About, Banner, Projects};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Banner />
            <About />
            <Projects />
            // <PokemonCard />

        </>
    }
}
