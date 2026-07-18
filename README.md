# sysmon-cli

> Real-time system monitoring and notification for DevOps teams.

## Overview

sysmon-cli is a command-line system monitor written in Rust, designed to provide real-time system metrics and notifications to DevOps teams. It addresses the need for continuous monitoring and immediate alerts on system performance, resource usage, and other critical metrics. By automating the process of collecting and analyzing system data, sysmon-cli enables teams to stay on top of system health, optimize resource allocation, and reduce downtime. With its modular architecture and extensible design, sysmon-cli is an invaluable tool for any organization relying on reliable and efficient system operations.

## Features

- **Real-time Metrics**: Collects and displays system metrics in real-time, including CPU usage, memory usage, disk usage, and network I/O.
- **Customizable Notifications**: Sends notifications based on system metrics, including email, SMS, and webhooks, to ensure prompt action on system issues.
- **Modular Architecture**: Designed with a modular architecture, allowing for easy addition of new metrics and notification channels.
- **Extensible Design**: Built with extensibility in mind, enabling users to add custom metrics and notifications as needed.
- **Command-Line Interface**: Provides a user-friendly command-line interface for easy interaction and configuration.
- **Highly Configurable**: Offers extensive configuration options to tailor the system monitor to specific needs and environments.
- **Robust Error Handling**: Includes robust error handling and logging to ensure reliable operation and minimize downtime.
- **Test-Driven Development**: Built with test-driven development principles, ensuring high-quality code and reliable operations.

## Getting Started

### Prerequisites

- Rust (version 1.64 or higher)
- Cargo (version 1.64 or higher)

### Installation

```bash
git clone https://github.com/username/sysmon-cli.git
cd sysmon-cli
cargo build
cargo run
```

### Usage

```bash
# Display system metrics
sysmon-cli metrics

# Set up email notifications
sysmon-cli notifications email --username your-email@example.com --password your-password

# Add custom metric
sysmon-cli metrics add --name custom_metric --expression "some_expression"
```

## Architecture

The sysmon-cli project is structured into three main modules:

- **src/main.rs**: The main entry point and command-line interface.
- **src/metrics.rs**: The module for collecting and processing system metrics.
- **src/notifications.rs**: The module for sending notifications based on system metrics.

The project configuration file is located at **Cargo.toml**.

## API Reference

sysmon-cli provides the following public interfaces:

- **sysmon_cli::Metrics**: Provides methods for collecting and processing system metrics.
- **sysmon_cli::Notifications**: Provides methods for sending notifications based on system metrics.

## Testing

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit changes
4. Push and open a PR

## License

MIT License

Note: This README file is subject to change as the project evolves. Please check the latest version on the project's GitHub page for the most up-to-date information.