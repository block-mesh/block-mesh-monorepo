import { ComputeBudgetProgram, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import {
  getOrCreateTokenAccountInstruction,
  processTransaction, sleep
} from '../../tests/helpers'
import assert from 'assert'
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddress, mintTo } from '@solana/spl-token'
import { createAirDropperInstruction } from '../../tests/merkle-distributor-helpers/wrapper'

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
  // const mint = await createMint(
  //   connection,
  //   admin,
  //   tokenMintAuthority.publicKey,
  //   tokenMintAuthority.publicKey,
  //   9
  // )
  const mint = new PublicKey('Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump')
  // console.log('mint created', mint.toBase58())

  // const ata = await createAssociatedTokenAccount(connection, admin, mint, admin.publicKey)
  // const instructions = await getOrCreateTokenAccountInstruction(
  //   mint,
  //   admin.publicKey,
  //   connection
  // )
  // const sig = await processTransaction(
  //   [instructions],
  //   connection,
  //   admin
  // )
  // const txn = await connection.getParsedTransaction(
  //   sig.Signature,
  //   'confirmed'
  // )
  // assert.equal(
  //   sig.SignatureResult.err,
  //   null,
  //   `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
  // )
  // console.log('getOrCreateTokenAccountInstruction done', ata.toBase58())
  // await mintTo(
  //   connection,
  //   admin,
  //   mint,
  //   await getAssociatedTokenAddress(mint, admin.publicKey),
  //   tokenMintAuthority,
  //   LAMPORTS_PER_SOL * 50_000
  // )
  // console.log('mintTO done')


  const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    units: 1000000
  })
  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 5000
  })

  const instruction = createAirDropperInstruction(admin.publicKey, mint)
  const sigx = await processTransaction(
    [modifyComputeUnits, addPriorityFee, instruction],
    connection,
    admin
  )
  console.log('sigx', sigx)
  const txnx = await connection.getParsedTransaction(
    sigx.Signature,
    'confirmed'
  )
  assert.equal(
    sigx.SignatureResult.err,
    null,
    `${mint.toBase58()}\n${txnx?.meta?.logMessages.join('\n')}`
  )
  console.log('createAirDropperInstruction done')
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