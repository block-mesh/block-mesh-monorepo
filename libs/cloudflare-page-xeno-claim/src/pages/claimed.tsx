import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import styles from './claimed.module.css'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import { useClaim } from '../context/claimContex.tsx'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'

const Claimed = () => {
  const claimContext = useClaim()
  const walletContextState = useWallet()
  const { connection } = useConnection()
  const [address, setAddress] = useState('')
  const [displayedAddress, setDisplayedAddress] = useState('')
  const navigate = useNavigate()

  async function disconnect() {
    console.log('disconnect')
    if (walletContextState.publicKey && connection) {
      await walletContextState.disconnect()
      await navigate('/')
    }
  }

  useEffect(() => {
    setDisplayedAddress(`${address.slice(0, 4)}…${address.slice(-4)}`)
  }, [address])

  useEffect(() => {
    (async () => {
      if (walletContextState.publicKey && connection) {
        setAddress(walletContextState.publicKey.toBase58())
      }
    })()
  }, [walletContextState.connected])

  return (
    <>
      <MenuMain current="claimed" />
      <FormMain>
        <p>
          <data value={claimContext.amount}>{claimContext.amount} $XENO</data>
          have been sent to
          <button
            type="button"
            className={`ghost ${styles.button}`}
            title="Connect another wallet"
          >
            <u>{displayedAddress}</u>
          </button>
        </p>
        <output className={styles.output}>Claim successful!</output>
        <img className={styles.img} src="/xeno-coin.png" aria-hidden="true" alt="" />
        <img className={styles.img} src="/xeno-coin.png" aria-hidden="true" alt="" />
        <img className={styles.img} src="/xeno-coin.png" aria-hidden="true" alt="" />
        <img className={styles.img} src="/xeno-coin.png" aria-hidden="true" alt="" />
      </FormMain>
      <button type="button" className={`ghost ${styles.button}`} onClick={disconnect}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Claimed