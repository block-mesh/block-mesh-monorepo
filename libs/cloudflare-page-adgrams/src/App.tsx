import './global.css'
import './styles/app.css'
import HeaderMain from './components/HeaderMain'
import '@solana/wallet-adapter-react-ui/styles.css'
import { BrowserRouter, Route, Routes } from 'react-router'
import AdgramsTest from './pages/adgramsTest.tsx'

const App = () => {

  return (
    <>
      <HeaderMain />
      <main>
        <hgroup>
          <h1>Adgrams Test</h1>
        </hgroup>
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<AdgramsTest />} />
          </Routes>
        </BrowserRouter>
      </main>
    </>
  )
}
export default App

