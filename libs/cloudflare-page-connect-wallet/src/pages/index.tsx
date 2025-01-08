import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import ButtonMain from '../components/ButtonMain'

const Starting = () => {
  return (
    <>
      <MenuMain current="connecting" />
      <FormMain>
        <p>Connect your Solana wallet address to check if you're eligible</p>
        <ButtonMain>Connect wallet</ButtonMain>
      </FormMain>
    </>
  )
}


export default Starting