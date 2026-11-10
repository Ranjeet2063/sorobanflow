# Changelog

All notable changes to SorobanFlow are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added
- Comprehensive 20-test invariant suite (`test.rs`) covering:
  - Linear vesting invariant (claimable ∝ elapsed time)
  - Solvency invariant (total claimed ≤ total_amount)
  - Boundary conditions (exact start_time, exact stop_time)
  - Unauthorized recipient rejection
  - Double-claim after full vesting panic
  - Multi-stream independence verification
  - Large-amount overflow safety (1B+ token streams)

---

## [0.2.0] — 2026-11-10

### Added
- GitHub Actions CI workflow (`ci.yml`) with automated `cargo test` on push/PR
- `SECURITY.md` with responsible disclosure policy
- Comprehensive formal test suite (20 test cases)
- `CHANGELOG.md` for versioned release tracking

### Changed
- Upgraded `soroban-sdk` to `22.0.11` (latest stable)

---

## [0.1.0] — 2026-10-24

### Added
- Core `SorobanFlowPayroll` smart contract on Stellar Soroban
- `create_stream`: Token-locked time-based streaming payment setup
- `claim_stream`: Linear pro-rata vesting calculation with auth guard
- `get_schedule`: Read-only stream state inspection
- `StreamSchedule` struct: `sender`, `recipient`, `token`, `total_amount`, `start_time`, `stop_time`, `claimed_amount`
- Initial unit test (`test_create_and_claim_stream`)

### Architecture
- Pure Soroban SDK (`#![no_std]`) — zero external runtime dependencies
- Instance storage for gas efficiency
- `require_auth()` on sender (create) and recipient (claim)
- Linear vesting formula: `vested = total_amount × elapsed / duration`
