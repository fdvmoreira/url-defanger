use web_sys::MouseEvent;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub button_label: AttrValue,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html!(<button onclick={&props.onclick.clone()}>{props.button_label.clone()}</button>)
}
