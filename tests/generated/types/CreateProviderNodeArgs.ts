/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type CreateProviderNodeArgs = {
  ipv4: number[] /* size: 4 */
  proxyPort: number
  clientPort: number
  reportBandwidthLimit: beet.bignum
}

/**
 * @category userTypes
 * @category generated
 */
export const createProviderNodeArgsBeet =
  new beet.BeetArgsStruct<CreateProviderNodeArgs>(
    [
      ['ipv4', beet.uniformFixedSizeArray(beet.u8, 4)],
      ['proxyPort', beet.u16],
      ['clientPort', beet.u16],
      ['reportBandwidthLimit', beet.u64],
    ],
    'CreateProviderNodeArgs'
  )
