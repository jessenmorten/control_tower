import { Api } from "./api.js";

async function main() {
    const api = new Api();
    const start = Date.now();
    const greeting = await api.greet("Tower");
    const elapsed = Date.now() - start;
    document.body.innerHTML += `
        <h4>${greeting.greeting}</h4>
        <p>It took ${elapsed}ms to get this greeting.</p>
    `;
}

main();
