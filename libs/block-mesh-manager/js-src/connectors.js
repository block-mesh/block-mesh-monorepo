export function onPostMessage(callback) {
  if (!window.message_channel_port) return
  if (window.message_channel_port?.addEventListener) {
    window.message_channel_port.addEventListener('message', (msg) => {
      const { data } = msg
      console.log('connectors.js::onPostMessage:: event listener', { msg, data })
      callback(data)
    })
  } else {
    window.addEventListener('message', (msg) => {
      const { data } = msg
      console.log('connectors.js::onPostMessage:: event listener', { msg, data })
      callback(data)
    })
  }
}

export async function send_message(msg) {
  try {
    if (!window.message_channel_port) {
      console.log('connectors.js::send_message:: message_channel_port is missing', msg)
      return
    }
    window.message_channel_port.postMessage(msg)
  } catch (e) {
    return ''
  }
}