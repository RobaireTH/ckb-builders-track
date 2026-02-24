# CKBuilder Track Weekly Report — Week 7

- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Feb 25, 2026**

## Courses Completed

- [RGB++ Light Paper](https://github.com/ckb-cell/RGBPlusPlus-design/blob/main/docs/light-paper-en.md)
- [CKB Transaction Structure RFC](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md)
- [DOB0 Render Protocol Docs](https://docs.spore.pro/)

## Key Learnings

- RGB++ achieves isomorphic binding: a Bitcoin UTXO maps one-to-one to a CKB cell, so ownership lives on Bitcoin while state and computation live on CKB. The reason this works is that both models are UTXO-descended. They just sit at different levels of expressiveness. RGB++ exploits that structural symmetry on purpose.
- The single-use seal primitive in RGB++ clicked for me in the context of ckb-pop. In RGB++, a Bitcoin UTXO is the seal. Once spent, the asset moves and the old state is permanently closed. ckb-pop badges work exactly the same way. Each badge cell is bound permanently to the attendee's lock script, non-transferable, non-reusable. The attendance window is the seal. The attendee's signature over the HMAC QR is the spending proof. The badge cell is the closed commitment. I had been describing badges as "soulbound tokens" but that framing doesn't fit CKB's model at all. Single-use seal closure is the right vocabulary.
- RGB++ commits to CKB state by embedding an OP_RETURN hash on Bitcoin. ckb-pop's event-anchor cell does something structurally similar from the other direction: it commits `SHA256(name || date || location || description)` into cell data on CKB, anchoring off-chain metadata to an immutable on-chain record. Both are commitment schemes. The difference is RGB++ bridges two chains; ckb-pop bridges the physical world and the chain.
- The transaction structure RFC answered questions I kept running into mid-implementation. The fee is implicit, the gap between input capacity and output capacity which means there is no fee field to set. If you miscalculate you either overpay or the node rejects the transaction silently. That is intentional but it is not obvious.
- DOB0 defines how clients should interpret a content hash in a cell and render a visual representation. ckb-pop badge cells currently store a raw content hash in their data. Reading DOB0 clarified the gap between what I have now and what a wallet that understands the standard would expect.

## Brief

- This week I built `ckb-pop-cli`, a terminal companion to the protocol. The goal was to let event organisers run the full flow —> create events, open attendance windows, rotate QR codes without the burden of the UI, for testing and also possible integration of the `pop-kiosk` in an actual terminal. Most of the week was transaction plumbing: sourcing live cells, calculating capacity, wiring cell deps, serialising correctly for the CKB node.
- Reading the RGB++ light paper while doing this gave me a much cleaner mental model of what ckb-pop actually is. I had been thinking about it in Ethereum terms — soulbound NFTs, on-chain registries but that framing fights against how CKB works. RGB++ made it clear that ckb-pop is more naturally described as a chain of UTXO-style seal closures. The event is a seal. Showing up and scanning the QR is how you spend it. The badge is what you get back. That is a much more honest description of the protocol and it is native to the UTXO model instead of borrowed from somewhere else.
- The other thing RGB++ made me think about is where ckb-pop could go. RGB++ lets a Bitcoin UTXO vouch for CKB state. In theory, `ckb-pop` event anchors could themselves be committed via RGB++, letting Bitcoin vouch for the event creation while `CKB` carries the badge state. The architecture would support it (the event-anchor type script args are just hashes, not address-specific). Not building that now, but I want to keep the design open to it.

## Practical Progress

- Built the full `ckb-pop-cli`: CLI argument parsing with `clap`, config management at `~/.ckb-pop/config.toml`, contract address registry for testnet, crypto module covering HMAC-SHA256 QR codes, SHA256 type-script args, and window secrets, plus an async RPC client wrapping both the CKB node and the indexer.
- Implemented a **browser signer**. Private keys live in browser wallets and not on disk, so the CLI spawns a localhost HTTP server, opens the signing page in the user's default browser, and collects the signed response via a POST back to localhost. The CCC SDK bundle is embedded in the binary at compile time with `include_str!()` so the signing page has no CDN dependency at runtime.
- Wrote a camelCase-to-snake_case translation layer between Rust's serialisation and CCC's JavaScript expectations. CKB's JSON API is entirely snake_case; CCC expects camelCase. The mismatch caused silent signature failures. CCC accepted the transaction and returned a result, but the signature didn't verify because the fields were hashed in the wrong form. This took a while to find.
- I bumped fee rate to 2000 shannons/KW and enforced minimum cell capacity on all outputs. Both were required for the node to accept transactions; without them the rejections gave no clear indication of the actual problem. (1000 was rejected by the mempool).
- Wrote integration tests for the RPC client against the live Pudge testnet and an end-to-end test covering the full proof-of-presence flow: create event → open window → derive QR HMAC → attend → mint badge → verify anchor cell on-chain.
- Diagnosed and fixed a UX gap: CLI-created events were not appearing on `ckb-pop.xyz` because the site filters My Events by exact `creator_address` string match, and the CLI can be signed into a different wallet than the one connected in the browser. Two fixes: the CLI now prints the creator address upfront with a tip to connect the same wallet on the site, and the site now has an "Add CLI Event by ID" input so you can paste the event ID the CLI prints and track it regardless of which wallet created it.
- The CLI now polls for anchor TX confirmation (every 15 seconds, up to 90 seconds) and calls the backend's `/activate` endpoint automatically on confirmation to record the TX hash. Previously this had to be done manually with curl. Note: Backend is non-authoritative, just an helper an can be replaced at any time.

## Proof of Participation

You can find

- the reference dApp [here](https://ckb-pop.vercel.app/#/)
- the ckb-pop source code [here](https://github.com/RobaireTH/ckb-PoP.git)
- the CLI source code [here](https://github.com/RobaireTH/ckb-pop-cli)
- other proof of participation [here](https://drive.google.com/drive/folders/1c4IRCa6h66r7JOFlvFkbvKoOScUwvFCq?usp=drive_link) or review my [ckb-pop commit history](https://github.com/robaireth/ckb-pop/commits) and [ckb-pop-cli commit history](https://github.com/RobaireTH/ckb-pop-cli/commits).

## Challenges

- The browser signer architecture worries me. The CLI opens a browser tab and waits for a POST back to localhost. If the browser is slow, the tab gets closed, or the port is already bound, the CLI hangs or throws a confusing error. I need a proper timeout with a clean failure message. This is the most likely thing to break for someone new to the tool.
- Fee rate and minimum cell capacity are still empirical. I settled on 2000 shannons/KW after 1000 was rejected, but I don't have a principled formula for a given transaction size. The SDK doesn't abstract this and I couldn't find a clear recommendation in the docs. Is there a standard testnet floor?
