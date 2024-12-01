console.log('In CONTENT.JS')
const MARKED_ID = 'data-testmarked'
const MARKED_VALUE = 'true'
const targetNode = document.body
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true }

let feed_origin = null
let feed_selector = null

function onSuccess(message) {
  // console.log(`Background::Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
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

let timer // This will hold the timeout reference

function debounce(func, delay) {
  return function(...args) {
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
function callback(mutationsList, observer) {
  for (let mutation of mutationsList) {
    if (mutation.type === 'childList') {
      debounce(send_dom_to_background, 500)()
    }
  }
}

const observer = new MutationObserver(callback)
observer.observe(targetNode, config)