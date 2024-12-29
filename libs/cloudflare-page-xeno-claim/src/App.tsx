import './global.css'
import './styles/app.css'
import HeaderMain from './components/HeaderMain'
import { useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { BrowserRouter, Route, Routes } from 'react-router'
import Connect from './pages/connect'
import Claimed from './pages/claimed'
import Claim from './pages/claim'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { ClaimProvider } from './context/claimContex.tsx'
import { network, rpc } from './constants.ts'

const App = () => {
  const endpoint = useMemo(() => rpc, [network])
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
                    Check if you are eligible to claim
                    <a href={'https://dexscreener.com/solana/Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump'}
                       style={{ marginLeft: '10px' }}
                       target={'_blank'}>$XENO</a>
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

