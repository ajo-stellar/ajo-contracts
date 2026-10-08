# Contract verification — local, 2026-10-08

- cargo fmt --all --check: PASS.
- cargo clippy --offline --locked --all-targets -- -D warnings: PASS (cached target directory reused).
- cargo test --offline --locked -j 1: 24 tests PASS.
- node --test scripts/*.test.mjs: 9 tests PASS.
- node scripts/check-errors.mjs: 11 variants synchronized.
- Stellar CLI28.1.0 contract build: PASS, 11,299-byte optimized Wasm, five exported methods.
- Wasm SHA256: bd221b7bfb9745f147a8dcd2a040a500a959359950c963c6ffb63578995636e2.

Tests cover full three-member/three-round SAC accounting after each step, independent circles, fixed payout order, duplicate payments, missing payments, authorization rejection, TTL renewal, every contract error, incoming token failure and outgoing token rollback. A separate mock token seeds balances and transfers actual mock-ledger balances before its deliberate outgoing failure; it does not replace the SAC invariant tests.

An inherited event test used an obsolete SDK API/cumulative-event assumption; corrected to SDK28 events() and per-invocation contract filtering. An inherited freeze test failed because its issuer was not AUTH_REVOCABLE; replaced with the explicit failing-token fixture.

No deployment, funded test, independent human custody review or pilot has occurred. These checks do not establish real-fund safety.
