export class Api {
    /**
     * @param {string} name
     */
    async greet(name) {
        const data = await post("/api/greet", { name });
        return new GreetingResponse(data.name, data.greeting);
    }
}

export class GreetingResponse {
    constructor(name, greeting) {
        this.name = String(name);
        this.greeting = String(greeting);
    }
}

async function get(url) {
    return await apiFetch("GET", url);
}

async function post(url, payload) {
    return await apiFetch("POST", url, payload);
}

async function apiFetch(method, url, payload) {
    try {
        const response = await fetch(url, {
            method: method,
            headers: {
                "Content-Type": "application/json",
                Accept: "application/json",
            },
            body: JSON.stringify(payload),
        });
        return await response.json();
    } catch (err) {
        console.log(err);
    }
}
