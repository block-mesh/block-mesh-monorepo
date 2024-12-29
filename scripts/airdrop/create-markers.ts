import { ComputeBudgetProgram, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
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
    return new Connection('https://mainnet.helius-rpc.com/?api-key=0b25ef9e-0fd3-4fb5-a5fb-deec31b9017b')
  }
}

async function main() {
  const admin = loadWalletKey('/Users/ohaddahan/.config/solana/id.json')
  const connection = getConnection(Network.MAINNET)
  const mint = new PublicKey('Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump')


  console.log('Here 1')
  const airDropped = await getAirDropperAccount(connection)
  console.log('airDropped', airDropped.pretty())
  console.log('Here 2')
  // const claimMarker = await getClaimMarkerAccount(connection, admin.publicKey)
  // console.log('claimMarker', claimMarker.pretty())

  const to = new PublicKey('HQ6SbfdKsPuvSBrkq538tvZjASoqcdZUTfSYgLe7jSSH')

  const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    units: 100000
  })
  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 10000
  })

  const instruction = createMarker(admin.publicKey, mint, to, LAMPORTS_PER_SOL / 1000)
  const sigx = await processTransaction(
    [modifyComputeUnits, addPriorityFee, instruction],
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