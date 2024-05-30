use crate::utils::ext_state::AppState;
use leptos::*;
use std::time::Duration;
use url::Url;

#[component]
pub fn Options() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    let _ = AppState::init_resource(state);

    let (error, set_error) = create_signal(None::<String>);
    let (success, set_success) = create_signal(None::<String>);
    let (url, set_url) = create_signal(state.blockmesh_url.get_untracked());

    let clear_action = create_action(move |_| async move {
        state.clear().await;
        set_success.update(|s| *s = Some("Cache cleared".to_string()));
        set_error.update(|e| *e = None);
        set_timeout(
            move || {
                set_success.update(|s| *s = None);
            },
            Duration::from_millis(1500),
        );
    });

    let save_action = create_action(move |_| async move {
        if url.get_untracked().is_empty() {
            set_error.update(|e| *e = Some("URL is empty".to_string()));
            return;
        }
        let raw_url = url.get_untracked();
        let url = Url::parse(&url.get_untracked());
        match url {
            Err(error) => {
                set_error.update(|e| *e = Some(format!("Invalid URL: {}", error)));
                set_timeout(
                    move || {
                        set_error.update(|e| *e = None);
                    },
                    Duration::from_millis(1500),
                );
                return;
            }
            Ok(url) => url,
        };
        state.blockmesh_url.update(|v| *v = raw_url.clone());
        set_url.update(|v| *v = raw_url.clone());
        AppState::store_blockmesh_url(raw_url).await;
        set_success.update(|s| *s = Some("URL saved".to_string()));
        set_error.update(|e| *e = None);
        set_timeout(
            move || {
                set_success.update(|s| *s = None);
            },
            Duration::from_millis(1500),
        );
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <p class="text-white">Options</p>
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}

                    {move || {
                        success
                            .get()
                            .map(|success| {
                                view! { <p style="color:green;">{success}</p> }
                            })
                    }}

                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="url">
                            BlockMesh URL
                        </label>
                        <input
                            type="url"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder=move || state.blockmesh_url.get()
                            name="url"
                            // prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_url.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_url.update(|v| *v = val);
                            }
                        />

                    </div>
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| save_action.dispatch(())
                        >
                            Submit
                        </button>
                    </div>

                </div>
            </div>
        </form>
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| clear_action.dispatch(())
                        >
                            Reset Cache
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
