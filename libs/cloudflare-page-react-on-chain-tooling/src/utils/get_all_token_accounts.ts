import { Connection, PublicKey } from '@solana/web3.js'
import { TOKEN_PROGRAM_ID } from '@solana/spl-token'
import { Metaplex } from '@metaplex-foundation/js'

export type TokenAccountDetails = {
  address: string;
  owner: string;
  decimals: number;
  amount: number;
  mint: string;
  name: string;
  uri: string;
  lamports: number;
}

const filter = [
  'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v',
  'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB'
]

export async function get_all_token_accounts(connection: Connection, owner: PublicKey, metaplex: Metaplex): Promise<TokenAccountDetails[]> {
  const output: TokenAccountDetails[] = []
  const response = await connection.getParsedTokenAccountsByOwner(owner, {
    programId: TOKEN_PROGRAM_ID
  })
  for (const account of response.value) {
    try {
      const mint = new PublicKey(account.account.data.parsed.info?.mint)
      if (filter.includes(mint.toBase58())) {
        continue
      }
      const metadata = await metaplex.nfts().findByMint({ mintAddress: mint })
      const token_account: TokenAccountDetails = {
        lamports: account.account.lamports,
        mint: account.account.data.parsed.info?.mint,
        address: account.pubkey.toBase58(),
        owner: account.account.owner.toBase58(),
        decimals: account.account.data.parsed.info?.tokenAmount?.decimals,
        amount: account.account.data.parsed.info?.tokenAmount?.amount,
        name: metadata.name,
        uri: metadata.uri
      }
      output.push(token_account)
    } catch (error) {
      console.error('Error for account', account.pubkey.toBase58(), 'error:', error)
    }
  }
  return output
}