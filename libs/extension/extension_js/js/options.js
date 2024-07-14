import initWasmModule, {mount_options} from './wasm/blockmesh_ext.js';

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

document.addEventListener('DOMContentLoaded', async function () {
    await initWasmModule().then(onSuccess, onError);
    mount_options();
    let delete_form = document.getElementById("delete_form");
    if (delete_form) {
        delete_form.parentNode.removeChild(delete_form);
    }
});

chrome.runtime.onMessage.addListener((msg, sender) => {
        if (msg !== "ping") {
            console.log("Options::onMessage", {msg, sender});
        }
        return true
    }
);