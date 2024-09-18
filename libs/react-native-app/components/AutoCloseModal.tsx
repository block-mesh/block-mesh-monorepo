import React, { useState, useEffect } from 'react'
import { View, Modal, Text, Button, StyleSheet } from 'react-native'

const AutoCloseModal = () => {
  const [modalVisible, setModalVisible] = useState(false)

  const showModal = () => {
    setModalVisible(true)
    setTimeout(() => {
      setModalVisible(false)
    }, 3000) // Auto close after 3 seconds
  }

  return (
    <View style={styles.container}>
      <Button title="Show Message" onPress={showModal} />

      <Modal
        animationType="slide"
        transparent={true}
        visible={modalVisible}
        onRequestClose={() => {
          setModalVisible(false)
        }}
      >
        <View style={styles.modalView}>
          <Text>This is a popup message!</Text>
        </View>
      </Modal>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center'
  },
  modalView: {
    margin: 20,
    backgroundColor: 'white',
    borderRadius: 10,
    padding: 35,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {
      width: 0,
      height: 2
    },
    shadowOpacity: 0.25,
    shadowRadius: 4,
    elevation: 5
  }
})

export default AutoCloseModal
