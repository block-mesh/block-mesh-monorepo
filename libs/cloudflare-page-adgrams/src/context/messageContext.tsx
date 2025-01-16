import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useState
} from 'react'

export interface MessageContextType {
  pubkey: string;
  message: string;
  signature: number[];
  setPubkey: (pubkey: string) => void;
  setMessage: (message: string) => void;
  setSignature: (signature: number[]) => void;
}

export const Context = createContext<MessageContextType>(
  {} as MessageContextType
)

export const useMessage = (): MessageContextType => {
  return useContext(Context)
}

export const MessageProvider: FC<PropsWithChildren<any>> = ({ children }) => {
  const [pubkey, setPubkey] = useState('')
  const [message, setMessage] = useState('')
  const [signature, setSignature] = useState<number[]>([])

  return (
    <Context.Provider
      value={{
        pubkey,
        message,
        signature,
        setPubkey,
        setMessage,
        setSignature
      }}
    >
      {children}
    </Context.Provider>
  )
}
