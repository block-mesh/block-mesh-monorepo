use crate::components::notifications::ExtensionWrapperNotifications;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use leptos::*;
use url::Url;

#[component]
pub fn ExtensionOptionsPage() -> impl IntoView {
    provide_context(ExtensionWrapperState::default());
    let state = use_context::<ExtensionWrapperState>().unwrap();
    let state = ExtensionWrapperState::init_resource(state);

    let (url, set_url) = create_signal(
        state
            .get()
            .map(|s| s.blockmesh_url.get_untracked())
            .unwrap_or_default(),
    );

    let (ws_url, set_ws_url) = create_signal(
        state
            .get()
            .map(|s| s.blockmesh_ws_url.get_untracked())
            .unwrap_or_default(),
    );

    let (data_sink_url, set_data_sink_url) = create_signal(
        state
            .get()
            .map(|s| s.blockmesh_data_sink_url.get_untracked())
            .unwrap_or_default(),
    );

    let clear_action = create_action(move |_| async move {
        match state.get() {
            None => (),
            Some(s) => {
                s.clear().await;
                ExtensionWrapperState::set_success("Cache cleared".to_string(), s.success);
            }
        }
    });

    let save_action = create_action(move |_| async move {
        let s = match state.get() {
            None => return,
            Some(s) => s,
        };
        if url.get_untracked().is_empty() {
            ExtensionWrapperState::set_error("URL is empty".to_string(), s.error);
            return;
        }
        if ws_url.get_untracked().is_empty() {
            ExtensionWrapperState::set_error("WS URL is empty".to_string(), s.error);
            return;
        }
        if data_sink_url.get_untracked().is_empty() {
            ExtensionWrapperState::set_error("Data-Sink URL is empty".to_string(), s.error);
            return;
        }
        let raw_url = url.get_untracked().trim_end_matches('/').to_string();
        let raw_ws_url = ws_url.get_untracked().trim_end_matches('/').to_string();
        let raw_data_sink_url = data_sink_url
            .get_untracked()
            .trim_end_matches('/')
            .to_string();
        if let Err(error) = Url::parse(&raw_url) {
            ExtensionWrapperState::set_error(format!("Invalid URL: {}", error), s.error);
            return;
        }
        if let Err(error) = Url::parse(&raw_ws_url) {
            ExtensionWrapperState::set_error(format!("Invalid WS URL: {}", error), s.error);
            return;
        }
        if let Err(error) = Url::parse(&raw_data_sink_url) {
            ExtensionWrapperState::set_error(format!("Invalid Data-Sink URL: {}", error), s.error);
            return;
        }
        s.blockmesh_url.update(|v| *v = raw_url.clone());
        s.blockmesh_ws_url.update(|v| *v = raw_ws_url.clone());
        s.blockmesh_data_sink_url
            .update(|v| *v = raw_data_sink_url.clone());
        set_url.update(|v| *v = raw_url.clone());
        set_ws_url.update(|v| *v = raw_ws_url.clone());
        ExtensionWrapperState::store_blockmesh_url(raw_url).await;
        ExtensionWrapperState::store_blockmesh_ws_url(raw_ws_url).await;
        ExtensionWrapperState::store_blockmesh_data_sink_url(raw_data_sink_url).await;
        ExtensionWrapperState::set_success("URL saved".to_string(), s.success);
    });

    view! {
        <ExtensionWrapperNotifications/>
        <form on:submit=|ev| ev.prevent_default()>
            <div class="bg-gray-700 flex justify-center items-center">
                <div class="bg-gray-800 p-8 shadow-md w-full">
                    <p class="text-white">Options</p>
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="url">
                            BlockMesh URL
                        </label>
                        <input
                            type="url"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder=move || state.get().map(|s| s.blockmesh_url.get())
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

                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="url">
                            BlockMesh WS URL
                        </label>
                        <input
                            type="url"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder=move || state.get().map(|s| s.blockmesh_ws_url.get())
                            name="url"
                            // prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_ws_url.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_ws_url.update(|v| *v = val);
                            }
                        />

                    </div>

                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="url">
                            BlockMesh Data-Sink URL
                        </label>
                        <input
                            type="url"
                            required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder=move || state.get().map(|s| s.blockmesh_data_sink_url.get())
                            name="url"
                            // prop:disabled=move || disabled.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_data_sink_url.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_data_sink_url.update(|v| *v = val);
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
                <div class="bg-gray-800 border-white p-8 shadow-md w-full">
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
