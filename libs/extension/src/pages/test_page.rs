use leptos::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <meta charset="UTF-8"/>
        <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        // <link rel="stylesheet" href="style.css">
        // <link rel="stylesheet" href="pulse.css">
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link href="https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap" rel="stylesheet"/>
        <title>LoggedIn</title>
    <div>

        <div class="auth-card">
            <img class="background-image" src="./background.png" alt="background"/>
            <div class="auth-card-frame">

            </div>
            <div class="auth-card-top">
                <div class="auth-card-logout">
                    <svg title="logout" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#fab457cc"><path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h280v80H200v560h280v80H200Zm440-160-55-58 102-102H360v-80h327L585-622l55-58 200 200-200 200Z"/>
                    </svg>
                </div>
            </div>
            <div class="auth-card-body">
                <img class="logo" src="./logo.png" alt="logo"/>
                <h1>
                    BlockMesh
                </h1>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="auth-card-version">
                        version: 0.0.27
                    </small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>
                            ohaddahan@gmail.com
                        </strong>
                    </div>
                </div>
            </div>
            <div class="auth-card-bottom logged-bottom">
                <button class="auth-card-button">
                    Open Dashboard
                </button>
                <button class="auth-card-button">
                    Refer
                </button>
            </div>
        </div>
    </div>
        }
}
