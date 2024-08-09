import { Image, StyleSheet, Platform, TextInput, Button } from 'react-native'
import { HelloWave } from '@/components/HelloWave'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { ThemedText } from '@/components/ThemedText'
import { ThemedView } from '@/components/ThemedView'
import { useStorage } from '@/hooks/useStorage'
import React, { useRef, useState } from 'react'
import RustModule from '@/native/native'


export default function HomeScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const urlRef = useRef()
  const storage = useStorage()
  const [email, setEmail] = useState(storage.email)
  const [password, setPassword] = useState(storage.password)
  const [url, setUrl] = useState(storage.url)

  function click() {
    console.log(RustModule.runLib('http://localhost:8000', 'ohaddahan@gmail.com', 'dudedude@'))
  }

  function stop() {
    console.log('Stop lib', Date.now())
    console.log(RustModule.stopLib())
  }


  // @ts-ignore
  return (
    <ParallaxScrollView
      headerBackgroundColor={{ light: '#0C1120', dark: '#0C1120' }}
      headerImage={
        <Image
          source={{ uri: 'https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/3ef1afb4-e176-4423-7bd3-3eed38102b00/public' }}
          style={styles.logo}
        />
      }>
      <ThemedView style={styles.titleContainer}>
        <ThemedText type="title">Welcome!</ThemedText>
        <HelloWave />
      </ThemedView>
      <ThemedView style={styles.stepContainer}>
        <ThemedText type="subtitle">Email</ThemedText>
        <TextInput
          ref={emailRef}
          style={styles.input}
          onChangeText={setEmail}
          value={email}
          placeholder="Fill email"
          autoCapitalize={false}
        />
        <ThemedText type="subtitle">Password</ThemedText>
        <TextInput
          ref={passwordRef}
          style={styles.input}
          onChangeText={setPassword}
          value={password}
          placeholder="Fill password"
          autoCapitalize={false}
        />
        <ThemedText type="subtitle">URL</ThemedText>
        <TextInput
          ref={urlRef}
          style={styles.input}
          onChangeText={setUrl}
          value={url}
          placeholder="Fill URL"
          autoCapitalize={false}
        />
        <Button
          title="Run"
          color="#f194ff"
          onPress={() => {
            storage.setEmail(email)
            storage.setPassword(password)
            storage.setUrl(url)
            click()
          }}
        />
        <Button
          title="Stop"
          color="#f194ff"
          onPress={() => {
            stop()
          }}
        />
      </ThemedView>
    </ParallaxScrollView>
  )
}

const styles = StyleSheet.create({
  titleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8
  },
  stepContainer: {
    gap: 8,
    marginBottom: 8
  },
  logo: {
    height: 178,
    width: 290,
    bottom: 0,
    left: 0,
    position: 'absolute'
  },
  input: {
    height: 40,
    margin: 12,
    borderWidth: 1,
    padding: 10
  }
})
