import MyRustModule from '@/modules/my-rust-module/src/MyRustModule'

interface RunLibInputs {
  url: string;
  email: string;
  password: string;
}

export async function sleep(time: number): Promise<void> {
  new Promise<void>((resolve) => setTimeout(() => resolve(), time))
}

export function run_lib({ url, email, password }: RunLibInputs) {
  MyRustModule.run_lib(url, email, password).then(() => {
      console.log('run_lib finished')
    },
    () => {
      console.log('run_lib error')
    })
}

export function stop_lib() {
  MyRustModule.stop_lib()
}