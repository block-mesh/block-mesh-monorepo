import React, { useEffect, useRef, useState } from 'react'
import { Nav, useStorage } from '@/hooks/useStorage'
import { colors, styles } from '@/utils/theme'
import { Button, TextInput, View } from 'react-native'
import { ThemedView } from '@/components/ThemedView'
import { ThemedText } from '@/components/ThemedText'
import CustomButton from '@/components/CustomButton'
import { get_token, login } from '@/utils/auth'

export default function LoginScreen() {
  const emailRef = useRef()
  const passwordRef = useRef()
  const urlRef = useRef()
  const storage = useStorage()
  //
  useEffect(() => {
  }, [storage.email, storage.url, storage.password, storage.api_token, storage.nav])

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
        secureTextEntry={true}
        ref={passwordRef as any}
        style={styles.input}
        onChangeText={storage.setPassword}
        value={storage.password}
        placeholder="Fill password"
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
          onPress={() => {
            storage.setNav('register')
          }}
        />
        <CustomButton
          title={'Login'}
          buttonStyles={styles.button}
          buttonText={styles.buttonText}
          onPress={async () => {
            const r = await get_token(
              storage.url + '/api/get_token',
              {
                email: storage.email,
                password: storage.password
              })
            console.log('r = ', r)
            if (r.isOk) {
              storage.setApiToken(r.value.api_token)
              storage.setNav('dashboard')
            }
          }}
        />
      </View>
    </ThemedView>
  )
}
