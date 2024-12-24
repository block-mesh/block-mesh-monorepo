import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import ButtonMain from '../components/ButtonMain'
import { useState } from 'react'
import { useNavigate } from 'react-router'

const Connect = () => {
  const navigate = useNavigate()
  const [connecting, setConnecting] = useState(false)

  return (
    <>
      <MenuMain current="connecting" />
      <FormMain 
        aria-busy={connecting} 
        data-current-item="claiming"
      >
        <p>Connect your Solana wallet address to check if you're eligible</p>
        <ButtonMain
          disabled={connecting}
          onClick={async (e) => {
            e.preventDefault()
            setConnecting(true)
            try {
              await new Promise((resolve) => {
                setTimeout(() => {
                  resolve(void 0)
                }, 1_000)
              })
              // --> redirect to /claim
              await navigate('/claim')
            } catch(error) {
              console.error(error)
            } finally {
              setConnecting(false)
            }
          }}
        >
          {connecting ? 'Connecting...' : 'Connect wallet'}
        </ButtonMain>
      </FormMain>
    </>
  )
}

export default Connect
