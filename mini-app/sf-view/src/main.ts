import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import { is_telegram_context, tg_init } from '@/services/tg'
import { createPinia } from 'pinia'

tg_init()

if (is_telegram_context()) {
    console.log("telegram context");
} else {
    console.log("not telegram context")
}
const pinia = createPinia()

createApp(App).use(pinia).use(router).mount('#app')
