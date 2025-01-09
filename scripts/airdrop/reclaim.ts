import { ComputeBudgetProgram, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
import { ClaimMarker } from '../../tests/merkle-distributor-libs'
import { processTransaction, sleep } from '../../tests/helpers'
import assert from 'assert'
import { createReclaimTransactionInstruction } from '../../tests/merkle-distributor-helpers/wrapper'

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
    return new Connection('https://mainnet.helius-rpc.com/?api-key=32c35600-ee87-4ba1-b348-7d41f9b1693c')
  }
}

async function main() {
  const admin = loadWalletKey('/Users/ohaddahan/.config/solana/id.json')
  const connection = getConnection(Network.MAINNET)
  const mint = new PublicKey('Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump')
  const promises = []

  // const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
  //   units: 1000000
  // })
  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 100000
  })

  const markers = await ClaimMarker.gpaBuilder().run(connection)
  console.log('markers.length = ', markers.length)
  const claimed = await ClaimMarker.gpaBuilder().addFilter('isClaimed', true).run(connection)
  let claimers = []
  console.log('claimed.length = ', claimed.length)
  const unclaimed = await ClaimMarker.gpaBuilder().addFilter('isClaimed', false).run(connection)
  const unclaimers = []
  console.log('unclaimed.length = ', unclaimed.length)

  let count = 0
  let claimed_sum = 0
  let sol = 0
  for (const c of claimed.values()) {
    const [account] = ClaimMarker.fromAccountInfo(c.account)
    reclaim(connection, mint, admin, account)
    // @ts-ignore
    // claimed_sum += account.pretty().amount
    // claimers.push(account.pretty())
    // sol += c.account.lamports
    await sleep(250)
  }
  let unclaimed_sum = 0
  let unclaimd_500 = 0
  for (const c of unclaimed.values()) {
    const [account] = ClaimMarker.fromAccountInfo(c.account)
    promises.push(reclaim(connection, mint, admin, account))
    // @ts-ignore
    // unclaimed_sum += account.pretty().amount
    // if (account.pretty().amount === 500 * (LAMPORTS_PER_SOL / 1000)) {
    // @ts-ignore
    unclaimd_500 += account.pretty().amount
    // }
    // unclaimers.push(account.pretty())
    sol += c.account.lamports
    await sleep(250)
    count += 1
  }
  console.log('claimed_sum', claimed_sum / (LAMPORTS_PER_SOL / 1000))
  console.log('unclaimed_sum', unclaimed_sum / (LAMPORTS_PER_SOL / 1000))
  console.log('unclaimd_500', unclaimd_500 / (LAMPORTS_PER_SOL / 1000))
  console.log('sol', sol / LAMPORTS_PER_SOL)
  await Promise.all(promises)
  //
  // fs.writeFileSync('claimers.json', JSON.stringify(claimers, null, 2))
  // fs.writeFileSync('unclaimers.json', JSON.stringify(unclaimers, null, 2))
  //
  // return
  // const c = Array.from(unclaimed.values())[0]
  // const [account] = ClaimMarker.fromAccountInfo(c.account)
  // const instruction = createReclaimInstruction(admin.publicKey, account.claimant, mint)
  // const sigx = await processTransaction(
  //   [addPriorityFee, instruction],
  //   connection,
  //   admin
  // )
  // console.log('sigx', sigx)
  // const txnx = await connection.getParsedTransaction(
  //   sigx.Signature,
  //   'confirmed'
  // )
  // console.log('txnx', txnx)
  // assert.equal(
  //   sigx.SignatureResult.err,
  //   null,
  //   `${mint.toBase58()}\n${txnx?.meta?.logMessages.join('\n')}`
  // )
}

async function reclaim(connection: Connection, mint: PublicKey, admin: Keypair, account: ClaimMarker) {
  try {
    const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
      microLamports: 100000
    })
    const instruction = createReclaimTransactionInstruction(admin.publicKey, account.claimant, mint)
    const sigx = await processTransaction(
      [addPriorityFee, instruction],
      connection,
      admin
    )
    console.log('sigx', sigx)
    const txnx = await connection.getParsedTransaction(
      sigx.Signature,
      'confirmed'
    )
    console.log('txnx', txnx)
    assert.equal(
      sigx.SignatureResult.err,
      null,
      `${mint.toBase58()}\n${txnx?.meta?.logMessages.join('\n')}`
    )
  } catch (error) {
    console.error('reclaim error', error)
  }
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