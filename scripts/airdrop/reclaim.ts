import { ComputeBudgetProgram, Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { Keypair } from '@solana/web3.js'
import fs from 'fs'
import { ClaimMarker } from '../../tests/merkle-distributor-libs'

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
    return new Connection('https://mainnet.helius-rpc.com/?api-key=beebdf43-fe25-4693-9eb2-6891e3d6cc5f')
  }
}

async function main() {
  const admin = loadWalletKey('/Users/ohaddahan/.config/solana/id.json')
  const connection = getConnection(Network.MAINNET)
  const mint = new PublicKey('Db7ZUaWTThwZy7bVhjn5Dda8D3fbbAhihcxPV4m9pump')

  const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    units: 1000000
  })
  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 5000
  })

  const markers = await ClaimMarker.gpaBuilder().run(connection)
  console.log('markets.length = ', markers.length)
  const claimed = await ClaimMarker.gpaBuilder().addFilter('isClaimed', true).run(connection)
  console.log('claimed.length = ', claimed.length)
  const unclaimed = await ClaimMarker.gpaBuilder().addFilter('isClaimed', false).run(connection)
  console.log('unclaimed.length = ', unclaimed.length)

  let claimed_sum = 0
  for (const c of claimed.values()) {
    const [account] = ClaimMarker.fromAccountInfo(c.account)
    // @ts-ignore
    claimed_sum += account.pretty().amount
  }

  let unclaimed_sum = 0
  for (const c of unclaimed.values()) {
    const [account] = ClaimMarker.fromAccountInfo(c.account)
    // @ts-ignore
    unclaimed_sum += account.pretty().amount
  }
  console.log('claimed_sum', claimed_sum / (LAMPORTS_PER_SOL / 1000))
  console.log('unclaimed_sum', unclaimed_sum / (LAMPORTS_PER_SOL / 1000))

  // const instruction = createAirDropperInstruction(admin.publicKey, mint)
  // const sigx = await processTransaction(
  //   [modifyComputeUnits, addPriorityFee, instruction],
  //   connection,
  //   admin
  // )
  // console.log('sigx', sigx)
  // const txnx = await connection.getParsedTransaction(
  //   sigx.Signature,
  //   'confirmed'
  // )
  // assert.equal(
  //   sigx.SignatureResult.err,
  //   null,
  //   `${mint.toBase58()}\n${txnx?.meta?.logMessages.join('\n')}`
  // )
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