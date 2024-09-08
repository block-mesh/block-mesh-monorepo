import React from 'react'
import { View, Text, StyleSheet, type ViewProps } from 'react-native'

const VerticalContainer = ({ style, ...otherProps }: ViewProps) => {
  return (
    <View style={styles.container} {...otherProps} />
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'column', // Stack children vertically
    justifyContent: 'center', // Center them vertically
    alignItems: 'center' // Center them horizontally
  },
  box: {
    padding: 20,
    backgroundColor: 'lightblue',
    margin: 10
  }
})

export default VerticalContainer
