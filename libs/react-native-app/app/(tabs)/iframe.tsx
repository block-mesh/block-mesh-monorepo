import {WebView} from 'react-native-webview';
import {StyleSheet, View} from 'react-native';

const styles = StyleSheet.create({
    container: {
        flex: 1,
    },
    webview: {
        flex: 1,
    },
});

export default function IFrameWrapper() {
    const injectedJavaScript = ``;

    return (
        <View style={styles.container}>
            <WebView
                originWhitelist={['*']}
                source={{uri: 'http://localhost:8000/tauri/login'}}
                style={styles.webview}
                injectedJavaScript={injectedJavaScript}
                javaScriptEnabled={true}
            />
        </View>
    )
}