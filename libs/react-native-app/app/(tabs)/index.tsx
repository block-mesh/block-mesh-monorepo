import { Alert, Button, Image, StyleSheet, TextInput } from 'react-native'
import { Case, Default, Switch } from 'react-if'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { ThemedView } from '@/components/ThemedView'
import { Nav, useStorage } from '@/hooks/useStorage'
import React, { useEffect, useRef, useState } from 'react'
import { run_lib, stop_lib } from '@/utils/background'
import LoginScreen from '@/app/screens/LoginScreen'
import RegisterScreen from '@/app/screens/RegisterScreen'
import DashboardScreen from '@/app/screens/DashboardScreen'
import { colors, styles } from '@/utils/theme'

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
  const storage = useStorage()
  const [email, setEmail] = useState(storage.email)
  const [password, setPassword] = useState(storage.password)
  const [url, setUrl] = useState(storage.url)

  useEffect(() => {
    setEmail(storage.email)
    setPassword(storage.password)
    setUrl(storage.url)
  }, [storage.email, storage.url, storage.password, storage.api_token, storage.nav])

  async function click() {
    if (url === '' || email === '' || password === '') {
      Alert.alert('Error', 'Please set URL/EMAIL/PASSWORD', [
        {
          text: 'OK',
          style: 'cancel'
        }
      ])
    } else {
      run_lib({ url, email, password })
    }
  }

  async function stop() {
    stop_lib()
  }

  return (
    <ParallaxScrollView
      headerBackgroundColor={{ light: colors['dark-blue'], dark: colors['dark-blue'] }}
      headerImage={
        <Image
          source={{ uri: 'https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/3ef1afb4-e176-4423-7bd3-3eed38102b00/public' }}
          style={styles.logo}
        />
      }>
      <ThemedView style={styles.titleContainer}>
      </ThemedView>
      <Switch>
        <Case condition={storage.nav === 'login'}>
          <LoginScreen />
        </Case>
        <Case condition={storage.nav === 'register'}>
          <RegisterScreen />
        </Case>
        <Case condition={storage.nav === 'dashboard'}>
          <DashboardScreen />
        </Case>
      </Switch>
    </ParallaxScrollView>
  )
}


