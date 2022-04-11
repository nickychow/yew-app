// use gloo::console::log;
// use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::organisms::{Footer, Header};
use crate::pages::{Form, Hello, Home, PageNotFound, Pokemon, Wordle};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    // #[at("/posts/:id")]
    // Post { id: u32 },
    // #[at("/posts")]
    // Posts,
    // #[at("/authors/:id")]
    // Author { id: u32 },
    #[at("/form")]
    Form,
    #[at("/wordle")]
    Wordle,
    #[at("/hello")]
    Hello,
    #[at("/pokemon")]
    Pokemon,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        // Route::Post { id } => {
        //     html! { <Post seed={id} /> }
        // }
        // Route::Posts => {
        //     html! { <PostList /> }
        // }
        // Route::Author { id } => {
        //     html! { <Author seed={id} /> }
        // }
        Route::Pokemon => {
            html! { <Pokemon /> }
        }
        Route::Hello => {
            html! { <Hello /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::Wordle => {
            html! { <Wordle /> }
        }
        Route::Form => {
            html! { <Form /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <main> // class="bg-gray-100 h-screen flex flex-col justify-center items-center"
                <Switch<Route> render={Switch::render(switch)} />
            </main>
            <Footer />
        </BrowserRouter>
    }
}
