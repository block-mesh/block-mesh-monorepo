use super::IfLet;
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Avatar(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] square: MaybeSignal<bool>,
    #[prop(optional, into)] initials: MaybeSignal<Option<String>>,
    #[prop(optional, into)] alt: MaybeSignal<Option<String>>,
) -> impl IntoView {
    let class = move || {
        tw_merge!(class,
            // Basic layout
            "inline-grid shrink-0 align-middle [--avatar-radius:20%] [--ring-opacity:20%] *:col-start-1 *:row-start-1",
            "outline outline-1 -outline-offset-1 outline-black/[--ring-opacity] dark:outline-white/[--ring-opacity]",
            // Add the correct border radius
            if square.get() { "rounded-[--avatar-radius] *:rounded-[--avatar-radius]" } else { "rounded-full *:rounded-full" }
        )
    };

    let alt = Signal::derive(move || alt.get());

    view! {
        <IfLet
            some=initials
            let:initials
        >
            <svg
              className="size-full select-none fill-current p-[5%] text-[48px] font-medium uppercase"
              viewBox="0 0 100 100"
              aria-hidden={move || if alt.get().is_some() { "false" } else { "true" }}
            >
                <IfLet some=alt let:alt>
                    <title>{alt}</title>
                </IfLet>

                <text x="50%" y="50%" alignmentBaseline="middle" dominantBaseline="middle" textAnchor="middle" dy=".125em">
                    {initials}
                </text>
            </svg>
        </IfLet>

        <IfLet
            some=src
            let:src
        >
            <img class="size-full" src=src alt=alt />
        </IfLet>
    }
}
