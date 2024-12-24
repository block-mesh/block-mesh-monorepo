import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useState
} from 'react'

export interface ClaimContextType {
  amount: number
  claimed: boolean
  setAmount: (amount: number) => void,
  setClaimed: (claimed: boolean) => void
}

export const Context = createContext<ClaimContextType>(
  {} as ClaimContextType
)

export const useClaim = (): ClaimContextType => {
  return useContext(Context)
}

export const ClaimProvider: FC<PropsWithChildren<any>> = ({ children }) => {
  const [amount, setAmount] = useState(0)
  const [claimed, setClaimed] = useState(false)

  return (
    <Context.Provider
      value={{
        amount,
        setAmount,
        claimed,
        setClaimed
      }}
    >
      {children}
    </Context.Provider>
  )
}
