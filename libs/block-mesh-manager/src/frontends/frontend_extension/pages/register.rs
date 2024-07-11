use leptos::*;
use leptos_router::A;

#[component]
pub fn ExtensionRegister() -> impl IntoView {
    view! {
        <div class="auth-card">
            <img
                class="background-image"
                src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/16475f13-7a36-4787-a076-580885250100/public"
                alt="background"
            />
            <div class="auth-card-frame"></div>
            <div class="auth-card-top"></div>
            <div class="auth-card-body">
                <img
                    class="logo"
                    src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                    alt="logo"
                />
                <h1>BlockMesh</h1>
                <form>
                    <div class="auth-card-input-container">
                        <input type="text" required=""/>
                        <label>Email</label>
                    </div>
                    <div class="auth-card-input-container">
                        <input type="password" required=""/>
                        <label>Password</label>
                    </div>
                    <div class="auth-card-input-container">
                        <input type="text" required=""/>
                        <label>Refer Code</label>
                    </div>
                    <button class="auth-card-button">Register</button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">You already have an account?</small>
                <br/>
                <small class="auth-card-link register-link">
                    <A href="/ext/login">Login now</A>
                </small>
            </div>
        </div>
    }
}
