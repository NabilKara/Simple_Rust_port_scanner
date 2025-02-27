# Rust Port Scanner

A simple, fast, and concurrent port scanner written in Rust using Tokio for asynchronous networking.

## Features

- Scan individual IP addresses or CIDR ranges
- Concurrent scanning for maximum performance
- Customizable port ranges
- Clear terminal output with color formatting

## Installation

```bash
# Clone the repository
git clone https://github.com/NabilKara/Simple_Rust_port_scanner.git
cd rust-port-scanner

# Build the project
cargo build --release
```

## Dependencies

This project depends on:
- `clap` - Command-line argument parsing
- `tokio` - Asynchronous runtime
- `colored` - Terminal text coloring
- `cidr` - CIDR notation handling

## Usage

```bash
# Scan a single IP address
./port-scanner 192.168.1.1 --port-start 1 --port-end 1000

# Scan a CIDR range
./port-scanner --cidr 192.168.1.0/24 --port-start 80 --port-end 443
```

### Command-line Options

- `addr`: Target IP address (required unless `--cidr` is specified)
- `--cidr`: Target network in CIDR notation (conflicts with `addr`)
- `--port-start`: Starting port number (default: 1)
- `--port-end`: Ending port number (default: 1024)

## Examples

Scan common ports on a single host:
```bash
./port-scanner 192.168.1.1 --port-start 20 --port-end 100
```

Scan a specific port across a network:
```bash
./port-scanner --cidr 10.0.0.0/24 --port-start 80 --port-end 81
```

## How It Works

The scanner works by spawning a Tokio task for each port and target combination. Each task attempts to establish a TCP connection to the target port. If the connection succeeds, the port is marked as open.

## License

MIT

