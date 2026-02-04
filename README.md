
# pinocchio-escrow-workspace

![Rust](https://img.shields.io/badge/Rust-1.92.0-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

**A comprehensive Solana monorepo featuring multiple Escrow program implementations using the lightweight Pinocchio framework.**

This repository tracks the technical evolution of Solana smart contracts: from classic account handling patterns to modern, zero-std optimized architectures. By leveraging the [Pinocchio](https://github.com/anza-xyz/pinocchio) framework, these programs achieve near-theoretical minimum Compute Unit (CU) consumption.

---

## 📂 Repository Structure

The workspace utilizes a **Cargo Workspace** to manage multiple versions of the Escrow logic independently:

```text
.
├── programs
│   ├── pinocchio_escrow          # Modern: Pinocchio v0.10.1, Rust 2024, Optimized
│   └── solana_pinocchio_escrow   # Classic: Pinocchio v0.9.2, Stable patterns
├── justfile                      # Unified command runner
├── deny.toml                     # Dependency & License policy
└── _typos.toml                   # Spell check configuration

```

## 🚀 The Evolution

This workspace documents the shift in Pinocchio development paradigms:

| Feature | `solana_pinocchio_escrow` | `pinocchio_escrow` |
| --- | --- | --- |
| **Pinocchio Core** | `v0.9.2` | **`v0.10.1`** |
| **Rust Edition** | `2024` (Experimental) | **`2024` (Stable)** |
| **SPL Libraries** | `v0.4.0` Series | **`v0.5.0` Series** |
| **Key Advantage** | Foundational logic | **Advanced Address Handling & CU savings** |

## 🛠 Development

### Prerequisites

* **Rust**: `1.88.0` or higher (successfully tested on `1.92.0`)
* **Solana CLI**: `2.1.0` or higher (Agave toolchain)
* **Just**: Command runner (optional but recommended)

### Build Instructions

From the root directory, you can build all programs or a specific target:

```bash
# Build all programs in the workspace
just build-all

# Build specific version
cargo build-sbf -p pinocchio_escrow

```

## 🔬 Technical Highlights

* **Zero-std & No-std**: Built without the Rust standard library, resulting in extremely compact binaries and minimal deployment costs.
* **CU Optimization**: Each instruction is handcrafted to minimize Compute Unit usage, essential for high-frequency DeFi operations.
* **Modern Rust**: Both implementations use the **Rust 2024 Edition**, showcasing early adoption of the latest language features.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) to learn how to contribute to the project.

## 📄 License

This project is licensed under the **MIT License**.

---

**Built with 🦀 and ⭐️ for the Solana Ecosystem.**

---
