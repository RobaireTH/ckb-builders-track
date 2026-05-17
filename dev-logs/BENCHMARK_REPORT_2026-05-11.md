# CKB-PoP Benchmark Report — 2026-05-11

Targets: `ckb-pop` (monorepo) and `ckb-pop-cli` (standalone Rust CLI).

## Environment

- Host: Linux 6.17.0-23-generic, x86_64
- Rust: stable toolchain (workspace builds via channel 1.81.0 for contracts)
- Node: v24.13.0
- Bench harnesses: `criterion 0.5` (Rust), `tinybench 6.0.1` (TypeScript)
- All Rust benches built with the default `bench` profile (release + opt level 3 unless overridden by per-crate `[profile.release]`)
- Numbers are wall-clock for one operation; ranges are criterion's lower/median/upper estimates

## Summary of where work landed

| Area | Harness | Location |
|---|---|---|
| ckb-pop-cli — crypto primitives | criterion | `ckb-pop-cli/benches/crypto.rs` |
| ckb-pop-cli — transaction builders | criterion | `ckb-pop-cli/benches/tx_builder.rs` |
| ckb-pop/scripts — CKB hashing, address parsing, signature recovery | criterion | `ckb-pop/scripts/benches/crypto.rs` |
| ckb-pop/packages/ckb-presence — TS pure functions | tinybench | `ckb-pop/packages/ckb-presence/bench/run.ts` |
| ckb-pop/contracts — binary size, build time | shell + `size` | this report |
| ckb-pop/contracts — on-chain cycle counts | (pending tooling install) | see *Contracts* section |

How to reproduce:

```
# ckb-pop-cli
cd ckb-pop-cli && cargo bench

# scripts (backend)
cd ckb-pop/scripts && cargo bench

# ckb-presence (TS)
cd ckb-pop/packages/ckb-presence && node --experimental-strip-types bench/run.ts

# contracts (size + rebuild)
cd ckb-pop/contracts && cargo clean && cargo build --release --target riscv64imac-unknown-none-elf
size target/riscv64imac-unknown-none-elf/release/{dob-badge,event-anchor}
```

---

## ckb-pop-cli — crypto primitives

Source: `ckb-pop-cli/src/crypto.rs`. All functions are pure, no IO.

| Function | Median |
|---|---:|
| `compute_event_id` (SHA256 over address+ts+nonce) | 4.146 µs |
| `build_type_script_args` (2× SHA256 truncated, 40 B out) | 2.741 µs |
| `build_badge_args/no_type_id` (60 B out, zero-prefix) | 4.370 µs |
| `build_badge_args/with_type_id` (60 B out, supplied prefix) | 2.819 µs |
| `QrPayload::parse` | 442 ns |
| `QrPayload::encode` | 639 ns |
| `derive_window_secret` (SHA256) | 3.967 µs |
| `generate_qr_hmac` (HMAC-SHA256 + hex + slice) | 8.760 µs |
| `verify_qr_hmac/hit` | 7.120 µs |
| `verify_qr_hmac/miss` | 7.614 µs |
| `build_badge_cell_data/no_proof` (JSON + SHA256) | 6.588 µs |
| `build_badge_cell_data/with_proof` | 9.445 µs |
| `build_anchor_cell_data/no_meta` (JSON serialize) | 2.400 µs |
| `build_anchor_cell_data/with_meta` | 3.812 µs |
| `attendance_message` (format!) | 1.520 µs |
| `window_message/bounded` | 1.066 µs |
| `window_message/open` | 924 ns |

Observations:

- HMAC path (`generate_qr_hmac`) dominates the QR-attendance pipeline at ~9 µs — about 2× the cost of plain SHA256. Most of that is the hex encode + slice; `format!("{}|{}|...")` style assembly is cheap.
- `verify_qr_hmac/miss` is only marginally slower than `/hit` (8% diff) — the implementation does a full HMAC compute then string-compare, so timing-leak is small but present. Constant-time `subtle::ConstantTimeEq` would close it if that matters.
- `build_badge_args` is *faster* with a type id supplied than without (2.82 vs 4.37 µs) — counterintuitive; the `None` path writes 20 zero bytes via `extend_from_slice(&[0u8; 20])` which has a different codegen than the `Some` path's `extend_from_slice(&tid[..20])`. Minor; both are sub-5 µs.

## ckb-pop-cli — transaction builders

Source: `ckb-pop-cli/src/tx_builder.rs`. Builds an unsigned tx (output + cell dep) for each protocol op.

| Function | Median |
|---|---:|
| `build_event_anchor/no_meta` | 19.020 µs |
| `build_event_anchor/with_meta` | 21.294 µs |
| `build_badge_mint/no_proof` | 18.932 µs |
| `build_badge_mint/with_proof` | 23.122 µs |

Observations:

- All four are ~19–23 µs. The crypto layer accounts for ~30% of that; the rest is `ckb_types::core::TransactionBuilder` packing/serialization (capacity computation, pack of `Bytes`, hex parsing of the contract hashes).
- `parse_h256` runs twice per build (code_hash + deploy_tx_hash). If you ever push this into a hot loop (e.g. batch minting), cache `ContractInfo` → packed `Script`+`CellDep` once and skip the hex parse.

---

## ckb-pop/scripts — CKB hashing, address parsing, signature recovery

Source: `ckb-pop/scripts/src/crypto/signatures.rs` (included into the bench via `#[path]` because the crate is binary-only).

Skipped (duplicates of ckb-pop-cli): `derive_window_secret`, `generate_qr_hmac`, `verify_qr_hmac` — same algorithms, comparable numbers.

| Function | Median |
|---|---:|
| `hash_message_ckb/short` (blake2b w/ "Nervos Message:" prefix, ~50 B input) | 1.941 µs |
| `hash_message_ckb/long` (~520 B input) | 6.378 µs |
| `parse_ckb_address/testnet_full` (bech32m decode + base32 unpack) | 9.000 µs |
| `parse_ckb_address/mainnet_full` | 7.870 µs |
| `parse_ckb_address/reject_invalid` (early-out path) | 99 ns |
| `verify_ckb_address_signature/hit` (parse addr + blake2b + secp256k1 recover + compare) | 241.82 µs |
| `verify_ckb_address_signature/reject_zero_sig` (parse-only, rejects on bad sig bytes) | 15.789 µs |

Observations:

- `verify_ckb_address_signature` at **242 µs** is the most expensive primitive in either backend. ~90% of that is secp256k1 ECDSA recovery — there's no cheap way to bring that down without a faster libsecp build or batch verification.
- A backend doing N=1000 attendance verifications/sec would burn ~240 ms of CPU per second just on signature recovery. If that matters, the relay-tx path is the place to look at concurrency, or pre-verify on a background worker.
- `hash_message_ckb` scales ~linearly with input length, as expected for blake2b.
- `parse_ckb_address` is dominated by bech32 decode + base32 unpack; for known-format addresses you could short-circuit by checking the HRP first.

---

## ckb-pop/packages/ckb-presence — TS pure functions

Source: `packages/ckb-presence/src/{ckb,module}.ts`. Run with `node --experimental-strip-types bench/run.ts`.

| Task | Ops/sec | Mean (µs) | Margin |
|---|---:|---:|---:|
| `bytesToHex(64 bytes)` | 20,190 | 150.0 | ±0.48% |
| `sha256Bytes(short string)` | 4,411 | 288.5 | ±0.65% |
| `sha256Hex(short string)` | 2,933 | 720.5 | ±2.20% |
| `sha256TruncatedHex(20 bytes)` | 1,644 | 1706.9 | ±6.11% |
| `buildIssuerAnchorArgs` | 956 | 2293.8 | ±6.39% |
| `buildScopeAnchorArgs` | 521 | 3896.1 | ±11.10% |
| `buildUniqueArtifactArgs` | 1,151 | 1976.1 | ±5.22% |
| `buildHashedCellData(small)` | 2,131 | 1275.7 | ±4.11% |
| `buildSignedClaimMessage` | 1,940,668 | 1.74 | ±0.04% |
| `buildWindowMessage(bounded)` | 2,876,839 | 1.03 | ±0.04% |
| `buildWindowMessage(open)` | 3,906,420 | 1.11 | ±0.04% |
| `encodeSignedClaimToken` | 65,205 | 42.3 | ±0.18% |
| `parseSignedClaimToken(hit)` | 33,677 | 99.2 | ±0.32% |
| `parseSignedClaimToken(garbage)` | 11,624 | 420.8 | ±0.99% |

Observations:

- The SHA-256 family goes through `crypto.subtle.digest`, which is a WebCrypto call from Node. There is fixed-cost overhead per call (~100–200 µs) that dominates for short inputs — that's why `sha256Hex` is 4× slower than the Rust SHA256 path even though the algorithm is the same.
- Anything that chains multiple `await sha256*` calls (`buildIssuerAnchorArgs`, `buildUniqueArtifactArgs`, `buildHashedCellData`) inherits that overhead per call. **Two consecutive `sha256TruncatedHex` calls in `buildIssuerAnchorArgs` cost ~2.3 ms** — the largest single hot-path cost in the TS module.
- Pure string functions (`buildWindowMessage`, `buildSignedClaimMessage`) run at >1.9M ops/sec; not a bottleneck anywhere.
- Recommendation: if the frontend ends up calling these hashing helpers per-attendee in a tight UI loop, batch them, or do the hashing in a single Web Worker that streams results back. For backend Node use, you'd swap to `node:crypto.createHash('sha256')` — likely a 20–50× win because the WebCrypto subtle interface allocates `ArrayBuffer` per call.

---

## ckb-pop/contracts — on-chain RISC-V scripts

Source: `contracts/{dob-badge,event-anchor}/src/main.rs`. Built for `riscv64imac-unknown-none-elf` with `opt-level='s'`, `lto=true`, `codegen-units=1`, `panic='abort'`.

### Build time (clean rebuild)

```
$ cargo clean && cargo build --release --target riscv64imac-unknown-none-elf
Finished `release` profile [optimized] target(s) in 2m 25s
   wall:  146.32 s
   user:  103.26 s
   sys:     7.15 s
   max rss: 411 MiB
```

### Binary sizes

| Contract | File size | .text | .data | .bss |
|---|---:|---:|---:|---:|
| `dob-badge` | 32 512 B (31.8 KiB) | 30 780 | 384 | 532 481 |
| `event-anchor` | 24 968 B (24.4 KiB) | 23 302 | 320 | 532 481 |

Notes:

- `.bss` of ~520 KiB is dominated by the ckb-std default heap reserve; it's allocated by the VM at exec time, not on-chain stored.
- `event-anchor` is ~25% smaller than `dob-badge` because `dob-badge` pulls in `ckb-hash` (blake2b) and the `type-id` feature; `event-anchor` is just `ckb-std`.

### Cycle counts (per-invocation)

`ckb-debugger 1.1.1` baseline (`--mode fast`, no tx fixture). Each contract is loaded into a stock CKB-VM, runs `_start` / `ckb_std` init, attempts to read the (absent) transaction context, and exits with `Run result: 1`. So these are *startup-only* numbers — they tell you the fixed cost the script pays before it ever touches its own validation logic.

| Contract | Cycles to early-exit (no tx) |
|---|---:|
| `dob-badge` | 12 069 |
| `event-anchor` | 10 503 |

```
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/dob-badge   --mode fast
# → Run result: 1 / All cycles: 12069(11.8K)
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/event-anchor --mode fast
# → Run result: 1 / All cycles: 10503(10.3K)
```

**For real validation-path cycle counts**, ckb-debugger needs `--mode full --tx-file <fixture.json>` where the fixture is a captured or synthesised transaction that exercises the script's success path. The existing `migrations/*.json` files are deployment artifacts and won't drive the scripts. To produce meaningful numbers:

1. Write a small `ckb-testtool` based harness under `contracts/tests/` (host-target crate, outside the riscv64 workspace) that constructs the canonical inputs/outputs/witnesses and reports `tx.verify(MAX_CYCLES).unwrap()` cycles. **Recommended** — ties cycle bounds to the source.
2. Capture a real on-chain mint/anchor tx via `ckb-cli rpc get_transaction <hash>` from testnet and feed it to `ckb-debugger --mode full`.

---

## Cross-cutting observations

1. **The most expensive primitive end-to-end is secp256k1 recovery** (242 µs) in the backend signature-verify path. Everything else combined to assemble + crypto-hash a typical attendance proof is < 50 µs.
2. **Rust vs TS SHA-256: the Rust side is ~75× faster** (3.97 µs vs 720 µs per hash on equivalent-sized inputs). If the TS package ever runs server-side in a high-throughput context, swap WebCrypto for `node:crypto`.
3. **Transaction assembly cost is dominated by ckb-types pack/unpack**, not the crypto layer. ~12 µs of the ~20 µs `build_event_anchor` budget is structural; tightening that requires changes inside `ckb-types`.
4. **Contracts are tiny** (24–32 KiB each). There's no size pressure; the LTO + opt-size flags are doing their job.

## Caveats

- Single-machine, single-run numbers; criterion's ±range gives you a confidence interval but doesn't account for thermal/system noise. Re-run several times if you want to track a regression.
- The TS `bytesToHex` showing 150 µs for 64 bytes (much slower than expected for a 64-element JS loop) is partly tinybench-overhead at the small end; the ranking among TS functions is meaningful, the absolute number for that one is noisy.
- Contract cycle counts (the metric that actually pays in CKB fees) are not in this report — see *Contracts → Cycle counts* above for what's needed.
