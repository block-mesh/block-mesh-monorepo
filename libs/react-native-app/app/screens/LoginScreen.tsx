import React, { useEffect, useRef, useState } from 'react'
import { Nav, useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { Button, TextInput, View } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import { ThemedText } from '@/components/ThemedText'

export default function LoginScreen() {
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
  }, [storage.email, storage.url, storage.password, storage.api_token, storage.nav])

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
        secureTextEntry={true}
        ref={passwordRef as any}
        style={styles.input}
        onChangeText={setPassword}
        value={password}
        placeholder="Fill password"
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
        placeholderTextColor={colors['off-white']}
        autoCapitalize={'none'}
      />
      <View style={styles.buttonContainer}>
        <Button
          title="Register"
          color="#f194ff"
          onPress={() => {
            storage.setNav('register')
          }}
        />
        <Button
          title="Login"
          color="#f194ff"
          onPress={() => {
            // stop()
          }}
        />
      </View>
    </ThemedView>
  )
}
