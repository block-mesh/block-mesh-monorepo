use leptos::*;
use leptos_meta::{Link, Meta, Script, Stylesheet, Title};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

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
    pub async fn send_message(msg: &str) -> JsValue;
}

#[component]
pub fn ExtensionServerHeader() -> impl IntoView {
    view! {
        <Script src="https://cdn.tailwindcss.com"/>
        <Stylesheet href="https://r2-assets.blockmesh.xyz/extension.css"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap"
            rel="stylesheet"
        />
        <Title text="BlockMesh Network"/>
        <Script>
            r#"
                window.addEventListener("message", onMessage);
                function onMessage(e) {
                    if (!e.ports.length) return;
                    e.ports[0].postMessage("A message from the iframe in page2.html");
                    window.message_channel_port = e.ports[0];
                }
                console.log("inblock")
            "#
        </Script>
    }
}
