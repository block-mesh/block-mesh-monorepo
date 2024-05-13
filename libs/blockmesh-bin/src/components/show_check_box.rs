use crate::state::LeptosTauriAppState;
use block_mesh_common::cli::CommandsEnum;
use leptos::*;

#[component]
pub fn ShowCheckBox(title: CommandsEnum) -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let command = move || match state.cli_args.get().command {
        Some(command) => {
            let c: CommandsEnum = command.clone().into();
            c
        }
        None => CommandsEnum::ClientNode,
    };
    let show = move || command().to_string() == title.to_string();
    let active = move || {
        let base = "relative flex cursor-pointer rounded-lg border bg-gray-300 p-4 shadow-sm focus:outline-none";
        if show() {
            format!("{} {}", base, "border-indigo-600 ring-2 ring-indigo-600")
        } else {
            format!("{} {}", base, "border-gray-300")
        }
    };
    view! {
        <label
            class=active
            on:click=move |_ev| {
                match &state.cli_args.get().command {
                    Some(command) => {
                        let command_result = command.convert(&title);
                        if command_result.is_some() {
                            let mut args = state.cli_args.get();
                            args.command = Some(command_result.unwrap());
                            state.cli_args.set(args);
                        }
                    }
                    None => {}
                }
            }
        >

            <input
                type="radio"
                name="project-type"
                value=title.to_string()
                class="sr-only"
                aria-labelledby="project-type-1-label"
                aria-describedby="project-type-1-description-0 project-type-1-description-1"
            />
            <span class="flex flex-1">
                <span class="flex flex-col">
                    <span id="project-type-1-label" class="block text-sm font-medium text-gray-900">
                        {title.to_string()}
                    </span>
                </span>
            </span>
            <Show
                when=show
                fallback=|| {
                    view! {
                        <span
                            class="pointer-events-none absolute -inset-px rounded-lg border-2"
                            aria-hidden="true"
                        ></span>
                    }
                }
            >

                <svg
                    class="h-5 w-5 text-indigo-600"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path
                        fill-rule="evenodd"
                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                        clip-rule="evenodd"
                    ></path>
                </svg>
            </Show>
        </label>
    }
}
