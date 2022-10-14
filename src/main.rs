use serde_json::Value;
use yew::prelude::*;

fn main() {
    println!("Hello, world!");
    yew::start_app::<Root>();
}

#[function_component(Root)]
fn root() -> Html {
    let onclick = Callback::from(|mouse_event: MouseEvent| {
        web_sys::console::log_1(&mouse_event);

        wasm_bindgen_futures::spawn_local(async move {
            let response = reqwest::get("https://pokeapi.co/api/v2/pokemon/23")
            .await
            // production rust apps should use better error handling
            // shouldn't use unwrap
            .unwrap();

            let text = response.text().await.unwrap();

            let v: Value = serde_json::from_str(&text).unwrap();
            let name = v["name"].as_str().unwrap();
            let image_src = v["sprites"]["front_default"].as_str().unwrap();

        web_sys::console::log_2(&name.into(), &image_src.into());
        });
    });

    html! {
        <div>
        <button {onclick}>{"get pokeman"}</button>
        {"hwody iman"}
        </div>
    }
}
