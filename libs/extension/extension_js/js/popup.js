import initWasmModule, {mount_popup} from './wasm/blockmesh_ext.js';

const channel = new MessageChannel();
window.channel = channel;

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
    let url = (((await chrome.storage.sync.get("blockmesh_url"))?.blockmesh_url) || "https://app.blockmesh.xyz");
    let iframe = document.createElement("iframe");
    iframe.src = `${url}/extension`;
    iframe.id = "ext-iframe";
    iframe.addEventListener("load", onLoad);

    function onLoad() {
        // Listen for messages on port1
        channel.port1.onmessage = onMessage;
        // Transfer port2 to the iframe
        iframe.contentWindow.postMessage(
            "A message from the index.html page!",
            "*",
            [channel.port2]
        );
    }

    function onMessage(e) {
        console.log("popup onMessage", e);
    }

    let body = document.body;
    body.appendChild(iframe);
    await initWasmModule().then(onSuccess, onError);
    console.log("pre mount");
    // mount_popup();
    console.log("post mount");
});

function send_channel_message(message) {
    let iframe = document.getElementById("ext-iframe");
    if (!iframe) return;
    iframe.contentWindow.postMessage(message, "*", [channel.port2]);
}

window.send_channel_message = send_channel_message;

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
        if (msg !== "ping") {
            console.log("Popup::onMessage", {msg, sender});
        }
        return true
    }
);