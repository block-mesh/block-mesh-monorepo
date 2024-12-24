import './global.css'
import './styles/app.css'
import HeaderMain from './components/HeaderMain'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { BrowserRouter, Route, Routes } from 'react-router'
import Connect from './pages/connect'
import Claimed from './pages/claimed'
import Claim from './pages/claim'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { ClaimProvider } from './context/claimContex.tsx'

const App = () => {
  const network = WalletAdapterNetwork.Mainnet
  // const endpoint = useMemo(() => 'https://radial-neat-fire.solana-mainnet.quiknode.pro/9cd8966a7147622cfa74581af240632b89a6109a', [network])
  const endpoint = useMemo(() => 'https://devnet.helius-rpc.com/?api-key=cb443ba5-0587-4bf8-8274-9194e993f45e', [network])
  // const endpoint = useMemo(() => 'https://cosmological-newest-sunset.solana-devnet.quiknode.pro/acc3f83bfc7d8b7e192949dedb444ad122c86112', [network])
  const wallets = useMemo(
    () => [],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  )

  return (
    <>
      <ConnectionProvider endpoint={endpoint}>
        <ClaimProvider>
          <WalletProvider wallets={wallets} autoConnect>
            <WalletModalProvider>
              <HeaderMain />
              <main>
                <hgroup>
                  <h1>Claim $XENO</h1>
                  <p>
                    Check if you are eligible to claim $XENO. Claim closes on
                    March 31st, 2025.
                  </p>
                </hgroup>
                <BrowserRouter>
                  <Routes>
                    <Route path="/" element={<Connect />} />
                    <Route path="/claim" element={<Claim />} />
                    <Route path="/claimed" element={<Claimed />} />
                  </Routes>
                </BrowserRouter>
              </main>
            </WalletModalProvider>
          </WalletProvider>
        </ClaimProvider>
      </ConnectionProvider>
    </>
  )
}
export default App

