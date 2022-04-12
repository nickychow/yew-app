use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-white border-b-2 border-gray-100">
            <div class="container mx-auto px-4 py-3 max-w-screen-lg">
                <div class="flex font-heading items-center justify-between">
                    <Link<Route> to={Route::Home} classes="flex items-center text-gray-600 hover:text-indigo-600">
                        <svg class="h-8 w-8 sm:h-10 sm:w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                        </svg>

                        <span class="mx-3 font-bold text-xl md:text-2xl flex">
                            {"Nick Chow Profilio"}
                        </span>
                    </Link<Route>>
                    <div class="flex text-gray-600 items-center gap-3 -mx-2">
                        // <span class="text-2xl font-semibold ">{"柯"}</span>
                        // <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        <Link<Route> to={Route::Form}>{"Form"}</Link<Route>>
                        <Link<Route> to={Route::Hello}>{"Hello"}</Link<Route>>
                        <Link<Route> to={Route::Pokemon}>{"Pokemon"}</Link<Route>>
                        <Link<Route> to={Route::Wordle}>{"Wordle"}</Link<Route>>
                    </div>
                </div>
            </div>
        </header>
    }
}
