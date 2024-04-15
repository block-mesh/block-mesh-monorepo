async function onClickSubmit() {
    try {
        console.log("Starting onClickSubmit");
        let blockmesh_url = document.getElementById("blockmesh_url").value;
        await chrome.storage.sync.set({blockmesh_url});
    } catch (e) {
        console.error(`Submit error: ${e}`);
    }
}

document.addEventListener('DOMContentLoaded', async function () {
    let login_button = document.getElementById("options_submit_button");
    login_button.addEventListener("click", onClickSubmit);
});