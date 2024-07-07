use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
    export async function get_storage_value(key) {
        try {
            if (!port) return;
            const id = Math.random().toString(36).substring(2,13);
            port.postMessage({ id, type: "get_storage_value", key});
            return "";
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    pub async fn get_storage_value(key: &str) -> JsValue;
}

#[wasm_bindgen(inline_js = r#"
    export function bla(key) {
        try {
            console.log("bla", key);
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    pub async fn bla(key: &str) -> JsValue;
}
