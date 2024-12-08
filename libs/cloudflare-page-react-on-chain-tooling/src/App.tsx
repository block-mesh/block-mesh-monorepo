import './App.css'
import './index.css'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import { ConnectionProvider, useConnection, useWallet, WalletProvider } from '@solana/wallet-adapter-react'
import { WalletModalProvider, WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { Metaplex } from '@metaplex-foundation/js'
import { FC, ReactNode, useMemo, useState } from 'react'
import '@solana/wallet-adapter-react-ui/styles.css'
import { get_all_token_accounts, TokenAccountDetails } from './utils/get_all_token_accounts.ts'
import { MetaplexProvider, useMetaplex } from './contexts/MetaplexContext.tsx'
import { Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { PythProvider, usePyth } from './contexts/PythContext.tsx'
import { build_transaction, CloseAccountInstructionInput } from './utils/close_account_instruction.ts'

const App: FC = () => {
  return (
    <Context>
      <Content />
    </Context>
  )
}


const Content: FC = () => {
  const { connection } = useConnection()
  const { publicKey, signAllTransactions, sendTransaction } = useWallet()
  const { metaplex } = useMetaplex()
  const { priceUsd } = usePyth()
  const [rows, setRows] = useState<TokenAccountDetails[]>([])
  const [selected, _setSelected] = useState<{ [key: string]: TokenAccountDetails | undefined }>({})
  const [totalUsd, setTotalUsd] = useState(0)
  const [totalSol, setTotalSol] = useState(0)

  async function get_token_accounts() {
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
      return
    }
    const token_accounts = await get_all_token_accounts(connection, publicKey, metaplex)
    console.log('token_accounts', { token_accounts })
    setRows(token_accounts)
  }

  async function lfg() {
    if (!sendTransaction) {
      console.error('Connect wallet')
      return
    }
    if (!signAllTransactions) {
      console.error('Connect wallet')
      return
    }
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
      return
    }
    if (!connection) {
      console.error('No connection')
      return
    }

    const inputs: CloseAccountInstructionInput[] = []
    for (const entry of Object.entries(selected)) {
      const [_mint, details] = entry
      if (details === undefined) {
        continue
      }
      const input: CloseAccountInstructionInput = {
        account: new PublicKey(details.address),
        destination: publicKey,
        authority: publicKey
      }
      inputs.push(input)
    }

    const txns = await build_transaction(connection, publicKey, inputs)
    console.log('sending it', txns)
    const signedTxns = await signAllTransactions(txns)
    console.log('signedTxns', signedTxns)
    for (const txn of signedTxns) {
      console.log('txn', txn)
      await sendTransaction(txn, connection)
    }
  }

  function selectMint(i: TokenAccountDetails, mint: string, usd: number, sol: number) {
    if (selected[mint]) {
      selected[mint] = undefined
      setTotalSol((value) => value - sol)
      setTotalUsd((value) => value - usd)
    } else {
      selected[mint] = i
      setTotalSol((value) => value + sol)
      setTotalUsd((value) => value + usd)
    }
  }

  return (
    <div className="App">
      <WalletMultiButton />
      <button onClick={get_token_accounts}>Get Token Accounts</button>
      <br />
      <div>Total: {totalUsd.toFixed(3)}$ | {totalSol.toFixed(3)} $SOL | <button onClick={lfg}>LFG</button></div>
      <table border={1}>
        <thead>
        <tr>
          <th>Select</th>
          <th>Name</th>
          <th>Mint</th>
          <th>Decimals</th>
          <th>Amount</th>
          <th>$SOL</th>
          <th>$</th>
        </tr>
        </thead>
        <tbody>
        {
          rows.map(i =>
            <tr>
              <td><input type={'checkbox'} checked={selected[i.mint] !== undefined}
                         onClick={() => selectMint(i, i.mint, (priceUsd || 0) * i.lamports / LAMPORTS_PER_SOL, i.lamports / LAMPORTS_PER_SOL)} />
              </td>
              <td>{i.name}</td>
              <td>{i.mint}</td>
              <td>{i.decimals}</td>
              <td>{i.amount}</td>
              <td>{(i.lamports / LAMPORTS_PER_SOL).toFixed(3)}</td>
              <td>{((priceUsd || 0) * (i.lamports / LAMPORTS_PER_SOL)).toFixed(3)}</td>
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
