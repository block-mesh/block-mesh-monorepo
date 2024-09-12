import React, { useEffect, useRef } from 'react'
import { useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { Alert, Image, TextInput } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import CustomButton from '@/components/CustomButton'
import { get_token } from '@/utils/auth'
import VerticalContainer from '@/components/VerticalContainer'
import { router } from 'expo-router'
import ParallaxScrollView from '@/components/ParallaxScrollView'


export default function Index() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const storage = useStorage()
  const secure = storage.env() === 'production'

  return (
    <ParallaxScrollView
      headerBackgroundColor={{ light: colors['dark-blue'], dark: colors['dark-blue'] }}
      headerImage={
        <Image
          source={{ uri: 'https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/3ef1afb4-e176-4423-7bd3-3eed38102b00/public' }}
          style={styles.logo}
        />
      }>
      <ThemedView style={styles.stepContainer}>
        <TextInput
          ref={emailRef as any}
          style={styles.input}
          onChangeText={storage.setEmail}
          value={storage.email}
          placeholder="Fill email"
          placeholderTextColor={colors['off-white']}
          autoCapitalize={'none'}
        />
        <TextInput
          secureTextEntry={secure}
          ref={passwordRef as any}
          style={styles.input}
          onChangeText={storage.setPassword}
          value={storage.password}
          placeholder="Fill password"
          placeholderTextColor={colors['off-white']}
          autoCapitalize={'none'}
        />
        <VerticalContainer>
          <CustomButton
            title={'Submit'}
            buttonStyles={styles.button}
            buttonText={styles.buttonText}
            onPress={async () => {
              if (storage.email.match(/^\s*$/)) {
                Alert.alert(
                  'Error',
                  'Please fill in email',
                  [
                    { text: 'OK', onPress: () => console.log('OK Pressed') }
                  ],
                  { cancelable: false }
                )
                return
              }
              if (storage.password.match(/^\s*$/)) {
                Alert.alert(
                  'Error',
                  'Please fill password',
                  [
                    { text: 'OK', onPress: () => console.log('OK Pressed') }
                  ],
                  { cancelable: false }
                )
                return
              }
              const r = await get_token(
                storage.url + '/api/get_token',
                {
                  email: storage.email,
                  password: storage.password
                })
              if (r.isOk) {
                storage.setApiToken(r.unwrap().api_token)
                router.replace('/(tabs)/DashboardScreen')
              } else {
                Alert.alert(
                  'Error',
                  'Failed to login',
                  [
                    { text: 'OK', onPress: () => console.log('OK Pressed') }
                  ],
                  { cancelable: false }
                )
              }
            }}
          />
        </VerticalContainer>
      </ThemedView>
    </ParallaxScrollView>
  )
}
