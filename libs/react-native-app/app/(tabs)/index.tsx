import { Image, StyleSheet, Alert, TextInput, Button } from 'react-native'
import { HelloWave } from '@/components/HelloWave'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { ThemedText } from '@/components/ThemedText'
import { ThemedView } from '@/components/ThemedView'
import { useStorage } from '@/hooks/useStorage'
import React, { useEffect, useRef, useState } from 'react'
import BackgroundService from 'react-native-background-actions'
import MyRustModule from '@/modules/my-rust-module/src/MyRustModule'

async function sleep(time: number): Promise<void> {
  new Promise<void>((resolve) => setTimeout(() => resolve(), time))
}

const options = {
  taskName: 'BlockMesh Network',
  taskTitle: 'BlockMesh Network Node',
  taskDesc: 'Running BlockMesh Network node',
  taskIcon: {
    name: 'ic_launcher',
    type: 'mipmap'
  },
  color: '#ff00ff',
  parameters: {
    delay: 1000
  }
}

export default function HomeScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const urlRef = useRef()
  const storage = useStorage()
  const [email, setEmail] = useState(storage.email)
  const [password, setPassword] = useState(storage.password)
  const [url, setUrl] = useState(storage.url)

  useEffect(() => {
    setEmail(storage.email)
    setPassword(storage.password)
    setUrl(storage.url)
  }, [storage.email, storage.url, storage.password, storage.api_token])

  async function run_lib(): Promise<void> {
    console.log('starting run_lib')
    await new Promise<void>(async (resolve): Promise<void> => {
      MyRustModule.run_lib(url, email, password).then(() => {
        console.log('run_lib finished')
      }, () => {
        console.log('run_lib error')
      })
      console.log('after run_lib')
      resolve()
    })
    console.log('finished run_lib')
  }

  async function click() {
    if (url === '' || email === '' || password === '') {
      Alert.alert('Error', 'Please set URL/EMAIL/PASSWORD', [
        {
          text: 'OK',
          style: 'cancel'
        }
      ])
    } else {
      await BackgroundService.start(run_lib, options)
    }
  }

  async function stop() {
    console.log('start stop 0')
    const r = await fetch('https://distinct-bison-merely.ngrok-free.app/health_check?hello=https&from=ui').then(() => {
      console.log('success')
    }, (e) => console.log('error', e))
    await BackgroundService.stop()
    console.log('hello', MyRustModule.hello())
    const x = await MyRustModule.stop_lib()
    console.log('x = ', x)
    Alert.alert('INFO', 'Node stopped', [
      {
        text: 'OK'
      }
    ])
    console.log('finished stop')
  }

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
