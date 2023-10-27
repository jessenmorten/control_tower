# 🛩 control_tower

![License](https://img.shields.io/github/license/jessenmorten/control_tower)

A web application, written in Rust, that provides a clear and concise display of the health statuses of various services.

## Features

- **Service Health Monitoring:** Keeps track of the health status of services and provides real-time updates, allowing you to promptly identify and respond to issues.

- **Intuitive Visualization:** The application leverages [mermaid](https://mermaid.js.org/) to create visually appealing and interactive dependency charts, making it easier to grasp the relationships between services.

- **Customization:** You can configure the services to monitor and their dependencies, tailoring the application to your specific system architecture.

- **User-Friendly Interface:** The user interface is designed for ease of use, ensuring that you can quickly access the information you need.

## Getting Started

Follow these steps to get Control Tower up and running:

1. **Clone the Repository:**

   ```
   git clone https://github.com/jessenmorten/control_tower.git
   ```

2. **Build the Application:**

   ```
   cd control_tower
   cargo build
   ```

3. **Configure Services:**

   Edit the `config.json` file to define the services you want to monitor and their dependencies.

4. **Run control_tower:**

   ```
   cargo run
   ```

5. **Access the Dashboard:**

   Open your web browser and navigate to [http://localhost:3000](http://localhost:3000).