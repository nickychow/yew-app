use yew::prelude::*;

use crate::components::molecules::PokemonCard;

#[function_component(Pokemon)]
pub fn pokemon() -> Html {
    html! {
        <>
            <PokemonCard />
        </>
    }
}
