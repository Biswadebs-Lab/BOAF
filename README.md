# BOAF
### Buffer Overflow Automated Framework

[![CI](https://github.com/Biswadebs-Lab/BOAF/actions/workflows/ci.yml/badge.svg)](https://github.com/Biswadebs-Lab/BOAF/actions/workflows/ci.yml)

BOAF is a high-performance network fuzzing and buffer-overflow testing framework written in native Rust. It assesses service stability, input-handling behaviour, and target resilience under varying payload conditions, over raw TCP or structured HTTP, with optional TLS.

## Features

- **Target resolver** — normalises host/URL input and selects the right port (80/443) automatically.
- **Raw & HTTP modes** — direct binary TCP streaming, or structured HTTP requests with a rotating User-Agent.
- **TLS support** — including a sandbox mode for self-signed certificates.
- **Progressive fuzzing** — sweeps payload sizes across a concurrent worker pool to locate instability boundaries.
- **Down-detection** — confirms a target is actually unresponsive (via reconnect retries) before reporting a crash, rather than treating a single dropped connection as proof.
- **Static injector** — a single-shot buffer-overflow payload profile (offset + return address + NOP sled + shellcode) for targeted testing.

## Installation

Grab the latest binary from [Releases](https://github.com/Biswadebs-Lab/BOAF/releases). Release assets are built automatically when a semantic-version tag such as `v1.0.2` is pushed, or fetch the latest release directly:

```bash
# Linux
wget -O boaf https://github.com/Biswadebs-Lab/BOAF/releases/latest/download/boaf-linux-x86_64
chmod +x boaf
```

```cmd
:: Windows (cmd.exe)
curl.exe -L -o boaf.exe https://github.com/Biswadebs-Lab/BOAF/releases/latest/download/boaf-windows-x86_64.exe
```

## Usage

```bash
boaf --target <host> [OPTIONS]
```

| Flag | Default | Description |
|---|---|---|
| `-t, --target <TARGET>` | — | Target host/URL (required) |
| `-m, --mode <raw\|http>` | `raw` | Wire encapsulation |
| `-f, --fuzz` | off | Run progressive fuzzing instead of the static injector |
| `--start-size <N>` | `100` | Starting payload size (fuzz mode) |
| `--max-size <N>` | `5000` | Maximum payload size (fuzz mode) |
| `--step <N>` | `100` | Size increment per iteration (fuzz mode) |
| `-w, --workers <N>` | `4` | Concurrent worker threads (fuzz mode) |
| `--down-check-attempts <N>` | `3` | Reconnect attempts before confirming the target is down |
| `--down-check-delay-ms <MS>` | `500` | Delay between down-check attempts |
| `--allow-self-signed` | `true` | Accept self-signed/invalid TLS certificates |
| `-h, --help` | — | Print help |

```bash
# Progressive fuzzing against an HTTP target with 8 workers
boaf --target https://192.168.1.50 --mode http --fuzz --workers 8

# Single-shot static injector over raw TCP
boaf --target 192.168.1.50:9000
```

## Intended Use

Authorised robustness assessments, buffer-overflow research, controlled fuzzing, CTF/lab environments, and protocol stability evaluation.

## License

This project is licensed under the [MIT License](LICENSE).

## Author

**Biswadeb Mukherjee** — Security Engineer & Researcher
[official-biswadeb941.in](https://official-biswadeb941.in)

## Disclaimer

BOAF is built exclusively for authorised security research, education, and controlled testing. You are responsible for obtaining explicit permission before testing any system or network. Unauthorised use is strictly prohibited; the developer assumes no liability for misuse.

**Use responsibly. Test ethically.**
