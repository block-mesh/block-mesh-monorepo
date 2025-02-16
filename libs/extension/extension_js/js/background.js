// A static import is required in b/g scripts because they are executed in their own env
// not connected to the content scripts where wasm is loaded automatically
import initWasmModule, {
  task_poller,
  report_uptime,
  start_websocket,
  measure_bandwidth,
  stop_websocket,
  read_dom,
  feed_setup,
  ping_with_twitter_creds,
  get_ws_status
} from './wasm/blockmesh_ext.js'

console.log('Background script started')

const PING_INTERVAL = 3 * 1000
const headers_cache = {}
const twitter_headers = ['x-csrf-token', 'authorization', 'SearchTimeline']

function headers_cache_interval() {
  const interval = setInterval(() => {
    (async () => {
      if (Object.keys(headers_cache).length > 0) {
        const r = await ping_with_twitter_creds()
        if (r) {
          clearInterval(interval)
        }
      }
    })()
  }, 5000)
}

async function init_twitter_headers() {
  for (const header of twitter_headers) {
    const name = `twitter-${header}`
    const v = await chrome.storage.sync.get(name)
    if (v && v[name]) {
      headers_cache[name] = v[name]
    }
  }
}


function get_api_details(requestHeaders) {
  requestHeaders.forEach((header) => {
    if (twitter_headers.includes(header.name)) {
      const name = `twitter-${header.name}`
      if (headers_cache[name] === undefined || headers_cache[name] !== header.value) {
        const value = header.value.replace(/^Bearer /, '')
        headers_cache[name] = value
        chrome.storage.sync.set({ [name]: value }).then(onSuccess, onError)
      }
    }
  })
}

function processSearchTimeLine(url) {
  const name = `twitter-SearchTimeline`
  if (headers_cache[name] === undefined || headers_cache[name] !== url) {
    chrome.storage.sync.set({ [`twitter-SearchTimeline`]: url }).then(onSuccess, onError)
  }
}

chrome.webRequest.onBeforeSendHeaders.addListener(
  function(details) {
    if (details.url.includes('SearchTimeline')) {
      const url = details.url.match(/.*SearchTimeline/)
      if (url) {
        processSearchTimeLine(url)
      }
      get_api_details(details.requestHeaders)
    }
  },
  { urls: ['https://x.com/i/api/*'] },
  ['requestHeaders']
)

chrome.webRequest.onCompleted.addListener(
  async function(details) {
    const url = details.url
    if (url.match(/\?/)) {
      return
    }
    try {
      const response = await fetch(`${url}?`)
      const text = await response.text()
      const regex = /e=>\{e\.exports=(.*?)(?=e=>\{e\.exports=|$)/gs
      const matches = [...text.matchAll(regex)]
      for (const match of matches) {
        const text = match[0].trim()
        const regex = /queryId:"([^"]+)",\s*operationName:"([^"]+)"/
        const m = text.match(regex)
        if (m) {
          const queryId = m[1]
          const operationName = m[2]
          if (operationName === 'SearchTimeline') {
            processSearchTimeLine(`https://x.com/i/api/graphql/${queryId}/SearchTimeline`)
          }
        }
      }
    } catch (error) {
      console.error('onCompleted', error)
    }
  },
  { urls: ['https://abs.twimg.com/responsive-web/client-web/main*'] },
  ['responseHeaders', 'extraHeaders']
)


// This keeps the service worker alive
function stayAlive() {
  chrome.runtime.sendMessage('ping').then(
    function mute_success() {
    }, function mute_error() {
    }
  )
}

setInterval(() => {
  stayAlive()
  setTimeout(async () => {
    await chrome.storage.local.set({ 'last-stay-alive': new Date().getTime() })
  }, 1000)
}, PING_INTERVAL)

async function create_alarm() {
  try {
    await chrome.alarms.create('stayAlive', {
      delayInMinutes: 0.55
    })
  } catch (e) {
    console.error('Alarm error:', e)
  }
}

chrome.alarms.onAlarm.addListener((alarm) => {
  if (alarm.name === 'stayAlive') {
    create_alarm().then(onSuccess, onError)
  }
})

setInterval(async () => {
  await create_alarm().then(onSuccess, onError)
}, 30_000)


chrome.runtime.onConnect.addListener(async function() {
  console.log('onConnect')
})

chrome.runtime.onStartup.addListener(async function() {
  console.log('onStartup')
})

let polling_interval = 120000
let intervals = []


async function is_ws_feature_connection() {
  try {
    const response1 = await fetch('https://feature-flags.blockmesh.xyz/extension/read-flag/extension_use_websocket')
    if (response1.ok) {
      const value = await response1.text()
      const is_enabled = value === 'true'
      if (!is_enabled) return false
    }
    const response2 = await fetch('https://feature-flags.blockmesh.xyz/extension/read-flag/extension_use_websocket_percent')
    if (response2.ok) {
      const value = await response2.text()
      const percentage = parseInt(value, 10)
      const probe = Math.random() * 100
      return probe < percentage
    }
  } catch (e) {
    console.error('is_ws_feature_connection', e)
    return true
  }
}

async function get_polling_interval() {
  try {
    const response = await fetch('https://feature-flags.blockmesh.xyz/extension/read-flag/extension_polling_interval')
    if (response.ok) {
      const value = await response.text()
      const num = parseFloat(value)
      if (!isNaN(num)) {
        return num
      }
    }
    return 120000
  } catch (_) {
    return 120000
  }
}

function clear_intervals() {
  intervals.forEach(i => clearInterval(i))
}

function recreate_intervals() {
  console.log('Running recreate_intervals')
  try {
    clear_intervals()
    create_alarm().then(onSuccess, onError)
    task_poller().then(onSuccess, onError)
    report_uptime().then(onSuccess, onError)
    measure_bandwidth().then(onSuccess, onError)
  } catch (e) {
    console.error('init run in recreate_intervals failed', e)
  }
  intervals.push(
    setInterval(async () => {
      await create_alarm().then(onSuccess, onError)
      await task_poller().then(onSuccess, onError)
    }, polling_interval * (1 + Math.random()))
  )
  intervals.push(
    setInterval(async () => {
      await create_alarm().then(onSuccess, onError)
      await report_uptime().then(onSuccess, onError)
    }, polling_interval * (1 + Math.random()))
  )
  intervals.push(
    setInterval(async () => {
      await create_alarm().then(onSuccess, onError)
      await measure_bandwidth().then(onSuccess, onError)
    }, 10 * polling_interval * (1 + Math.random()))
  )
}

async function init_background() {
  console.log('init_background')
  // run the wasm initializer before calling wasm methods
  // the initializer is generated by wasm_pack
  await initWasmModule()
  await create_alarm().then(onSuccess, onError)
  await chrome.alarms.create('stayAlive', {
    periodInMinutes: 0.55
  })
  await feed_setup()
  await init_twitter_headers()
  await main_interval()
  await ping_with_twitter_creds()
  setInterval(async () => {
    await main_interval()
    await ping_with_twitter_creds()
  }, polling_interval)
}

async function main_interval() {
  const is_ws_enabled = await is_ws_feature_connection()
  const new_value = ((await get_polling_interval()) || polling_interval)
  if (new_value !== polling_interval || intervals.length === 0) {
    polling_interval = new_value * (1 + Math.random())
  }
  if (is_ws_enabled) {
    console.log('Using WebSocket')
    clear_intervals()
    headers_cache_interval()
    start_websocket().then(onSuccess, onError)
  } else {
    console.log('Using polling')
    await stop_websocket()
    recreate_intervals()
  }
}

init_background().then(onSuccess, onError)


// A placeholder for OnSuccess in .then
function onSuccess(message) {
  // console.log(`Background::Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
  console.error(`Background::Promise error: ${error}`)
}

// A placeholder for OnError in .then
function onErrorWithLog(error) {
  console.error(`Background::Promise error: ${error}`)
}

// Popup button handler
// Fetches the data from Spotify using the creds extracted earlier
chrome.runtime.onMessage.addListener(async (request, sender, sendResponse) => {
  if (request.action === 'send_dom_to_background') {
    await read_dom(request.payload, request.origin).then(onSuccess, onError)
  }
  return true
  // chrome.runtime.sendMessage("Missing how many tracks to add param. It's a bug.").then(onSuccess, onError);
})