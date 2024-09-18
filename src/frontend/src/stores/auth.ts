import { defineStore } from 'pinia'
import { type UserData } from '@/models'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    user: null as UserData | null
  }),
  actions: {
    setUser(userData: UserData) {
      this.user = userData
    },
    clearUser() {
      this.user = null
    }
  }
})
