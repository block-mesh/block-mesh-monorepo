use block_mesh_common::interfaces::server_api::{
    GetEmailViaTokenRequest, GetEmailViaTokenResponse,
};
use leptos::Suspense;
use leptos::*;
use leptos_router::{use_query_map, A};

#[component]
pub fn NewPasswordPage() -> impl IntoView {
    logging::log!("Starting");
    let params = use_query_map();
    let token = params
        .get()
        .get("token")
        .unwrap_or(&"".to_string())
        .to_string();
    let (token, _) = create_signal(token);
    let (origin, set_origin) = create_signal(None::<String>);

    create_effect(move |_| {
        set_origin.set(Some(window().origin()));
        logging::log!("\n\norigin = {:?}\n\n", window().origin());
    });

    logging::log!("x token = {}", token.get());

    let async_data = create_resource(
        move || origin.get(),
        move |_| async move {
            origin.get()?;
            let client = reqwest::Client::new();
            logging::log!("\n\norigin = {:?}\n\n", window().origin());
            let response = client
                .post(&format!(
                    "{}/api/get_email_via_token",
                    origin.get().unwrap_or_default()
                ))
                .json(&GetEmailViaTokenRequest { token: token.get() })
                .send()
                .await;
            logging::log!("response = {:?}", response);

            match response {
                Ok(response) => match response.json::<GetEmailViaTokenResponse>().await {
                    Ok(json) => Some(json.email),
                    Err(e) => {
                        logging::log!("error: {}", e);
                        None
                    }
                },
                Err(e) => {
                    logging::log!("error: {}", e);
                    None
                }
            }
        },
    );

    view! {
        <Suspense fallback=|| {
            view! { <div class="lds-roller"></div> }
        }>
            <form action="/new_password" method="post">
                <div class="bg-gray-700 flex justify-center items-center h-screen">
                    <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                        <h2 class="text-white text-2xl font-semibold text-center mb-6">
                            New Password
                        </h2>
                        <div class="flex justify-around mb-4">
                            <A
                                class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                href="/ui/login"
                            >
                                Login
                            </A>
                        </div>
                        <div class="mb-4">
                            <label class="block text-white text-sm font-bold mb-2" for="email">
                                Email
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                type="text"
                                id="email"
                                placeholder="Email"
                                name="email"
                                required
                                readonly
                                value=move || async_data.get().map(|email| email.unwrap_or_default())
                            />
                        </div>
                        <div class="mb-4">
                            <label class="block text-white text-sm font-bold mb-2" for="password">
                                Password
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="password"
                                name="password"
                                placeholder="******************"
                                required
                            />
                        </div>
                        <div class="mb-4">
                            <label
                                class="block text-white text-sm font-bold mb-2"
                                for="password_confirm"
                            >
                                Confirm
                                Password
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="password_confirm"
                                name="password_confirm"
                                placeholder="******************"
                                required
                            />
                        </div>
                        <div class="mb-4 hidden">
                            <label class="block text-white text-sm font-bold mb-2" for="token">
                                Token
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="token"
                                name="token"
                                placeholder="******************"
                                value=move || token.get()
                                required
                                readonly
                            />
                        </div>
                        <div class="flex items-center justify-between">
                            <button
                                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                                type="submit"
                            >
                                Submit
                            </button>
                        </div>
                    </div>
                </div>
            </form>
        </Suspense>
    }
}
