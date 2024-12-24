import { PublicKey } from '@solana/web3.js'
import * as anchor from '@coral-xyz/anchor'
import { PROGRAM_ID } from '../merkle-distributor-libs'

export function getAirDropperAddress(): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('AirDropper'))],
    PROGRAM_ID
  )
}

export function getClaimMarker(claimant: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('ClaimMarker')), claimant.toBuffer()],
    PROGRAM_ID
  )
}

export function getClaimMarkerTokenAccount(mint: PublicKey, claimant: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode('ClaimMarker')), mint.toBuffer(), claimant.toBuffer()],
    PROGRAM_ID
  )

}