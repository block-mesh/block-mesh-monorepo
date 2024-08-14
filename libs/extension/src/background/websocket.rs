use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{ ErrorEvent, MessageEvent, WebSocket };

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn start_websocket() -> Result<(), JsValue> {
    let ws = WebSocket::new("ws://127.0.0.1:8001/ws")?;
    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => console_log!("Sent a ping message."),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Just supporting text transmission now
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            if
            let Err(err) = cloned_ws.send_with_str(
                    &format!("Extension recieved the message: {:?}", txt)
                )
            {
                console_log!("Message recieved, but error while sending back response : {:?}", err);
            }
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    console_log!("Listening to incoming messages");
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    Ok(())
}
