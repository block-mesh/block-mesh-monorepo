import { FC } from 'react'
import { Accordion } from 'flowbite-react'

const Faq: FC = () => {
  return (
    <div className="px-[20%] mt-20">
      <Accordion>
        <Accordion.Panel>
          <Accordion.Title>How it works?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              On Solana, each time you buy a token, you pay for the account storage (roughly 0.002 $SOL or $0.5).
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              If you sell all the tokens, or the token price drops too low, you should reclaim the storage cost
              you paid for it.
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              We take a small commission off the storage you reclaim, and put it into $CYCOIN token.
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              Millions of token are created daily on <a className={'text-blue-500 hover:text-blue-800 hover:underline'}
                                                        href={'https://pump.fun'} target={'_blank'}>pump.fun</a>, most
              of them aren't being recycled.
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              Close to a billion token accounts have been create until now on Solana, with a TVL of storage alone
              of ~$500M
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              <a className={'text-blue-500 hover:text-blue-800 hover:underline'}
                 href={'https://dune.com/ohad/solana-token-accounts'}
                 target={'_blank'}>Check our dune chart</a>
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              Win win for everyone.
            </p>
            <p className="mb-2 text-gray-300 dark:text-gray-400">
              This isn't financial advice, use at your own risk, we don't force or encourage you trade $CYCOIN.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Where can I get help?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
              Send a DM to <a href={'https://x.com/_cycoin_'}
                              className={'text-blue-500 hover:text-blue-800 hover:underline'}
                              target={'_blank'}>$CYCOIN</a>
            </p>
            <p className="text-gray-500 dark:text-gray-400">
              You can find the link&nbsp;
              <a href="https://discord.gg/solslugs" className="text-cyan-600 hover:underline dark:text-cyan-500">
                here!
              </a>
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Where is the recycled SOL coming from?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
              Any accounts on Solana require a small rent fee to open them. By
              recycling a token, we reclaim the storage fee.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Why I can't connect my wallet?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
              If you are on mobile, please use the browser inside your wallets browser, instead of via your
              regular browser.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
      </Accordion>
    </div>

  )
}

export default Faq