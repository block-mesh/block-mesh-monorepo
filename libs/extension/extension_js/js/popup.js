console.log("popup.js loaded");

function onSuccess(message) {
    console.log(`Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`Promise error: ${error}`);
}

async function onClickLoging() {
    try {
        console.log("Starting onClickLoging");
        let email = document.getElementById("email").value;
        let password = document.getElementById("password").value;
        let message = {
            action: "loging",
            email,
            password
        }
        console.log("Logging in", {message});
        await chrome.runtime.sendMessage(message).then(onSuccess, onError);
    } catch (e) {
        console.error(`Login error: ${e}`);
    }
}

// Popups cannot have any inline scripts with our security policies.
// Click handlers should be added when the popup is opened.
document.addEventListener('DOMContentLoaded', async function () {
    let login_button = document.getElementById("popup_login_button");
    login_button.addEventListener("click", onClickLoging);
});

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
        console.log("Popup::onMessage", {msg, sender});
        return true
    }
);