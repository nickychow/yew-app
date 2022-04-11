use yew::prelude::*;

use crate::components::molecules::WordleBlock;

#[function_component(Wordle)]
pub fn wordle() -> Html {
    html! {
        <>
            <WordleBlock />
        </>
    }
}
