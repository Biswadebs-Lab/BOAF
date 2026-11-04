# BOAF
### Buffer Overflow Automated Framework

**A Rust-powered framework for network fuzzing and automated buffer-overflow testing.**

[![CI](https://github.com/Biswadebs-Lab/BOAF/actions/workflows/ci.yml/badge.svg)](https://github.com/Biswadebs-Lab/BOAF/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/Biswadebs-Lab/BOAF)](https://github.com/Biswadebs-Lab/BOAF/releases)
[![License](https://img.shields.io/github/license/Biswadebs-Lab/BOAF)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/Biswadebs-Lab/BOAF?style=flat)](https://github.com/Biswadebs-Lab/BOAF/stargazers)

---

## Overview

BOAF is a native Rust framework designed for network fuzzing, buffer-overflow testing, and service stability assessment.

It provides a unified command-line interface for testing how network services handle varying input sizes and payload conditions. By combining raw TCP communication, structured HTTP requests, progressive fuzzing, and target availability verification, BOAF offers researchers a configurable environment for controlled robustness assessments.

The framework is designed around two execution strategies:

- **Raw mode:** Direct binary-streaming communication over TCP.
- **HTTP mode:** Structured HTTP POST requests for application-layer testing.

BOAF also provides configurable concurrency, TLS handling, and a static payload injection workflow.

**Built for security researchers, penetration testers, and developers working on application and network robustness.**

## Core Capabilities

### Network Testing

- **Target Resolution:** Normalises hostnames, IP addresses, and URL inputs, with automatic port selection where applicable.
- **Raw TCP Communication:** Supports direct binary-streaming operations against TCP services.
- **Structured HTTP Testing:** Sends HTTP POST requests with configurable target parameters and rotating User-Agent values.
- **TLS Support:** Supports encrypted connections, with an optional setting for accepting self-signed or otherwise invalid certificates in controlled environments.

### Fuzzing Engine

- **Progressive Payload Testing:** Increases payload sizes across configurable testing intervals.
- **Concurrent Execution:** Uses a configurable worker pool to execute fuzzing operations.
- **Crash Boundary Assessment:** Uses configurable reconnection attempts and delays to verify target availability after failed sends.
- **Configurable Test Parameters:** Controls starting payload size, maximum payload size, increment size, and worker count.

### Static Payload Injection

BOAF includes a single-shot static injection profile for targeted buffer-overflow research.

The profile supports configurable payload construction parameters, including:

- Buffer offset
- Return address
- NOP sled
- Shellcode

This functionality is intended exclusively for controlled, authorised testing environments.

## Architecture and Execution

BOAF exposes two primary execution modes and two testing workflows.

| Component | Purpose |
|---|---|
| Raw mode | Direct TCP binary streaming |
| HTTP mode | Structured HTTP POST communication |
| Progressive fuzzing | Incremental payload-size testing |
| Static injection | Single-shot payload injection |
| Worker pool | Concurrent fuzzing execution |
| Down-check mechanism | Target availability verification |
| TLS handling | Encrypted connection support |

The execution strategy and testing parameters can be configured through command-line arguments.

## Installation

### Download a Prebuilt Release

Precompiled binaries are available through GitHub Releases.

**[View BOAF Releases](https://github.com/Biswadebs-Lab/BOAF/releases)**

#### Linux (x86_64)

```bash
wget -O boaf https://github.com/Biswadebs-Lab/BOAF/releases/latest/download/boaf-linux-x86_64

chmod +x boaf

./boaf --version
```

#### Windows (x86_64)

```cmd
curl.exe -L -o boaf.exe https://github.com/Biswadebs-Lab/BOAF/releases/latest/download/boaf-windows-x86_64.exe

boaf.exe --version
```

### Build from Source

BOAF is written in Rust and uses Cargo for dependency management and compilation.

**Prerequisites:**
- Rust toolchain
- Cargo

Clone the repository:

```bash
git clone https://github.com/Biswadebs-Lab/BOAF.git

cd BOAF
```

Build the release binary:

```bash
cargo build --release
```

Run BOAF:

```bash
./target/release/boaf --help
```

## Quick Start

All examples below assume a controlled testing environment where you have explicit authorisation.

### 1. Inspect Available Options

```bash
boaf --help
```

### 2. Progressive Fuzzing

Run progressive fuzzing against an HTTP service:

```bash
boaf \
  --target http://192.168.1.50 \
  --mode http \
  --fuzz \
  --workers 8
```

### 3. Raw TCP Testing

Run the default raw execution workflow against a laboratory TCP service:

```bash
boaf --target 192.168.1.50:9000
```

### 4. TLS Testing

For a controlled environment using a self-signed certificate:

```bash
boaf \
  --target https://192.168.1.50 \
  --mode http \
  --allow-self-signed
```

**Note:** Accepting invalid certificates disables normal certificate validation for the relevant connection. Use this option only in controlled testing environments.

## Command-Line Reference

| Option | Default | Description |
|---|---|---|
| `-t, --target` | Required | Target hostname, IP address, or URL |
| `-m, --mode` | `raw` | Execution mode: `raw` or `http` |
| `-f, --fuzz` | Disabled | Enable progressive fuzzing |
| `--start-size` | `100` | Initial payload size |
| `--max-size` | `5000` | Maximum payload size |
| `--step` | `100` | Payload-size increment |
| `-w, --workers` | `4` | Concurrent worker threads |
| `--down-check-attempts` | `3` | Target availability verification attempts |
| `--down-check-delay-ms` | `500` | Delay between availability checks |
| `--allow-self-signed` | Disabled | Accept invalid or self-signed TLS certificates |
| `-h, --help` | — | Display help information |
| `-V, --version` | — | Display version information |

Payload-size and worker settings apply to progressive fuzzing. Other execution details depend on the selected mode and implementation.


The release workflow builds the configured platform binaries and publishes the release assets.

Refer to the [Releases](https://github.com/Biswadebs-Lab/BOAF/releases) page for available versions and downloads.

## Intended Use

BOAF is intended for:

- Authorised security assessments.
- Network service robustness testing.
- Buffer-overflow research.
- Controlled fuzzing experiments.
- CTF and isolated laboratory environments.
- Protocol input-handling evaluation.

## Security and Responsible Use

BOAF is a security research and testing framework.

Only use it against systems and networks for which you have explicit authorisation. Unauthorised testing may disrupt services, affect availability, or cause unintended damage.

Researchers should use isolated laboratory environments whenever possible, particularly when evaluating static payload injection functionality.

The developer does not authorise unlawful use of this software and assumes no responsibility for misuse.

## Contributing

Contributions, technical reviews, bug reports, and feature suggestions are welcome.

Before submitting a contribution:

1. Review the existing implementation.
2. Test changes in an authorised laboratory environment.
3. Include reproducible steps for reported bugs.
4. Provide clear descriptions of proposed changes.

Use the repository's GitHub Issues and Pull Requests to participate.

## License

BOAF is distributed under the [MIT License](LICENSE).

## Author

**Biswadeb Mukherjee**  
Security Engineer & Researcher

Website: [official-biswadeb941.in](https://official-biswadeb941.in)

GitHub: [@Biswadebs-Lab](https://github.com/Biswadebs-Lab)

---

**BOAF — Built for security research. Engineered in Rust.**