import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { MerkleDistributor } from '../../target/types/merkle_distributor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from '@solana/web3.js'
import assert from 'assert'
import {
  airdrop,
  generateHashInput,
  getOrCreateTokenAccountInstruction,
  processTransaction
} from '../helpers'
import {
  createMint,
  getAssociatedTokenAddress,
  mintTo
} from '@solana/spl-token'
import { distributor } from './pda'

export const tokenMintAuthority = Keypair.generate()
export let mint: PublicKey
export let token9Decimals: PublicKey

export const admin = Keypair.generate()

export const users = [
  Keypair.generate(),
  Keypair.generate(),
  Keypair.generate()
]

describe('merkle-test', () => {
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

  it('Create distributor', async () => {
    const root = []
    const base = Keypair.generate()
    const [distributor, bump] = distributor(base.publicKey)

    const create_inst = program.methods.createDistributor({

    }).accounts(
      ({
          base,
          distributor,
          mint,
          payer: admin.publicKey,
          systemProgram: SystemProgram.programId,
        }
    ).instruction();

  })


})