import { Connection, PublicKey } from '@solana/web3.js'
import * as anchor from '@coral-xyz/anchor'
import { AirDropper, ClaimMarker, PROGRAM_ID } from '../merkle-distributor-libs'

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