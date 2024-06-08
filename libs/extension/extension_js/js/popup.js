import initWasmModule, {mount_popup} from './wasm/blockmesh_ext.js';

function onSuccess(message) {
    try {
        console.log(`onSuccess: ${JSON.stringify(message)}`);
    } catch (e) {
        console.error(`onSuccess error: ${e}`);
    }
}

function onError(error) {
    try {
        console.error(`onError: ${JSON.stringify(error)}`);
    } catch (e) {
        console.error(`onError error (1): ${error}`);
        console.error(`onError error (2): ${e}`);
    }
}

// Popups cannot have any inline scripts with our security policies.
// Click handlers should be added when the popup is opened.
document.addEventListener('DOMContentLoaded', async function () {
    await initWasmModule().then(onSuccess, onError);
    console.log("pre mount");
    mount_popup();
    console.log("post mount");
});

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
        if (msg !== "ping") {
            console.log("Popup::onMessage", {msg, sender});
        }
        return true
    }
);