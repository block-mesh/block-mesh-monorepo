import './App.css'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, useConnection, useWallet, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { Metaplex } from '@metaplex-foundation/js'
import { FC, ReactNode, useMemo, useState } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { get_all_token_accounts, TokenAccountDetails } from './utils/get_all_token_accounts.ts'
import { MetaplexProvider, useMetaplex } from './contexts/MetaplexContext.tsx'
import { Connection } from '@solana/web3.js'
import { PythProvider, usePyth } from './contexts/PythContext.tsx'

const App: FC = () => {
  return (
    <Context>
      <Content />
    </Context>
  )
}


const Content: FC = () => {
  const { connection } = useConnection()
  const { publicKey } = useWallet()
  const { metaplex } = useMetaplex()
  const { priceUsd } = usePyth()
  const [rows, setRows] = useState<TokenAccountDetails[]>([])

  async function get_token_accounts() {
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
    }
    const token_accounts = await get_all_token_accounts(connection, publicKey, metaplex)
    console.log('token_accounts', { token_accounts })
    setRows(token_accounts)
  }

  return (
    <div className="App">
      <WalletMultiButton />
      <button onClick={get_token_accounts}>Get Token Accounts</button>
      <table>
        <thead>
        <tr>
          <th>Name</th>
          <th>Mint</th>
          <th>Decimals</th>
          <th>Amount</th>
          <th>Lamports</th>
        </tr>
        </thead>
        <tbody>
        {
          rows.map(i =>
            <tr>
              <td>{i.name}</td>
              <td>{i.mint}</td>
              <td>{i.decimals}</td>
              <td>{i.amount}</td>
              <td>{i.lamports}</td>
            </tr>
          )
        }
        </tbody>
      </table>
    </div>
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
