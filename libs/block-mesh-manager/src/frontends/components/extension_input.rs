use leptos::*;
use leptos_use::math::use_or;
use tailwind_fuse::*;

#[component]
pub fn ExtensionInput(#[prop(into)] label: String, #[prop(into)] type_: String) -> impl IntoView {
    let (is_focused, set_focused) = create_signal(false);
    let (is_valid, set_valid) = create_signal(false);

    let handle_blur = move |e| {
        set_focused.set(false);
        set_valid.set(!event_target_value(&e).is_empty());
    };

    let fv = use_or(is_focused, is_valid);

    let label_class = move || {
        tw_join!(
            "font-jetbrains absolute left-0 text-base pointer-events-none transition-all duration-300",
            if fv.get() { "top-0 text-xs" } else { "top-2.5" }
        )
    };

    let border_bottom_color = move || {
        if is_focused.get() {
            "var(--blue)"
        } else {
            "#555"
        }
    };

    view! {
        <div class="relative mb-2 w-6/12 m-2.5 mx-auto auth-card-input-container">
            <input
                type=type_
                class="font-jetbrains bg-transparent w-full text-base text-white border-b border-gray-600 pt-2 pb-1.5 focus:outline-none focus:border-none focus:border-b-2"
                style="border-bottom: 1px solid #555"
                style:border-buttom-color=border_bottom_color
                on:focus=move |_| set_focused.set(true)
                on:blur=handle_blur
            />
            <label class=label_class style:color="#fffc">
                {label}
            </label>
        </div>
    }
}
