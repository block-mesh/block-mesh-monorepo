import { FC } from 'react'
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui'
import { Navbar } from "flowbite-react";

const Header: FC = () => {
  return (
    <Navbar fluid>
      <Navbar.Brand href="/">
        <img src="https://r2-images.blockmesh.xyz/recyclo-no-bg.png" className="mr-3 h-6 sm:h-9" alt="CYCOIN" />
        <span className="self-center whitespace-nowrap text-xl font-semibold dark:text-white">CYCOIN</span>
      </Navbar.Brand>
      <div className="flex md:order-2">
        <WalletMultiButton />
        <Navbar.Toggle />
      </div>
      <Navbar.Collapse>
        {/* <Navbar.Link href="#" active>
          Home
        </Navbar.Link> */}
        <Navbar.Link href="https://dexscreener.com/solana/Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump">Dexscreener</Navbar.Link>
        <Navbar.Link href="https://x.com/_cycoin_">Twitter</Navbar.Link>
        <Navbar.Link href="https://dune.com/ohad/solana-token-accounts">Dune</Navbar.Link>
      </Navbar.Collapse>
    </Navbar>
  )
}

export default Header