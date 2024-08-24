use crate::frontends::components::extension_button::ExtensionButton;
use crate::frontends::components::extension_input::ExtensionInput;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use leptos::*;

#[derive(Copy, Clone)]
pub enum CardMode {
    Login,
    Register,
    LoggedIn,
}

#[component]
pub fn ExtensionCard(mode: CardMode) -> impl IntoView {
    let content = match mode {
        CardMode::Login => LoginExtensionCard().into_view(),
        CardMode::Register => RegisterExtensionCard().into_view(),
        CardMode::LoggedIn => LoggedInExtensionCard().into_view(),
    };

    view! {
        <div class="extension-wrapper flex justify-between flex-col relative h-[400px] w-[300px] bg-[#202525]">
            <img
                class="absolute w-full h-auto right-0 top-0"
                style:opacity="0.3"
                src="/app/extension-background.png"
                alt="background"
            />
            <div
                class="absolute m-2 z-0 border-4"
                style="height: calc(100% - 16px); width: calc(100% - 16px); border-color: #76abaebb; "
            ></div>
            <div></div>
            <div class="text-center relative z-10" style="marginBottom: 10%">
                <img class="h-16 w-16 m-auto" src="/app/logo.png" alt="logo" />
                <h1 class="text-darkOrange font-jetbrains">BlockMesh</h1>
                {content}
            </div>
            <div class="text-center relative" style:bottom="7%">
                <Show
                    when=move || matches!(mode, CardMode::LoggedIn)
                    fallback=move || {
                        view! {
                            <div>
                                <small class="font-jetbrains text-orange opacity-80">
                                    {if matches!(mode, CardMode::Register) {
                                        "You already have an account?"
                                    } else {
                                        "Doesn't have an account yet?"
                                    }}

                                </small>
                                <br />
                                <small class="font-jetbrains cursor-pointer text-orange underline">
                                    {if matches!(mode, CardMode::Register) {
                                        "Login Now"
                                    } else {
                                        "Register Now"
                                    }}

                                </small>
                            </div>
                        }
                    }
                >

                    <div class="flex justify-center logged-bottom" style:bottom="10%">
                        <ExtensionButton fit=true text="Open Dashboard" />
                        <ExtensionButton fit=true text="Refer" />
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
pub fn LoginExtensionCard() -> impl IntoView {
    view! {
        <form>
            <ExtensionInput label="Email" type_="text" />
            <ExtensionInput label="Password" type_="password" />
            <br />
            <ExtensionButton text="login" />
        </form>
    }
}

#[component]
pub fn RegisterExtensionCard() -> impl IntoView {
    view! {
        <div>
            <ExtensionInput label="Email" type_="text" />
            <ExtensionInput label="Password" type_="password" />
            <ExtensionInput label="Referer Code" type_="text" />
            <ExtensionButton text="Register" />
        </div>
    }
}

#[component]
pub fn LoggedInExtensionCard() -> impl IntoView {
    view! {
        <div class="auth-card-content">
            <br />
            <br />
            <div
                class="pulse h-24 w-48 overflow-hidden absolute top-0 bottom-0 left-0 right-0 m-auto"
                style:scale="0.4"
            ></div>
            <br />
            <br />
            <small class="relative" style="color: #fff8; top: -30px;">
                version: 0.0.27
            </small>
            <div
                class="m-auto font-extrabold relative py-1 px-2 rounded-lg"
                style="background-color: #fff8; width: fit-content; color: #222; top: -20px;"
            >
                <strong>ohaddahan@gmail.com</strong>
            </div>
        </div>
    }
}

#[component]
pub fn Extension() -> impl IntoView {
    view! {
        <ApplicationLayout>
            <p>Login</p>
            <ExtensionCard mode=CardMode::Login />
            <br />
            <p>Register</p>
            <ExtensionCard mode=CardMode::Register />
            <br />
            <p>Logged In</p>
            <ExtensionCard mode=CardMode::LoggedIn />
        </ApplicationLayout>
    }
}
