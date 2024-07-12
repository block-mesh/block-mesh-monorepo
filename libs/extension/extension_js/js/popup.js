import initWasmModule, {mount_popup} from './wasm/blockmesh_ext.js';

const channel = new MessageChannel();
window.channel = channel;
window.message_channel_port = channel.port1;
window.mounted = false;

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

function onLoad(iframe) {
    // Listen for messages on port1
    window.channel.port1.onmessage = onMessage;
    // Transfer port2 to the iframe
    iframe.contentWindow.postMessage("Hello from the main page!", "*", [
        window.channel.port2,
    ]);
}

async function onMessage(e) {
    console.log("popup", window.location.href, "onMessage e => ", e);
    if (!window.mounted) {
        setTimeout(() => {
            mount_popup();
        }, 500)
        window.mounted = true;
    }
    const {data} = e;
    const {type, key, value} = data;
    if (type === "GET" && key) {
        let val = await chrome.storage.sync.get(key);
        if (val) {
            console.log("value =", val[key]);
        }
    }
    if (type === "SET" && key) {
        await chrome.storage.sync.set({[key]: value});
    }
    if (type === "DELETE" && key) {
        await chrome.storage.sync.remove(key);
    }
}

// Popups cannot have any inline scripts with our security policies.
// Click handlers should be added when the popup is opened.
document.addEventListener('DOMContentLoaded', async function () {
    await initWasmModule().then(onSuccess, onError);
    console.log("pre mount");
    const iframe = document.createElement("iframe");
    const url = (((await chrome.storage.sync.get("blockmesh_url"))?.blockmesh_url) || "https://app.blockmesh.xyz");
    iframe.src = `${url}/ext/login`; // "http://localhost:8000/ext/login";
    iframe.width = "300";
    iframe.height = "400"
    iframe.addEventListener("load", (_) => onLoad(iframe));
    const body = document.body;
    body.appendChild(iframe);
});

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
        if (msg !== "ping") {
            console.log("Popup::onMessage", {msg, sender});
        }
        return true
    }
);