import { Result } from '@badrap/result'
import axios from 'axios'
import {
  CheckTokenRequest,
  DashboardRequest,
  DashboardResponse,
  GetTokenRequest,
  GetTokenResponse,
  LoginForm,
  RegisterForm,
  RegisterResponse
} from '@/utils/apiTypes'
import { router, Router } from 'expo-router'

export async function register(url: string, form: RegisterForm): Promise<Result<RegisterResponse, Error>> {
  try {
    const response = await axios.post(url, form, {
      maxRedirects: 0,
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
      maxRedirects: 0,
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
      maxRedirects: 0,
      headers: {
        'Content-Type': 'application/json'
      },
      withCredentials: true
    }).then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('GetToken error', url, body, e)
    return Result.err(e)
  }
}

export async function dashboard(url: string, body: DashboardRequest): Promise<Result<DashboardResponse, Error>> {
  try {
    const response = await axios.post(url, body, {
      maxRedirects: 0,
      headers: {
        'Content-Type': 'application/json'
      },
      withCredentials: true
    }).then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('Dashboard error', url, e)
    return Result.err(e)
  }
}

export async function check_token(url: string, body: CheckTokenRequest): Promise<Result<GetTokenResponse>> {
  try {
    const response = await axios.post(url, body, {
      maxRedirects: 0,
      headers: {
        'Content-Type': 'application/json'
      },
      withCredentials: true
    }).then((res: { data: any }) => res.data)
    return Result.ok(response)
  } catch (e: any) {
    console.error('check_token error', url, body, e)
    return Result.err(e)
  }
}

export type CheckTokenWithCallbacks = {
  url: string;
  email: string;
  password: string;
  api_token: undefined | string;
  setApiToken: (api_token: string) => void
}

export async function check_token_with_callbacks({
                                                   url,
                                                   email,
                                                   password,
                                                   api_token,
                                                   setApiToken
                                                 }: CheckTokenWithCallbacks) {
  if (api_token) {
    const token_check = await check_token(url + '/api/check_token', { email, api_token })
    if (token_check.isOk) {
      router.push('/DashboardScreen')
      return
    }
  }
  const r = await get_token(
    url + '/api/get_token',
    {
      email: email,
      password: password
    })
  if (r.isOk) {
    const token = r.unwrap().api_token
    setApiToken(token)
    router.push('/DashboardScreen')
  }
}