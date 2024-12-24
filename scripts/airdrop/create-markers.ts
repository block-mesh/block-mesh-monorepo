import { Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import {
  getOrCreateTokenAccountInstruction,
  processTransaction, sleep
} from '../../tests/helpers'
import assert from 'assert'
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, mintTo } from '@solana/spl-token'
import { createAirDropperInstruction, createMarker } from '../../tests/merkle-distributor-helpers/wrapper'
import { getAirDropperAccount, getClaimMarkerAccount } from '../../tests/merkle-distributor-helpers/pda'

export const tokenMintAuthority = Keypair.generate()

export function loadWalletKey(keypair: string): Keypair {
  if (!keypair || keypair == '') {
    throw new Error('Keypair is required!')
  }
  const loaded = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync(keypair).toString()))
  )
  console.log(`wallet public key: ${loaded.publicKey}`)
  return loaded
}

enum Network {
  DEVNET = 'DEVNET',
  MAINNET = 'MAINNET',
}

function getConnection(network: Network): Connection {
  if (network === Network.DEVNET) {
    return new Connection('https://devnet.helius-rpc.com/?api-key=cb443ba5-0587-4bf8-8274-9194e993f45e')
  } else {
    return new Connection('https://api.mainnet-beta.solana.com')
  }
}

async function main() {
  const admin = loadWalletKey('/Users/ohaddahan/.config/solana/id.json')
  const connection = getConnection(Network.DEVNET)
  const mint = new PublicKey('3XP1qCMCKsNmCp2G2inog3ztvFKPJRsAoZBjMtv1geGQ')


  const airDropped = await getAirDropperAccount(connection)
  console.log('airDropped', airDropped.pretty())
  const claimMarker = await getClaimMarkerAccount(connection, admin.publicKey)
  console.log('claimMarker', claimMarker.pretty())
  // return

  const to = new PublicKey('FYTeY3nrrd2EqxXGq98tZ2GRfYjhjeQnNjZ54HunG1mg')

  const instruction = createMarker(admin.publicKey, mint, to, LAMPORTS_PER_SOL)
  const sigx = await processTransaction(
    [instruction],
    connection,
    admin
  )
  const txnx = await connection.getParsedTransaction(
    sigx.Signature,
    'confirmed'
  )
  assert.equal(
    sigx.SignatureResult.err,
    null,
    `${mint.toBase58()}\n${txnx?.meta?.logMessages.join('\n')}`
  )
  console.log('createMarker done')
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