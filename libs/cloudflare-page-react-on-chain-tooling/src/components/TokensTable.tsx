import { CSSProperties, FC, useState } from 'react'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import { useMetaplex } from '../contexts/MetaplexContext.tsx'
import { usePyth } from '../contexts/PythContext.tsx'
import { get_all_token_accounts, TokenAccountDetails } from '../utils/get_all_token_accounts.ts'
import { build_transaction, CloseAccountInstructionInput } from '../utils/close_account_instruction.ts'
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { Switch, Case } from 'react-if'
import { RingLoader } from 'react-spinners'

const override: CSSProperties = {
  display: 'block',
  margin: '0 auto',
  borderColor: 'red'
}

const TokensTable: FC = () => {
  const { connection } = useConnection()
  const { publicKey, signAllTransactions, sendTransaction } = useWallet()
  const { metaplex } = useMetaplex()
  const { priceUsd } = usePyth()
  const [rows, setRows] = useState<TokenAccountDetails[]>([])
  const [selected, _setSelected] = useState<{ [key: string]: TokenAccountDetails | undefined }>({})
  const [totalUsd, setTotalUsd] = useState(0)
  const [totalSol, setTotalSol] = useState(0)
  const [loading, setLoading] = useState(false)
  let [color, _setColor] = useState('#ffffff')

  async function get_token_accounts() {
    if (!publicKey) {
      console.error('Connect wallet')
      return
    }
    if (!metaplex) {
      console.error('No metaplex')
      return
    }
    setLoading(true)
    const token_accounts = await get_all_token_accounts(connection, publicKey, metaplex)
    console.log('token_accounts', { token_accounts })
    setRows(token_accounts)
    setLoading(false)
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
    <div className="">
      <div className="bg-gray-800 rounded-lg p-4 m-2">
        <h2 className="text-xl font-bold mb-2 text-center">Summary</h2>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <button className={'text-white border border-gray-50 rounded p-2 bg-green-500 hover:bg-green-800'}
                    onClick={get_token_accounts}>Get Token
              Accounts
            </button>
          </div>
          <div>
            <button className={'text-white border border-gray-50 rounded p-2 bg-green-500 hover:bg-green-800'}
                    onClick={lfg}>Recycle
              Accounts
            </button>
          </div>
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <p><strong>Total $USD:</strong><span className={'m-1'}>{totalUsd.toFixed(3)}</span></p>
          </div>
          <div>
            <p><strong>Total $SOL:</strong><span className={'m-1'}>{totalSol.toFixed(3)}</span></p>
          </div>
        </div>
      </div>
      <Switch>
        <Case condition={loading}>
          <RingLoader
            color={color}
            loading={loading}
            cssOverride={override}
            size={150}
            aria-label="Loading Spinner"
            data-testid="loader"
          />
        </Case>
        <Case condition={rows.length === 0}>
          <div className={'m-2 p-2 text-center'}>
            <p>No token accounts loaded</p>
          </div>
        </Case>
        <Case condition={rows.length > 0}>
          <table className={'table-auto w-full divide-y divide-gray-400'}>
            <thead>
            <tr>
              <th className={'p-1'}>Select</th>
              <th className={'p-1'}>Name</th>
              <th className={'p-1'}>Mint</th>
              <th className={'p-1'}>Amount</th>
              <th className={'p-1'}>$SOL</th>
              <th className={'p-1'}>$USD</th>
            </tr>
            </thead>
            <tbody className={'divide-y divide-gray-600'}>
            {
              rows.map(i =>
                <tr>
                  <td className={'p-1'}>
                    <input
                      className="rounded border border-gray-300 bg-white checked:border-indigo-600 checked:bg-indigo-600 indeterminate:border-indigo-600 indeterminate:bg-indigo-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 forced-colors:appearance-auto"
                      type={'checkbox'} checked={selected[i.mint] !== undefined}
                      onClick={() => selectMint(i, i.mint, (priceUsd || 0) * i.lamports / LAMPORTS_PER_SOL, i.lamports / LAMPORTS_PER_SOL)}
                    />
                  </td>
                  <td>{i.name}</td>
                  <td>{i.mint}</td>
                  <td>{i.amount}</td>
                  <td>{(i.lamports / LAMPORTS_PER_SOL).toFixed(3)}</td>
                  <td>{((priceUsd || 0) * (i.lamports / LAMPORTS_PER_SOL)).toFixed(3)}</td>
                </tr>
              )
            }
            </tbody>
          </table>
        </Case>
      </Switch>
    </div>
  )
}


export default TokensTable

