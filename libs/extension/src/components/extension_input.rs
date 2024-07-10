use leptos::*;
use web_sys::FocusEvent;

#[component]
pub fn ExtensionInput(label: String, input_type: String) -> impl IntoView {
    let (is_focused, set_is_focused) = create_signal(false);
    let (is_valid, set_is_valid) = create_signal(false);
    let border_class = move || {
        if is_focused.get() {
            "border-bottom-color: 'var(--blue)'"
        } else {
            "border-bottom-color: '#555'"
        }
    };

    let handle_blur = move |e: FocusEvent| {
        set_is_focused.set(false);
        if e.target().is_some() {
            set_is_valid.set(true);
        } else {
            set_is_valid.set(false);
        }
    };

    let label_class = move || {
        let class = "font-jetbrains absolute left-0 text-base pointer-events-none transition-all duration-300";
        let class = format!(
            "{} {}",
            class,
            if is_focused.get() || is_valid.get() {
                "top-0 text-xs"
            } else {
                ""
            }
        );
        let class = format!(
            "{} {}",
            class,
            if !is_focused.get() || !is_valid.get() {
                "top-2.5"
            } else {
                ""
            }
        );
        class
    };

    view! {
        <div className="relative mb-2 w-6/12 m-2.5 mx-auto auth-card-input-container">
            <input
                type=input_type
                class="font-jetbrains bg-transparent w-full text-base text-white border-b border-gray-600 pt-2 pb-1.5 focus:outline-none focus:border-none focus:border-b-2"
                style=move || format!("borderBottom: '1px solid #555'; {}", border_class())
                on:focus=move |_| set_is_focused.set(true)
                on:blur=handle_blur
            />
            <label class=label_class style=r#"color: \"\#fffc""#>
                {label}
            </label>
        </div>
    }
}
