import { grpc } from "@improbable-eng/grpc-web";
import { is_telegram_context, initData } from "./tg";
import { AuthService, AuthServiceClientImpl, GrpcWebImpl, SendVerificationCodeResponse, TgAuthError, VerifyCodeError } from "@/generated/auth";

export class Auth {
    private token: string | null;
    private auth_type: AuthType;
    private auth_service: AuthService | null;
    private rpc_client: GrpcWebImpl;


    constructor() {
        this.token = get_auth_token();
        this.auth_type = auth_type();
        this.auth_service = null;
        const metadata = new grpc.Metadata();
        metadata.set("Authorization", this.token ? `Bearer ${this.token}` : "");
        this.rpc_client = new GrpcWebImpl("api", {
            metadata,
        });
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

    getRpcClient(): GrpcWebImpl {
        return this.rpc_client;
    }

    setToken(token: string | null) {
        this.token = token;
        const metadata = new grpc.Metadata();
        metadata.set("Authorization", this.token ? `Bearer ${this.token}` : "");
        this.rpc_client = new GrpcWebImpl("api", {
            metadata,
        });
        if (token) {
            localStorage.setItem('token', token);
        } else {
            localStorage.removeItem('token');
        }
    }

    async authThroughTelegram(): Promise<string | null> {
        if (this.isAuthenticated()) {
            return null;
        }
        if (this.auth_type !== "telegram") {
            return "Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.";
        }
        if (this.auth_service === null) {
            this.auth_service = new AuthServiceClientImpl(this.rpc_client);
        }

        try {
            const result = await this.auth_service.tg_auth({
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
                    return "Не удалось авторизоваться. Попробуйте позже.";
                }
            }

        } catch (e) {
            console.error("Error during Telegram authentication:", e);
            return "Не удалось авторизоваться. Попробуйте позже.";
        }
    }


    async sendVerificationCode(phone: string): Promise<SendVerificationCodeResponse | null> {
        if (this.auth_type !== "phone") {
            throw new Error("Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.");
        }
        if (this.auth_service === null) {
            this.auth_service = new AuthServiceClientImpl(this.rpc_client);
        }

        return await this.auth_service.send_verification_code({
            phoneNumber: phone,
        });
    }

    async verifyCode(phone: string, code: string): Promise<string | null> {
        if (this.auth_type !== "phone") {
            throw new Error("Неизвестный тип авторизации. Пожалуйста, перезайдите в приложение.");
        }
        if (this.auth_service === null) {
            this.auth_service = new AuthServiceClientImpl(this.rpc_client);
        }

        const result = await this.auth_service.verify_code({
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