use gloo::console::log;
use rand::Rng;
use serde_json::Value;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use std::ops::Deref;

#[derive(Debug, PartialEq, Default, Clone)]
struct Pokemon {
    id: i32,
    name: String,
    image_src: String,
}

#[derive(Debug, PartialEq, Clone)]
enum Guess {
    Correct,
    Incorrect,
}

#[function_component(PokemonCard)]
pub fn pokemon_card() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);
    let pokemon_state_outer = pokemon_state.clone();

    let guess_state = use_state_eq::<Option<Guess>, _>(|| None);
    let guess_state_outer = guess_state.clone();

    // log!(format!("{:?}", &pokemon_state));

    let onclick = Callback::from(move |_: MouseEvent| {
        // clean pokemon and guess state
        pokemon_state.set(None);
        guess_state.set(None);

        let pokemon_state = pokemon_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let mut rng = rand::thread_rng();
            let id: i32 = rng.gen_range(1..=100);

            let response = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}", id))
                .await
                .expect("Failed to fetch Pokemon!");

            let content = response
                .text()
                .await
                .expect("Failed to read response body!");

            let v: Value = serde_json::from_str(&content).expect("Failed to parse JSON!");

            let name = v["name"].as_str().unwrap();
            let image_src = v["sprites"]["front_default"].as_str().unwrap();

            let pokemon = Pokemon {
                id,
                name: name.into(),
                image_src: image_src.into(),
            };

            // log!(name, image_src);
            pokemon_state.set(Some(pokemon));
        });
    });

    let button_classes = {
        classes!(vec![
            "bg-blue-500",
            "hover:bg-blue-400",
            "text-white",
            "font-bold",
            "py-2",
            "px-4",
            "border-b-4",
            "border-blue-700",
            "hover:border-blue-500",
            "rounded"
        ])
    };

    html! {
        <section id="messageDisplay" class="col max-w-sm bg-white shadow-md rounded-lg px-8 pt-6 pb-4 mb-4 text-center">
            <button {onclick} class={button_classes}>{ "Load Pokemon" }</button>
            <ViewPokemon pokemon={pokemon_state_outer.deref().clone()} guess_state={guess_state_outer.clone()} />
        </section>

    }
}

#[derive(PartialEq, Properties, Clone)]
struct ViewPokemonProps {
    pokemon: Option<Pokemon>,
    guess_state: UseStateHandle<Option<Guess>>,
}

#[function_component(ViewPokemon)]
fn view_pokemon(props: &ViewPokemonProps) -> Html {
    let ViewPokemonProps {
        pokemon,
        guess_state,
    } = props;

    let pokeman = match pokemon {
        Some(pokemon) => pokemon,
        None => return html! {},
    };

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();

    let name = pokeman.name.clone();

    let onclick = {
        let guess_state = guess_state.clone();
        Callback::from(move |_: MouseEvent| {
            let input = input_ref
                .cast::<HtmlInputElement>()
                .expect("Failed to cast input");
            let guess = input.value().to_lowercase();

            log!(&guess);

            if guess == name {
                // input.set_class_name("bg-green-500");
                guess_state.set(Some(Guess::Correct));
            } else {
                // input.set_class_name("bg-red-500");
                guess_state.set(Some(Guess::Incorrect));
            }
        })
    };

    let input_classes = {
        classes!(vec![
            "shadow",
            "appearance-none",
            "border",
            "rounded",
            "w-full",
            "py-2",
            "px-3",
            "mb-4",
            "text-gray-700",
            "leading-tight",
            "focus:outline-none",
            "focus:shadow-outline"
        ])
    };

    let button_classes = {
        classes!(vec![
            "bg-blue-500",
            "hover:bg-blue-400",
            "text-white",
            "font-bold",
            "py-2",
            "px-4",
            "border-b-4",
            "border-blue-700",
            "hover:border-blue-500",
            "rounded"
        ])
    };

    html! {
        <div>
            <img src={pokeman.image_src.clone()}/>
            <input ref={input_ref_outer.clone()} class={input_classes} type="text"/>
            <button {onclick} class={button_classes}>{ "Click me" }</button>
            <ViewGuess guess={guess_state.deref().clone()}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ViewGuessProps {
    guess: Option<Guess>,
}

#[function_component(ViewGuess)]
fn view_guess(props: &ViewGuessProps) -> Html {
    let ViewGuessProps { guess } = props;

    let guess_text = match guess {
        Some(Guess::Correct) => "Correct!",
        Some(Guess::Incorrect) => "Incorrect!",
        None => return html! {},
    };

    html! {
        <p> {guess_text} </p>
    }
}
