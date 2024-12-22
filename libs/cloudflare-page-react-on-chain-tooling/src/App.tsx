import './App.css'
import './index.css'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { Metaplex } from '@metaplex-foundation/js'
import { FC, ReactNode, useMemo } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { MetaplexProvider } from './contexts/MetaplexContext.tsx'
import { Connection } from '@solana/web3.js'
import { PythProvider } from './contexts/PythContext.tsx'
import TokensTable from './components/TokensTable.tsx'
import Header from './components/Header.tsx'
import Footer from './components/Footer.tsx'

const App: FC = () => {
  return (
    <Context>
      <Header />
      <div className={'bg-gray-900 text-white p-4 w-screen h-screen'}>
        <div className={'flex-col max-w-3xl mx-auto'}>
          <div className="flex justify-center">
            <WalletMultiButton />
          </div>
          <TokensTable />
          <div className={'m-4 p-4'}>
            <p className="text-sm text-gray-400 mb-2">
              By using this service you agree that:
            </p>
            <p className="text-sm text-gray-400 mb-2">
              <li>This isn't financial advice.</li>
              <li>We don't advise , force or encourage you to trade $CYCOIN.</li>
              <li>We take no responsibility.</li>
              <li>You use at your own risk.</li>
              <li>You checked the transactions prior to approving them.</li>
              <li>This is legal in your jurisdiction.</li>
              <li>You are above the age of legal consent.</li>
            </p>
          </div>
        </div>
      </div>
      <Footer />
    </Context>
  )
}

const Context: FC<{ children: ReactNode }> = ({ children }) => {
  const network = WalletAdapterNetwork.Mainnet

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => 'https://radial-neat-fire.solana-mainnet.quiknode.pro/9cd8966a7147622cfa74581af240632b89a6109a', [network])
  const connection = new Connection(endpoint)
  const metaplex = new Metaplex(connection)


  const wallets = useMemo(
    () => [],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [network]
  )

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <PythProvider>
          <MetaplexProvider metaplex={metaplex}>
            <WalletModalProvider>{children}</WalletModalProvider>
          </MetaplexProvider>
        </PythProvider>
      </WalletProvider>
    </ConnectionProvider>
  )
}

export default App
