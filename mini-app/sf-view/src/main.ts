import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import { is_telegram_context, tg_init } from '@/services/tg'

tg_init()

if (is_telegram_context()) {
    console.log("telegram context");
} else {
    console.log("not telegram context")
}

createApp(App).use(router).mount('#app')
