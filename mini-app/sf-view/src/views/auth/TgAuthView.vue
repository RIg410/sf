<template>
    <div class="tg-auth">
        <div class="logo-container">
            <img src="@/assets/logo.webp" alt="SoulFamily Logo" class="logo">
            <h1>Добро пожаловать в SoulFamily</h1>
        </div>

        <div class="auth-form">
            <div v-if="authError" class="error-message">{{ authError }}</div>
            <div v-if="authError" class="error-message">Свяжитесь с администратором 🔧</div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { getAuthService } from '@/services/auth';

export default defineComponent({
    name: 'TgAuthView',
    components: {},
    setup() {
        const router = useRouter();
        const authError = ref('');
        const auth = getAuthService();

        onMounted(async () => {
            const result = await auth.authThroughTelegram();
            if (result) {
                authError.value = result;
            } else {
                await router.replace("/");
            }
        });

        return {
            authError
        };
    }
});
</script>

<style scoped>
.tg-auth {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem;
    max-width: 480px;
    margin: 0 auto;
}

.logo-container {
    text-align: center;
    margin-bottom: 2rem;
}

.logo {
    width: 120px;
    height: auto;
    margin-bottom: 1rem;
}

h1 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: var(--primary-color);
}

.error-message {
    color: #ff5252;
    font-size: 1rem;
    margin-top: 10px;
}

:root {
    --primary-color: #1976D2;
    --primary-dark-color: #1565C0;
}
</style>