# NyxLink

A peer-to-peer communication system designed for high-risk environments, with strong cryptographic guarantees and minimal reliance on centralized infrastructure.

## Motivation

Most modern communication systems rely on centralized infrastructure and implicit trust in service providers, stable connectivity, and low personal risk to users.

This project is designed for environments where those assumptions do not hold. It prioritizes peer-to-peer communication, explicit trust boundaries, and cryptographic protections that remain meaningful under network surveillance, infrastructure disruption, and partial compromise.

## Design Goals

The system aims to provide a *practically usable* communication platform while:

- Minimizing required trust in infrastructure and intermediaries
- Reducing metadata exposure where technically feasible
- Making security trade-offs explicit and user-controllable
- Remaining functional under partial connectivity and adversarial conditions

Rather than assuming a single threat model, the system is designed to adapt to different risk levels without silently weakening guarantees.

## Non-Goals

This project does not attempt to:
- Guarantee perfect anonymity
- Defend against total device compromise
- Eliminate all forms of metadata
- Replace operational or legal safety practices

Security is treated as a set of trade-offs that must be made explicit to users.

## High-Level Architecture

The system is organized around explicit trust and responsibility boundaries:

- **Core**: A memory-safe cryptographic and protocol core implemented in Rust
- **Bridge**: A minimal foreign-function interface (FFI) layer exposing the core via a stable C ABI
- **Runtime**: Networking, peer discovery, and orchestration implemented in Go
- **Apps**: Thin clients (mobile, desktop, CLI) that consume the engine

This separation is intentional and designed to keep the trusted computing base small and auditable.

## Threat Model (High-Level)

The system assumes:
- Network-level adversaries
- Malicious or unreliable peers
- Infrastructure interference
- Potential device seizure

The system does not assume:
- Trusted servers
- Always-online connectivity
- Honest majority of peers

## Repository Structure

- `engine/`   – Core system logic with explicit trust boundaries
- `apps/`     – End-user applications (mobile, desktop, CLI)
- `docs/`     – Architecture notes and protocol specifications
- `schemas/`  – Canonical data formats and wire schemas
- `scripts/`  – Development, testing, and audit tooling
- `deploy/`   – Build, packaging, and deployment configurations

## Project Status

This project is in early development. This repository is not yet intended for production use. Interfaces, protocols, and assumptions may change as the design is refined.

## License

This project is licensed under the GNU General Public License v3.0.

See the [`LICENSE`](./LICENSE) file for details.

