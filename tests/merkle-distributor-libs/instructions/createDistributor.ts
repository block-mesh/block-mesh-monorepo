/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  CreateDistributorArgs,
  createDistributorArgsBeet,
} from '../types/CreateDistributorArgs'

/**
 * @category Instructions
 * @category CreateDistributor
 * @category generated
 */
export type CreateDistributorInstructionArgs = {
  args: CreateDistributorArgs
}
/**
 * @category Instructions
 * @category CreateDistributor
 * @category generated
 */
export const createDistributorStruct = new beet.BeetArgsStruct<
  CreateDistributorInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['args', createDistributorArgsBeet],
  ],
  'CreateDistributorInstructionArgs'
)
/**
 * Accounts required by the _createDistributor_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] signerTokenAccount
 * @property [_writable_] distributor
 * @property [_writable_] distributorTokenAccount
 * @property [] mint
 * @property [] associatedTokenProgram
 * @category Instructions
 * @category CreateDistributor
 * @category generated
 */
export type CreateDistributorInstructionAccounts = {
  signer: web3.PublicKey
  signerTokenAccount: web3.PublicKey
  distributor: web3.PublicKey
  distributorTokenAccount: web3.PublicKey
  mint: web3.PublicKey
  systemProgram?: web3.PublicKey
  tokenProgram?: web3.PublicKey
  associatedTokenProgram: web3.PublicKey
  rent?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const createDistributorInstructionDiscriminator = [
  184, 103, 26, 71, 141, 64, 49, 177,
]

/**
 * Creates a _CreateDistributor_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CreateDistributor
 * @category generated
 */
export function createCreateDistributorInstruction(
  accounts: CreateDistributorInstructionAccounts,
  args: CreateDistributorInstructionArgs,
  programId = new web3.PublicKey('AZMc26abaSP7si1wtLaV5yPxTxpWd895M8YpJFFdQ8Qw')
) {
  const [data] = createDistributorStruct.serialize({
    instructionDiscriminator: createDistributorInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.signerTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.distributor,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.distributorTokenAccount,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.mint,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.associatedTokenProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}