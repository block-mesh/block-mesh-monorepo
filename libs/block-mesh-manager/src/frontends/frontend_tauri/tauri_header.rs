use leptos::*;
use leptos_meta::{Link, Meta, Stylesheet, Title};

#[component]
pub fn TauriHeader() -> impl IntoView {
    view! {
        <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap"
            rel="stylesheet"
        />
        <Title text="BlockMesh Network"/>
    }
}
