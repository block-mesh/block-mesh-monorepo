import { Connection, PublicKey } from '@solana/web3.js'
import * as anchor from '@coral-xyz/anchor'
import { AirDropper, ClaimMarker, ClaimStatus, MerkleDistributor, PROGRAM_ID } from '../merkle-distributor-libs'

export function getClaimStatusAddress(mint: PublicKey, claimant: PublicKey): [PublicKey, number] {
  const [distributor] = getDistributorAddress(mint)
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode('ClaimStatus')),
      distributor.toBuffer(),
      claimant.toBuffer()
    ],
    PROGRAM_ID
  )
}

export async function getClaimStatusAccount(connection: Connection, mint: PublicKey, claimant: PublicKey): Promise<ClaimStatus> {
  const [claim_status] = getClaimStatusAddress(mint, claimant)
  return await ClaimStatus.fromAccountAddress(connection, claim_status)
}

export function getDistributorAddress(mint: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode('MerkleDistributor')),
      mint.toBuffer()
    ],
    PROGRAM_ID
  )
}

export async function getDistributorAccount(connection: Connection, mint: PublicKey): Promise<MerkleDistributor> {
  const [distributor] = getDistributorAddress(mint)
  return await MerkleDistributor.fromAccountAddress(connection, distributor)
}

export function getDistributorTokenAccount(mint: PublicKey): [PublicKey, number] {
  const [distributor] = getDistributorAddress(mint)
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode('MerkleDistributor')),
      distributor.toBuffer()
    ],
    PROGRAM_ID
  )
}

export function getAirDropperAddress(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('AirDropper'))],
    PROGRAM_ID
  )
}

export async function getAirDropperAccount(connection: Connection): Promise<AirDropper> {
  const [airDropper] = getAirDropperAddress()
  return await AirDropper.fromAccountAddress(connection, airDropper)
}

export function getClaimMarkerAddress(claimant: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('ClaimMarker')), claimant.toBuffer()],
    PROGRAM_ID
  )
}

export function getClaimMarkerAddress2(claimant: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('ClaimMarker2')), claimant.toBuffer()],
    PROGRAM_ID
  )
}

export async function getClaimMarkerAccount(connection: Connection, claimant: PublicKey): Promise<ClaimMarker> {
  const [claimMarker] = getClaimMarkerAddress(claimant)
  return await ClaimMarker.fromAccountAddress(connection, claimMarker)
}

export function getClaimMarkerTokenAccount(mint: PublicKey, claimant: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('ClaimMarker')), mint.toBuffer(), claimant.toBuffer()],
    PROGRAM_ID
  )
}