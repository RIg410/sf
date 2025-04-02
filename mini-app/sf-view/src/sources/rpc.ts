interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}

export class FetchRpc implements Rpc {
    private readonly baseUrl: string;
    private readonly token: string | null;

    constructor(baseUrl: string, token: string | null) {
        this.token = token;
        this.baseUrl = baseUrl;
    }

    async request(service: string, method: string, data: Uint8Array): Promise<Uint8Array> {
        
        const response = await fetch(`${this.baseUrl}/${service}/${method}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/proto',
                ...this.token ? {'Authorization': `Bearer ${this.token}`} : {}
            },
            body: data,
        });
        if (this.token) {
            response.headers.append('Authorization', `Bearer ${this.token}`);
        }
    
        if (!response.ok) {
            throw new Error(`HTTP Error: ${response.status}, ${response.statusText}`);
        }

        return new Uint8Array(await response.arrayBuffer());
    }
}