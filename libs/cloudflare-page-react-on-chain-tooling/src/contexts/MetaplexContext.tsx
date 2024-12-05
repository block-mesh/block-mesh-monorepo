import { Metaplex } from '@metaplex-foundation/js'
import {
  createContext,
  FC,
  PropsWithChildren,
  useContext,
  useState
} from 'react'

export interface MetaplexContextType {
  metaplex: Metaplex | undefined;
  setMetaplex: (metaplex: Metaplex) => void;
}

export const Context = createContext<MetaplexContextType>(
  {} as MetaplexContextType
)

export const useMetaplex = (): MetaplexContextType => {
  return useContext(Context)
}

export const MetaplexProvider: FC<PropsWithChildren<any>> = ({ children, metaplex }) => {
  const [inner_metaplex, setMetaplex] = useState<Metaplex>(metaplex)

  return (
    <Context.Provider
      value={{
        metaplex: inner_metaplex,
        setMetaplex
      }}
    >
      {children}
    </Context.Provider>
  )
}
