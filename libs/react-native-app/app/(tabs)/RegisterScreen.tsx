import React, { useEffect, useRef, useState } from 'react'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { Alert, Image } from 'react-native'
import { TextInput } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import CustomButton from '@/components/CustomButton'
import { register } from '@/utils/auth'
import VerticalContainer from '@/components/VerticalContainer'
import { router, useFocusEffect } from 'expo-router'
import AutoCloseModal from '@/components/AutoCloseModal'
import { Case, Default, Switch } from 'react-if'
import { ThemedText } from '@/components/ThemedText'

export default function RegisterScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const inviteRef = useRef()
  const passwordConfirmed = useRef()
  const storage = useStorage()
  const [invite_code, setInviteCode] = useState('')
  const [passwordConfirm, setPasswordConfirm] = useState('')
  const secure = storage.env() === 'production'
  const [redirect, setRedirect] = useState(false)

  useFocusEffect(() => {
    if (storage.api_token) {
      setRedirect(true)
      setTimeout(() => {
        router.replace('/(tabs)/DashboardScreen')
      }, 1_500)
    } else {
      setRedirect(false)
    }
  })


  return (
    <ParallaxScrollView
      headerBackgroundColor={{ light: colors['dark-blue'], dark: colors['dark-blue'] }}
      headerImage={
        <Image
          source={{ uri: 'https://r2-images.blockmesh.xyz/3ef1afb4-e176-4423-7bd3-3eed38102b00.png' }}
          style={styles.logo}
        />
      }>
      <ThemedView style={styles.stepContainer}>
        <Switch>
          <Case condition={redirect}>
            <VerticalContainer>
              <ThemedText type="title">Already logged in</ThemedText>
            </VerticalContainer>
          </Case>
          <Default>


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
              ref={passwordRef as any}
              secureTextEntry={secure}
              style={styles.input}
              onChangeText={storage.setPassword}
              value={storage.password}
              placeholder="Fill password"
              placeholderTextColor={colors['off-white']}
              autoCapitalize={'none'}
            />
            <TextInput
              ref={passwordConfirmed as any}
              secureTextEntry={secure}
              style={styles.input}
              onChangeText={setPasswordConfirm}
              value={passwordConfirm}
              placeholder="Fill password confirmation"
              placeholderTextColor={colors['off-white']}
              autoCapitalize={'none'}
            />
            <TextInput
              ref={inviteRef as any}
              style={styles.input}
              onChangeText={setInviteCode}
              value={invite_code}
              placeholder="Invite Code"
              placeholderTextColor={colors['off-white']}
              autoCapitalize={'none'}
            />
            <VerticalContainer>
              <CustomButton
                title={'Submit'}
                buttonStyles={styles.button}
                buttonText={styles.buttonText}
                onPress={async () => {
                  if (invite_code.match(/^\s*$/)) {
                    Alert.alert(
                      'Error',
                      'Please fill invite code',
                      [
                        { text: 'OK', onPress: () => console.log('OK Pressed') }
                      ],
                      { cancelable: false }
                    )
                    return
                  }
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
                  if (passwordConfirm.match(/^\s*$/)) {
                    Alert.alert(
                      'Error',
                      'Please fill password confirm',
                      [
                        { text: 'OK', onPress: () => console.log('OK Pressed') }
                      ],
                      { cancelable: false }
                    )
                    return
                  }
                  if (storage.password !== passwordConfirm) {
                    Alert.alert(
                      'Error',
                      'Password and password confirm do not match',
                      [
                        { text: 'OK', onPress: () => console.log('OK Pressed') }
                      ],
                      { cancelable: false }
                    )
                    return
                  }
                  const r = await register(storage.url + '/register_api', {
                    email: storage.email,
                    password: storage.password,
                    password_confirm: passwordConfirm,
                    invite_code: invite_code
                  })
                  if (r.isOk && r.unwrap().status_code < 400) {
                    Alert.alert(
                      'Success',
                      'Successfully registered, please login with same credentials',
                      [
                        { text: 'OK', onPress: () => console.log('OK Pressed') }
                      ],
                      { cancelable: false }
                    )
                  }
                }}
              />
            </VerticalContainer>
          </Default>
        </Switch>
      </ThemedView>
    </ParallaxScrollView>
  )
}
