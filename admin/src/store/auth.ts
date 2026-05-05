import { create } from 'zustand'
import { persist } from 'zustand/middleware'

interface UserInfo {
  id: number
  name: string
  email: string
  is_admin: boolean
  edition: string
}

interface AuthState {
  token: string | null
  userInfo: UserInfo | null
  setToken: (token: string) => void
  setUserInfo: (userInfo: UserInfo) => void
  clearToken: () => void
  isAdmin: () => boolean
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      token: null,
      userInfo: null,
      setToken: (token) => set({ token }),
      setUserInfo: (userInfo) => set({ userInfo }),
      clearToken: () => set({ token: null, userInfo: null }),
      isAdmin: () => get().userInfo?.is_admin ?? false,
    }),
    {
      name: 'auth-storage',
    }
  )
)
