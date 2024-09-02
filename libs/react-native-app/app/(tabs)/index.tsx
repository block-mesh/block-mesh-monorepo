import { Image } from 'react-native'
import { Case, Switch } from 'react-if'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { ThemedView } from '@/components/ThemedView'
import { useStorage } from '@/hooks/useStorage'
import React, { useEffect, useState } from 'react'
import LoginScreen from '@/app/screens/LoginScreen'
import RegisterScreen from '@/app/screens/RegisterScreen'
import DashboardScreen from '@/app/screens/DashboardScreen'
import { colors, styles } from '@/utils/theme'

export default function HomeScreen() {
  const storage = useStorage()
  const [_email, setEmail] = useState(storage.email)
  const [_password, setPassword] = useState(storage.password)
  const [_url, setUrl] = useState(storage.url)

  useEffect(() => {
    setEmail(storage.email)
    setPassword(storage.password)
    setUrl(storage.url)
  }, [storage.email, storage.url, storage.password, storage.api_token, storage.nav])

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


