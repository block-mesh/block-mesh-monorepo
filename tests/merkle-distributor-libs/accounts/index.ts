export * from './AirDropper'
export * from './ClaimMarker'
export * from './ClaimStatus'
export * from './MerkleDistributor'

import { AirDropper } from './AirDropper'
import { ClaimMarker } from './ClaimMarker'
import { ClaimStatus } from './ClaimStatus'
import { MerkleDistributor } from './MerkleDistributor'

export const accountProviders = {
  AirDropper,
  ClaimMarker,
  ClaimStatus,
  MerkleDistributor,
}
