use yew::prelude::*;

use crate::components::molecules::{LabelInput, WordleBlock};

#[function_component(Wordle)]
pub fn wordle() -> Html {
    html! {
        <>
            <LabelInput />
            <WordleBlock />
        </>
    }
}
