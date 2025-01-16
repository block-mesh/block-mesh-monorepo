import FormMain from '../components/FormMain'
import ButtonMain from '../components/ButtonMain'

const AdgramsTest = () => {
  return (
    <>
      <FormMain
        aria-busy={false}
        data-current-item="connecting"
      >
        <ButtonMain onClick={async (_) => {
          // @ts-ignore
          (window.AdController as any).show().then((results: any) => {
            // user watch ad till the end or close it in interstitial format
            // your code to reward user for rewarded format
            console.log('results', results)
          }).catch((error: any) => {
            // user get error during playing ad or skip ad
            // do nothing or whatever you want
            console.error('error', error)
          })
        }}>
          Submit
        </ButtonMain>
      </FormMain>
    </>
  )
}
export default AdgramsTest
