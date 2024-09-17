import React from 'react'
import { View, Text, StyleSheet } from 'react-native'
import { TabBarIcon } from '@/components/navigation/TabBarIcon'
import { Case, Default, Switch } from 'react-if'
import { DECLARATION_TYPES } from '@babel/types'

export type StatusWithIconProps = {
  status: boolean;
  text: string;
}

const StatusWithIcon = ({ status, text }: StatusWithIconProps) => {
  return (
    <View style={styles.container}>
      <View style={styles.statusContainer}>
        <Switch>
          <Case condition={status}>
            <Text style={styles.statusText}>{text}</Text>
            <TabBarIcon name="checkbox-outline" size={20} color="green" />
          </Case>
          <Default>
            <Text style={styles.statusText}>Not {text}</Text>
            <TabBarIcon name="close" size={20} color="red" />
          </Default>
        </Switch>
      </View>
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'center',
    marginTop: 3,
    marginBottom: 3
  },
  statusContainer: {
    justifyContent: 'space-between',
    flexDirection: 'row',
    alignItems: 'center',
    width: '80%'
  },
  statusText: {
    color: 'white',
    marginLeft: 10,
    fontSize: 16
  }
})

export default StatusWithIcon
