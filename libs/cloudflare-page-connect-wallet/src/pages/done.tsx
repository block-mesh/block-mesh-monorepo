import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import styles from './done.module.css'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import { useNavigate } from 'react-router'

const Done = () => {
  const walletContextState = useWallet()
  const { connection } = useConnection()
  const navigate = useNavigate()

  async function disconnect() {
    console.log('disconnect')
    if (walletContextState.publicKey && connection) {
      await walletContextState.disconnect()
      await navigate('/')
    }
  }

  return (
    <>
      <MenuMain current="done" />
      <FormMain>
        <p>
          <data>Connect Wallet Perk Applied</data>
        </p>
        <output className={styles.output}>Connect wallet successful!</output>
        <img className={styles.img} src="https://landing-page-assets.blockmesh.xyz/logo-symbol.svg" aria-hidden="true"
             alt="" />
        <img className={styles.img} src="https://landing-page-assets.blockmesh.xyz/logo-symbol.svg" aria-hidden="true"
             alt="" />
        <img className={styles.img} src="https://landing-page-assets.blockmesh.xyz/logo-symbol.svg" aria-hidden="true"
             alt="" />
        <img className={styles.img} src="https://landing-page-assets.blockmesh.xyz/logo-symbol.svg" aria-hidden="true"
             alt="" />
      </FormMain>
      <button type="button" className={`ghost ${styles.button}`} onClick={disconnect}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Done