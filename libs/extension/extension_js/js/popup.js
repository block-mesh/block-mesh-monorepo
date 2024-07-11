import initWasmModule, {mount_popup} from './wasm/blockmesh_ext.js';

const channel = new MessageChannel();

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
    const iframe = document.createElement("iframe");
    iframe.src = "http://localhost:8000/ext/login";
    iframe.width = "300";
    iframe.height = "400"

    iframe.addEventListener("load", onLoad);

    function onLoad() {
        // Listen for messages on port1
        channel.port1.onmessage = onMessage;

        // Transfer port2 to the iframe
        iframe.contentWindow.postMessage("Hello from the main page!", "*", [
            channel.port2,
        ]);
    }

    function onMessage(e) {
        console.log("popup onMessage e => ", e);
    }

    const body = document.body;
    body.appendChild(iframe);
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