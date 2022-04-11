use yew::prelude::*;

#[function_component(LabelInput)]
pub fn label_input() -> Html {
    html! {
        <label class="relative block p-3 border-2 border-gray-200 rounded-lg" for="name">
            <span class="text-xs font-medium text-gray-500" for="name">
                {"Name"}
            </span>

            <input class="w-full p-0 text-sm border-none focus:ring-0" id="name" type="text" placeholder="John Doe" />
        </label>
    }
}
