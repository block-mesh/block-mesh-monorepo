import { Connection, PublicKey } from '@solana/web3.js'
import { processAndValidateTransaction } from '../../tests/helpers'
import { loadWalletKey } from '../../tests/wallet-loader'
import {
  CloseProviderNodeInstructionAccounts,
  CloseProviderNodeInstructionArgs,
  createCloseProviderNodeInstruction,
  PROGRAM_ID
} from '../../tests/generated'
import * as anchor from '@coral-xyz/anchor'

enum Network {
  DEVNET = 'DEVNET',
  MAINNET = 'MAINNET',
}

function getConnection(network: Network): Connection {
  if (network === Network.DEVNET) {
    return new Connection('https://api.devnet.solana.com')
  } else {
    return new Connection('https://api.mainnet-beta.solana.com')
  }
}

async function main() {
  const providerNodeWallet = loadWalletKey('./example-keys/proxy-master.json')
  const connection = getConnection(Network.DEVNET)
  const [providerNode, bump] = PublicKey.findProgramAddressSync([
    Buffer.from(anchor.utils.bytes.utf8.encode('PROVIDER_NODE')),
    providerNodeWallet.publicKey.toBuffer()
  ], PROGRAM_ID)
  const args: CloseProviderNodeInstructionArgs = {
    args: {
      bump

    }
  }
  const accounts: CloseProviderNodeInstructionAccounts = {
    signer: providerNodeWallet.publicKey,
    providerNode
  }
  const instruction = createCloseProviderNodeInstruction(accounts, args)
  await processAndValidateTransaction([instruction], connection, providerNodeWallet)
}

main()
  .then(() => {
    console.log('Done')
    process.exit(0)
  })
  .catch((err) => {
    console.error(err)
    process.exit(1)
  })
