// use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub class: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        let target = event
            .target()
            .expect("Event should have a target when dispatched");
        let value = target.unchecked_into::<HtmlInputElement>().value();
        // log!(&value);
        handle_onchange.emit(value);
    });
    html! {
        <input type="text" class={&props.class} name={props.name.clone()} onchange={onchange} />
    }
}
