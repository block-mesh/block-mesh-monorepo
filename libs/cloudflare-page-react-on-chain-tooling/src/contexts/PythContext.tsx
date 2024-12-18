import {
  createContext,
  FC,
  PropsWithChildren,
  useContext, useEffect,
  useState
} from 'react'
import { PriceFeed, PriceServiceConnection } from '@pythnetwork/price-service-client'

export interface PythContextType {
  price: PriceFeed | undefined;
  priceUsd: number | undefined;
}

export const Context = createContext<PythContextType>(
  {} as PythContextType
)

export const usePyth = (): PythContextType => {
  return useContext(Context)
}

export const PythProvider: FC<PropsWithChildren<any>> = ({ children }) => {
  const [price, setPrice] = useState<PriceFeed>()
  const [priceUsd, setPriceUsd] = useState<number>()
  const pyth_connection = new PriceServiceConnection('https://hermes.pyth.network', {
    priceFeedRequestConfig: {
      // Provide this option to retrieve signed price updates for on-chain contracts.
      // Ignore this option for off-chain use.
      binary: true
    }
  })
  const priceIds = [
    '0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d'
  ]

  useEffect(() => {
    (async () => {
      const currentPrices = await pyth_connection.getLatestPriceFeeds(priceIds)
      if (currentPrices) {
        const feed = currentPrices[0]
        console.log('currentPrices', feed)
        const p = feed.getPriceUnchecked()
        const p2 = parseFloat(p.price) * (10 ** p.expo)
        console.log('p', p, 'p2', p2)
        setPrice(feed)
        setPriceUsd(p2)
      }
    })()
  }, [])

  return (
    <Context.Provider
      value={{
        price,
        priceUsd
      }}
    >
      {children}
    </Context.Provider>
  )
}
