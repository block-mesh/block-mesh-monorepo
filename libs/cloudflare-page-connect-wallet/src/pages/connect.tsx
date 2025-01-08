import MenuMain from '../components/MenuMain'
import {
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui'
import FormMain from '../components/FormMain'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { useWallet } from '@solana/wallet-adapter-react'
import { useMessage } from '../context/messageContext.tsx'
import { v4 as uuidv4 } from 'uuid'
import { decodeUTF8 } from 'tweetnacl-util'

const Connect = () => {
  const navigate = useNavigate()
  const [connecting, _setConnecting] = useState(false)
  const [error, _setError] = useState('')
  const walletContextState = useWallet()
  const messageContext = useMessage()

  useEffect(() => {
    (async () => {
      if (walletContextState.connected && walletContextState.signMessage && walletContextState.publicKey) {
        const message = uuidv4()
        const messageBytes = decodeUTF8(message)
        const signature = Array.from(await walletContextState.signMessage(messageBytes))
        messageContext.setMessage(message)
        messageContext.setSignature(signature)
        messageContext.setPubkey(walletContextState.publicKey.toBase58())
        await navigate('/login')
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
        <p>Connect your Solana wallet</p>
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
