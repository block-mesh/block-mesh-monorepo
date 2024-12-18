import { FC, useEffect, useState } from 'react'

interface ModalProps {
  modal: boolean
  setModal: (modal: boolean) => void
}

const HowItWorks: FC<ModalProps> = ({ modal, setModal }) => {
  const [modalClass, setModalClass] = useState('relative z-10')

  useEffect(() => {
    if (modal) {
      setModalClass('relative z-10')
    } else {
      setModalClass('hidden relative z-10')
    }
  }, [modal])

  return (
    <div className={modalClass} aria-labelledby="modal-title" role="dialog" aria-modal="true"
         onClick={() => setModal(!modal)}>
      <div className="fixed inset-0 bg-gray-500/75 transition-opacity" aria-hidden="true"></div>
      <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
        <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
          <div
            className="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6">
            <div>
              <div className="mx-auto flex size-12 items-center justify-center rounded-full bg-green-100">
                <img className="size-12" src={'https://r2-images.blockmesh.xyz/recyclo-no-bg.png'}
                     alt={''} />
              </div>
              <div className="mt-3 text-center sm:mt-5">
                <h3 className="text-base font-semibold text-gray-900" id="modal-title">How it works</h3>
                <div className="mt-2">
                  <p className="text-sm text-gray-500 mb-2">
                    On Solana, each time you buy a token, you pay for the account storage (roughly 0.002 $SOL or $0.5).
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    If you sell all the tokens, or the token price drops too low, you should reclaim the storage cost
                    you paid for it.
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    We take a small commission off the storage you reclaim, and put it into $CYCOIN token.
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    Millions of token are created daily on pump.fun, most of them aren't being recycled.
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    Close to a billion token accounts have been create until now on Solana, with a TVL of storage alone
                    of ~$500M
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    <a className={'text-blue-500 hover:text-blue-800 hover:underline'}
                       href={'https://dune.com/ohad/solana-token-accounts'}
                       target={'_blank'}>Check our dune chart</a>
                  </p>
                  <p className="text-sm text-gray-500 mb-2">
                    Win win for everyone.
                  </p>
                  <p className="text-sm text-gray-500 mt-2 border-t">
                    This isn't financial advice, use at your own risk, we don't force or encourage you trade $CYCOIN.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

  )
}

export default HowItWorks