import './global.css'
import './styles/app.css'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import React, { useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import HeaderMain from './components/HeaderMain'

const App = () => {
  const network = WalletAdapterNetwork.Mainnet
  const endpoint = useMemo(() => 'https://radial-neat-fire.solana-mainnet.quiknode.pro/9cd8966a7147622cfa74581af240632b89a6109a', [network])
  const wallets = useMemo(
    () => [],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  )

  return (
    <>
      <ConnectionProvider endpoint={endpoint}>
        <WalletProvider wallets={wallets} autoConnect>
          <HeaderMain />
          <main>
            <hgroup>
              <h1>Claim $XENO</h1>
              <p>
                Check if you are eligible to claim $XENO. Claim closes on
                March 31st, 2025.
              </p>
            </hgroup>
            <slot />
          </main>
        </WalletProvider>
      </ConnectionProvider>
    </>
  )
}
export default App