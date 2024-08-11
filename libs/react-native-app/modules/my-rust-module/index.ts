import { NativeModulesProxy, EventEmitter, Subscription } from 'expo-modules-core'

// Import the native module. On web, it will be resolved to MyRustModule.web.ts
// and on native platforms to MyRustModule.ts
import MyRustModule from './src/MyRustModule'
import MyRustModuleView from './src/MyRustModuleView'
import { ChangeEventPayload, MyRustModuleViewProps } from './src/MyRustModule.types'

// Get the native constant value.
export const PI = MyRustModule.PI

export function hello(): string {
  return MyRustModule.hello()
}

export async function run_lib() {
  await MyRustModule.run_lib()
}

export async function stop_lib() {
  await MyRustModule.stop_lib()
}

export async function setValueAsync(value: string) {
  return await MyRustModule.setValueAsync(value)
}

const emitter = new EventEmitter(MyRustModule ?? NativeModulesProxy.MyRustModule)

export function addChangeListener(listener: (event: ChangeEventPayload) => void): Subscription {
  return emitter.addListener<ChangeEventPayload>('onChange', listener)
}

export { MyRustModuleView, MyRustModuleViewProps, ChangeEventPayload }
