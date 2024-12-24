import { PublicKey } from '@solana/web3.js'

const PROGRAM_ID = new PublicKey('AZMc26abaSP7si1wtLaV5yPxTxpWd895M8YpJFFdQ8Qw')

export function distributor(base: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync([
      Buffer.from(anchor.utils.bytes.utf8.encode('MerkleDistributor')),
      base.toBuffer()
    ],
    PROGRAM_ID)
}