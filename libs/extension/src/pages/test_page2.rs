use leptos::*;

#[component]
pub fn TestPage2() -> impl IntoView {
    view! {
    <meta charset="UTF-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    // <link rel="stylesheet" href="style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
    <link href="https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap" rel="stylesheet"/>
    <title>Login</title>
    <div class="auth-card">
        <img class="background-image" src="./background.png" alt="background"/>
        <div class="auth-card-frame">

        </div>
        <div class="auth-card-top">

        </div>
        <div class="auth-card-body">
            <img class="logo" src="./logo.png" alt="logo"/>
            <h1>
                BlockMesh
            </h1>
            <form>
                <div class="auth-card-input-container">
                    <input type="text" required=""/>
                    <label>Email</label>
                </div>
                <div class="auth-card-input-container">
                    <input type="password" required=""/>
                    <label>Password</label>
                </div>
                <br/>
                <button class="auth-card-button">
                    Login
                </button>
            </form>
        </div>
        <div class="auth-card-bottom">
            <small class="auth-card-sub-text">
                Doesnt have an account yet?
            </small>
            <br/>
            <small class="auth-card-link register-link">
                Register now
            </small>
        </div>
    </div>
    }
}
