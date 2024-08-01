import { WebView } from 'react-native-webview'
import React from 'react'
import { API_TOKEN, EMAIL } from '@/utils/constants'
import { getData, storeData } from '@/utils/storage'
import { Pos } from '@jridgewell/gen-mapping/dist/types/types'

export type MessageType =
  'GET_ALL' |
  'GET' |
  'SET' |
  'DELETE' |
  'COPY_TO_CLIPBOARD';

export type MessageKey = 'all' |
  'blockmesh_url' |
  'email' |
  'blockmesh_api_token' |
  'device_id' |
  'uptime' |
  'invite_code' |
  'download_speed' |
  'upload_speed' |
  'last_update';

export type PostMessage = {
  msg_type: MessageType;
  key: MessageKey;
  value: any
}

export async function handleMessage(event: any, webview_ref: React.RefObject<WebView>): Promise<void> {
  console.log('handleMessage:: Message from WebView:', { data: event.nativeEvent.data })
  const data = event.nativeEvent.data
  const { msg_type, key, value } = data
  const msg = data as PostMessage
  if (msg.msg_type === 'SET' && msg.key) {
    if ([EMAIL, API_TOKEN].includes(msg.key)) {
      await storeData(msg.key, msg.value)
    }
  } else if (msg.msg_type === 'GET' && msg.key) {
    if ([EMAIL, API_TOKEN].includes(msg.key)) {
      const value = await getData(msg.key)
      if (value) {
        const message: PostMessage = {
          msg_type: 'SET',
          key,
          value
        }
        webview_ref.current?.postMessage(JSON.stringify(message))
      }
    }
  } else if (msg_type === 'GET_ALL') {
    for (const key in [EMAIL, API_TOKEN]) {
      const value = await getData(key)
      if (value) {
        const message: PostMessage = {
          msg_type: 'SET',
          key: key as MessageKey,
          value
        }
        webview_ref.current?.postMessage(JSON.stringify(message))
      }
    }
  }
}