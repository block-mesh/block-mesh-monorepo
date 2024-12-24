import MenuMain from '../components/MenuMain'
import FormMain from '../components/FormMain'
import ButtonMain from '../components/ButtonMain'


const ConnectError = () => {
  return (
    <>
      <MenuMain current="connecting" />
      <FormMain data-current-item="connecting">
        <p>Connect your Solana wallet address to check if you're eligible</p>
        <output>Wrong network, switch to Solana</output>
        <ButtonMain>Connect</ButtonMain>
      </FormMain>
    </>
  )
}

export default ConnectError

// <style>
//     output {
//         color: var(--color-error-1);
//     }
// </style>
