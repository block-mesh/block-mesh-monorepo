import React, { useState } from 'react'
import { View, Text, Button } from 'react-native'
import { useStorage } from '@/hooks/useStorage'

export default function DashboardScreen() {
  const storage = useStorage()
  const [nav, setNave] = useState(storage.nav)
  const [isToggled, setIsToggled] = useState(false)

  return (
    <View>
      {/*<Text>Dashboard Screen</Text>*/}
      {/*<Button title={isToggled ? 'ON' : 'OFF'} onPress={() => setIsToggled(!isToggled)} />*/}
    </View>
  )
}
