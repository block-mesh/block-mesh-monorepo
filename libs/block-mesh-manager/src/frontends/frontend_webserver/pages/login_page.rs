use leptos::*;
use leptos_router::A;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="bg-dark-blue h-screen">
            <form action="/login" method="post">
                <div class="bg-dark-blue flex justify-center items-center h-screen">
                    <div class="border-off-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                        <h2 class="font-bebas-neue text-off-white text-2xl text-center mb-6">
                            Login
                        </h2>
                        <div class="flex justify-around mb-4">
                            <A
                                class="font-bebas-neue px-4 py-2 rounded font-bold text-sm text-cyan hover:text-orange"
                                href="/ui/register"
                            >
                                Register
                            </A>
                        </div>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm mb-2"
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
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue block text-off-white text-sm mb-2"
                                for="password"
                            >
                                Password
                            </label>
                            <input
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-black mb-3 leading-tight focus:outline-none focus:shadow-outline"
                                type="password"
                                id="password"
                                name="password"
                                placeholder="******************"
                            />
                        </div>
                        <div class="flex items-center justify-between">
                            <button
                                class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                                type="submit"
                            >
                                Submit
                            </button>
                            <div class="flex flex-col ml-2">
                                <A
                                    class="font-open-sans mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:text-cyan"
                                    href="/ui/reset_password"
                                >
                                    Reset Password
                                </A>
                                <A
                                    class="font-open-sans inline-block align-baseline font-bold text-xs text-cyan hover:text-cyan"
                                    href="/ui/resend_confirmation_email"
                                >
                                    Resend Verification Email
                                </A>
                            </div>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    }
}
