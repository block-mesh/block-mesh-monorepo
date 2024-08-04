import { WebView } from 'react-native-webview'
import { BackHandler, Image, Platform, StyleSheet, View } from 'react-native'
import React, { useEffect, useRef, useState } from 'react'
import { getData, storeData } from '@/utils/storage'
import { API_TOKEN, EMAIL } from '@/utils/constants'
import { handleMessage, PostMessage } from '@/utils/messages'


const styles = StyleSheet.create({
  container: {
    flex: 1
  },
  webview: {
    flex: 1
  },
  logo: {
    height: 178,
    width: 290,
    bottom: 0,
    left: 0,
    position: 'absolute'
  }
})

export default function ConnectPage() {
  const webview_ref = useRef<WebView>(null)
  const [email, setEmail] = useState<string>()
  const [apiToken, setApiToken] = useState<string>()
  const [url, setUrl] = useState<string>('http://localhost:8000')

  useEffect(() => {
    (async () => {
      const e = await getData(EMAIL)
      if (e) {
        setEmail(e)
      }
      const token = await getData(API_TOKEN)
      if (token) {
        setApiToken(token)
      }

    })()
  }, [])


  const injectedJavaScript = `
      true; // note: this is required, or you'll sometimes get silent failures
    `
  const onAndroidBackPress = () => {
    if (webview_ref.current) {
      webview_ref.current.goBack()
      return true // prevent default behavior (exit app)
    }
    return false
  }

  useEffect(() => {
    if (Platform.OS === 'android') {
      BackHandler.addEventListener('hardwareBackPress', onAndroidBackPress)
      return () => {
        BackHandler.removeEventListener('hardwareBackPress', onAndroidBackPress)
      }
    }
  }, [])


  return (
    <View style={styles.container}>
      <WebView
        allowsBackForwardNavigationGestures={true}
        ref={webview_ref}
        onMessage={(e) => handleMessage(e, webview_ref)}
        onLoad={() => {
          if (webview_ref.current === null) return
          setTimeout(() => {
            console.log('on load')
            const message: PostMessage = {
              msg_type: 'SET',
              key: 'blockmesh_url',
              value: url
            }
            webview_ref.current?.postMessage(JSON.stringify(message))
          }, 500)
        }}
        originWhitelist={['*']}
        source={{ uri: `${url}/tauri/login` }}
        style={styles.webview}
        injectedJavaScript={injectedJavaScript}
        javaScriptEnabled={true}
        injectedJavaScriptBeforeContentLoaded={`
         if (window?.webkit?.messageHandlers?.ReactNativeWebView && !window.message_channel_port) {
            window.message_channel_port = window?.webkit?.messageHandlers?.ReactNativeWebView;
         }
        true`}
      />
    </View>
  )
}