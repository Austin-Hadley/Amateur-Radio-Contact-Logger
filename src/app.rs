use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// after the #[wasm_bindgen] attribute, we define a function that will be called from JavaScript
#[wasm_bindgen]
pub async fn init_db() -> Result<(), JsValue> {
    let cmd = "init_db";
    // create a
    let result = invoke(cmd, args).await;
    Ok(())
}

pub fn App() -> Html {

}