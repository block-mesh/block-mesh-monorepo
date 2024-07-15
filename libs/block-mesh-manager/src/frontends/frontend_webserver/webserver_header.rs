use leptos::*;
use leptos_meta::{Link, Script, Stylesheet, Title};

#[component]
pub fn WebServerHeader() -> impl IntoView {
    view! {
        <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Agbalumo&family=Varela+Round&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,500;1,600;1,700;1,800;1,900&display=swap"
            rel="stylesheet"
        />
        // <Link
        // href="https://fonts.googleapis.com/css2?family=Bebas+Neue&family=Open+Sans:wght@400;600&display=swap"
        // rel="stylesheet"
        // />
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Varela+Round&display=swap"
            rel="stylesheet"
        />
        <Title text="BlockMesh Network"/>
        <Link
            rel="icon"
            href="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/e4f3cdc0-c2ba-442d-3e48-e2f31c0dc100/public"
        />

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
    }
}
