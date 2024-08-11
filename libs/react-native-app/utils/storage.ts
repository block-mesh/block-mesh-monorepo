import AsyncStorage from '@react-native-async-storage/async-storage'

export async function storeData(key: string, value: string): Promise<void> {
  try {
    console.log(`storeData: ${key} => ${value}`)
    await AsyncStorage.setItem(key, value)
  } catch (e) {
    console.error(`storeData Error: ${key} => ${value} | error ${e}`)
  }
}

export async function getData(key: string): Promise<string | null | undefined> {
  try {
    console.log(`getData : ${key}`)
    return await AsyncStorage.getItem(key)
  } catch (e) {
    console.error(`getData Error: ${key} | error ${e}`)
  }
}