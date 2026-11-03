# BOAF
### Buffer Overflow Automated Framework

**BOAF (Buffer Overflow Automated Framework)** is a high-performance network testing and progressive linear fuzzing framework developed in native Rust. It is designed to assess service stability, evaluate input-handling behaviour, and examine target resilience under varying payload conditions.

The framework supports both raw binary TCP communication and structured HTTP interactions, including encrypted SSL/TLS connections. It provides configurable testing modes, dynamic request simulation, and flexible protocol handling to facilitate controlled security assessments and authorised robustness testing.

## Core Features

- **Smart Target Resolver:** Automatically normalises target addresses, handles protocol identification, and applies dynamic port selection with support for HTTP (80) and HTTPS (443) configurations.

- **TLS Sandbox Support:** Provides configurable TLS certificate verification handling, including support for testing environments that use self-signed certificates.

- **Dynamic Header Simulation:** Incorporates a request randomisation engine capable of cycling through different client fingerprint and User-Agent configurations during testing.

- **Multi-Encapsulation Engine:** Supports two distinct communication modes:
  - **Raw Binary Mode:** Enables direct binary payload transmission over TCP connections.
  - **Structured HTTP Mode:** Facilitates testing through standard HTTP request and header structures.

- **Progressive Linear Fuzzing:** Implements a progressive input-testing approach to evaluate service behaviour and identify potential stability issues across varying payload sizes and configurations.

- **Native Rust Implementation:** Leverages Rust's performance, memory safety guarantees, and concurrency capabilities to provide an efficient testing environment.

## Intended Use Cases

BOAF is designed to support:

- Authorised network and service robustness assessments.
- Buffer overflow research and input-validation testing.
- Controlled fuzzing experiments.
- CTF and cybersecurity laboratory environments.
- Protocol handling and service stability evaluation.
- Security research and defensive verification.

## Developer & Author

**Biswadeb Mukherjee**  
Security Engineer & Researcher

- **Website:** https://official-biswadeb941.in
- **Project:** BOAF (Buffer Overflow Automated Framework)

## Disclaimer

BOAF is developed exclusively for authorised security research, educational purposes, controlled laboratory experiments, and defensive security assessments.

Users are responsible for ensuring that they have explicit permission to test any system or network using this framework. Unauthorised access, disruption, exploitation, or testing of third-party infrastructure is strictly prohibited.

The developer assumes no responsibility for misuse, unauthorised activities, or damages resulting from the use of this software.

**Use responsibly. Test ethically. Research continuously.**