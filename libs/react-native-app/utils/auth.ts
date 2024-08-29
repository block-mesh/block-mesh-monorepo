import { Result } from '@badrap/result'
import axios from 'axios'
import { GetTokenRequest, GetTokenResponse, LoginForm, RegisterForm, RegisterResponse } from '@/utils/apiTypes'

export async function register(url: string, form: RegisterForm): Promise<Result<RegisterResponse, Error>> {
  try {
    const response = await axios.post(url, form, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      withCredentials: true
    }).then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('Register error', e)
    return Result.err(e)
  }
}

export async function login(url: string,
                            form: LoginForm): Promise<Result<null, Error>> {
  try {
    const response = await axios.post(url, form, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      withCredentials: true
    })
    if (response.status < 400) {
      return Result.ok(null)
    } else {
      console.error('Login response is ', response.status)
      return Result.err(new Error(`Unexpected response status ${response.status}`))
    }
  } catch (e: any) {
    console.error('Login error', e)
    return Result.err(e)
  }
}

export async function get_token(url: string, body: GetTokenRequest):
  Promise<Result<GetTokenResponse, Error>> {
  try {
    const response = await axios.post(url, body, {
      headers: {
        'Content-Type': 'application/json'
      },
      withCredentials: true
    }).then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('GetToken error', e)
    return Result.err(e)
  }
}