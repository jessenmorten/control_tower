import { Api, Service } from "./api.js";
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';

async function main() {
    const main = document.querySelector("main");
    const api = new Api();
    const services = await api.getServices();
    const newElement = document.createElement("div");
    newElement.classList.add("mermaid");
    let mermaidText = "flowchart TD\n";
    for (const service of services) {
        mermaidText += `  ${service.name}(${service.name}):::${service.status}\n`;
        for (const dependency of service.dependencies) {
            mermaidText += `  ${service.name}:::${service.status} -.-> ${dependency}\n`;
        }
    }
    mermaidText += "classDef Healthy color:white,fill:green,stroke:#333,stroke-width:2px;\n";
    mermaidText += "classDef Unhealthy color:white,fill:red,stroke:#333,stroke-width:2px;\n";
    newElement.innerHTML = mermaidText;
    main.innerHTML = "";
    main.appendChild(newElement);

    await mermaid.run({
      querySelector: '.mermaid',
    });
}

mermaid.initialize({ startOnLoad: false });
setInterval(main, 5_000);
main();
