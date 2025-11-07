import { createRouter, createWebHistory } from 'vue-router'
import SystemMonitor from '@/views/SystemMonitor.vue'
import Settings from '@/views/Settings.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'SystemMonitor',
      component: SystemMonitor
    },
    {
      path: '/settings',
      name: 'Settings',
      component: Settings
    }
  ]
})

router.afterEach((to) => {
  if (typeof document === 'undefined') return
  const isSettings = to.name === 'Settings'
  document.body.classList.toggle('settings-page', isSettings)
})

export default router
