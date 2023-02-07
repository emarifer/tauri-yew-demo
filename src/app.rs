use gloo_dialogs::alert;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{header_section::HeaderSection, response_controls::ResponseControls};

#[wasm_bindgen]
extern "C" {
    // Le agregamos el atributo "catch" que nos permite usar un "Result"
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_msg = use_state(|| None);

    let on_response = {
        let greet_msg = greet_msg.clone();

        Callback::from(move |name: String| {
            let greet_msg = greet_msg.clone();
            // Necesitamos clonar "greet_msg" 2 veces porque lo «movemos» 2 veces en los 2 closures
            spawn_local(async move {
                let new_msg = invoke("greet", to_value(&GreetArgs { name: &*name }).unwrap()).await;
                match new_msg {
                    Ok(result_ok) => {
                        log(&result_ok.as_string().unwrap());
                        greet_msg.set(Some(result_ok.as_string().unwrap()))
                    }
                    Err(result_err) => {
                        log(&result_err.as_string().unwrap());
                        // greet_msg.set(result_err.as_string().unwrap());
                        alert(&result_err.as_string().unwrap());
                    }
                }
            });
        })
    };

    let result_section = match (*greet_msg).clone() {
        Some(result) => html! {
          <p class="mt-6"><b>{ &result }</b></p>
        },
        None => html! {},
    };

    html! {
        <main class="m-0 pt-[10vh] flex flex-col justify-center text-center">
            <HeaderSection />

            <ResponseControls onresponse={on_response} />

            {result_section}
        </main>
    }
}

/*
 * https://yew.rs/docs/concepts/function-components/callbacks
 */
