import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'

export const network = WalletAdapterNetwork.Mainnet
export const rpc = import.meta.env.VITE_RPC

export const BASE_URL = import.meta.env.VITE_BASE_URL

console.log('BASE_URL', BASE_URL)
