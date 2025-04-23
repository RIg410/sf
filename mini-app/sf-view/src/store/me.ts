import { UserView } from '@/generated/user'
import { getGRPC } from '@/services/grpc'
import { defineStore } from 'pinia'

export const usersStore = defineStore('me', {
    state: () => {
        return {
            me: null as UserView | null,
            lastUpdate: 0,
            loading: false,
            error: null as string | null,
        }
    },
    actions: {
        async fetchMe() {
            const userClient = getGRPC().userService;
            this.loading = true
            this.error = null
            try {
                const user = await userClient.get({ id: undefined });
                this.me = user
                this.lastUpdate = Date.now();

            } catch (error) {
                console.error("Error fetching user:", error);
                this.error = "Не удалось загрузить данные пользователя. Пожалуйста, проверьте подключение к интернету и попробуйте еще раз";
            } finally {
                this.loading = false
            }
        },
    },
})