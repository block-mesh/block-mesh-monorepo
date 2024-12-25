import FormMain from '../components/FormMain'
import MenuMain from '../components/MenuMain'
import FigureTier from '../components/FigureTier'
import ButtonMain from '../components/ButtonMain'
import styles from './claim.module.css'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import {
  BlockheightBasedTransactionConfirmationStrategy,
  LAMPORTS_PER_SOL,
  Transaction
} from '@solana/web3.js'
import { getClaimMarkerAccount } from '../airdrop/merkle-distributor-helpers/pda.ts'
import { claimMarker } from '../airdrop/merkle-distributor-helpers/wrapper.ts'
import { useClaim } from '../context/claimContex.tsx'
import { mint } from '../constants.ts'

const Claim = () => {
  const navigate = useNavigate()
  const [claiming, setClaiming] = useState(false)
  const claimContext = useClaim()
  const [error, setError] = useState('')
  const walletContextState = useWallet()
  const { connection } = useConnection()
  const [address, setAddress] = useState('')
  const [displayedAddress, setDisplayedAddress] = useState('')

  useEffect(() => {
    setDisplayedAddress(`${address.slice(0, 4)}…${address.slice(-4)}`)
  }, [address])

  async function disconnect() {
    console.log('disconnect')
    if (walletContextState.publicKey && connection) {
      await walletContextState.disconnect()
      await navigate('/')
    } else {
      setError('Please connect wallet')
    }
  }

  useEffect(() => {
    (async () => {
      if (walletContextState.publicKey && connection) {
        setAddress(walletContextState.publicKey.toBase58())
        const claimMarker = await getClaimMarkerAccount(connection, walletContextState.publicKey)
        // @ts-ignore
        claimContext.setAmount(claimMarker.pretty().amount / LAMPORTS_PER_SOL)
        claimContext.setClaimed(claimMarker.isClaimed)
      }
    })()
  }, [walletContextState.connected])


  async function claim() {
    try {
      setClaiming(true)
      claimContext.setClaimed(true)
      if (walletContextState.publicKey && connection && walletContextState.signTransaction) {
        const block = await connection.getLatestBlockhash('confirmed')
        const instruction = claimMarker(walletContextState.publicKey, mint)
        const txn = new Transaction()
        txn.lastValidBlockHeight = block.lastValidBlockHeight
        txn.feePayer = walletContextState.publicKey
        txn.recentBlockhash = block.blockhash
        txn.add(instruction)
        const signedTxn = await walletContextState.signTransaction(txn)
        const sig = await walletContextState.sendTransaction(signedTxn, connection)
        console.log('sig', sig)
        const strategy: BlockheightBasedTransactionConfirmationStrategy = {
          signature: sig,
          blockhash: block.blockhash,
          lastValidBlockHeight: block.lastValidBlockHeight
        }
        const result = await connection.confirmTransaction(strategy, 'confirmed')
        console.log('result', result)
        if (result.value.err === null) {
          claimContext.setClaimed(true)
          await navigate('/claimed')
        } else {
          setError('Transaction failed')
        }
      }
    } catch (error) {
      console.log('CLAIM error', error)
    }
    setClaiming(false)
  }

  return (
    <>
      <MenuMain current="claiming" />
      <FormMain
        aria-busy={claiming}
        data-current-item="claiming"
      >
        <FigureTier className={styles.offset}>Tier 1</FigureTier>
        <p>
          Congrats! <button
          type="button"
          className={`ghost ${styles.button}`}
          title="Connect another wallet"
        >
          <u>{displayedAddress}</u>
        </button> is eligible to <data value={claimContext.amount} className={styles.amount}>
          {claimContext.amount} $XENO
        </data>
        </p>
        {!!error &&
          (
            <output className="error">
              {error}
            </output>
          )
        }
        <ButtonMain onClick={async (e) => {
          e.preventDefault()
          await claim()
        }}>
          {claiming ? 'Claiming...' : 'Claim now'}
        </ButtonMain>
      </FormMain>
      <button type="button" className={`ghost ${styles.button}`} onClick={disconnect}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Claim
