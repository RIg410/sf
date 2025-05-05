
export function getToken(): string | null {
    const token = localStorage.getItem("token");
    return token ? token : null;
}

export function setToken(token: string) {
    localStorage.setItem("token", token);
}

