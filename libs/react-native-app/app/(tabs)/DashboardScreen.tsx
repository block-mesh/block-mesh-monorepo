import React, { useEffect, useState } from 'react'
import ParallaxScrollView from '@/components/ParallaxScrollView'
import { Alert, Image } from 'react-native'
import { useStorage } from '@/hooks/useStorage'
import { ThemedText } from '@/components/ThemedText'
import { ThemedView } from '@/components/ThemedView'
import { colors, styles } from '@/utils/theme'
import CustomButton from '@/components/CustomButton'
import { check_token, dashboard } from '@/utils/auth'
import { DashboardResponse } from '@/utils/apiTypes'
import { useInterval } from '@/hooks/useInterval'
import { get_lib_status, run_lib, stop_lib } from '@/utils/background'
import { ActivityIndicator } from 'react-native'
import { Switch, Case, Default } from 'react-if'
import VerticalContainer from '@/components/VerticalContainer'
import * as Location from 'expo-location'
import { router } from 'expo-router'

export default function DashboardScreen() {
  const storage = useStorage()
  const [data, setData] = useState<DashboardResponse>()
  const [valid_token, setValidToken] = useState(false)
  const [status, setStatus] = useState(-1)
  const [stringStatus, setStringStatus] = useState('Click to Turn On')
  const [disableToggleButton, setDisableToggleButton] = useState(false)
  const [location, setLocation] = useState('')


  useEffect(() => {
    (async () => {
      let { status: fg_status } = await Location.requestForegroundPermissionsAsync()
      if (fg_status !== 'granted') {
        return
      }

      let { status: bg_status } = await Location.requestBackgroundPermissionsAsync()
      if (bg_status !== 'granted') {
        return
      }

      let location = await Location.getCurrentPositionAsync({})
      const address = await Location.reverseGeocodeAsync(location.coords)
      if (address.length > 0) {
        const add = address[0]
        setLocation(`${add.city}`)
      }
    })()
  }, [])


  useEffect(() => {
    if (status === 1) {
      setStringStatus('Click to Turn Off')
    } else {
      setStringStatus('Click to Turn On')
    }
  }, [status])

  async function get_dashboard() {
    if (!storage.url || !storage.email || !storage.api_token) {
      return
    }
    const response = await dashboard(storage.url + '/api/dashboard', {
      email: storage.email,
      api_token: storage.api_token
    })
    console.log('response', response)
    if (response.isOk) {
      setData(response.unwrap())
    }
  }

  useEffect(
    () => {
      (async () => {
        if (!storage.url || !storage.email || !storage.api_token) {
          return
        }
        setStatus(get_lib_status())
        const token_response = await check_token(storage.url + '/api/check_token', {
          email: storage.email,
          api_token: storage.api_token
        })
        if (token_response.isOk) {
          setValidToken(true)
          await get_dashboard()
        }
      })()
    }, [storage.url, storage.email, storage.api_token])

  useInterval(async () => {
    console.log('valid_token', valid_token)
    setStatus(get_lib_status())
    if (valid_token) {
      await get_dashboard()
    }
  }, 15_000)

  return (
    <ParallaxScrollView
      headerBackgroundColor={{ light: colors['dark-blue'], dark: colors['dark-blue'] }}
      headerImage={
        <Image
          source={{ uri: 'https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/3ef1afb4-e176-4423-7bd3-3eed38102b00/public' }}
          style={styles.logo}
        />
      }>
      <ThemedView style={styles.stepContainer}>
        <VerticalContainer>
          <Switch>
            <Case condition={data?.connected && status === 1}>
              <VerticalContainer>
                <ThemedText type="subtitle">Connected</ThemedText>
                <ActivityIndicator size="large" color="#00ff00" />
              </VerticalContainer>
            </Case>
            <Default>
              <ThemedText type="subtitle">Not connected</ThemedText>
            </Default>
          </Switch>
          <ThemedText type="subtitle">{location}</ThemedText>
          <CustomButton
            disabled={disableToggleButton}
            title={`${stringStatus}`}
            buttonStyles={styles.button}
            buttonText={styles.buttonText}
            onPress={
              () => {
                if (disableToggleButton) {
                  return
                }
                setDisableToggleButton(true)
                setTimeout(() => {
                  setDisableToggleButton(false)
                }, 500)
                if (status !== 1) {
                  if (!valid_token) {
                    Alert.alert(
                      'Error',
                      'Please logout and re-login',
                      [
                        { text: 'OK', onPress: () => console.log('OK Pressed') }
                      ],
                      { cancelable: false }
                    )
                    return
                  }
                  run_lib({
                    url: storage.url,
                    email: storage.email,
                    password: storage.password
                  })
                } else {
                  stop_lib(storage.url)
                }
                setTimeout(() => {
                  setStatus(get_lib_status())
                }, 500)
              }
            }
          />
          <CustomButton
            title={'Logout'}
            buttonStyles={styles.button}
            buttonText={styles.buttonText}
            onPress={async () => {
              stop_lib(storage.url)
              storage.clear()
              router.replace('/(tabs)/')
            }}
          />
        </VerticalContainer>
      </ThemedView>
    </ParallaxScrollView>

  )
}
