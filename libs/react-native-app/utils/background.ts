import MyRustModule from '@/modules/my-rust-module/src/MyRustModule'

export interface RunLibInputs {
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

export function stop_lib(url: string): string {
  return MyRustModule.stop_lib(url)
}

export function get_lib_status(): number {
  return MyRustModule.get_lib_status()
}