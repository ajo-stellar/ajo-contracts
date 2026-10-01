# Roadmap

What is next for `ajo-contracts`, in order. Anything not listed as done is
**not implemented**.

## Status

- [x] Repository governance: AGENTS.md, CONTRIBUTING.md, ROADMAP.md, LICENSE,
      .gitignore, .gitattributes (2026-10-01).
- [ ] v0 contract from the project's playbook section.

## Next

- [ ] v0 contract from `STELLAR-BUILD-PLAYBOOK-v3.md` section 8
      (present in `~/Desktop/Drips/_reference/playbooks/`, confirmed
      2026-10-02). v0 scope from that section, deliberately small: members
      contribute a fixed token amount each round; one member receives the
      whole pot each round; order fixed at creation. Entrypoints:
      `create_circle` (members fixed, unique, capped), `contribute` (one
      per member per round), `settle_round` (anyone calls it; succeeds
      only when every member has contributed; pays the pot and advances),
      `get_circle`, `get_round_status`. Core invariant: the contract's
      balance for a circle equals contributions received minus payouts
      made, tested after every step including a full multi-round circle.
      Planned per the program stack: thin `lib.rs`; `types.rs` (error
      enum, stored types, events); `storage.rs`; `error_paths.rs` with one
      test per variant; `test.rs` lifecycle tests; `ERRORS.md` +
      `scripts/check-errors.mjs` and its tests; rust-toolchain pinned to
      `wasm32v1-none`; release profile with `overflow-checks = true`.
- [ ] Day 11 hardening: same shape as duestreasury Day 9 (invariant tests,
      review checklist, SECURITY.md, TESTING.md, DEPLOYMENT_CHECKLIST.md).
- [ ] CI (`contract.yml`): fmt, clippy -D warnings, cargo test, node --test
      scripts/, check-errors, `stellar contract build` (CLI v28.1.0). Lands
      with the first code that can pass it.

## Deliberately unimplemented (from playbook section 8)

Listed there as the v0 boundary; each will get a draft issue when the v0
contract lands:

- Handling late or missed contributions and penalties.
- Removing a defaulting member with refunds.
- Cancellation with refunds.
- Deadlines tied to ledger time (research how round length maps to ledger
  close time first).
- Variable payout order, with a discussion of on-chain randomness risks.
- Listing circles with pagination.
- Property-based tests on the balance invariant.
- A security review checklist issue.

## Decisions needed from Tim

1. **Playbook v3 section 8 vs v4 doc set.** v3 section 8 defines the v0
   contract scope and is authoritative for it; v4 adds the standard doc set
   and error-sync checker on top. Build v0 from v3 section 8 plus the v4
   layer, or wait for Tim's call.
2. **Second reviewer.** Name the second human reviewer required before any funded test.

## Explicitly out of scope

Mainnet deployment. Anything the v0 design does not ask for.
