import { FC, useState } from 'react'
import HowItWorks from './HowItWorks.tsx'

const Header: FC = () => {
  const [modal, setModal] = useState(false)
  return (
    <>
      <HowItWorks modal={modal} setModal={setModal} />
      <nav className="bg-gray-800">
        <div className="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
          <div className="relative flex h-16 items-center justify-between">
            <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
              <div className="flex shrink-0 items-center">
                <img className="h-8 w-auto" src="https://r2-images.blockmesh.xyz/recyclo-no-bg.png"
                     alt="CYCOIN" />
              </div>
              <div className="hidden sm:ml-6 sm:block">
                <div className="flex space-x-4">
                  <a href="https://dexscreener.com/solana/Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump"
                     target={'_blank'}
                     className="rounded-md hover:bg-gray-900 px-3 py-2 text-sm font-medium text-white"
                     aria-current="page">dexscreener</a>
                  <a href="https://x.com/_cycoin_" target={'_blank'}
                     className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white">
                    twitter</a>
                  <a href="https://dune.com/ohad/solana-token-accounts" target={'_blank'}
                     className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white">
                    dune</a>
                  <div
                    className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white"
                    onClick={() => setModal(value => !value)}
                  >
                    how it works
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="sm:hidden" id="mobile-menu">
          <div className="space-y-1 px-2 pb-3 pt-2">
            <div className="flex space-x-4">
              <a href="https://dexscreener.com/solana/Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump"
                 target={'_blank'}
                 className="rounded-md hover:bg-gray-900 px-3 py-2 text-sm font-medium text-white"
                 aria-current="page">dexscreener</a>
              <a href="https://x.com/_cycoin_" target={'_blank'}
                 className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white">
                twitter</a>
              <a href="https://dune.com/ohad/solana-token-accounts" target={'_blank'}
                 className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white">
                dune</a>
              <div
                className="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-900 hover:text-white"
                onClick={() => setModal(value => !value)}
              >
                how it works
              </div>
            </div>
          </div>
        </div>
      </nav>
    </>
  )
}

export default Header