import initWasmModule, {mount_options} from './wasm/blockmesh_ext.js';

function onSuccess(message) {
    console.log(`onSuccess: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`onError: ${error}`);
}

document.addEventListener('DOMContentLoaded', async function () {
    await initWasmModule().then(onSuccess, onError);
    console.log("pre mount");
    mount_options();
    console.log("post mount");
});