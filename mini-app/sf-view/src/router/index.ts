import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import InstructorsView from '@/views/InstructorsView.vue'
import MoreView from '@/views/MoreView.vue'
import ProfileView from '@/views/ProfileView.vue'
import ScheduleView from '@/views/ScheduleView.vue'

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
