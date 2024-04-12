console.log("popup.js loaded");

// A placeholder for OnSuccess in .then
function onSuccess(message) {
    // console.log(`Send OK: ${JSON.stringify(message)}`);
}

// A placeholder for OnError in .then
function onError(error) {
    console.error(`Promise error: ${error}`);
}

// Popups cannot have any inline scripts with our security policies.
// Click handlers should be added when the popup is opened.
document.addEventListener('DOMContentLoaded', async function () {
    // console.log("Toolbar button clicked");

    // contact us / feedback link
    // links cannot be open on-click as in a normal web page
    // they have to go through an API and open in a new tab
    // https://developer.chrome.com/docs/extensions/reference/tabs/#opening-an-extension-page-in-a-new-tab
    let btn = document.getElementById("btn_contact");
    btn.addEventListener("click", async () => {
        chrome.tabs.create({url: "https://github.com/rimutaka/spotify-playlist-builder/issues"});
    });

    btn = document.getElementById("btn_info");
    btn.addEventListener("click", async () => {
        chrome.tabs.create({url: "https://github.com/rimutaka/spotify-playlist-builder/#readme"});
    });

    // add tracks button
    btn = document.getElementById("btn_add");
    btn.addEventListener("click", async (evt) => {
        console.log("btn_add button clicked");


        // Chrome grants host permissions on install.
        // Firefox treats them as optional and we have to request them at runtime in response to a user action.
        // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/permissions/request
        //

        const permissions = {
            origins: ["*://*.spotify.com/*", "*://open.spotify.com/*"]
        };

        // this line must at the start of the handler and it is very sensitive to what comes before that
        // e.g. having an await or or an if block returns Error: permissions.request may only be called from a user input handler
        // This post explains the situation https://stackoverflow.com/questions/47723297/firefox-extension-api-permissions-request-may-only-be-called-from-a-user-input
        //
        // the Promise always resolves to success even if the permission was denied, so we have to check if the permission is there after the request
        // We cannot check beforehand before then the request fails.
        // If the permission had been set earlier, the user gets no dialog and the code just runs
        // If the permission is denied, the code exits and the user will be asked again next time
        // If the permission is allowed, the code runs, but cannot do the job because the tokens are not there yet
        // So it fails gracefully and asks the user to reload the page.
        // This only happens in Firefox. Chrome runs perfectly fine.
        chrome.permissions.request(permissions).then(async () => {
            // console.log("Permission decided");
            if (await chrome.permissions.contains(permissions)) {
                // console.log("Permission granted");
                await chrome.runtime.sendMessage({action: "btn_add", qty: document.getElementById("tracks_qty").value});
                document.getElementById("btn_add").disabled = true;
            } else {
                console.log("Permission denied");
            }
        }, (error) => {
            console.error(`Permissions dialog error: ${error}`);
            document.getElementById("log-summary").innerText = "Permissions dialog error. Reload the page and try again.";
        });
    });

    chrome.action.getBadgeText({}).then((badgeText) => {
        // console.log(`Badge: ${badgeText}`);
        if (badgeText) {
            btn.disabled = true;
            document.getElementById("log-summary").innerText = "Waiting for progress update ..."
        }
        ;
    }, onError)

    // hide the help message about opening the extension on a playlist page if it is a playlist page
    if (await isPlaylist()) {
        document.getElementById("otherPage").style.display = "none"
    } else {
        // it is not a playlist page - disable the button
        btn.disabled = true;
    }

});

// listens for msgs from WASM
chrome.runtime.onMessage.addListener((msg, sender) => {

        // background.js may send a status update as boolean because
        // there is no badge change event
        if (typeof msg === "boolean") {
            document.getElementById("btn_add").disabled = msg;
            return;
        }

        // if it's not a bool, then it is a log entry as a string
        const log = document.getElementById("log");

        const lastMsg = document.getElementById("log-summary").innerText;
        if (lastMsg) {
            const p = document.createElement("p");
            p.innerText = lastMsg;

            log.insertBefore(p, log.firstChild);
        }

        document.getElementById("log-summary").innerText = msg;
    }
);

// Returns true if the current tab URL contains /playlist/ part
async function isPlaylist() {

    let [tab] = await chrome.tabs.query({active: true, currentWindow: true});
    // console.log(JSON.stringify(tab));

    return tab?.url?.includes("/playlist/")
}