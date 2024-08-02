use leptos::Suspense;
use leptos::*;
use leptos_router::A;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! { <div class="lds-roller"></div> }
        }>

            <div class="bg-dark-blue h-screen">
                <form action="/reset_password" method="post">
                    <div class="bg-dark-blue flex justify-center items-center h-screen">
                        <div class="bg-dark-blue border-cyan border-solid border-2 p-8 rounded-lg shadow-md w-80">
                            <h2 class="font-bebas-neue text-off-white text-2xl font-semibold text-center mb-6">
                                Reset Password
                            </h2>
                            <div class="flex justify-around mb-4">
                                <A
                                    class="font-open-sans inline-block align-baseline font-bold text-xs text-cyan hover:text-cyan"
                                    href="/ui/login"
                                >
                                    Login
                                </A>
                            </div>
                            <div class="mb-4">
                                <label
                                    class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                                    for="email"
                                >
                                    Email
                                </label>
                                <input
                                    class="shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline"
                                    type="text"
                                    id="email"
                                    placeholder="Email"
                                    name="email"
                                />
                            </div>
                            <div class="flex items-center justify-between">
                                <button
                                    class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                                    type="submit"
                                >
                                    Submit
                                </button>
                            </div>
                        </div>
                    </div>
                </form>
            </div>

        </Suspense>
    }
}
