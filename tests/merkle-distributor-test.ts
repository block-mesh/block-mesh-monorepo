import * as anchor from '@coral-xyz/anchor'
import * as fs from 'fs'
import { MerkleDistributor } from '../target/types/merkle_distributor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import assert from 'assert'
import {
  airdrop,
  getOrCreateTokenAccountInstruction, getWalletBalance, loadWalletKey,
  processTransaction
} from './helpers'
import {
  createMint,
  getAssociatedTokenAddress,
  mintTo
} from '@solana/spl-token'
import { Program } from '@coral-xyz/anchor'
import {
  claimMarker,
  createAirDropperInstruction, createClaimStatusInstruction,
  createDistributorInstruction,
  createMarker
} from './merkle-distributor-helpers/wrapper'
import { getDistributorAccount, getDistributorTokenAccount } from './merkle-distributor-helpers/pda'

export type Claimant = {
  claimant: number[];
  amount: number;
}

export type Leaf = {
  index: number;
  proof: number[];
  claimant: Claimant;
  leaves_to_prove: number[][];
}

export type MerkleOutput = {
  root: number[];
  leafs: Leaf[];
}

export let merkle_json: MerkleOutput = null

export const admin = Keypair.generate()
export const users = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate()
]
export const tokenMintAuthority = Keypair.generate()
export let mint: PublicKey
export let token9Decimals: PublicKey

export let keys: Keypair[] = []

describe('0-prep', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.MerkleDistributor as Program<MerkleDistributor>

  it('Program ID', async () => {
    console.log(`program.id: ${program.programId.toBase58()}`)
  })

  it('Airdrops', async () => {
    for (const key of [...users, admin]) {
      await airdrop(program, key.publicKey, LAMPORTS_PER_SOL * 50_000)
    }
  })

  it('Create main mint', async () => {
    mint = await createMint(
      program.provider.connection,
      admin,
      tokenMintAuthority.publicKey,
      tokenMintAuthority.publicKey,
      9
    )
  })

  it('Mint tokens', async () => {
    for (const key of [admin]) {
      const instructions = await getOrCreateTokenAccountInstruction(
        mint,
        key.publicKey,
        program.provider.connection
      )
      if (instructions === null) {
        continue
      }
      const sig = await processTransaction(
        [instructions],
        program.provider.connection,
        key
      )
      const txn = await program.provider.connection.getParsedTransaction(
        sig.Signature,
        'confirmed'
      )
      assert.equal(
        sig.SignatureResult.err,
        null,
        `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
      )

      await mintTo(
        program.provider.connection,
        admin,
        mint,
        await getAssociatedTokenAddress(mint, key.publicKey),
        tokenMintAuthority,
        LAMPORTS_PER_SOL * 50_000
      )
    }
  })


  it('Create air dropper', async () => {
    const instruction = createAirDropperInstruction(admin.publicKey, mint)
    const sig = await processTransaction(
      [instruction],
      program.provider.connection,
      admin
    )
    const txn = await program.provider.connection.getParsedTransaction(
      sig.Signature,
      'confirmed'
    )
    assert.equal(
      sig.SignatureResult.err,
      null,
      `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
    )
  })


  it('create claims', async () => {
    for (let i = 0; i < users.length; i++) {
      const user = users[i]
      const amount = LAMPORTS_PER_SOL * (i + 1)
      const instruction = createMarker(admin.publicKey, mint, user.publicKey, amount)
      const sig = await processTransaction(
        [instruction],
        program.provider.connection,
        admin
      )
      const txn = await program.provider.connection.getParsedTransaction(
        sig.Signature,
        'confirmed'
      )
      assert.equal(
        sig.SignatureResult.err,
        null,
        `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
      )
    }
  })


  it('claim marker', async () => {
    for (let i = 0; i < users.length; i++) {
      const user = users[i]
      const instruction = claimMarker(user.publicKey, mint)
      const sig = await processTransaction(
        [instruction],
        program.provider.connection,
        user
      )
      const txn = await program.provider.connection.getParsedTransaction(
        sig.Signature,
        'confirmed'
      )
      assert.equal(
        sig.SignatureResult.err,
        null,
        `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
      )
      const balance = await getWalletBalance(program.provider.connection, user.publicKey, mint)
      console.log('collect claims user', user.publicKey.toBase58(), ' balance', balance)
    }
  })

  it('create distributor', async () => {
    const cwd = process.cwd()
    const merkle_file = fs.readFileSync(`${cwd}/programs/merkle-distributor/test-merkle/merkle.json`).toString()
    merkle_json = JSON.parse(merkle_file)
    console.log('merkle_json = ', merkle_json)
    const instruction = createDistributorInstruction(merkle_json.root,
      LAMPORTS_PER_SOL * 1_000, 12,
      admin.publicKey,
      mint,
      merkle_json.leafs.length
    )
    const sig = await processTransaction(
      [instruction],
      program.provider.connection,
      admin
    )
    const txn = await program.provider.connection.getParsedTransaction(
      sig.Signature,
      'confirmed'
    )
    assert.equal(
      sig.SignatureResult.err,
      null,
      `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
    )

    const distributor = await getDistributorAccount(program.provider.connection, mint)
    console.log('distributor = ', distributor.pretty())
    const [distributorTokenAccountAddress] = getDistributorTokenAccount(mint)
    const tokenBalance = await program.provider.connection.getTokenAccountBalance(
      distributorTokenAccountAddress,
      'confirmed'
    )
    console.log('tokenBalance = ', parseInt(tokenBalance.value.amount))
  })

  it('claim-by-users', async () => {
    const cwd = process.cwd()
    for (let i = 1; i <= 12; i++) {
      let key = loadWalletKey(`${cwd}/programs/merkle-distributor/test-keys/${i}.json`)
      await airdrop(program, key.publicKey, LAMPORTS_PER_SOL * 50_000)
      keys.push(key)
    }

    for (let index = 0; index < keys.length; index++) {
      const key = keys[index]
      const leaf = merkle_json.leafs[index]
      const proof = leaf.proof
      const instruction = createClaimStatusInstruction(
        index,
        leaf.claimant.amount,
        proof,
        key.publicKey,
        mint,
        leaf.leaves_to_prove
      )
      const sig = await processTransaction(
        [instruction],
        program.provider.connection,
        key
      )
      const txn = await program.provider.connection.getParsedTransaction(
        sig.Signature,
        'confirmed'
      )
      assert.equal(
        sig.SignatureResult.err,
        null,
        `${mint.toBase58()}\n${txn?.meta?.logMessages.join('\n')}`
      )
      const distributor = await getDistributorAccount(program.provider.connection, mint)
      console.log('distributor = ', distributor.pretty())
    }
  })
})
