import { PublicKey } from '@solana/web3.js'
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'

export const mint = new PublicKey(import.meta.env.VITE_MINT)

export const network = WalletAdapterNetwork.Mainnet
export const rpc = import.meta.env.VITE_RPC

console.log('rpc', rpc, ' mint', mint.toBase58())
