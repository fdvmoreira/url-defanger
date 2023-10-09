#![allow(unused_imports)]
mod components;
use std::ops::Deref;

use crate::components::button::Button;
use crate::components::form::Form;
use crate::components::output::ResultItem;
use js_sys::{Function, Promise};
use url_defanger::defange_url;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::{console, HtmlDivElement, HtmlInputElement, Navigator};
use yew::functional::{function_component, use_node_ref};
use yew::{classes, html, use_state, Callback, Html, MouseEvent, SubmitEvent};

#[function_component]
fn App() -> Html {
    let input_ref = use_node_ref();
    let result_ref = use_node_ref();
    let btn_label = use_state(|| "copy");
    let show_copy_btn = use_state(|| false);

    let mut onsubmit: Callback<SubmitEvent> = Callback::default();
    let mut onclick: Callback<MouseEvent> = Callback::default();

    {
        let result_ref = result_ref.clone();
        let btn_label = btn_label.clone();
        onclick = Callback::from(move |_event: MouseEvent| {
            let div = result_ref.cast::<HtmlDivElement>().unwrap();
            let result: String = div.inner_text();

            // This API is unstable and requires --cfg=web_sys_unstable_apis to be activated, as described in the wasm-bindgen guide
            #[cfg(web_sys_unstable_apis)]
            {
                let _ = url_defanger::save_to_clipboard(result.as_str());
            }

            // TODO: use wasm_bindgen_futures to get the result
            // ex: wasm_bindgen_futures::JsFuture::from(promise).await?;

            console::log_1(&JsValue::from_str(
                format!("Copied {} to clipboard!", result).as_str(),
            ));
            btn_label.set("copied");
        });
    }

    {
        let input_ref = input_ref.clone();
        let result_ref = result_ref.clone();
        let show_copy_btn = show_copy_btn.clone();
        let btn_label = btn_label.clone();

        onsubmit = Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let output = result_ref.cast::<HtmlDivElement>().unwrap();

            if let Some(defanged_url) = defange_url(input.value().as_str()) {
                //console::log_1(&JsValue::from_str(defanged_url.as_str()));

                output.set_inner_text(defanged_url.as_str());

                if output.inner_text().trim().is_empty() {
                    show_copy_btn.set(false);
                } else {
                    show_copy_btn.set(true);
                    btn_label.set("Copy");
                }
            }
        });
    }
    html! {
        <div class={classes!(String::from("m-4"))}>
            <h1 class={classes!(String::from("text-center text-5xl opacity-50 mb-8"))}>{"URL Defanger"}</h1>
            <Form  input_ref={input_ref.clone()} {onsubmit}/>
            <div class={classes!(String::from("px-6"))}>
                <ResultItem result_ref={result_ref} text={"".to_string()} />
                {
                    if (*show_copy_btn).clone() {
                         html!(<Button {onclick} button_label={*btn_label} />)
                    }else{
                        html!(<></>)
                    }
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
