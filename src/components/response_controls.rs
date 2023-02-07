use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResponseControlsProps {
    pub onresponse: Callback<String>,
}

#[function_component(ResponseControls)]
pub fn response_controls(props: &ResponseControlsProps) -> Html {
    let greet_input_ref = use_node_ref();

    let on_pressed_button = {
        let onresponse = props.onresponse.clone();
        let greet_input_ref = greet_input_ref.clone();

        Callback::from(move |_| {
            let name = greet_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value();

            onresponse.emit(name);

            // Limpamos el input
            greet_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .set_value("");
        })
    };

    {
        let greet_input_ref = greet_input_ref.clone();
        use_effect(move || {
            if let Some(input) = greet_input_ref.cast::<web_sys::HtmlInputElement>() {
                input.focus().unwrap();
            }
        });
    }

    html! {
        <div class="flex justify-center mt-6 gap-4">
          <input
            ref={greet_input_ref}
            class="p-2 rounded-lg text-slate-700 focus:outline-none focus:ring focus:ring-blue-400"
            placeholder="Enter a name…"
          />
          <button
            onclick={on_pressed_button}
            type="button"
            class="rounded-lg border border-transparent px-3 py-1 bg-sky-600 hover:bg-sky-500"
          >
            {"Greet"}
          </button>
        </div>
    }
}
