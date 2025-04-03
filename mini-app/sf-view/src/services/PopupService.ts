import { ref, shallowRef } from 'vue';

type PopupType = 'ok' | 'confirm';
type PopupResult = boolean | null;
type PopupCallback = (result: boolean) => void;

interface PopupOptions {
    text: string;
    type?: PopupType;
    closeOnOverlayClick?: boolean;
    onConfirm?: () => void;
    onCancel?: () => void;
}

class PopupService {
    private visible = ref(false);
    private text = ref('');
    private type = ref<PopupType>('ok');
    private closeOnOverlayClick = ref(true);
    private resolvePromise: ((value: PopupResult) => void) | null = null;
    private onConfirmCallback = shallowRef<(() => void) | null>(null);
    private onCancelCallback = shallowRef<(() => void) | null>(null);

    getVisible() {
        return this.visible;
    }

    getText() {
        return this.text;
    }

    getType() {
        return this.type;
    }

    getCloseOnOverlayClick() {
        return this.closeOnOverlayClick;
    }

    getOnConfirm() {
        return this.onConfirmCallback.value;
    }

    getOnCancel() {
        return this.onCancelCallback.value;
    }

    showOk(text: string, options: Partial<Omit<PopupOptions, 'text' | 'type'>> = {}): Promise<void> {
        return this.show({
            text,
            type: 'ok',
            ...options
        }).then(() => {
            return;
        });
    }

    showConfirm(text: string, options: Partial<Omit<PopupOptions, 'text' | 'type'>> = {}): Promise<boolean> {
        return this.show({
            text,
            type: 'confirm',
            ...options
        }).then((result) => {
            return result === true;
        });
    }

    private show(options: PopupOptions): Promise<PopupResult> {
        return new Promise((resolve) => {
            this.text.value = options.text;
            this.type.value = options.type || 'ok';
            this.closeOnOverlayClick.value = options.closeOnOverlayClick !== undefined ? options.closeOnOverlayClick : true;
            this.onConfirmCallback.value = options.onConfirm || null;
            this.onCancelCallback.value = options.onCancel || null;
            this.resolvePromise = resolve;
            this.visible.value = true;
        });
    }

    handleConfirm() {
        if (this.onConfirmCallback.value) {
            this.onConfirmCallback.value();
        }
        if (this.resolvePromise) {
            this.resolvePromise(true);
        }
        this.close();
    }

    handleCancel() {
        if (this.onCancelCallback.value) {
            this.onCancelCallback.value();
        }
        if (this.resolvePromise) {
            this.resolvePromise(false);
        }
        this.close();
    }

    handleClose() {
        if (this.type.value === 'ok' && this.resolvePromise) {
            this.resolvePromise(null);
        } else if (this.resolvePromise) {
            this.resolvePromise(false);
        }
        this.close();
    }

    private close() {
        this.visible.value = false;
        this.resolvePromise = null;
    }
}

export const popupService = new PopupService();