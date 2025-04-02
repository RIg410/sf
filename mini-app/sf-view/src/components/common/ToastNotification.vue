<template>
    <v-snackbar v-model="toastState.show" :color="toastState.type" :timeout="toastState.timeout"
        :location="mapPosition(toastState.position)">
        <div class="d-flex align-center">
            <v-icon :icon="getIcon(toastState.type)" class="mr-2" />
            <span>{{ toastState.message }}</span>
        </div>

        <template v-slot:actions v-if="toastState.closable">
            <v-btn icon="mdi-close" @click="closeToast" />
        </template>
    </v-snackbar>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useToast } from '@/composables/useToast';

export default defineComponent({
    name: 'ToastNotification',
    setup() {
        const { toastState, closeToast } = useToast();

        /**
         * Maps our position format to Vuetify location format
         */
        const mapPosition = (position: string): string => {
            const positionMap: Record<string, string> = {
                'top': 'top',
                'bottom': 'bottom',
                'top-right': 'top right',
                'top-left': 'top left',
                'bottom-right': 'bottom right',
                'bottom-left': 'bottom left'
            };
            return positionMap[position] || 'bottom';
        };

        /**
         * Get the icon for the toast type
         */
        const getIcon = (type: string): string => {
            const iconMap: Record<string, string> = {
                'success': 'mdi-check-circle',
                'info': 'mdi-information',
                'warning': 'mdi-alert',
                'error': 'mdi-alert-circle'
            };
            return iconMap[type] || 'mdi-information';
        };

        return {
            toastState,
            closeToast,
            mapPosition,
            getIcon
        };
    }
});
</script>

<style scoped>
/* Additional styling if needed */
</style>