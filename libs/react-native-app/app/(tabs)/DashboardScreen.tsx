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
import { init_background } from '@/utils/backgroundFetch'

export default function DashboardScreen() {
  const storage = useStorage()
  const [data, setData] = useState<DashboardResponse>()
  const [status, setStatus] = useState(-1)
  const [stringStatus, setStringStatus] = useState('Please login first')
  const [disableToggleButton, setDisableToggleButton] = useState(false)
  const [location, setLocation] = useState('')
  const [bgInit, setBgInit] = useState(false)
  const [grantedLocation, setGrantedLocation] = useState(false)
  const [canStart, setCanStart] = useState(false)

  useEffect(() => {
    (async () => {
      if (storage.url.length !== 0 && storage.email.length !== 0 && storage.password.length !== 0 && storage.api_token.length !== 0) {
        let r = await check_token(storage.url + '/api/check_token', {
          email: storage.email,
          api_token: storage.api_token
        })
        if (r.isOk) {
          setCanStart(true)
          setStringStatus('Click to turn on')
        }
      } else {
        setCanStart(false)
      }
    })()
  }, [storage.url, storage.password, storage.email, storage.api_token])

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
      setGrantedLocation(true)

      let location = await Location.getCurrentPositionAsync({})
      const address = await Location.reverseGeocodeAsync(location.coords)
      if (address.length > 0) {
        const add = address[0]
        setLocation(`${add.city}`)
      }
    })()
  }, [])

  useInterval(async () => {
    if (!grantedLocation) {
      return
    }
    let location = await Location.getCurrentPositionAsync({})
    const address = await Location.reverseGeocodeAsync(location.coords)
    if (address.length > 0) {
      const add = address[0]
      setLocation(`${add.city}`)
    }
  }, 15_000)

  useInterval(() => {
    // Ping FFI for status
    setStatus(get_lib_status())
  }, 3_000)

  async function get_dashboard() {
    if (storage.url.length === 0 || storage.email.length === 0 || storage.api_token.length === 0) {
      return
    }
    const response = await dashboard(storage.url + '/api/dashboard', {
      email: storage.email,
      api_token: storage.api_token
    })
    if (response.isOk) {
      setData(response.unwrap())
    }
  }

  useEffect(() => {
    (async () => {
      if (storage.url.length === 0 || storage.email.length === 0 || storage.api_token.length === 0) {
        return
      }
      if (!bgInit) {
        await init_background({
          url: storage.url,
          email: storage.email,
          password: storage.password
        })
        setBgInit(true)
      }
    })()
  }, [storage.url, storage.email, storage.password])

  useInterval(async () => {
    if (canStart) {
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
                if (!storage.url || !storage.email || !storage.password) {
                  return
                }
                if (get_lib_status() === 1) {
                  stop_lib(storage.url)
                } else {

                  run_lib({
                    url: storage.url,
                    email: storage.email,
                    password: storage.password
                  })
                  setStringStatus('Click to turn off')
                }
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
