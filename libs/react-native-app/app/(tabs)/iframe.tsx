import { WebView } from 'react-native-webview'
import { Button, StyleSheet, View } from 'react-native'
import { useMemo, useRef } from 'react'

const styles = StyleSheet.create({
  container: {
    flex: 1
  },
  webview: {
    flex: 1
  }
})

export default function IFrameWrapper() {
  const webref = useRef<WebView>(null)

  console.log('iframewrapper loaded')
  const handleMessage = (event: any) => {
    console.log('Message from WebView:', { data: event.nativeEvent.data })
    // alert(event.nativeEvent.data)
  }

  const injectedJavaScript = `
      setTimeout(function() { window.alert('hi') }, 2000);
      true; // note: this is required, or you'll sometimes get silent failures
    `

  const run = `
      setTimeout(function() { window.alert('bye') }, 2000);
      true;
    `

  setTimeout(() => {
    if (webref.current === null) return
    // webref.current.injectJavaScript(run)
    setTimeout(() => {
      webref.current?.postMessage('sending!')
    }, 1_000)
  }, 3000)

  return (
    <View style={styles.container}>
      <WebView
        ref={webref}
        onMessage={handleMessage}
        onLoad={() => {
          console.log('on load')
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