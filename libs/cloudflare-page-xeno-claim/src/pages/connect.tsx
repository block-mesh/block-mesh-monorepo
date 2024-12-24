import MenuMain from '../components/MenuMain'
import {
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui'
import FormMain from '../components/FormMain'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { useWallet } from '@solana/wallet-adapter-react'

const Connect = () => {
  const navigate = useNavigate()
  const [connecting, _setConnecting] = useState(false)
  const [error, _setError] = useState('')
  const walletContextState = useWallet()

  useEffect(() => {
    (async () => {
      if (walletContextState.connected) {
        await navigate('/claim')
      }
    })()
  }, [walletContextState.connected])

  return (
    <>
      <MenuMain current="connecting" />
      <FormMain
        aria-busy={connecting}
        data-current-item="connecting"
      >
        <p>Connect your Solana wallet address to check if you're eligible</p>
        {!!error &&
          (
            <output className="error">
              {error}
            </output>
          )
        }
        <WalletMultiButton />
      </FormMain>
    </>
  )
}

export default Connect
