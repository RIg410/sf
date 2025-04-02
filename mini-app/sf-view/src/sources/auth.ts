import { is_telegram_context } from "./tg";


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
