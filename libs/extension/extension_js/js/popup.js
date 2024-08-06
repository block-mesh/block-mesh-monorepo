import initWasmModule, { mount_popup } from './wasm/blockmesh_ext.js'

const channel = new MessageChannel()
window.channel = channel
window.message_channel_port = channel.port1
window.mounted = false

function onSuccess(message) {
  try {
    console.log(`onSuccess: ${JSON.stringify(message)}`)
  } catch (e) {
    console.error(`onSuccess error: ${e}`)
  }
}

function onError(error) {
  try {
    console.error(`onError: ${JSON.stringify(error)}`)
  } catch (e) {
    console.error(`onError error (1): ${error}`)
    console.error(`onError error (2): ${e}`)
  }
}

function onLoad(iframe) {
  // Listen for messages on port1
  window.channel.port1.onmessage = onMessage
  // Transfer port2 to the iframe
  iframe.contentWindow.postMessage('READY', '*', [
    window.channel.port2
  ])
}

async function onMessage(e) {
  const { data } = e
  console.log('popup.js::onMessage', JSON.stringify(data))
  if (!window.mounted && data === 'READY') {
    setTimeout(() => {
      mount_popup()
    }, 250)
    window.mounted = true
  }
  const { msg_type, key, value } = data
  if (msg_type === 'GET' && key) {
    const val = await chrome.storage.sync.get(key)
    window.message_channel_port.postMessage({ [key]: val })
  }
  if (msg_type === 'SET' && key) {
    if (key === 'email') {
      await chrome.storage.sync.set({ [key]: value.toLocaleLowerCase() })
    } else {
      await chrome.storage.sync.set({ [key]: value })
    }
  }
  if (msg_type === 'DELETE' && key) {
    await chrome.storage.sync.remove(key)
  }
  if (msg_type === 'GET_ALL') {
    console.log('GET_ALL', window.location.href)
    await chrome.storage.sync.get(null, async function(items) {
      const allKeys = Object.keys(items)
      if (window.message_channel_port) {
        for (const key of allKeys) {
          const val = await chrome.storage.sync.get(key)
          window.message_channel_port.postMessage({ [key]: val })
        }
      }
    })
  }
  if (msg_type === 'COPY_TO_CLIPBOARD' && key === 'invite_code') {
    try {
      await navigator.clipboard.writeText(value)
    } catch (e) {
      console.error('Failed to copy to clipboard')
    }
  }
}

// Popups cannot have any inline scripts with our security policies.
// Click handlers should be added when the popup is opened.
document.addEventListener('DOMContentLoaded', async function() {
  await initWasmModule().then(onSuccess, onError)
  const iframe = document.createElement('iframe')
  const url = (((await chrome.storage.sync.get('blockmesh_url'))?.blockmesh_url) || 'https://app.blockmesh.xyz')
  iframe.src = `${url}/ext/login` // "http://localhost:8000/ext/login";
  iframe.width = '300'
  iframe.height = '400'
  iframe.style = 'border: 0px'
  iframe.addEventListener('load', (_) => onLoad(iframe))
  const body = document.body
  body.appendChild(iframe)
})

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {
    if (msg !== 'ping') {
      console.log('Popup::onMessage', { msg, sender })
    }
    return true
  }
)