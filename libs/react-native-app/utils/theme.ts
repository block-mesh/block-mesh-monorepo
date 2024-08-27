import { StyleSheet } from 'react-native'

export const colors = {
  'dark-blue': '#0C1120',
  'magenta': '#DA00DB',
  'magenta-1': '#FE49FF',
  'magenta-2': '#E426E6',
  'magenta-3': '#A700A8',
  'magenta-4': '#670068',
  'cyan': '#01FEFC',
  'orange': '#FF7E07',
  'off-white': '#EDEDED',
  'light-blue': '#25e4d3',
  darkBlue: '#2f8985',
  darkOrange: '#f97432',
  light: '#55555588',
  lightDark: '#88888811',
  dark: '#202525'
}

export const styles = StyleSheet.create({
  buttonContainer: {
    flexDirection: 'row',  // Places buttons side by side
    justifyContent: 'space-between',  // Adjusts the space between the buttons
    width: '80%'  // Adjust the width of the container as needed
  },
  titleContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8
  },
  stepContainer: {
    gap: 8,
    marginBottom: 8,
    backgroundColor: colors['dark-blue']
  },
  logo: {
    height: 178,
    width: 290,
    bottom: 0,
    left: 0,
    position: 'absolute'
  },
  input: {
    color: colors['off-white'],
    borderColor: colors['off-white'],
    borderRadius: 10,
    height: 40,
    margin: 12,
    borderWidth: 1,
    padding: 10
  }
})

