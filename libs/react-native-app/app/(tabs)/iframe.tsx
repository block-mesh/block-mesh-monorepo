import {Iframe} from "@bounceapp/iframe"


export default function IFrameWraooer() {
    return (
        <Iframe uri="http://localhost:8000/tauri/login" style={{flex: 1}}/>
    )
}