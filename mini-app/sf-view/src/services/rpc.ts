interface Rpc {
    request(service: string, method: string, data: Uint8Array): Promise<Uint8Array>;
}

export class FetchRpc implements Rpc {
    private readonly baseUrl: string;
    private readonly token: string | null;


    constructor(token: string | null) {
        this.token = token;
        //this.baseUrl = "http://localhost:3000";
        this.baseUrl = "api";
    }

    async request(service: string, method: string, data: Uint8Array): Promise<Uint8Array> {

        console.log({
            method: 'POST',
            headers: {
                'Content-Type': 'application/proto',
                ...this.token ? { 'Authorization': `Bearer ${this.token}` } : {}
            },
            body: data,
        });
        const response = await fetch(`${this.baseUrl}/${service}/${method}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/proto',
                ...this.token ? { 'Authorization': `Bearer ${this.token}` } : {}
            },
            body: data,
        });

        if (!response.ok) {
            throw new Error(`HTTP Error: ${response.status}, ${response.statusText}`);
        }

        return new Uint8Array(await response.arrayBuffer());
    }
}