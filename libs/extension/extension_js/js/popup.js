console.log("popup.js loaded");

let state = {
    email: "",
    blockmesh_api_token: "",
    blockmesh_url: "http://localhost:8000"
}

async function getStorageValueWithDefault(key, defaultValue) {
    console.debug("getStorageValueWithDefault::", {key, defaultValue});
    try {
        let result = await chrome.storage.sync.get(key);
        if (!result?.key) {
            console.debug(`getStorageValueWithDefault:: Key ${key} not found, returning default value: ${defaultValue}`);
            return defaultValue;
        } else {
            return result.key;
        }
    } catch (e) {
        console.error(`getStorageValueWithDefault::Error getting ${key} , error : ${e}`);
    }
    return defaultValue
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
        let blockmesh_url = await getStorageValueWithDefault("blockmesh_url", state.blockmesh_url);

        let message = {
            action: "loging",
            email,
            password,
            blockmesh_url
        }
        console.log("Sending message", {blockmesh_url, message});
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
            console.log("Logging success", {message, data});
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
    let email_from_storage = await getStorageValueWithDefault("email", undefined);
    if (email_from_storage !== undefined) {
        state.email = email_from_storage;
    }
    let blockmesh_api_token_from_storage = await getStorageValueWithDefault("blockmesh_api_token", undefined);
    if (blockmesh_api_token_from_storage !== undefined) {
        state.blockmesh_api_token = blockmesh_api_token_from_storage;
    }
    let blockmesh_url_from_storage = await getStorageValueWithDefault("blockmesh_url", "http://localhost:8000");
    if (blockmesh_url_from_storage !== undefined) {
        state.blockmesh_url = blockmesh_url_from_storage;
    }
    console.log("Popup::DOMContentLoaded", JSON.stringify(state, null, 2));
    let login_button = document.getElementById("popup_login_button");
    login_button.addEventListener("click", onClickLoging);
});

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
        console.log("Popup::onMessage", {msg, sender});
        return true
    }
);