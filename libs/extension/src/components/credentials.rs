use leptos::*;

#[component]
pub fn CredentialsForm(
    url: Signal<String>,
    title: &'static str,
    action_label: &'static str,
    action: Action<Vec<String>, ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>,
    register: bool,
) -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (password_confirm, set_password_confirm) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (invite_code, set_invite_code) = create_signal(String::new());

    let dispatch_action = move || {
        action.dispatch(vec![
            email.get(),
            password.get(),
            password_confirm.get(),
            invite_code.get(),
        ])
    };

    let button_is_disabled = Signal::derive(move || {
        disabled.get()
            || password.get().is_empty()
            || email.get().is_empty()
            || (register && password_confirm.get().is_empty())
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <p class="text-white">{title}</p>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}

                    <div class="mb-4">
                        <input
                            type="url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="URL"
                            name="url"
                            prop:disabled=true
                            prop:value=url
                        />
                    </div>
                    <div class="mb-4">
                        <input
                            type="email"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Email"
                            name="email"
                            prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                        />

                    </div>
                    <div class="mb-4">
                        <input
                            type="password"
                            required
                            placeholder="Password"
                            prop:disabled=move || disabled.get()
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            name="password"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        dispatch_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_password.update(|p| *p = val);
                                    }
                                }
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.update(|p| *p = val);
                            }
                        />

                    </div>
                    <Show
                        when=move || register
                        fallback=|| {
                            view! { <span></span> }
                        }
                    >

                        <div class="mb-4">
                            <input
                                type="password"
                                required
                                placeholder="Confirm Password"
                                prop:disabled=move || disabled.get()
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                name="confirm_password"
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    match &*ev.key() {
                                        "Enter" => {
                                            dispatch_action();
                                        }
                                        _ => {
                                            let val = event_target_value(&ev);
                                            set_password_confirm.update(|p| *p = val);
                                        }
                                    }
                                }

                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_password_confirm.update(|p| *p = val);
                                }
                            />

                        </div>
                        <div class="mb-4">
                            <input
                                type="text"
                                placeholder="Invite Code"
                                prop:disabled=move || disabled.get()
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                name="invite_code"
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    match &*ev.key() {
                                        "Enter" => {
                                            dispatch_action();
                                        }
                                        _ => {
                                            let val = event_target_value(&ev);
                                            set_invite_code.update(|p| *p = val);
                                        }
                                    }
                                }

                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_password_confirm.update(|p| *p = val);
                                }
                            />

                        </div>
                    </Show>
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            prop:disabled=move || button_is_disabled.get()
                            on:click=move |_| dispatch_action()
                        >
                            {action_label}
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
