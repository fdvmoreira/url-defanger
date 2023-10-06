use yew::{classes, function_component, html, AttrValue, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct ResultProps {
    pub text: AttrValue,
    pub result_ref: NodeRef,
}

#[function_component]
pub fn ResultItem(props: &ResultProps) -> Html {
    html! {
        <div ref={&props.result_ref} class={classes!(String::from("h-12 overflow-x-scroll py-2 px-4 shadow cursor-copy font-bold text-stone-500"))}>{props.text.clone()}</div>
    }
}
