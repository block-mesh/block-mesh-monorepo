use leptos::*;
use leptos_router::A;

#[component]
pub fn ResendConfirmationEmailPage() -> impl IntoView {
    view! {
        <form action="/resend_confirmation_email" method="post">
            <div class="bg-gray-700 flex justify-center items-center h-screen">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="text-white text-2xl font-semibold text-center mb-6">
                        Resend Confirmation Email
                    </h2>
                    <div class="flex justify-around mb-4">
                        <A
                            class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                            href="/ui/login"
                        >
                            Login
                        </A>
                    </div>
                    <form>
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
                    </form>
                </div>
            </div>
        </form>
    }
}
