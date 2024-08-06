import * as React from 'react'
import {
  createContext,
  FC,
  PropsWithChildren,
  useContext, useEffect,
  useState
} from 'react'
import { getData, storeData } from '@/utils/storage'
import { API_TOKEN, BLOCKMESH_URL, EMAIL } from '@/utils/constants'


export interface StorageType {
  email: string;
  api_token: string;
  url: string;
  setEmail: (email: string) => void;
  setApiToken: (api_token: string) => void;
  setUrl: (url: string) => void;
}

export const Context = createContext<StorageType>(
  {} as StorageType
)

export const useStorage =
  (): StorageType => {
    return useContext(Context)
  }


export const StorageProvider: FC<PropsWithChildren<any>> = ({
                                                              children
                                                            }) => {
  const [email, setEmailInternal] = useState('')
  const [api_token, setApiTokenInternal] = useState('')
  const [url, setUrlInternal] = useState('')

  useEffect(() => {
    (async () => {
      const e = await getData(EMAIL)
      if (e) {
        setEmailInternal(e)
      }
      const token = await getData(API_TOKEN)
      if (token) {
        setApiTokenInternal(token)
      }
      const url = await getData(BLOCKMESH_URL)
      if (url) {
        setUrlInternal(url)
      }
    })()
  }, [])

  function setEmail(email: string) {
    try {
      storeData(EMAIL, email).then(() => {
        setEmailInternal(email)
      }).catch((e) => {
        console.error(`setEmail:: email = '${email} , error = '${e}`)
      })
    } catch (e: any) {
      console.error(`setEmail:: email = '${email} , error = '${e}`)
    }
  }

  function setUrl(url: string) {
    try {
      storeData(BLOCKMESH_URL, url).then(() => {
        setUrlInternal(url)
      }).catch((e) => {
        console.error(`setUrl:: url = '${url} , error = '${e}`)
      })
    } catch (e: any) {
      console.error(`setUrl:: url = '${url} , error = '${e}`)
    }
  }

  function setApiToken(api_token: string) {
    try {
      storeData(EMAIL, api_token).then(() => {
        setApiTokenInternal(api_token)
      }).catch((e) => {
        console.error(`setApiToken:: api_token = '${api_token} , error = '${e}`)
      })
    } catch (e: any) {
      console.error(`setApiToken:: api_token = '${api_token} , error = '${e}`)
    }
  }


  return (
    <Context.Provider
      value={{
        email,
        api_token,
        url,
        setEmail,
        setApiToken,
        setUrl

      }}
    >
      {children}
    </Context.Provider>
  )
}
