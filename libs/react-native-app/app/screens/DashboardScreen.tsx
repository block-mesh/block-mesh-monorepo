import React, { useState } from 'react'
import { View, Text, Button, TextInput } from 'react-native'
import { useStorage } from '@/hooks/useStorage'
import { ThemedText } from '@/components/ThemedText'
import { ThemedView } from '@/components/ThemedView'
import { colors, styles } from '@/utils/theme'
import CustomButton from '@/components/CustomButton'
import { get_token } from '@/utils/auth'

export default function DashboardScreen() {
  const storage = useStorage()
  const [nav, setNave] = useState(storage.nav)
  const [isToggled, setIsToggled] = useState(false)

  return (
    <ThemedView style={styles.stepContainer}>
      <ThemedText type="subtitle">Dashboard</ThemedText>
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
