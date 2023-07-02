use super::convert;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use yew::prelude::*;
#[function_component]
pub fn Converter() -> Html {
    let text = use_state(String::default);
    let converted_text = use_state(String::default);

    let text_input_ref = use_node_ref();

    let on_input_change = {
        let text = text.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            text.set(value);
        })
    };

    let onclick = {
        let converted_text = converted_text.clone();

        Callback::from(move |_| {
            let mut converter = convert::Conveter::new();
            converted_text.set(converter.convert(&text.clone()).unwrap().romaji.clone());
        })
    };

    html! {
        <div class="p-2">
            <form
                class={"mt-[20px]"}
            >
                <div
                    class={"flex gap-2 flex-col items-start mx-w-[20px] justify-start"}
                >
                    <div class={"flex flex-col md:flex-row gap-2"}>
                        <textarea 
                            ref={text_input_ref}
                            placeholder="Text korea"
                            oninput={on_input_change}
                            rows={5}
                            class={"border border-1 border-gray-500 p-1 rounded-lg text-white bg-[#202023]"}
                        />
                        <textarea 
                            placeholder="Converted"
                            rows={5}
                            class={"border border-1 border-gray-500 p-1 rounded-lg text-white bg-[#202023]"}
                            value={(*converted_text).clone()}
                        />
                    </div>
                    <button {onclick} type={"button"} class={"text-white bg-gradient-to-r from-purple-500 via-purple-600 to-purple-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-purple-300 shadow-lg shadow-purple-500/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2"}>{"Convert"}</button>
                </div>
            </form>
        </div>
    }
}
