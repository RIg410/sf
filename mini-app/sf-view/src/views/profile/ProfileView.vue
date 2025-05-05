<template>
    <div class="profile">
        <div class="profile-header">
            <h1>Личный кабинет</h1>
        </div>

    </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from 'vue';
import { usersStore } from '@/store/me';

export default defineComponent({
    name: 'ProfileView',
    setup() {
        const store = usersStore();

        const refreshUserData = () => {
            store.fetchMe();
        };

        const formatDate = (timestamp: number) => {
            if (!timestamp) return 'Никогда';
            return new Date(timestamp).toLocaleString();
        };

        onMounted(() => {
            if (!store.me || Date.now() - store.lastUpdate > 5 * 60 * 1000) {
                refreshUserData();
            }
        });

        return {
            store,
            refreshUserData,
            formatDate
        };
    }
});
</script>

<style scoped>
.profile {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.profile-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
}

.refresh-button {
    padding: 8px 16px;
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.refresh-button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
}

.loading-indicator {
    text-align: center;
    margin: 40px 0;
    color: #666;
}

.error-message {
    background-color: #ffebee;
    border-left: 4px solid #f44336;
    padding: 15px;
    margin: 20px 0;
    border-radius: 4px;
}

.retry-button {
    padding: 6px 12px;
    background-color: #f44336;
    color: white;
    border: none;
    border-radius: 4px;
    margin-top: 10px;
    cursor: pointer;
}

.user-data {
    background-color: #f9f9f9;
    border-radius: 8px;
    padding: 20px;
}

.user-info-section {
    margin-bottom: 20px;
}

.user-info-item {
    margin: 10px 0;
    padding-bottom: 10px;
    border-bottom: 1px solid #eee;
}

.label {
    font-weight: bold;
    margin-right: 10px;
    min-width: 100px;
    display: inline-block;
}

.update-info {
    font-size: 0.8em;
    text-align: right;
    color: #999;
    margin-top: 20px;
}

.no-data {
    text-align: center;
    margin: 40px 0;
    color: #666;
}
</style>