import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import HistoryView from '../views/HistoryView.vue'
import { useAuthStore } from '@/stores/auth'
import { type UserData } from '@/models'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/history',
      name: 'history',
      component: HistoryView
    }
  ]
})

router.beforeEach(async (to, _from, next) => {
  const authStore = useAuthStore() // Access the auth store

  try {
    await fetch('/api/v1/auth/')
      .then(async (response) => {
        if (response.status != 200) {
          throw Error('Bad Return Code')
        }
        const jsonData: UserData = await response.json()
        return jsonData
      })
      .then((jsonData) => {
        if (jsonData.type == 'CSH') {
          jsonData.picture = `https://profiles.csh.rit.edu/image/${jsonData.preferred_username}`
        }
        authStore.setUser(jsonData)
        next()
      })
  } catch (error) {
    console.error('Error fetching data:', error)
    if (to.matched.some((record) => record.path == '/login')) {
      next()
    } else {
      next('/login')
    }
  }
})

export default router
