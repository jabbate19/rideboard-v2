import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import HistoryView from '../views/HistoryView.vue'

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

async function isAuthenticated() {
  return await fetch("/api/v1/auth/").then(resp => resp.status == 200)
}

router.beforeEach(async (to, from, next) => {
  let authenticated = await isAuthenticated();

  if (to.matched.some(record => record.path == '/login')) {
    next();
  } else {
    if (!authenticated) {
      next('/login');  // Redirect to login if not authenticated
    } else {
      next();  // Allow access
    }
  }
});

export default router
