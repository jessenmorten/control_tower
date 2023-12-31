export class Api {
    async getServices() {
        const data = await get("/api/services");
        const services = [];
        for (const service of data) {
            services.push(new Service(service.name, service.status, service.dependencies));
        }
        return services;
    }
}

export class Service {
    constructor(name, status, dependencies) {
        this.name = String(name);
        this.status = String(status);
        this.dependencies = dependencies;
    }

    isHealthy() {
        return this.status === "Healthy";
    }
}

async function get(url) {
    return await apiFetch("GET", url);
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
        console.error(err);
        throw err;
    }
}
