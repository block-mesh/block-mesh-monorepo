import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import ButtonMain from '../components/ButtonMain'


const Connecting = () => {
  return (
    <>
      <MenuMain current="connecting" />
      <FormMain aria-busy={true} data-current-item="connecting">
        <p>Connect your Solana wallet address to check if you're eligible</p>
        <ButtonMain disabled={true}>Checking...</ButtonMain>
      </FormMain>
    </>
  )
}

export default Connecting