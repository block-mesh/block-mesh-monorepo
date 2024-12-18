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

const App: FC = () => {
  return (
    <Context>
      <div className={'bg-gray-900 text-white p-4 w-screen h-screen'}>
        <div className={'flex-col max-w-3xl mx-auto'}>
          <div className="flex justify-center">
            <WalletMultiButton />
          </div>
          <TokensTable />
        </div>
      </div>
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
