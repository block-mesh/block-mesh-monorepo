use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
    export function storageOnChangeViaPostMessage(callback) {
        console.log("msg in callback X");
        if (!window.message_channel_port) return;
        window.message_channel_port.addEventListener("message", (msg) => {
            const { data } = msg;
            console.log("msg in callback", data);
            callback(data);
        });
    }
"#)]
extern "C" {
    pub fn storageOnChangeViaPostMessage(callback: &Closure<dyn Fn(JsValue)>);
}

#[wasm_bindgen(inline_js = r#"
    export async function send_message(msg) {
        try {
            if (! window.message_channel_port ) {
                console.log("message_channel_port is missing");
                return;
            }
           window.message_channel_port.postMessage(msg);
        } catch (e) {
            return ""
        }
    };
"#)]
extern "C" {
    pub async fn send_message(msg: JsValue) -> JsValue;
}
