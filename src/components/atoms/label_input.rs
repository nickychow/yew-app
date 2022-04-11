use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Properties, Default)]
pub struct Props {
    pub name: String,
    #[prop_or("John Doe".to_string())]
    pub placeholder: String,
    #[prop_or_default]
    pub classes: Classes,
    pub onchange: Callback<String>,
}

#[function_component(LabelInput)]
pub fn label_input(props: &Props) -> Html {
    let label_classes = classes!(
        "relative",
        "block",
        "p-3",
        "border-2",
        "border-gray-200",
        "rounded-lg"
    );

    let input_classes = classes!(
        "shadow-none",
        "focus:shadow-outline",
        "border-none",
        "outline-none",
        "text-sm",
        "w-full",
        "p-0",
        "focus:ring-0",
        "focus:ring-brand-gray-200",
        "focus:border-brand-gray-200",
    );

    let span_classes = classes!("text-xs", "font-medium", "text-gray-500",);

    let Props {
        name,
        placeholder,
        classes,
        onchange,
    } = props;

    let onchange = {
        let onchange = onchange.clone();
        Callback::from(move |event: Event| {
            let target = event
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlInputElement>().value();
            onchange.emit(value);
        })
    };

    html! {
        <label class={classes!(label_classes,classes.clone())}>
            <span class={span_classes}>
                {name}
            </span>
            <input class={input_classes} type="text" placeholder={placeholder.clone()} onchange={onchange}/>
        </label>
    }
}
