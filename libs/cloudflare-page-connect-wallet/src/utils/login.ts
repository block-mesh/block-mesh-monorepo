import { Result } from '@badrap/result'
import axios from 'axios'

export interface LoginForm {
  email: string;
  password: string;
}

export async function get_api_token(url: string, form: LoginForm): Promise<Result<{
  api_token: string,
  message: string | null
}, Error>> {
  try {
    let final_url = url.includes('app') ? url.replace('app', 'api') : url
    final_url = final_url.includes('8000') ? final_url.replace('8000', '8001') : final_url
    console.log('get_api_token : final_url = ', final_url)
    const response = await axios.post(final_url, form, {
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
