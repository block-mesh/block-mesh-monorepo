export function onPostMessage(callback) {
  if (!window.message_channel_port) return
  if (window.message_channel_port?.addEventListener) {
    window.message_channel_port.addEventListener('message', (msg) => {
      const { data } = msg
      console.debug('connectors.js::onPostMessage:: event listener', { msg, data })
      callback(data)
    })
  } else {
    window.addEventListener('message', (msg) => {
      const { data } = msg
      console.debug('connectors.js::onPostMessage:: event listener', { msg, data })
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

export async function pubkey(wallet) {
  if (!window[wallet]) {
    console.error(`pubkey::window doesnt have ${wallet}`)
    return ''
  }
  await window[wallet].connect()
  return window[wallet].publicKey.toBase58()
}

export async function sign_message(msg, wallet) {
  try {
    if (!window[wallet]) {
      console.error(`sign_message::window doesnt have ${wallet}`)
      return ''
    }
    await window[wallet].connect()
    const message = new TextEncoder().encode(msg)
    const out = await window[wallet].signMessage(message)
    const sig = out.signature
    console.log('sig_message sig = ', sig)
    return sig
  } catch (e) {
    console.error('sign_message error:', e)
    return ''
  }
}