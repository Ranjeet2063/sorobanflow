# 🌊 SorobanFlow
> **Turnkey Automated Payroll, Continuous Streaming & Session-Guarded Vesting Protocol for Stellar / Soroban**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Soroban](https://img.shields.io/badge/Soroban-v22+-purple.svg)](https://soroban.stellar.org)
[![Build Status](https://img.shields.io/badge/CI-Passing-brightgreen.svg)]()

---

## 📌 Overview
**SorobanFlow** is a decentralized, non-custodial streaming and payroll infrastructure natively engineered for the **Stellar / Soroban** network. It allows decentralized organizations, startups, and cross-border teams to execute continuous token streaming, milestone payouts, and cliff vesting in **USDC/EURC** with sub-cent transaction costs and zero gas surprises.

---

## 🌟 Key Features
* **Continuous Real-Time Streaming**: Stream salaries block-by-block with linear interpolation.
* **Epoch-Based Automated Payroll**: Schedule recurring batch distributions without manual approval bottlenecks.
* **Freighter Preflight Session Guards**: Built-in client validation catching RPC/account mismatches before submitting Soroban transactions.
* **Non-Custodial Multi-Token Support**: Native support for Stellar USDC, EURC, and custom Soroban SAC tokens.
* **Developer SDK**: Lightweight TypeScript SDK (`@sorobanflow/sdk`) with Next.js/React hooks.

---

## 🏗️ Architecture
```
┌─────────────────────────────────────────────────────────────┐
│             Next.js 15 / React Admin & Contributor UI       │
│      (Freighter Wallet Provider + Session Preflight Guard)   │
└──────────────────────────────┬──────────────────────────────┘
                               │ TypeScript SDK RPC calls
┌──────────────────────────────▼──────────────────────────────┐
│                  @sorobanflow/sdk Client                     │
│    - Transaction Simulation & Preflight Network Verification│
│    - XDR Encoding / Decoding & Native Value Mapping         │
└──────────────────────────────┬──────────────────────────────┘
                               │ Soroban RPC Server
┌──────────────────────────────▼──────────────────────────────┐
│               Soroban Smart Contracts (Rust)                 │
│  - Payroll Engine (Recurring Cron / Epoch Distributions)    │
│  - Vesting Engine (Continuous Block-Timestamp Interpolation)│
│  - Storage Management (Persistent & Instance TTL Automation)│
└─────────────────────────────────────────────────────────────┘
```

---

## 🧪 Testing & Verification
```bash
# Build contracts
cargo build --target wasm32-unknown-unknown --release

# Run unit and integration tests
cargo test
```

---

## 📄 License
Released under the [MIT License](LICENSE).
