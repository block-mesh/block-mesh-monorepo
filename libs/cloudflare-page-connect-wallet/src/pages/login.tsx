import FormMain from '../components/FormMain'
import MenuMain from '../components/MenuMain'
import ButtonMain from '../components/ButtonMain'
import styles from './login.module.css'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { useConnection, useWallet } from '@solana/wallet-adapter-react'
import { Switch, Case } from 'react-if'
import { useMessage } from '../context/messageContext.tsx'
import { get_api_token } from '../utils/login.ts'
import { BASE_URL } from '../constants.ts'
import { connect_wallet } from '../utils/connect_wallet_api.ts'

const Login = () => {
  const navigate = useNavigate()
  const [loggedIn, setLoggedIn] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const walletContextState = useWallet()
  const message = useMessage()
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const { connection } = useConnection()

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
      if (!loading && loggedIn) {
        await navigate('/done')
      }
    })()
  }, [loggedIn, loading])


  useEffect(() => {
    (async () => {
      if (message.message === '' || message.pubkey === '' || message.signature.length === 0) {
        await navigate('/')
      }
    })()
  }, [message.signature, message.message, message.pubkey])

  async function login_and_apply_perk() {
    try {
      setLoading(true)
      if (!walletContextState.publicKey) {
        alert('Please connect wallet')
        setLoading(false)
        return
      }
      if (message.message === '' || message.pubkey === '' || message.signature.length === 0) {
        alert('Please connect wallet and sign message')
        setLoading(false)
        return
      }
      if (email === undefined || email === '' || password === undefined || password === '') {
        alert('Please fill in email and password')
        setLoading(false)
        return
      }
      const url = `${BASE_URL}/api/get_token`
      const loginResult = await get_api_token(url, { email, password })
      console.log('loginResult', loginResult)
      if (loginResult.isErr) {
        alert('Failed to login, please retry')
        setLoading(false)
        return
      }
      const api_token = loginResult.unwrap().api_token

      const connect_url = `${BASE_URL}/api/connect_wallet_api`
      const connectResult = await connect_wallet(connect_url, {
        api_token,
        email,
        pubkey: message.pubkey,
        message: message.message,
        signature: message.signature
      })
      console.log('connectResult', connectResult)
      if (connectResult.isOk) {
        const result = connectResult.unwrap()
        if (result.status !== 200) {
          alert(`Error ${result.message}`)
          setLoading(false)
          return
        } else {
          setLoggedIn(true)
          setLoading(false)
        }
      } else {
        alert('Failed to apply perk, please retry')
        setLoading(false)
      }
    } catch (error) {
      setLoading(false)
      console.log('login error', error)
    }
  }

  return (
    <>
      <MenuMain current="login" />
      <FormMain
        aria-busy={loggedIn}
        data-current-item="connecting"
      >
        <Switch>
          <Case condition={!loggedIn && loading}>
            <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none"
                 viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path className="opacity-75" fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
          </Case>
          <Case condition={!loggedIn && !loading}>
            <input type={'email'} placeholder={'Email'}
                   onChange={e => setEmail(e.target.value)}
                   className={'shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline'} />
            <input type={'password'} placeholder={'password'}
                   onChange={e => setPassword(e.target.value)}
                   className={'shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline'} />
            <input type={'text'} placeholder={'public key'}
                   readOnly={true}
                   value={message.pubkey}

                   className={'shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline'} />
            <input type={'text'} placeholder={'message'}
                   readOnly={true}
                   value={message.message}
                   className={'shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline'} />
            <input type={'text'} placeholder={'signature'}
                   readOnly={true}
                   value={JSON.stringify(message.signature)}
                   className={'shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline'} />
            <ButtonMain onClick={async (e) => {
              e.preventDefault()
              await login_and_apply_perk()
            }}>
              Submit
            </ButtonMain>
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
