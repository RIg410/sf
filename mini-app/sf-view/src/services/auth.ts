import { is_telegram_context, initData } from "./tg";
import { SendVerificationCodeResponse, TgAuthError } from "@/generated/auth";
import { getGRPC, initGRPC } from "./grpc";

export class Auth {
    private token: string | null;
    private auth_type: AuthType;

    constructor() {
        this.token = get_auth_token();
        this.auth_type = auth_type();
    }

    getToken(): string | null {
        return this.token;
    }

    getAuthType(): AuthType {
        return this.auth_type;
    }

    isAuthenticated(): boolean {
        return this.token !== null;
    }

    checkAuthStatus() {
        this.token = get_auth_token();
    }

    setToken(token: string | null) {
        this.token = token;
        if (token) {
            localStorage.setItem('token', token);
        } else {
            localStorage.removeItem('token');
        }
        initGRPC();
    }

    async authThroughTelegram(): Promise<string | null> {
        const auth_client = getGRPC().authService;

        if (this.isAuthenticated()) {
            return null;
        }
        if (this.auth_type !== "telegram") {
            return "Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.";
        }

        try {
            const result = await auth_client.tg_auth({
                key: initData(),
            });
            if (result.error) {
                const error = result.error;
                console.error("Telegram authentication error:", result.error);
                switch (+error) {
                    case TgAuthError.INVALID_TOKEN:
                        return "Недействительный токен авторизации.";
                    case TgAuthError.TOO_OLD_TOKEN:
                        return "Истек срок действия токена.";
                    default:
                        return "Неизвестная ошибка авторизации.";
                }
            } else {
                if (result.token) {
                    this.setToken(result.token);
                    return null;
                } else {
                    console.error("Telegram authentication error: No token received.");
                    return "Не удалось авторизоваться.";
                }
            }

        } catch (e) {
            console.error("Error during Telegram authentication:", e);
            return "Не удалось авторизоваться.";
        }
    }


    async sendVerificationCode(phone: string): Promise<SendVerificationCodeResponse | null> {
        if (this.auth_type !== "phone") {
            throw new Error("Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.");
        }
        const auth_client = getGRPC().authService;

        return await auth_client.send_verification_code({
            phoneNumber: phone,
        });
    }

    async verifyCode(phone: string, code: string): Promise<string | null> {
        if (this.auth_type !== "phone") {
            throw new Error("Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.");
        }
        const auth_client = getGRPC().authService;

        const result = await auth_client.verify_code({
            phoneNumber: phone,
            code: code,
        });

        if (result.error) {
            console.error("Verification error:", result.error);
            return "Неверный код подтверждения.";
        } else {
            if (result.token) {
                this.setToken(result.token);
                return null;
            } else {
                console.error("Telegram authentication error: No token received.");
                return "Не удалось авторизоваться. Попробуйте позже.";
            }
        }

    }
}

export type AuthType = "telegram" | "phone";

function auth_type(): AuthType {
    if (is_telegram_context()) {
        return "telegram";
    } else {
        return "phone";
    }
}

function get_auth_token(): string | null {
    const value = localStorage.getItem('token');
    if (!value) return null;
    return value;
}

const auth = new Auth();

export function getAuthService(): Auth {
    return auth;
}