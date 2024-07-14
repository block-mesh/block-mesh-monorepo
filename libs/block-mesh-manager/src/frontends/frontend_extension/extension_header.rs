use leptos::*;
use leptos_meta::{Link, Meta, Script, Stylesheet, Title};

#[component]
pub fn ExtensionServerHeader() -> impl IntoView {
    view! {
        <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
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
                    e.ports[0].postMessage("READY");
                    window.message_channel_port = e.ports[0];
                    window.message_channel_port.onmessage = (msg) => {
                        // console.log("msg", window.location.href , msg, msg?.data);
                    }
                }
            "#
        </Script>
    }
}
