import React, { useEffect, useRef, useState } from 'react'
import { Nav, useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { View } from 'react-native'
import { Button, TextInput } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import { ThemedText } from '@/components/ThemedText'

export default function RegisterScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const passworConfirmdRef = useRef()
  const urlRef = useRef()
  const storage = useStorage()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [passwordConfirm, setPasswordConfirm] = useState('')
  const [url, setUrl] = useState(storage.url)
  const [nav, setNav] = useState(storage.nav)

  useEffect(() => {
    setEmail(storage.email)
    setPassword(storage.password)
    setUrl(storage.url)
  }, [storage.email, storage.url, storage.password, storage.api_token])

  return (
    <ThemedView style={styles.stepContainer}>
      <ThemedText type="subtitle">Email</ThemedText>
      <TextInput
        ref={emailRef as any}
        style={styles.input}
        onChangeText={setEmail}
        value={email}
        placeholder="Fill email"
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <ThemedText type="subtitle">Password</ThemedText>
      <TextInput
        ref={passwordRef as any}
        secureTextEntry={true}
        style={styles.input}
        onChangeText={setPassword}
        value={password}
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
      <ThemedText type="subtitle">URL</ThemedText>
      <TextInput
        ref={urlRef as any}
        style={styles.input}
        onChangeText={setUrl}
        value={url}
        placeholder="Fill URL"
        autoCapitalize={'none'}
      />
      <View style={styles.buttonContainer}>
        <Button
          title="Login"
          color="#f194ff"
          onPress={() => {
            storage.setNav('login')
          }}
        />
        <Button
          title="Register"
          color="#f194ff"
          onPress={() => {
            // stop()
          }}
        />
      </View>
    </ThemedView>
  )
}
