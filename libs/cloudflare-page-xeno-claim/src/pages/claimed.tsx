import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import styles from './claimed.module.css'

const address = `HN7cABqLq46Es1jh92dQQisAq662SmxELLLsHHe4YWrH`
const displayedAddress = `${address.slice(0, 4)}…${address.slice(-4)}`

const Claimed = () => {
  return (
    <>
      <MenuMain current="claimed" />
      <FormMain>
        <p>
          <data value={17_842.36}>17,842.36 $XENO</data>
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
      <button type="button" className={`ghost ${styles.button}`}>
        <u>Connect another wallet</u>
      </button>
    </>
  )
}
export default Claimed