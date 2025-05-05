import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HomeView from '@/views/home/HomeView.vue'
import InstructorsView from '@/views/InstructorsView.vue'
import MoreView from '@/views/MoreView.vue'
import ProfileView from '@/views/profile/ProfileView.vue'
import ScheduleView from '@/views/ScheduleView.vue'
import PhoneAuthView from '@/views/auth/PhoneAuthView.vue'
import TgAuthView from '@/views/auth/TgAuthView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/instructors',
    name: 'instructors',
    component: InstructorsView
  },
  {
    path: '/more',
    name: 'more',
    component: MoreView
  },
  {
    path: '/profile',
    name: 'profile',
    component: ProfileView
  },
  {
    path: '/schedule',
    name: 'schedule',
    component: ScheduleView
  },
  {
    path: '/auth',
    name: 'auth',
    component: PhoneAuthView
  },
  {
    path: '/tg-auth',
    name: 'tg-auth',
    component: TgAuthView
  }
  // {
  //   path: '/about',
  //   name: 'about',
  //   // route level code-splitting
  //   // this generates a separate chunk (about.[hash].js) for this route
  //   // which is lazy-loaded when the route is visited.
  //   component: () => import(/* webpackChunkName: "about" */ '../views/AboutView.vue')
  // }
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
