import { PublicKey, TransactionInstruction } from '@solana/web3.js'
import {
  ClaimInstructionAccounts,
  ClaimInstructionArgs,
  ClaimMarkerInstructionAccounts,
  CreateAirDropperInstructionAccounts, createClaimInstruction, createClaimMarkerInstruction,
  createCreateAirDropperInstruction, createCreateDistributorInstruction,
  createCreateMarkerInstruction, CreateDistributorInstructionAccounts, CreateDistributorInstructionArgs,
  CreateMarkerInstructionAccounts,
  CreateMarkerInstructionArgs, createReclaimMarkerInstruction, ReclaimMarkerInstructionAccounts
} from '../merkle-distributor-libs'
import {
  getAirDropperAddress,
  getClaimMarkerAddress,
  getClaimMarkerAddress2,
  getClaimMarkerTokenAccount, getClaimStatusAddress,
  getDistributorAddress, getDistributorTokenAccount
} from './pda'
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync
} from '@solana/spl-token'

export function createReclaimInstruction(
  signer: PublicKey,
  claimant: PublicKey,
  mint: PublicKey
): TransactionInstruction {
  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer)
  const [airDropper] = getAirDropperAddress()
  const [claimMarker] = getClaimMarkerAddress(claimant)
  const [claimMarkerTokenAccount] = getClaimMarkerTokenAccount(mint, claimant)
  const [claimMarker2] = getClaimMarkerAddress2(claimant)
  const accounts: ReclaimMarkerInstructionAccounts = {
    signer,
    signerTokenAccount,
    claimant,
    airDropper,
    claimMarker,
    claimMarker2,
    claimMarkerTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }
  return createReclaimMarkerInstruction(accounts)
}

export function createDistributorInstruction(
  root: number[],
  maxTotalClaim: number,
  maxNumNodes: number,
  signer: PublicKey,
  mint: PublicKey,
  leavesLen: number
): TransactionInstruction {

  const args: CreateDistributorInstructionArgs = {
    args: {
      root,
      maxTotalClaim,
      maxNumNodes,
      leavesLen
    }
  }

  const signerTokenAccount = getAssociatedTokenAddressSync(mint, signer)
  const [distributor] = getDistributorAddress(mint)
  const [distributorTokenAccount] = getDistributorTokenAccount(mint)

  const accounts: CreateDistributorInstructionAccounts = {
    signer,
    signerTokenAccount,
    distributor,
    distributorTokenAccount,
    mint,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  }

  return createCreateDistributorInstruction(accounts, args)
}

export function createClaimStatusInstruction(
  index: number,
  amount: number,
  proof: number[],
  signer: PublicKey,
  mint: PublicKey,
  leavesToProve: number[][]
): TransactionInstruction {

  const args: ClaimInstructionArgs = {
    args: {
      index,
      amount,
      proof: new Uint8Array(proof),
      leavesToProve: leavesToProve.map(i => new Uint8Array(i))
    }
  }

  const claimant = signer
  const claimantTokenAccount = getAssociatedTokenAddressSync(mint, signer)
  const [distributor] = getDistributorAddress(mint)
  const [distributorTokenAccount] = getDistributorTokenAccount(mint)
  const [claimStatus] = getClaimStatusAddress(mint, claimant)

  const accounts: ClaimInstructionAccounts = {
    claimant,
    claimantTokenAccount,
    distributor,
    distributorTokenAccount,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    mint,
    claimStatus
  }

  return createClaimInstruction(accounts, args)
}

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