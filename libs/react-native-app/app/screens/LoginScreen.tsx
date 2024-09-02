import React, { useEffect, useRef } from 'react'
import { useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { Alert, TextInput } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import CustomButton from '@/components/CustomButton'
import { get_token } from '@/utils/auth'
import VerticalContainer from '@/components/VerticalContainer'

export default function LoginScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const storage = useStorage()
  useEffect(() => {
  }, [storage.email, storage.url, storage.password, storage.api_token, storage.nav])

  return (
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
        secureTextEntry={true}
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
              storage.setApiToken(r.value.api_token)
              storage.setNav('dashboard')
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
        <CustomButton
          title={'Go to Register'}
          buttonStyles={styles.button}
          buttonText={styles.buttonText}
          onPress={() => {
            storage.setNav('register')
          }}
        />
      </VerticalContainer>
    </ThemedView>
  )
}
