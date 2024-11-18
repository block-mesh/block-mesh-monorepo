console.log('In CONTENT.JS')
const MARKED_ID = 'data-testmarked'
const MARKED_VALUE = 'true'
const targetNode = document.body
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true }

function send_dom_to_background(el) {
  console.log('send_dom_to_background')
  const elements = Array.from(document.querySelectorAll('[data-testid="tweet"]:not([data-testmarked="true"])'))
  for (const el of elements) {
    el.setAttribute(MARKED_ID, MARKED_VALUE)
    chrome.runtime.sendMessage({ action: 'send_dom_to_background', payload: el.outerHTML }, (response) => {
      console.log('Response from background script:', response)
    })
  }

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