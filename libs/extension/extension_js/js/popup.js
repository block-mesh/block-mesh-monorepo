console.log("popup.js loaded");

async function getStorageValueWithDefault(key, defaultValue) {
    try {
        let result = await chrome.storage.sync.get(key);
        if (!result?.key) {
            console.log(`Key ${key} not found, returning default value: ${defaultValue}`);
            return defaultValue;
        } else {
            return result.key;
        }
    } catch (e) {
        console.error(`Error getting ${key} , error : ${e}`);
    }
}

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
        let blockmesh_url = await getStorageValueWithDefault("blockmesh_url", "https://app.blockmesh.xyz");

        let message = {
            action: "loging",
            email,
            password,
            blockmesh_url
        }
        let response = await fetch(blockmesh_url + "/api/get_token", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                email,
                password
            })
        })
        let data = await response.json();
        if (data?.api_token) {
            await chrome.storage.sync.set({blockmesh_api_token: data.api_token});
            await chrome.storage.sync.set({email});
            console.log("Logging success", {message});
            // await chrome.runtime.sendMessage(message).then(onSuccess, onError);
        } else {
            console.error("Login failed, missing api_token in response.", {data});
        }
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