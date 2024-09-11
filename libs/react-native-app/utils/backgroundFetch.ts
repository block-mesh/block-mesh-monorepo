import BackgroundFetch from 'react-native-background-fetch'
import { run_lib, RunLibInputs, stop_lib } from '@/utils/background'

const blockMeshTaskId = 'com.transistorsoft.blockmesh'
const TIMEOUT = 60 * 15 * 1000

export async function init_background({ url, email, password }: RunLibInputs) {
  console.log('starting init_background')

  async function onEvent(taskId: string) {
    console.log('[BackgroundFetch] task: ', taskId)
    // Do your background work...
    // IMPORTANT:  You must signal to the OS that your task is complete.
    run_lib({ url, email, password })
    setTimeout(() => {
      stop_lib(url)
      BackgroundFetch.finish(taskId)
    }, TIMEOUT)
  }

  async function onTimeout(taskId: string) {
    console.warn('[BackgroundFetch] TIMEOUT task: ', taskId)
    stop_lib(url)
    BackgroundFetch.finish(taskId)
  }

  // Initialize BackgroundFetch only once when component mounts.
  console.log('pre configure')
  await BackgroundFetch.configure({ minimumFetchInterval: 15 }, onEvent, onTimeout)
  console.log('post configure')
  await BackgroundFetch.scheduleTask({
    taskId: blockMeshTaskId,
    forceAlarmManager: true,
    periodic: true,
    delay: 5000  // <-- milliseconds
  }).catch(e => {
    console.error('scheduleTask error', e)
  })
  console.log('post scheduleTask')
}