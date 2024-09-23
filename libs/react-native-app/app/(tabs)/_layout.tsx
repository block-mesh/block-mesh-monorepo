import { router, Tabs } from 'expo-router'
import React, { useEffect } from 'react'

import { TabBarIcon } from '@/components/navigation/TabBarIcon'
import { Colors } from '@/constants/Colors'
import { useColorScheme } from '@/hooks/useColorScheme'
import { colors } from '@/utils/theme'
import { useStorage } from '@/hooks/useStorage'

export default function TabLayout() {
  const colorScheme = useColorScheme()


  return (
    <Tabs
      screenOptions={{
        tabBarActiveTintColor: Colors[colorScheme ?? 'dark'].tint,
        headerShown: false
      }}
    >
      <Tabs.Screen
        name="index"
        options={{
          tabBarStyle: { backgroundColor: colors['dark-blue'] },
          title: 'Login',
          tabBarIcon: ({ color, focused }) => (
            <TabBarIcon name={focused ? 'log-in' : 'log-in-outline'} color={color} />
          )
        }}
      />
      <Tabs.Screen
        name="RegisterScreen"
        options={{
          tabBarStyle: { backgroundColor: colors['dark-blue'] },
          title: 'Register',
          tabBarIcon: ({ color, focused }) => (
            <TabBarIcon name={focused ? 'person-add' : 'person-add-outline'} color={color} />
          )
        }}
      />
      <Tabs.Screen
        name="DashboardScreen"
        options={{
          tabBarStyle: { backgroundColor: colors['dark-blue'] },
          title: 'Dashboard',
          tabBarIcon: ({ color, focused }) => (
            <TabBarIcon name={focused ? 'bar-chart' : 'bar-chart-outline'} color={color} />
          )
        }}
      />
    </Tabs>
  )
}
