<template>
    <div class="phone-home">
        <PopupView />

        <div class="logo-container">
            <img src="@/assets/logo.webp" alt="SoulFamily Logo" class="logo">
            <h1>Добро пожаловать в SoulFamily</h1>
        </div>

        <div class="auth-form">
            <template v-if="!isCodeSent">
                <div class="input-container">
                    <label for="phone-input" class="input-label">Номер телефона</label>
                    <input id="phone-input" v-model="phoneNumber" type="tel" class="text-input"
                        :class="{ 'input-error': phoneNumberError }" @blur="validatePhone" />
                    <div v-if="phoneNumberError" class="error-message">{{ phoneNumberError }}</div>
                </div>
                <button @click="sendAuthCode" class="text-button">
                    Отправить код
                </button>
            </template>

            <template v-else>
                <div class="input-container">
                    <label for="code-input" class="input-label">Проверочный код</label>
                    <input id="code-input" v-model="verificationCode" type="number" class="text-input"
                        :class="{ 'input-error': codeError }" @blur="validateCode" />
                    <div v-if="codeError" class="error-message">{{ codeError }}</div>
                </div>
                <div v-if="cooldownActive" class="countdown">
                    Код будет доступен для повторной отправки через {{ countdown }} секунд
                </div>
                <button v-if="!cooldownActive" @click="sendAuthCode" class="text-button">
                    Отправить код
                </button>
                <button @click="verifyCode" class="text-button">
                    Подтвердить
                </button>
                <button @click="goBack" class="text-button">
                    Попробовать другой номер
                </button>
            </template>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref, onBeforeUnmount, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { getAuthService } from '@/services/auth';
import { SendVerificationCodeError } from '@/generated/auth';
import PopupView from '@/components/common/PopupView.vue';
import { popupService } from '@/services/PopupService';

export default defineComponent({
    name: 'PhoneAuthView',
    components: {
        PopupView
    },
    setup() {
        const router = useRouter();
        const auth = getAuthService();

        const phoneNumber = ref('');
        const verificationCode = ref('');
        const isCodeSent = ref(false);
        const cooldownActive = ref(false);
        const countdown = ref(0);
        const phoneNumberError = ref('');
        const codeError = ref('');
        let countdownTimer: number | null = null;

        onMounted(async () => {
            if (auth.isAuthenticated()) {
                router.back();
            }
        });

        const validatePhone = () => {
            if (!phoneNumber.value) {
                phoneNumberError.value = 'Обязательное поле';
                return false;
            }

            const phonePattern = /^\+?[0-9]{10,15}$/;
            if (!phonePattern.test(phoneNumber.value)) {
                phoneNumberError.value = 'Неверный формат номера телефона. Пример: +79991234567';
                return false;
            }

            phoneNumberError.value = '';
            return true;
        };

        const validateCode = () => {
            if (!verificationCode.value) {
                codeError.value = 'Обязательное поле';
                return false;
            }
            codeError.value = '';
            return true;
        };

        const startCooldown = (seconds = 60) => {
            cooldownActive.value = true;
            countdown.value = seconds;

            countdownTimer = window.setInterval(() => {
                countdown.value--;
                if (countdown.value <= 0) {
                    cooldownActive.value = false;
                    if (countdownTimer) {
                        clearInterval(countdownTimer);
                        countdownTimer = null;
                    }
                }
            }, 1000);
        };

        const sendAuthCode = async () => {
            try {
                if (!validatePhone()) {
                    return;
                }
                const formattedPhone = phoneNumber.value.replace(/\s/g, '');

                const resp = await auth.sendVerificationCode(formattedPhone);

                if (resp) {
                    switch (resp.error) {
                        case SendVerificationCodeError.INVALID_PHONE_NUMBER:
                            await popupService.showOk("Пользователь не найден. Пожалуста свяжитесь с администратором.");
                            return;
                        case SendVerificationCodeError.V_USER_NOT_FOUND:
                            await popupService.showOk("Пользователь не найден. Пожалуста свяжитесь с администратором.");
                            return;
                        case SendVerificationCodeError.ALREADY_SENT:
                            isCodeSent.value = true;
                            startCooldown();
                            countdown.value = resp.leftTime ? resp.leftTime : 60;
                            await popupService.showOk(`Код уже был отправлен. Повторная отправка возможна через ${countdown.value} секунд.`);
                            return;
                        case SendVerificationCodeError.NOT_AVAILABLE:
                            await popupService.showOk("Данный тип авторизации не доступен. Пожалуста свяжитесь с администратором.");
                            return;
                        case SendVerificationCodeError.UNRECOGNIZED:
                            await popupService.showOk("Неизвестная ошибка");
                            return;
                        default:
                            break;
                    }
                }

                isCodeSent.value = true;
                startCooldown();
            } catch (error) {
                await popupService.showOk('Не удалось отправить код. Пожалуйста, проверьте номер телефона и попробуйте еще раз.');
                console.error('Error sending verification code:', error);
            }
        };

        const verifyCode = async () => {
            try {
                if (!validateCode()) {
                    return;
                }
                const formattedPhone = phoneNumber.value.replace(/\s/g, '');

                let err = await auth.verifyCode(formattedPhone, verificationCode.value);
                if (err) {
                    await popupService.showOk("Неверный код. Пожалуйста, проверьте код и попробуйте еще раз.");
                    return;
                }
                console.log('Code verified successfully');
                await router.back();
            } catch (error) {
                await popupService.showOk("Неверный код. Пожалуйста, проверьте код и попробуйте еще раз.");
                console.error('Error verifying code:', error);
            }
        };

        const goBack = () => {
            isCodeSent.value = false;
            verificationCode.value = '';
            codeError.value = '';
        };

        onBeforeUnmount(() => {
            if (countdownTimer) {
                clearInterval(countdownTimer);
                countdownTimer = null;
            }
        });

        return {
            phoneNumber,
            verificationCode,
            isCodeSent,
            cooldownActive,
            countdown,
            phoneNumberError,
            codeError,
            validatePhone,
            validateCode,
            sendAuthCode,
            verifyCode,
            goBack
        };
    }
});
</script>

<style scoped>
.phone-home {
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

.auth-form {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

h1 {
    font-size: 1.5rem;
    margin-bottom: 1rem;
    color: var(--primary-color);
}

.input-container {
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
    width: 100%;
}

.input-label {
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
    color: rgba(0, 0, 0, 0.6);
}

.text-input {
    padding: 12px 16px;
    border: 1px solid rgba(0, 0, 0, 0.23);
    border-radius: 4px;
    font-size: 1rem;
    transition: border-color 0.2s ease;
    outline: none;
    background-color: transparent;
}

.text-input:focus {
    border-color: var(--primary-color);
    box-shadow: 0 0 0 1px var(--primary-color);
}

.input-error {
    border-color: #ff5252;
}

.error-message {
    color: #ff5252;
    font-size: 0.75rem;
    margin-top: 4px;
}

.primary-button {
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 12px 16px;
    font-size: 1rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    cursor: pointer;
    transition: background-color 0.2s ease;
    width: 100%;
}

.primary-button:hover:not(:disabled) {
    background-color: var(--primary-dark-color);
}

.button-disabled {
    opacity: 0.65;
    cursor: not-allowed;
}

.text-button {
    background-color: transparent;
    color: var(--primary-color);
    border: none;
    border-radius: 4px;
    padding: 12px 16px;
    font-size: 1rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    cursor: pointer;
    transition: background-color 0.2s ease;
    width: 100%;
}

.text-button:hover {
    background-color: rgba(0, 0, 0, 0.04);
}

:root {
    --primary-color: #1976D2;
    --primary-dark-color: #1565C0;
}
</style>