import { Result } from '@badrap/result'
import axios from 'axios'

export interface ConnectWalletApiRequest {
  email: string;
  api_token: string;
  pubkey: string;
  message: string;
  signature: number[];
}

export async function connect_wallet(url: string, form: ConnectWalletApiRequest): Promise<Result<{
  status: number,
  message: string | null
}, Error>> {
  try {
    const response = await axios.post(url, form, {
      maxRedirects: 0,
      headers: {
        'Content-Type': 'application/json'
      }
    })
      .then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('Login error', e)
    return Result.err(e)
  }
}
