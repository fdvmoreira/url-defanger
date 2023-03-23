use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <form action="" >
            <input type="text" class="text" placeholder="Insert URL here" />
            <input type="submit" value="defange" />
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
