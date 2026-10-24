# 🌊 SorobanFlow
> **Turnkey Automated Payroll, Continuous Streaming & Session-Guarded Vesting Protocol for Stellar / Soroban**

[![CI](https://github.com/Ranjeet2063/sorobanflow/actions/workflows/ci.yml/badge.svg)](https://github.com/Ranjeet2063/sorobanflow/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Ranjeet2063/sorobanflow?color=blue)](https://github.com/Ranjeet2063/sorobanflow/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Soroban](https://img.shields.io/badge/Soroban-v22+-purple.svg)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org)

---

## 📌 Overview
**SorobanFlow** is a decentralized, non-custodial streaming and payroll infrastructure natively engineered for the **Stellar / Soroban** network. It empowers decentralized organizations, startups, and cross-border teams to execute continuous token streaming, milestone payouts, and cliff vesting in **USDC/EURC** with sub-cent transaction costs (< $0.0001) and deterministic 4–5 second finality.

---

## 🌟 Key Features
* **Continuous Real-Time Streaming**: Stream salaries block-by-block using linear time-decay interpolation.
* **Epoch-Based Automated Payroll**: Schedule recurring batch distributions without manual approval bottlenecks.
* **Cliff & Vesting Milestone Locks**: Lock team/advisor tokens with enforceable cliff periods and linear unlock curves.
* **Freighter Preflight Session Guards**: Built-in client validation catching network/account mismatches before submitting Soroban transactions.
* **Autonomous Storage TTL Renewal**: Automatic persistent storage lifetime extension on contract invocation, preventing ledger expiration.
* **Non-Custodial Multi-Token Support**: Native support for Stellar USDC, EURC, and custom Soroban SAC tokens.
* **Developer SDK**: Lightweight TypeScript SDK (`@sorobanflow/sdk`) with Next.js/React hooks.

---

## 🏗️ Architecture

```mermaid
flowchart TD
    subgraph UI ["Frontend Application Layer (Next.js 15)"]
        A1["Employer / DAO Dashboard<br/>(Bulk CSV, Stream Creator, Analytics)"]
        A2["Contributor Portal<br/>(Live Streaming Balance, 1-Click Claim)"]
        A3["Freighter Wallet Session Provider<br/>(Preflight Validation & Network Assertion)"]
    end

    subgraph SDK ["Developer SDK Layer (@sorobanflow/sdk)"]
        B1["Simulation Engine<br/>(Pre-evaluates Soroban Auth & Footprint)"]
        B2["XDR & ScVal Serializer<br/>(Type-safe Arguments & Event Parsing)"]
        B3["Session Integrity Guard<br/>(Rejects network/account divergence)"]
    end

    subgraph ONCHAIN ["Soroban Host Environment (Rust Smart Contracts)"]
        C1["Vesting & Streaming Engine<br/>• Linear Interpolation<br/>• Cliff Milestone Locks<br/>• Revocable Stream Escrow"]
        C2["Automated Payroll Engine<br/>• Epoch Distributions<br/>• Per-Second Rate Debit<br/>• Non-Custodial Multi-Token Vault"]
        C3["Treasury & Storage Guard<br/>• Emergency Pause Circuit Breaker<br/>• Dynamic Persistent Storage TTL Renewal<br/>• Graceful Zero-Rent Archival"]
    end

    UI -->|Typed RPC Invocations| SDK
    SDK -->|Soroban RPC API| ONCHAIN
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
