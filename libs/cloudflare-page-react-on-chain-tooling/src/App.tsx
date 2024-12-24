import './App.css'
import './index.css'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { Metaplex } from '@metaplex-foundation/js'
import { FC, useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { MetaplexProvider } from './contexts/MetaplexContext.tsx'
import { Connection } from '@solana/web3.js'
import { PythProvider } from './contexts/PythContext.tsx'
import AppRoutes from './routes.tsx'
import { BrowserRouter } from 'react-router-dom'

const App: FC = () => {
  const network = WalletAdapterNetwork.Mainnet
  const endpoint = useMemo(() => 'https://radial-neat-fire.solana-mainnet.quiknode.pro/9cd8966a7147622cfa74581af240632b89a6109a', [network])
  const connection = new Connection(endpoint)
  const metaplex = new Metaplex(connection)

  const wallets = useMemo(
    () => [],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  )

  return (
    <BrowserRouter>
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <PythProvider>
            <MetaplexProvider metaplex={metaplex}>
              <WalletModalProvider>
                <AppRoutes />
              </WalletModalProvider>
            </MetaplexProvider>
          </PythProvider>
        </WalletProvider>
      </ConnectionProvider>
    </BrowserRouter>
  )

}
export default App
