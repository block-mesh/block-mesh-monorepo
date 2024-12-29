import { ComputeBudgetProgram, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { parse } from 'csv-parse/sync'
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
  const f = await fs.readFileSync('/Users/ohaddahan/Downloads/xeno-tiers/tier-3-final.csv')
  const records: [] = parse(f.toString())
  let count = 0
  let promises = []
  for (const record of records) {
    count += 1
    const to_raw = record[4]
    if (to_raw === 'wallet_address') {
      continue
    }
    if (count < 19000) {
      continue
    }
    const to = new PublicKey(record[4])
    console.log('count', count, ' to', to.toBase58())
    const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
      units: 100000
    })
    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 1000
    })

    try {
      const instruction = createMarker(admin.publicKey, mint, to, 25 * LAMPORTS_PER_SOL / 1000)
      const sigx = processTransaction(
        [modifyComputeUnits, addPriorityFee, instruction],
        connection,
        admin
      )
      await sleep(200)
      promises.push(sigx)
    } catch (error) {
      console.log('error wallet', to.toBase58(), ' error ', error)
    }
  }
  await Promise.allSettled(promises)
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