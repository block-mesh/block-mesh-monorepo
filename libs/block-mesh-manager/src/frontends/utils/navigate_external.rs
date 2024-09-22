use leptos::window;

pub fn navigate_to_external_url(url: String) {
    let window = window();
    window.location().set_href(&url).unwrap();
}
