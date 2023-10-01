use web_sys::SubmitEvent;
use yew::{function_component, html, Callback, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<SubmitEvent>,
    pub input_ref: NodeRef,
}

#[function_component]
pub fn Form(props: &Props) -> Html {
    html! {
         <form onsubmit={&props.onsubmit}>
             <input type="text" name="url" class="text" ref={&props.input_ref} placeholder="Insert URL here" />
             <input type="submit" value="Defange URL" />
         </form>
    }
}
