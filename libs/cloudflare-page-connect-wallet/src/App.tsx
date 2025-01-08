import './global.css'
import './styles/app.css'
import HeaderMain from './components/HeaderMain'
import { useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { BrowserRouter, Route, Routes } from 'react-router'
import Connect from './pages/connect'
import Done from './pages/done.tsx'
import Login from './pages/login.tsx'
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
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
        <WalletProvider wallets={wallets} autoConnect>
          <WalletModalProvider>
            <HeaderMain />
            <main>
              <hgroup>
                <h1>Connect Wallet Perk</h1>
              </hgroup>
              <BrowserRouter>
                <Routes>
                  <Route path="/" element={<Connect />} />
                  <Route path="/login" element={<Login />} />
                  <Route path="/done" element={<Done />} />
                </Routes>
              </BrowserRouter>
            </main>
          </WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    </>
  )
}
export default App

