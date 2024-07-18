use leptos::*;
use leptos_router::A;
#[component]
pub fn TauriLogin() -> impl IntoView {
    let (_email, set_email) = create_signal(String::new());
    let (_password, set_password) = create_signal(String::new());
    let submit_action = create_action(move |_| async move {});
    let on_submit_action = move || {
        submit_action.dispatch(());
    };

    view! {
           <div>
            <div class="bg-dark-blue flex justify-center items-center h-screen">
                <div class="bg-dark-blue border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="font-bebas-neue text-off-white text-2xl font-semibold text-center mb-6">Login</h2>
                    <div class="flex justify-around mb-4">
                        <A
                            class="font-bebas-neue px-4 py-2 rounded font-bold text-sm text-cyan hover:text-orange"
                            href="/tauri/register"
                        >
                            Register
                        </A>
                    </div>
                    <div class="mb-4">
                        <label class="font-bebas-neue block text-off-white text-sm font-bold mb-2" for="email">
                            Email
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline"
                            type="text"
                            id="email"
                            placeholder="Email"
                            name="email"
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
                        <label class="font-bebas-neue block text-off-white text-sm font-bold mb-2" for="password">
                            Password
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-black mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password"
                            name="password"
                            placeholder="******************"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        on_submit_action();
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
                    <div class="flex items-center justify-between">
                        <button
                            class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_| { on_submit_action() }
                        >
                            Submit
                        </button>
                    </div>
                </div>
            </div>
        </div>

    }
}
