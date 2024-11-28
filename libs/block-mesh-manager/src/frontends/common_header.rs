use leptos::*;
use leptos_meta::{Link, Meta, Script, Style, Stylesheet, Title};

#[component]
pub fn CommonHeader() -> impl IntoView {
    view! {
        <Script
            src="https://challenges.cloudflare.com/turnstile/v0/api.js"
            defer="true"
        />
        <Script src="https://cdn.jsdelivr.net/npm/eruda"/>
        <script>
            r#"
            if (window?.origin.includes("localhost")) {
              eruda.init();
            }
            "#
        </script>
        <Title text="BlockMesh Network"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta charset="UTF-8"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap"
            rel="stylesheet"
        />

        <Link rel="preconnect" href="https://rsms.me/"/>
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0"
        />
        <Meta http_equiv="cache-control" content="no-cache"/>
        <Meta http_equiv="expires" content="0"/>
        <Meta http_equiv="pragma" content="no-cache"/>
        <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
        <Stylesheet id="leptos" href="/pkg/block-mesh-manager.css"/>

        <Link
            href="https://fonts.googleapis.com/css2?family=Agbalumo&family=Varela+Round&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,500;1,600;1,700;1,800;1,900&display=swap"
            rel="stylesheet"
        />
        <Link
            href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Open+Sans:wght@400;600&display=swap"
            rel="stylesheet"
        />
        <Link
            href="https://fonts.googleapis.com/css2?family=Varela+Round&display=swap"
            rel="stylesheet"
        />

        <Link
            rel="icon"
            href="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/e4f3cdc0-c2ba-442d-3e48-e2f31c0dc100/public"
        />
        <Script>
            r#"
            window.addEventListener("message", onMessage);
            function onMessage(e) {
              const {data} = e;
              console.log("data:", data);
              if (window.message_channel_port) {
                  window.message_channel_port.postMessage("READY");
                  return;
              }
              if (!e.ports.length) return;
              window.message_channel_port = e.ports[0];
              window.message_channel_port.postMessage("READY");
              window.message_channel_port.onmessage = (msg) => {
                  // console.log("msg", window.location.href , msg, msg?.data);
              }
            }
            "#
        </Script>
        <Script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"/>
        <Script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"/>
        <Script src="https://www.googletagmanager.com/gtag/js?id=G-RYHLW3MDK2"/>
        <Script>
            r#"
            window.dataLayer = window.dataLayer || [];
            function gtag() {
            dataLayer.push(arguments);
            }
            gtag('js', new Date());
            gtag('config', 'G-RYHLW3MDK2');
            "#
        </Script>
        <Style>
            r#"
            body {
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            text-rendering: optimizeLegibility;
            }
            "#
        </Style>
    }
}
