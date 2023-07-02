mod components;

use components::{
    converter::{Converter},
    header::{Header},
    contact::{Contact},
};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={"container mx-auto"}>
            <Header />
            <Converter />
            <Contact />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
