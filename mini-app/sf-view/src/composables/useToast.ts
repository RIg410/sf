import { reactive } from 'vue';

interface ToastOptions {
    message: string;
    type: 'success' | 'info' | 'warning' | 'error';
    timeout?: number;
    position?: 'top' | 'bottom' | 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
    closable?: boolean;
}

interface ToastState {
    show: boolean;
    message: string;
    type: 'success' | 'info' | 'warning' | 'error';
    timeout: number;
    position: 'top' | 'bottom' | 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left';
    closable: boolean;
}

const toastState = reactive<ToastState>({
    show: false,
    message: '',
    type: 'info',
    timeout: 5000,
    position: 'bottom',
    closable: true
});

let toastTimer: number | null = null;


export function useToast() {
    /**
     * Display a toast notification
     * @param message The message to display
     * @param type The notification type (success, info, warning, error)
     * @param options Additional options for the toast
     */
    const showToast = (
        message: string,
        type: 'success' | 'info' | 'warning' | 'error' = 'info',
        options: Partial<Omit<ToastOptions, 'message' | 'type'>> = {}
    ) => {
        // Clear any existing timer
        if (toastTimer !== null) {
            clearTimeout(toastTimer);
            toastTimer = null;
        }

        // Update toast state
        Object.assign(toastState, {
            message,
            type,
            show: true,
            timeout: options.timeout !== undefined ? options.timeout : 5000,
            position: options.position || 'bottom',
            closable: options.closable !== undefined ? options.closable : true
        });

        // Set up auto-dismiss if timeout > 0
        if (toastState.timeout > 0) {
            toastTimer = window.setTimeout(() => {
                closeToast();
            }, toastState.timeout);
        }
    };

    /**
     * Close the currently shown toast notification
     */
    const closeToast = () => {
        toastState.show = false;

        if (toastTimer !== null) {
            clearTimeout(toastTimer);
            toastTimer = null;
        }
    };

    /**
     * Helper methods for specific toast types
     */
    const success = (message: string, options: Partial<Omit<ToastOptions, 'message' | 'type'>> = {}) =>
        showToast(message, 'success', options);

    const info = (message: string, options: Partial<Omit<ToastOptions, 'message' | 'type'>> = {}) =>
        showToast(message, 'info', options);

    const warning = (message: string, options: Partial<Omit<ToastOptions, 'message' | 'type'>> = {}) =>
        showToast(message, 'warning', options);

    const error = (message: string, options: Partial<Omit<ToastOptions, 'message' | 'type'>> = {}) =>
        showToast(message, 'error', options);

    return {
        // State (reactive)
        toastState,

        // Methods
        showToast,
        closeToast,

        // Shorthand methods
        success,
        info,
        warning,
        error
    };
}