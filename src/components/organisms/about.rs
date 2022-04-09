use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="bg-gray-800 pattern py-20">
            <div class="max-w-5xl px-6 mx-auto text-center">
                <h2 class="text-2xl font-semibold text-white">{"About Me"}</h2>

                <p class="text-gray-400 mt-4">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ullamcorper
                nulla nunc quis molestie volutpat elementum at. Ultrices ipsum, enim
                cursus lorem ac. Orci maecenas praesent arcu eget orci est orci
                nullam. Leo purus est pellentesque massa at tortor, est. Aliquet
                pulvinar a mattis sagittis. Suspendisse porta id elementum, massa."}
                </p>
            </div>
        </section>
    }
}
