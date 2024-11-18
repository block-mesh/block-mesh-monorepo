console.log('In CONTENT.JS')
const targetNode = document.body
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true }

function send_dom_to_background() {
  console.log('send_dom_to_background')
  const dom = document.documentElement.outerHTML
  // Sending a message from the content script to the background script
  chrome.runtime.sendMessage({ action: 'send_dom_to_background', payload: dom }, (response) => {
    console.log('Response from background script:', response)
  })
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
      console.log('before debounce')
      const t = Array.from(document.querySelectorAll('[data-testid="tweet"]'))
      debounce(send_dom_to_background, 500)()
    }
  }
}

const observer = new MutationObserver(callback)
observer.observe(targetNode, config)