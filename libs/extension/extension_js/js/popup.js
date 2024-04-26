import initWasmModule, {mount_popup} from './wasm/blockmesh_ext.js';

function onSuccess(message) {
    console.log(`onSuccess: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`onError: ${error}`);
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
        console.log("Popup::onMessage", {msg, sender});
        return true
    }
);