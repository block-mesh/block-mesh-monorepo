import Mellowtel from 'mellowtel'

(async () => {
  const mellowtel = new Mellowtel('54488468') // Replace with your configuration key
  await mellowtel.initContentScript()
  await mellowtel.optIn()
})()

console.log('In CONTENT.JS')
const MARKED_ID = 'data-testmarked'
const MARKED_VALUE = 'true'

// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true }

let feed_origin: string | null = null
let feed_selector: string | null = null

function onSuccess(message: any) {
  // console.log(`Background::Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error: any) {
  console.error(`Background::Promise error: ${error}`)
}


async function prep() {
  console.debug('running prep')
  try {
    if (feed_origin === null) {
      let storage_feed_origin = await chrome.storage.sync.get('feed_origin')
      if (storage_feed_origin && storage_feed_origin?.feed_origin && storage_feed_origin?.feed_origin !== '') {
        feed_origin = storage_feed_origin?.feed_origin
      }
    }
    if (feed_selector === null) {
      let storage_feed_selector = await chrome.storage.sync.get('feed_selector')
      if (storage_feed_selector && storage_feed_selector?.feed_selector && storage_feed_selector?.feed_selector !== '') {
        feed_selector = storage_feed_selector?.feed_selector
      }
    }
  } catch (error) {
    console.error('prep', error)
  }
}


function send_dom_to_background() {
  prep().then(() => {
    if (feed_origin === null || feed_origin === '') {
      console.log('send_dom_to_background early return feed_origin', feed_origin)
      return
    }
    if (feed_selector === null || feed_selector === '') {
      console.log('send_dom_to_background early return feed_selector', feed_selector)
      return
    }
    if (window.origin !== feed_origin) {
      console.log('send_dom_to_background early return origin mismatch', window.origin, feed_origin)
      return
    }
    // console.log('send_dom_to_background running with', feed_origin, feed_selector)
    console.log('send_dom_to_background running')
    const elements = Array.from(document.querySelectorAll(feed_selector))
    for (const el of elements) {
      el.setAttribute(MARKED_ID, MARKED_VALUE)
      chrome.runtime.sendMessage({
        action: 'send_dom_to_background',
        payload: el.outerHTML,
        origin: window.origin
      }, (response) => {
        console.log('Response from background script:', response)
      })
    }
  })
    .catch(onError)
}

let timer: NodeJS.Timeout // This will hold the timeout reference

function debounce(func: any, delay: number) {
  return function(...args: any[]) {
    // @ts-ignore
    const context = this // Preserve the context
    // Clear the existing timer
    clearTimeout(timer)
    // Set a new timer to execute the function after the delay
    timer = setTimeout(() => {
      func.apply(context, args) // Execute the function with the correct context and arguments
    }, delay)
  }
}


// Callback function to execute when mutations are observed
function callback(mutationsList: any, observer: any) {
  for (let mutation of mutationsList) {
    if (mutation.type === 'childList') {
      debounce(send_dom_to_background, 500)()
    }
  }
}

const urls: string[] = [
  'https://.*.x.com/.*',
  'https://x.com/.*',
  'https://.*.twitter.com/.*',
  'https://twitter.com/.*'
]

let observer: MutationObserver

document.addEventListener('DOMContentLoaded', () => {
  const targetNode = document.body

  for (const url of urls) {
    const regexp = new RegExp(url)
    const match = regexp.test(window.location.href)
    if (match) {
      observer = new MutationObserver(callback)
      observer.observe(targetNode, config)
      break
    }
  }
})
