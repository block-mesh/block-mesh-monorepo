export function onPostMessage(callback) {
    if (!window.message_channel_port) return;
    window.message_channel_port.addEventListener("message", (msg) => {
        const {data} = msg;
        callback(data);
    });
}

export async function send_message(msg) {
    try {
        if (!window.message_channel_port) {
            console.log("connectors.js: message_channel_port is missing");
            return;
        }
        window.message_channel_port.postMessage(msg);
    } catch (e) {
        return ""
    }
}