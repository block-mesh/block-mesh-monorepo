// A static import is required in b/g scripts because they are executed in their own env
// not connected to the content scripts where wasm is loaded automatically
import initWasmModule, {task_poller, report_uptime, uptime_fetcher, measure_bandwidth} from './wasm/blockmesh_ext.js';

console.log("Background script started");

const PING_INTERVAL = 3 * 1000;

// This keeps the service worker alive
function stayAlive() {
    chrome.runtime.sendMessage("ping").then(
        function mute_success() {
        }, function mute_error() {
        }
    );
}

setInterval(() => {
    stayAlive();
}, PING_INTERVAL);


chrome.runtime.onConnect.addListener(async function () {
    console.log("onConnect");
});

chrome.runtime.onStartup.addListener(async function () {
    console.log("onStartup");
});

async function init_background() {
    console.log("init_background");
    // run the wasm initializer before calling wasm methods
    // the initializer is generated by wasm_pack
    await initWasmModule();
    setInterval(async () => {
        await task_poller().then(onSuccess, onError);
    }, 30_000 + Math.random());
    setInterval(async () => {
        await report_uptime().then(onSuccess, onError);
    }, 30_000 + Math.random());
    setInterval(async () => {
        await measure_bandwidth().then(onSuccess, onError);
    }, 60_000 + Math.random());
}

init_background().then(onSuccess, onError);


// A placeholder for OnSuccess in .then
function onSuccess(message) {
    // console.log(`Background::Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`Background::Promise error: ${error}`);
}

// A placeholder for OnError in .then
function onErrorWithLog(error) {
    console.error(`Background::Promise error: ${error}`);
}

// Popup button handler
// Fetches the data from Spotify using the creds extracted earlier
chrome.runtime.onMessage.addListener(async (request, sender, sendResponse) => {
    console.log(`Popup message received: ${JSON.stringify(request)}, ${JSON.stringify(sender)}`);
    return true;
    // chrome.runtime.sendMessage("Missing how many tracks to add param. It's a bug.").then(onSuccess, onError);
});