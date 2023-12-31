import { Api, Service } from "./api.js";
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs";

function hasUnhealthyDependency(service, services) {
    return service.dependencies.some((dependency) => {
        const dependencyService = services.find((s) => s.name === dependency);
        if (!dependencyService) return false;
        return !dependencyService.isHealthy();
    });
}

async function main() {
    const main = document.querySelector("main");
    const api = new Api();
    const services = await api.getServices();
    const newElement = document.createElement("div");
    newElement.classList.add("mermaid");
    let mermaidText = "flowchart TD\n";
    for (const service of services) {
        const unhealthyDependency = hasUnhealthyDependency(service, services);

        if (unhealthyDependency) {
            mermaidText += `  ${service.name}(${service.name}):::UnhealthyDependency\n`;
        } else {
            mermaidText += `  ${service.name}(${service.name}):::${service.status}\n`;
        }

        for (const dependency of service.dependencies) {
            mermaidText += `  ${service.name} -.-> ${dependency}\n`;
        }

        if (!service.isHealthy() || unhealthyDependency) {
            mermaidText += `  class ${service.name} animate-pulse\n`;
        }
    }

    mermaidText +=
        "classDef Healthy font-weight:bold,color:#022c22,fill:#4ade80,stroke:#022c22,stroke-width:2px;\n";
    mermaidText +=
        "classDef Unhealthy font-weight:bold,color:#450a0a,fill:#ef4444,stroke:#450a0a,stroke-width:2px;\n";
    mermaidText +=
        "classDef UnhealthyDependency font-weight:bold,color:#422006,fill:#facc15,stroke:#422006,stroke-width:2px;\n";
    newElement.innerHTML = mermaidText;
    main.innerHTML = "";
    main.appendChild(newElement);

    await mermaid.run({
        querySelector: ".mermaid",
    });
    newElement.firstChild.classList.add("mx-auto");
}

mermaid.initialize({ startOnLoad: false });
setInterval(main, 5_000);
main();
