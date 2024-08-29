import React from 'react'
import { Text, StyleSheet, Pressable } from 'react-native'

// @ts-ignore
export default function CustomButton(props) {
  const { onPress, title = '' } = props
  return (
    <Pressable style={{ ...props?.buttonStyles }} onPress={onPress}>
      <Text style={{ ...props?.buttonText }}>{title}</Text>
    </Pressable>
  )
}


