use leptos::*;
use leptos_meta::{Link, Script, Title};

#[component]
pub fn WebServerHeader() -> impl IntoView {
    view! {
        <Script src="https://cdn.tailwindcss.com"/>
        // <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Agbalumo&family=Varela+Round&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,500;1,600;1,700;1,800;1,900&display=swap"
            rel="stylesheet"
        />
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Varela+Round&display=swap"
            rel="stylesheet"
        />
        <Title text="BlockMesh Network"/>
        <Link
            rel="icon"
            // type_=Some(i)
            href="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
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
