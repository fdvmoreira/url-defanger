use web_sys::SubmitEvent;
use yew::{classes, function_component, html, Callback, Html, NodeRef, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<SubmitEvent>,
    pub input_ref: NodeRef,
}

#[function_component]
pub fn Form(props: &Props) -> Html {
    html! {
         <form class={classes!(String::from("flex justify-center items-center m-4"))} onsubmit={&props.onsubmit}>
             <input type="text" name="url" class={classes!(String::from("text-xl border p-4 m-2 xs:w-full md:w-96"))} ref={&props.input_ref} placeholder="ex: https://defange.url" />
             <input type="submit" value="Defange" class={classes!(String::from("uppercase bg-indigo-600 hover:bg-indigo-800 font-bold text-slate-50 p-4 px-8 m-2"))} />
         </form>
    }
}
