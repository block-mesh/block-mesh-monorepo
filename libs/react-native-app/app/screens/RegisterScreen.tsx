import React, { useEffect, useRef, useState } from 'react'
import { Nav, useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { View } from 'react-native'
import { Alert } from 'react-native'
import { Button, TextInput } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import { ThemedText } from '@/components/ThemedText'
import CustomButton from '@/components/CustomButton'
import { register } from '@/utils/auth'

export default function RegisterScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const passworConfirmdRef = useRef()
  const urlRef = useRef()
  const storage = useStorage()
  const [invite_code, setInviteCode] = useState('')
  const [passwordConfirm, setPasswordConfirm] = useState('')

  useEffect(() => {
  }, [storage.email, storage.url, storage.password, storage.api_token])

  return (
    <ThemedView style={styles.stepContainer}>
      <ThemedText type="subtitle">Email</ThemedText>
      <TextInput
        ref={emailRef as any}
        style={styles.input}
        onChangeText={storage.setEmail}
        value={storage.email}
        placeholder="Fill email"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <ThemedText type="subtitle">Password</ThemedText>
      <TextInput
        ref={passwordRef as any}
        secureTextEntry={true}
        style={styles.input}
        onChangeText={storage.setPassword}
        value={storage.password}
        placeholder="Fill password"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <ThemedText type="subtitle">Password Confirm</ThemedText>
      <TextInput
        ref={passworConfirmdRef as any}
        secureTextEntry={true}
        style={styles.input}
        onChangeText={setPasswordConfirm}
        value={passwordConfirm}
        placeholder="Fill password confirmation"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <ThemedText type="subtitle">Invite Code</ThemedText>
      <TextInput
        ref={urlRef as any}
        style={styles.input}
        onChangeText={setInviteCode}
        value={invite_code}
        placeholder="Invite Code"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <ThemedText type="subtitle">URL</ThemedText>
      <TextInput
        ref={urlRef as any}
        style={styles.input}
        onChangeText={storage.setUrl}
        value={storage.url}
        placeholder="Fill URL"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <View style={styles.buttonContainer}>
        <CustomButton
          title={'Register'}
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
            const r = await register(storage.url + '/register', {
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
              storage.setNav('login')
            }

          }}
        />
        <CustomButton
          title={'Login'}
          buttonStyles={styles.button}
          buttonText={styles.buttonText}
          onPress={() => {
            storage.setNav('login')
          }}
        />

      </View>
    </ThemedView>
  )
}
