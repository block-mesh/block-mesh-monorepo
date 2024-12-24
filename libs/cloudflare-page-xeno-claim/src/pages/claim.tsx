import FormMain from '../components/FormMain'
import MenuMain from '../components/MenuMain'
import FigureTier from '../components/FigureTier'
import ButtonMain from '../components/ButtonMain'
import styles from './claim.module.css'
import { useState } from 'react'
import { useNavigate } from 'react-router'

const address = `HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH`
const displayedAddress = `${address.slice(0, 4)}…${address.slice(-4)}`

const Claim = () => {
  const navigate = useNavigate()
  const [claiming, setClaiming] = useState(false)
  const [error, setError] = useState('')

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
          </button> is eligible to <data value={17_842.36} className={styles.amount}>
            17,842.36 $XENO
          </data>
        </p>
        {!!error && 
          (
            <output className="error">
              {error}
            </output>
          )
        }
        <ButtonMain
          onClick={async (e) => {
            e.preventDefault()
            setClaiming(true)
            setError('')
            try {
              await new Promise((resolve, reject) => {
                setTimeout(() => {
                  reject(new Error('an Error'))
                  resolve(void 0)
                }, 10_000)
              })
              // --> redirect to /claim
              await navigate('/claimed')
            } catch(error) {
              console.error(error)
              setError(`An error occured`)
            } finally {
              setClaiming(false)
            }
          }}
        >
          {claiming ? 'Claiming...' : 'Claim now'}
        </ButtonMain>
      </FormMain>
      <button type="button" className={`ghost ${styles.button}`}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Claim
