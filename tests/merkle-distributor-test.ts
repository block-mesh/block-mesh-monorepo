import * as anchor from '@coral-xyz/anchor'
import * as fs from 'fs'
import { MerkleDistributor } from '../target/types/merkle_distributor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import assert from 'assert'
import {
  airdrop, findDataInMerkle,
  getOrCreateTokenAccountInstruction, getWalletBalance, MerkleOutput,
  processTransaction, walletMap
} from './helpers'
import {
  createAssociatedTokenAccount,
  createMint,
  getAssociatedTokenAddress,
  mintTo
} from '@solana/spl-token'
import { Program } from '@coral-xyz/anchor'
import {
  claimMarkerTransactionInstruction,
  createAirDropperTransactionInstruction,
  createClaimStatusTransactionInstruction, createCloseTokenAccountWithFeeTransactionInstruction,
  createDistributorTransactionInstruction, createFeeCollectorTransactionInstruction,
  createMarkerTransactionInstruction,
  createReclaimTransactionInstruction
} from './merkle-distributor-helpers/wrapper'
import {
  getAirDropperAddress, getClaimMarkerAccount, getClaimMarkerAddress, getClaimMarkerTokenAccount,
  getDistributorAccount,
  getDistributorTokenAccount, getFeeCollectorAddress
} from './merkle-distributor-helpers/pda'

export let merkle_json: MerkleOutput = null

export const admin = Keypair.generate()
export const users = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate()
]
export const tokenMintAuthority = Keypair.generate()
export let mint: PublicKey

export let keys: Keypair[] = []

export const user_for_close = Keypair.generate()

describe('0-prep', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.MerkleDistributor as Program<MerkleDistributor>

  it('Program ID', async () => {
    console.log(`program.id: ${program.programId.toBase58()}`)
  })

  it('Airdrops', async () => {
    for (const key of [...users, admin, user_for_close]) {
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

  it('create fee collector', async () => {
    const [feeCollector] = getFeeCollectorAddress()
    const accountPre = await program.provider.connection.getAccountInfo(feeCollector)
    assert(accountPre === null)

    const instruction = createFeeCollectorTransactionInstruction(admin.publicKey, 100)
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
    const accountPost = await program.provider.connection.getAccountInfo(feeCollector)
    assert(accountPost !== null)
  })

  it('close account with fee', async () => {
    await createAssociatedTokenAccount(program.provider.connection, user_for_close, mint, user_for_close.publicKey)
    const instruction = createCloseTokenAccountWithFeeTransactionInstruction(user_for_close.publicKey, mint)
    const sig = await processTransaction(
      [instruction],
      program.provider.connection,
      user_for_close
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


  it('Create air dropper', async () => {
    const [airDropperPre] = getAirDropperAddress()
    const accountPre = await program.provider.connection.getAccountInfo(airDropperPre)
    assert(accountPre === null)
    const instruction = createAirDropperTransactionInstruction(admin.publicKey, mint)
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
    const [airDropperPost] = getAirDropperAddress()
    const accountPost = await program.provider.connection.getAccountInfo(airDropperPost)
    assert(accountPost !== null)
  })


  it('create claims', async () => {
    for (let i = 0; i < users.length; i++) {
      const user = users[i]
      const amount = LAMPORTS_PER_SOL * (i + 1)
      const [claimMarkerPre] = getClaimMarkerAddress(user.publicKey)
      const accountPre = await program.provider.connection.getAccountInfo(claimMarkerPre)
      assert(accountPre === null)

      const instruction = createMarkerTransactionInstruction(admin.publicKey, mint, user.publicKey, amount)
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

      const [claimMarkerPost] = getClaimMarkerAddress(user.publicKey)
      const accountPost = await program.provider.connection.getAccountInfo(claimMarkerPost)
      assert(accountPost !== null)
    }
  })


  it('claim marker', async () => {
    for (let i = 0; i < users.length - 1; i++) {
      const user = users[i]
      const claimMarkerAccount = await getClaimMarkerAccount(program.provider.connection, user.publicKey)
      assert(claimMarkerAccount.pretty().isClaimed === false)
      const instruction = claimMarkerTransactionInstruction(user.publicKey, mint)
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
      const claimMarkerAccount2 = await getClaimMarkerAccount(program.provider.connection, user.publicKey)
      assert(claimMarkerAccount2.pretty().isClaimed === true)
      assert(claimMarkerAccount2.pretty().amount === balance)
    }
  })

  it('reclaim marker 1', async () => {
    const claimed_user = users[0]
    const claimMarkerAccount = await getClaimMarkerAccount(program.provider.connection, claimed_user.publicKey)
    assert(claimMarkerAccount.pretty().isClaimed === true)
    const [claimMarkerPre] = getClaimMarkerAddress(claimed_user.publicKey)
    const [claimMarkerTokenPre] = getClaimMarkerTokenAccount(mint, claimed_user.publicKey)
    const accountPre = await program.provider.connection.getAccountInfo(claimMarkerPre)
    assert(accountPre !== null)
    const accountTokenPre = await program.provider.connection.getAccountInfo(claimMarkerTokenPre)
    assert(accountTokenPre !== null)
    const claim_instruction = createReclaimTransactionInstruction(admin.publicKey, claimed_user.publicKey, mint)
    const sig = await processTransaction(
      [claim_instruction],
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
    const [claimMarkerPost] = getClaimMarkerAddress(claimed_user.publicKey)
    const [claimMarkerTokenPost] = getClaimMarkerTokenAccount(mint, claimed_user.publicKey)
    const accountPost = await program.provider.connection.getAccountInfo(claimMarkerPost)
    assert(accountPost === null)
    const accountTokenPost = await program.provider.connection.getAccountInfo(claimMarkerTokenPost)
    assert(accountTokenPost === null)
  })

  it('reclaim marker 2', async () => {
    const unclaimed_user = users[users.length - 1]
    const claimMarkerAccount = await getClaimMarkerAccount(program.provider.connection, unclaimed_user.publicKey)
    assert(claimMarkerAccount.pretty().isClaimed === false)
    const [claimMarkerPre] = getClaimMarkerAddress(unclaimed_user.publicKey)
    const [claimMarkerTokenPre] = getClaimMarkerTokenAccount(mint, unclaimed_user.publicKey)
    const accountPre = await program.provider.connection.getAccountInfo(claimMarkerPre)
    assert(accountPre !== null)
    const accountTokenPre = await program.provider.connection.getAccountInfo(claimMarkerTokenPre)
    assert(accountTokenPre !== null)
    const unclaim_instruction = createReclaimTransactionInstruction(admin.publicKey, unclaimed_user.publicKey, mint)
    const sig = await processTransaction(
      [unclaim_instruction],
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
    const [claimMarkerPost] = getClaimMarkerAddress(unclaimed_user.publicKey)
    const [claimMarkerTokenPost] = getClaimMarkerTokenAccount(mint, unclaimed_user.publicKey)
    const accountPost = await program.provider.connection.getAccountInfo(claimMarkerPost)
    assert(accountPost === null)
    const accountTokenPost = await program.provider.connection.getAccountInfo(claimMarkerTokenPost)
    assert(accountTokenPost === null)
  })

  it('create distributor', async () => {
    const cwd = process.cwd()
    const merkle_file = fs.readFileSync(`${cwd}/programs/merkle-distributor/test-merkle/merkle.json`).toString()
    merkle_json = JSON.parse(merkle_file)
    // console.log('merkle_json = ', merkle_json)
    const instruction = createDistributorTransactionInstruction(merkle_json.root,
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
    const [distributorTokenAccountAddress] = getDistributorTokenAccount(mint)
    const tokenBalance = await program.provider.connection.getTokenAccountBalance(
      distributorTokenAccountAddress,
      'confirmed'
    )
  })

  it('claim-by-users', async () => {
    const wallets = walletMap()
    for (const [_pubkey, key] of wallets.entries()) {
      await airdrop(program, key.publicKey, LAMPORTS_PER_SOL * 50_000)
    }
    for (const [_pubkey, key] of wallets.entries()) {
      const leaf = findDataInMerkle(key.publicKey, merkle_json)
      const distributorPre = await getDistributorAccount(program.provider.connection, mint)
      const pre = distributorPre.pretty().totalAmountClaimed as number
      const instruction = createClaimStatusTransactionInstruction(
        leaf.index,
        leaf.claimant.amount,
        leaf.proof,
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
      const distributorPost = await getDistributorAccount(program.provider.connection, mint)
      const post = distributorPost.pretty().totalAmountClaimed as number
      const balancePost = await getWalletBalance(program.provider.connection, key.publicKey, mint)
      assert(balancePost == leaf.claimant.amount)
      assert(post - pre == balancePost)
    }
  })
})
