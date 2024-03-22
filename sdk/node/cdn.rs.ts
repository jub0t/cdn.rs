import { request } from 'undici'
import { JsonHeader } from "./misc"

export default class RustNetwork {
    private url: String;
    private token?: String;
    private userInfo?: {
        username: String
    }

    constructor(url: String) {
        this.url = url;
    }

    // Misc
    async getDiagnostics() {
        const {
            body
        } = await request(`${this.url}/diagnostics`)
        const data = await body.json()
        return data;
    }

    // Authentication
    async authenticate(username: String, password: String) {
        const {
            body
        } = await request(`${this.url}/authenticate`, {
            method: "POST",
            headers: {
                ...JsonHeader
            },
            body: JSON.stringify({
                username,
                password
            })
        })

        const data: any = await body.json()

        if (data.success) {
            this.token = data.token;
            this.userInfo = {
                username,
            }
        }

        return data;
    }

    async createUser(username: String, password: String) {
        const {
            body
        } = await request(`${this.url}/create-user`, {
            method: "POST",
            headers: {
                ...JsonHeader
            },
            body: JSON.stringify({
                username,
                password
            })
        })

        const data = await body.json()
        return data;
    }

    //  Other
    async getLatency() {
        const start = process.hrtime.bigint();

        const {
            body
        } = await request(`${this.url}`)

        const _ = await body.json()

        const time = (process.hrtime.bigint() - start);

        return time;
    }
}