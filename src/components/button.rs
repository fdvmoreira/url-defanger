use web_sys::MouseEvent;
use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub button_label: AttrValue,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html!(<button onclick={&props.onclick.clone()} class={classes!(String::from("px-4 border bg-indigo-100 hover:bg-indigo-400 text-gray-400 hover:text-gray-200 font-bold capitalize"))}>{props.button_label.clone()}</button>)
}
