<template>
    <Teleport to="body">
        <div v-if="visible" class="popup-overlay" @click="handleOverlayClick">
            <div class="popup-container" @click.stop>
                <div class="popup-content">
                    <p class="popup-text">{{ text }}</p>

                    <div class="popup-buttons">
                        <button v-if="type === 'ok'" class="popup-button popup-button-primary" @click="handleOk">
                            OK
                        </button>

                        <template v-if="type === 'confirm'">
                            <button class="popup-button popup-button-secondary" @click="handleNo">
                                Нет
                            </button>
                            <button class="popup-button popup-button-primary" @click="handleYes">
                                Да
                            </button>
                        </template>
                    </div>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { popupService } from '@/services/PopupService';

export default defineComponent({
    name: 'PopupView',
    setup() {
        const handleOk = () => {
            popupService.handleClose();
        };

        const handleYes = () => {
            popupService.handleConfirm();
        };

        const handleNo = () => {
            popupService.handleCancel();
        };

        const handleOverlayClick = () => {
            if (popupService.getCloseOnOverlayClick().value) {
                popupService.handleClose();
            }
        };

        return {
            visible: popupService.getVisible(),
            text: popupService.getText(),
            type: popupService.getType(),
            handleOk,
            handleYes,
            handleNo,
            handleOverlayClick,
        };
    },
});
</script>

<style scoped>
.popup-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}

.popup-container {
    background-color: white;
    border-radius: 8px;
    padding: 24px;
    width: 90%;
    max-width: 400px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.popup-content {
    display: flex;
    flex-direction: column;
}

.popup-text {
    font-size: 16px;
    line-height: 1.5;
    margin-bottom: 20px;
    color: #333;
}

.popup-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.popup-button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.popup-button-primary {
    background-color: #4a6cfa;
    color: white;
}

.popup-button-primary:hover {
    background-color: #3a5ceb;
}

.popup-button-secondary {
    background-color: #f1f1f1;
    color: #333;
}

.popup-button-secondary:hover {
    background-color: #e1e1e1;
}
</style>