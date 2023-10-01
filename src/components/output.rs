use yew::{function_component, html, AttrValue, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ResultProps {
    pub text: AttrValue,
    pub result_ref: NodeRef,
}

#[function_component]
pub fn ResultItem(props: &ResultProps) -> Html {
    html! {
        <div ref={&props.result_ref}>{props.text.clone()}</div>
    }
}
