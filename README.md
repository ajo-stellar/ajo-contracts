<!-- project-brand -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="brand/logo-dark.svg">
  <img src="brand/logo.svg" alt="Ajo" height="72">
</picture>

# Ajo — rotating savings circles on Stellar testnet

Status: local v0 prototype implemented; validation results are recorded in TESTING.md. No deployment, real-wallet end-to-end run or pilot has happened. Testnet only, no real money.

**Unreviewed custody code. A second human review is required before any funded test.** One missing contribution can lock a pot indefinitely. No cancellation or refund exists.

## Implementation

- Contracts: create fixed circles of 2–20 unique members, one contribution per member per round, permissionless all-paid settlement, fixed payout order, bounded accounting and TTL renewal.
- App: create/load circles, view current contributions, contribute/settle, testnet wallet network checks, transaction hash and explorer receipt. Public addresses only; no personal data.
- Book: architecture, privacy, limits, threat model, synthetic worked example, pilot gate and verification checklist.

See [ROADMAP.md](ROADMAP.md), [SECURITY.md](SECURITY.md), [TESTING.md](TESTING.md) and [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md).

## Run locally

Run each command from its corresponding repository.

Contracts: cargo test --offline --locked; node --test scripts/; node scripts/check-errors.mjs.

App: npm ci; copy .env.example to .env.local and configure reviewed deployment values only after the custody gate; npm run dev. With missing config the interface remains visible but actions are disabled. Contributions use token base units, not assumed decimals.

Docs: node --test scripts/; node scripts/check-links.mjs; mdbook build.
