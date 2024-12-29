import { PublicKey, TransactionInstruction } from '@solana/web3.js'
import {
  ClaimMarkerInstructionAccounts,
  CreateAirDropperInstructionAccounts, createClaimMarkerInstruction,
  createCreateAirDropperInstruction,
  createCreateMarkerInstruction,
  CreateMarkerInstructionAccounts,
  CreateMarkerInstructionArgs
} from '../merkle-distributor-libs'
import { getAirDropperAddress, getClaimMarkerAddress, getClaimMarkerAddress2, getClaimMarkerTokenAccount } from './pda'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync
} from '@solana/spl-token'


export function createAirDropperInstruction(signer: PublicKey, mint: PublicKey): TransactionInstruction {
  const [airDropper, _] = getAirDropperAddress()
  const accounts: CreateAirDropperInstructionAccounts = {
    signer,
    airDropper,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }
  return createCreateAirDropperInstruction(accounts)
}


export function createMarker(signer: PublicKey, mint: PublicKey, claimant: PublicKey, amount: number): TransactionInstruction {
  const [airDropper] = getAirDropperAddress()
  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer)
  const [claimMarker] = getClaimMarkerAddress(claimant)
  const [claimMarkerTokenAccount] = getClaimMarkerTokenAccount(mint, claimant)
  const accounts: CreateMarkerInstructionAccounts = {
    signer,
    signerTokenAccount,
    claimant,
    airDropper,
    claimMarker,
    claimMarkerTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }
  const args: CreateMarkerInstructionArgs = {
    args: {
      amount
    }
  }
  return createCreateMarkerInstruction(accounts, args)
}

export function claimMarker(signer: PublicKey, mint: PublicKey): TransactionInstruction {
  const [airDropper] = getAirDropperAddress()
  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer)
  const [claimMarker] = getClaimMarkerAddress(signer)
  const [claimMarker2] = getClaimMarkerAddress2(signer)
  const [claimMarkerTokenAccount] = getClaimMarkerTokenAccount(mint, signer)

  const accounts: ClaimMarkerInstructionAccounts = {
    signer,
    signerTokenAccount,
    airDropper,
    claimMarker,
    claimMarker2,
    claimMarkerTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }

  return createClaimMarkerInstruction(accounts)
}