use yew::{function_component, html, Html};

#[function_component(HeaderSection)]
pub fn header_section() -> Html {
    html! {
        <>
          <div class="flex justify-center mt-6 mb-10">
            <a href="https://tauri.app" target="_blank">
              <img
                src="img/tauri.svg"
                class="h-40 p-6 will-change-[filter] tauri"
                alt="Tauri logo"
              />
            </a>
            <a href="https://yew.rs" target="_blank">
              <img
                src="img/yew.png"
                class="h-40 p-6 will-change-[filter] yew"
                alt="Yew logo"
              />
            </a>
          </div>

          <p>{"Click on the Tauri and Yew logos to learn more."}</p>

          <p>
            {"Recommended IDE setup: "}
            <a
              class="text-indigo-400 hover:text-indigo-300 font-semibold"
              href="https://code.visualstudio.com/"
              target="_blank"
              >{"VS Code"}</a
            >
            {" + "}

            <a
              class="text-indigo-400 hover:text-indigo-300 font-semibold"
              href="https://github.com/tauri-apps/tauri-vscode"
              target="_blank"
              >{"Tauri"}</a
            >
            {" + "}

            <a
              class="text-indigo-400 hover:text-indigo-300 font-semibold"
              href="https://github.com/rust-lang/rust-analyzer"
              target="_blank"
              >{"rust-analyzer"}</a
            >
          </p>
        </>
    }
}
