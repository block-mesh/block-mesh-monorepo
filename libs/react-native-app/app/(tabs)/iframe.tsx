import { WebView } from 'react-native-webview'
import { BackHandler, Platform, StyleSheet, View } from 'react-native'
import { useEffect, useRef, useState } from 'react'
import { getData, storeData } from '@/utils/storage'
import { API_TOKEN, EMAIL } from '@/utils/constants'


const styles = StyleSheet.create({
  container: {
    flex: 1
  },
  webview: {
    flex: 1
  }
})

export default function IFrameWrapper() {
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


  async function handleMessage(event: any): Promise<void> {
    console.log('Message from WebView:', { data: event.nativeEvent.data })
    const data = event.nativeEvent.data
    const { msg_type, key, value } = data
    if (msg_type === 'SET' && key) {
      if ([EMAIL, API_TOKEN].includes(key)) {
        await storeData(key, value)
      }
    } else if (msg_type === 'GET' && key) {
      if ([EMAIL, API_TOKEN].includes(key)) {
        const value = await getData(key)
        if (value) {
          webview_ref.current?.postMessage(JSON.stringify({ [key]: value }))
        }
      }
    } else if (msg_type === 'GET_ALL') {
      for (const key in [EMAIL, API_TOKEN]) {
        const value = await getData(key)
        if (value) {
          webview_ref.current?.postMessage(JSON.stringify({ [key]: value }))
        }
      }
    }
  }

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
        onMessage={handleMessage}
        onLoad={() => {
          if (webview_ref.current === null) return
          console.log('on load')
          // webview_ref.current?.postMessage(JSON.stringify({ blockmesh_url: url }))
        }}
        originWhitelist={['*']}
        source={{ uri: 'http://localhost:8000/tauri/login' }}
        style={styles.webview}
        injectedJavaScript={injectedJavaScript}
        javaScriptEnabled={true}
      />
    </View>
  )
}