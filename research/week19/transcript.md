# Week 19 — SSRI Practical Transcript

This file captures the actual commands and responses from the SSRI research session: cloning, building, patching, and calling real testnet deployments. It is meant to be linked from `dev-logs/week19.md` under Proof of Participation.

- Date: 2026-05-24 / 2026-05-25
- Machine: local Ubuntu dev box
- CKB testnet RPC: `https://testnet.ckb.dev/`
- SSRI server endpoint: `http://localhost:9090`

---

## 1. Repos cloned

```
~/ckb-builders-track/research/week19/
├── pausable-udt/        github.com/Alive24/pausable-udt
├── ssri-server/         github.com/ckb-devrel/ssri-server
└── ssri-test/           github.com/Hanssen0/ssri-test
```

Commit hashes were captured by `git rev-parse HEAD` in each working tree.

---

## 2. Building ssri-server

First attempt — failed on missing system dep:

```
$ cargo build --release
...
error: failed to run custom build command for `openssl-sys v0.9.102`
  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
```

Installed deps:

```
$ sudo apt-get install -y libssl-dev pkg-config
Setting up libssl-dev:amd64 (3.0.13-0ubuntu3.9) ...
Setting up pkg-config:amd64 (1.8.1-2build1) ...
```

Retry:

```
$ cargo build --release
   ...
    Finished `release` profile [optimized] target(s) in 3m 13s
```

---

## 3. Server boot

```
$ RUST_LOG=ssri_server=info ./target/release/ssri-server
CKB RPC URI: https://testnet.ckb.dev/
Listening on: 0.0.0.0:9090
Script debug disabled
```

Note: the README's `curl` example posts to `:8090`. The actual default in `config.toml` is `9090`.

Liveness probe (JSON-RPC dispatch loop alive — wrong method, but real response):

```
$ curl -sS -H 'content-type: application/json' \
  -d '{"id":1,"jsonrpc":"2.0","method":"rpc_methods","params":[]}' \
  http://localhost:9090
{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"Method not found"}}
```

---

## 4. Computing SSRI method paths

Method path = first 8 bytes of `blake2b_256(method_signature)` with the `ckb-default-hash` personalization, encoded little-endian. Cross-checked against `ssri-test/ckb-ssri-proc-macro/src/lib.rs:19` (`method_path(name)`):

```python
import hashlib
def mp(name):
    return '0x' + hashlib.blake2b(
        name.encode(), digest_size=32, person=b'ckb-default-hash'
    ).digest()[:8].hex()
```

Output:

| signature             | method path             |
|-----------------------|-------------------------|
| `SSRI.version`        | `0x6f2a4642323106f8`    |
| `SSRI.get_methods`    | `0x58f02409de9de7b1`    |
| `SSRI.has_methods`    | `0xb43d1128f8726c19`    |
| `SSRI.get_cell_deps`  | `0xb751cc5bbca63da9`    |
| `UDT.name`            | `0xc78a67cec2fcc54f`    |
| `UDT.symbol`          | `0x35fa711c8c918aad`    |
| `UDT.decimals`        | `0x2f87f08056af234d`    |
| `UDT.balance`         | `0x912ea7d939004f3a`    |
| `UDT.icon`            | `0xa306f89e40893737`    |
| `UDT.transfer`        | `0x2e04fafee9f986ea`    |
| `UDT.mint`            | `0x03cd9ce840759d42`    |
| `UDTPausable.pause`   | `0x849def40c0e9a525`    |
| `UDTPausable.unpause` | `0x43f92b1ceda6fa2b`    |
| `UDTPausable.is_paused`        | `0x235c6c5c6ee04b08` |
| `UDTPausable.enumerate_paused` | `0x9adf445d336222e1` |

The `SSRI.get_methods` value matches the README's `0x58f02409de9de7b1` exactly — confirms encoding.

---

## 5. First real call — pre-patch (invalid syscall 2103)

Target: Hanssen0 `ssri-test` deployed at code_hash
`0x900afcf79235e88f7bdf8a5d320365b7912f8074f4489a68405f43586fc51e5c`.

```
$ curl -sS -H 'content-type: application/json' -d '{
  "id":1,"jsonrpc":"2.0","method":"run_script_level_code",
  "params":["0x900afcf79235e88f7bdf8a5d320365b7912f8074f4489a68405f43586fc51e5c",0,
            ["0x6f2a4642323106f8"]]
}' http://localhost:9090
{"jsonrpc":"2.0","id":1,"error":{"code":1004,"message":"Failed to run program: invalid syscall 2103"}}
```

Same error for `UDT.name`, `UDT.decimals`, `UDT.balance`, `SSRI.get_methods`.

Target: Alive24 `pausable-udt` deployed at code_hash
`0xb99f540caf4b03d152aa27626fbe62bf5559a9166a9ed1984b2d4fcbf063f964`
(extracted from `pausable-udt/contracts/pausable-udt/tests/src/ssri.rs:35`).

Same `invalid syscall 2103` for every method.

---

## 6. Root cause — `ssri_vm.rs:393-401`

```rust
// set_content - code
// 2103 => {
//     let addr = machine.registers()[A0].to_u64();
//     let len = machine.registers()[A1];
//     let len = machine.memory_mut().load64(&len)?;
//
//     *self.content.clone().lock().unwrap() =
//         Some(machine.memory_mut().load_bytes(addr, len)?);
// }
// pipe - code
2604 => self.pipe(machine)?,
// write - code
// NOTE: This would be working in the set_content way but using write for compatibility
2605 => {
    let addr = machine.registers()[A1].to_u64();
    let len = machine.registers()[A2];
    let len = machine.memory_mut().load64(&len)?;

    *self.content.clone().lock().unwrap() =
        Some(machine.memory_mut().load_bytes(addr, len)?);
}
```

Syscall 2103 = legacy `set_content`. Master branch replaced it with 2604/`pipe` + 2605/`write`. Testnet deployments are still on the old ABI.

Patch: see `ssri-server-2103-shim.patch` in this directory. One-line re-enable of the existing (already-correct) handler. Rebuild took 7.65s.

---

## 7. Real responses — post-patch

### Hanssen0 ssri-test (`0x900afc...`)

```
$ curl -sS ... '["0x6f2a4642323106f8"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x00","cell_deps":[]}}
# SSRI.version → 0 (the only defined version)

$ curl -sS ... '["0xc78a67cec2fcc54f"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x54657374","cell_deps":[]}}
# UDT.name → "Test" (0x54 0x65 0x73 0x74)
# Matches contracts/ssri-test/src/udt.rs line 5: Ok(Cow::from("Test".as_bytes()))

$ curl -sS ... '["0x2f87f08056af234d"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x08","cell_deps":[]}}
# UDT.decimals → 8
# Matches contracts/ssri-test/src/main.rs line 49: "UDT.decimals" => Ok(Cow::from(&[8][..]))

$ curl -sS ... '["0x912ea7d939004f3a"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x00000000000000000100000000000000","cell_deps":[]}}
# UDT.balance → u128 little-endian 2^64
# Matches contracts/ssri-test/src/main.rs line 50:
#   "UDT.balance" => Ok(Cow::from(&[0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0][..]))
```

### Alive24 pausable-udt (`0xb99f54...`)

```
$ curl -sS ... '["0x6f2a4642323106f8"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x00","cell_deps":[]}}
# SSRI.version → 0

$ curl -sS ... '["0xc78a67cec2fcc54f"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x554454","cell_deps":[]}}
# UDT.name → "UDT" (0x55 0x44 0x54)

$ curl -sS ... '["0x35fa711c8c918aad"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x554454","cell_deps":[]}}
# UDT.symbol → "UDT"

$ curl -sS ... '["0x2f87f08056af234d"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":"0x08","cell_deps":[]}}
# UDT.decimals → 8
```

### Reflection — the SSRI promise made concrete

```
$ curl -sS ... '["0x58f02409de9de7b1","0x0000000000000000","0x0a00000000000000"]' ...
{"jsonrpc":"2.0","id":1,"result":{"content":
  "0x6f2a4642323106f8
     58f02409de9de7b1
     b43d1128f8726c19
     b751cc5bbca63da9
     c78a67cec2fcc54f
     35fa711c8c918aad
     2f87f08056af234d
     912ea7d939004f3a
     eddf8c3d41d9ff5a
     c3e2ecfe2433d4b1
     33ae2ed03309e3ae",
  "cell_deps":[]}}
```

88 bytes = 11 method paths × 8 bytes. Decoded against the table in §4:

| #  | method path           | signature                |
|----|-----------------------|--------------------------|
| 1  | `6f2a4642323106f8`    | `SSRI.version`           |
| 2  | `58f02409de9de7b1`    | `SSRI.get_methods`       |
| 3  | `b43d1128f8726c19`    | `SSRI.has_methods`       |
| 4  | `b751cc5bbca63da9`    | `SSRI.get_cell_deps`     |
| 5  | `c78a67cec2fcc54f`    | `UDT.name`               |
| 6  | `35fa711c8c918aad`    | `UDT.symbol`             |
| 7  | `2f87f08056af234d`    | `UDT.decimals`           |
| 8  | `912ea7d939004f3a`    | `UDT.balance`            |
| 9  | `eddf8c3d41d9ff5a`    | (drift — see note)       |
| 10 | `c3e2ecfe2433d4b1`    | (drift — see note)       |
| 11 | `33ae2ed03309e3ae`    | (drift — see note)       |

The on-disk `master` of `pausable-udt` dispatches on `UDT.transfer`, `UDT.mint`, `UDT.icon`, `UDTPausable.{pause,unpause,is_paused,enumerate_paused}` — none of those signatures hash to the three unknown paths. The deployed testnet binary was built from an earlier or differently-named trait surface, and the on-disk source has drifted since the deployment. This is exactly the self-describing-script property SSRI promises: I can interrogate the deployed contract directly without trusting the repo to be in sync.

---

## 8. Files in this directory

- `transcript.md` — this file
- `ssri-server-2103-shim.patch` — the one-line ABI shim
- `ssri-test/`, `ssri-server/`, `pausable-udt/` — the three cloned repos
- `ssri-server/target/release/ssri-server` — patched binary (only present after running `cargo build --release` in that subdir)
