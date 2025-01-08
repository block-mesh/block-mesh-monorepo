import FormMain from '../components/FormMain'
import MenuMain from '../components/MenuMain'
import FigureTier from '../components/FigureTier'
import ButtonMain from '../components/ButtonMain'
import styles from './login.module.css'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import { Switch, Case } from 'react-if'

const Login = () => {
  const navigate = useNavigate()
  const [loggedIn, setLoggedIn] = useState(false)
  const [error, setError] = useState('')
  const walletContextState = useWallet()
  const { connection } = useConnection()
  const [address, setAddress] = useState('')
  const [displayedAddress, setDisplayedAddress] = useState('')
  const [tier, setTier] = useState('')

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

      }
    })()
  }, [walletContextState.connected])


  async function login() {
    try {

    } catch (error) {
      console.log('login error', error)
    }
  }

  return (
    <>
      <MenuMain current="login" />
      <FormMain
        aria-busy={loggedIn}
        data-current-item="login"
      >

        <Switch>
          <Case condition={true}>
            <FigureTier className={styles.offset}>{tier}</FigureTier>
            <p>
              Congrats! <button
              type="button"
              className={`ghost ${styles.button}`}
              title="Connect another wallet"
            >
              <u>{displayedAddress}</u>
            </button> is eligible to <data value={0} className={styles.amount}>
              0 $XENO
            </data>
            </p>
            <ButtonMain onClick={async (e) => {
              e.preventDefault()
              await login()
            }}>
              {loggedIn ? 'Logging In...' : 'Logged In now'}
            </ButtonMain>
          </Case>
          <Case condition={true}>
            <p>
              Congrats!
              <u>{displayedAddress}</u>
              already logged in <data value={0} className={styles.amount}>
              $XENO
            </data>
            </p>
          </Case>
          <Case condition={true}>
            <p>
              Sorry!
              <u>{displayedAddress}</u>
              is not eligible for $XENO
            </p>
          </Case>
        </Switch>
        {!!error &&
          (
            <output className="error">
              {error}
            </output>
          )
        }
      </FormMain>
      <button type="button" className={`ghost ${styles.button}`} onClick={disconnect}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Login
