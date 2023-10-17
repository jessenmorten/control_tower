import { Api } from "./api.js";

function createPingElement(color) {
  const ping = document.createElement("div");
  ping.classList.add("relative", "flex", "h-3", "w-3");
  const span1 = document.createElement("span");
  span1.classList.add(
    "animate-ping",
    "absolute",
    "inline-flex",
    "h-full",
    "w-full",
    "rounded-full",
    `bg-${color}-400`,
    "opacity-75",
  );
  const span2 = document.createElement("span");
  span2.classList.add(
    "relative",
    "inline-flex",
    "rounded-full",
    "h-3",
    "w-3",
    `bg-${color}-500`,
  );
  ping.appendChild(span1);
  ping.appendChild(span2);
  return ping;
}

/**
 * @param {Service} service
 * @returns {HTMLElement}
 */
function createServiceElement(service) {
  const name = document.createElement("h2");
  name.innerText = service.name;
  const pingColor = {
    Healthy: "green",
    Unhealthy: "red",
  };
  const ping = createPingElement(pingColor[service.status]);

  const container = document.createElement("div");
  container.classList.add(
    "flex",
    "flex-col",
    "justify-center",
    "items-center",
    "rounded-full",
    "border",
    "border-gray-400",
    "h-24",
    "w-24",
    "m-4",
  );
  container.appendChild(ping);
  container.appendChild(name);
  return container;
}

async function main() {
  const main = document.querySelector("main");
  main.classList.add("flex", "flex-wrap", "justify-center", "items-center");
  const api = new Api();
  const services = await api.getServices();

  main.innerHTML = "";
  for (const service of services) {
    main.appendChild(createServiceElement(service));
  }
}

setInterval(main, 3_000);
main();
