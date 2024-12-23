import { FC } from 'react'
import { Accordion } from "flowbite-react";
const Faq: FC = () => {
  return (
    <div className='px-[20%] mt-20'>
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
              Millions of token are created daily on pump.fun, most of them aren't being recycled.
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
          <Accordion.Title>Where can I get support?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
              You can join the Sol Slugs discord for assitance and troubleshooting.
              Use the #create-a-ticket channel and the developers will assist you as soon as possbile!
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
          <Accordion.Title>Where is the reclaimed SOL coming from?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Any accounts on Solana require a small storage fee to open them. By
            burning a token, we can close this account and reclaim the storage fee.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>How much can I reclaim from burning?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Most NFTs will give you 0.01 SOL when you burn. Most tokens will give 
            you 0.002 SOL. Certain NFTs, such as scam tokens, will also only return 
            0.002 SOL. If the NFT was minted with magic eden's open creator 
            protocol, you can reclaim 0.004 SOL. Finally, compressed NFTs 
            unfortunately cannot reclaim any SOL.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>What does 'cleanup' do?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Cleanup closes vacant token accounts and unused serum accounts.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Any NFT or unique token requires an account to store it. Whenever you 
            list, transfer or dispose of the token or NFT, the account is left 
            empty and can be closed to reclaim a small amount of SOL.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Some token swaps or exchanges also require a serum account to store 
            some swap related data. This account can be safely closed once the swap 
            has completed to again reclaim a small amount of SOL.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            This operation is completely safe to perform. It does not have any 
            effect on the NFTs & tokens in your wallet or limit orders on exchanges.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            In rare instances some dApps may not create token accounts as needed. 
            This is a bug and should be reported to that dApps' developer.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Why can't I see my wallet to connect it?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            If you are on mobile, you need to open the incinerator inside your wallets browser, instead of via your regular browser.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Why can't I reclaim any SOL from a compressed NFT?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            To make compressed NFTs cheaper to mint, the metadata and token accounts are not stored on chain like normal.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Instead, they are indexed off chain. While this makes them cheaper to mint, it also means there are no accounts to close on chain, and reclaim the SOL from.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Do you charge any fees?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Yes, we charge roughly 2-5% fees depending on the exact type of item 
            being burnt. Cleaning up vacant accounts has a 2.3% fee, resizing NFTs 
            has a 5% fee, burning NFTs has a 5% fee, and so on. There are no fees 
            for burning compressed NFTs. These fees helps fund further development 
            of the Incinerator, along with ongoing costs such as RPCs and hosting.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Note: this fee is taken from the reclaimed SOL - you will never lose SOL by burning, you can only gain it.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>How can I burn an LP (Luquidity Pool)?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Enable Pro mode in the footer to reveal the LP tab.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>How do I burn a specific amount of tokens?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            If you want to burn just a specific amount of your tokens, for example, 
            a % of your token supply, we suggest sending this amount to another 
            wallet, and burning there. In future, we will have an easier method to 
            perform this.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>I burned but I don't think I got anything. What's going on?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            At most you are getting 0.01 SOL per NFT and 0.002 SOL for closing a 
            token account. To reclaim 1 SOL you'd have to burn at least 100 NFTs. 
            The transaction summary when you complete your burn should provide an 
            accurate breakdown of the SOL you reclaimed.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>I burned for a token(i.e. $BONK) and didn't get anything/only got SOL, What happened?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            There are 2 transactions. The first is the burn that reclaims the rent 
            in SOL. The second is a Jupiter swap of that SOL for your token of 
            choice. Occasionally, the Jupiter swap can fail, but you would have 
            received the SOL in your wallet. Go to the transaction on Solscan and 
            look at the Account Inputs tab. This will show a positive change in SOL 
            from the burn.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>I have a stack of tokens. Can I get rich by burning I at a time?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            The SOL you reclaim from burning is achieved by closing the account that stores that token. This amount is the same regardless of whether the account holds 1 or 100,000 tokens.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Also, if you try to send 1 token at a time to a new account, this would be futile, as you'd need to pay to instantiate new token accounts, negating any profit you'd make.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>What does 'frozen' mean?</Accordion.Title>
          <Accordion.Content>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            A frozen token cannot be transferred or burned. We indicate a frozen token so there is no confusion when you are unable to select it for burning.
            </p>
            <p className="mb-2 text-gray-500 dark:text-gray-400">
            Our dev has posted about this issue on Solana's Github in hopes that they make a change, but as of now we are stuck with these frozen tokens.
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>SOL Incinerator Tutorial</Accordion.Title>
          <Accordion.Content>
            <p className="text-gray-500 dark:text-gray-400">
              You can find a tutorial on using the SOL Incinerator on our Youtube channel&nbsp;
              <a href="https://www.youtube.com/watch?v=5iFI2jHmqFA" className="text-cyan-600 hover:underline dark:text-cyan-500">
                 here!
              </a>
            </p>
          </Accordion.Content>
        </Accordion.Panel>
        <Accordion.Panel>
          <Accordion.Title>Who made the SOL Incinerator?</Accordion.Title>
          <Accordion.Content>
            <p className="text-gray-500 dark:text-gray-400">
            The incinerator was made by the Sol Slugs team, a deflationary collection centered around burning.
            </p>
            <p className="text-gray-500 dark:text-gray-400">
            You can find more info on Sol Slugs by visiting our Discord or Twitter in the links above!
            </p>
          </Accordion.Content>
        </Accordion.Panel>
      </Accordion>
    </div>
    
  );
}

export default Faq