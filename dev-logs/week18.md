# CKBuilder Track Weekly Report — Week 18
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **May 16, 2026**

This week, I handled the backend trust-boundary tightening pass on `pckt`: the relay, profile upsert, message store, and indexer-rollback paths all got stricter server-side checks so the backend cannot be used as an oracle for things only the chain or the user should attest to. This all followed my research for last week after @OfficeYutong advised. 
-I had concrete invariants the new validators are enforcing for the week.

## Issue-Focused Research Completed
- [CKB Address format (RFC 0021)](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0021-ckb-address-format/0021-ckb-address-format.md)
- [bech32 / bech32m specification](https://en.bitcoin.it/wiki/BIP_0350)
- [CKB script hash and `hash_type`](https://docs.nervos.org/docs/script/intro-to-script)
- [Cell deps and `dep_type`](https://docs.nervos.org/docs/tech-explanation/cell-deps)
- [The Blake2b Paper](https://www.blake2.net/blake2.pdf)
- [OWASP API Security — Broken Object Property Level Authorization](https://owasp.org/API-Security/editions/2023/en/0xa3-broken-object-property-level-authorization/)

## Key Learnings
- This week, I learned that "the chain is authoritative" is not enough as a slogan. The backend still has to refuse to lie on the chain's behalf. Several endpoints were happily accepting client-supplied identifiers (sender addresses, message hashes, signed transactions) that the server was never re-deriving. That gap is small per endpoint but it adds up to a backend that can be tricked into publishing claims it never verified.
- Re-deriving the `owner_lock_hash` from a submitted CKB `sender_address` turned out to be more involved than I expected. The full-format address payload carries `code_hash || hash_type || args`, and getting the script hash back means decoding bech32m, validating the version byte, range-checking `hash_type`, and only then hashing. Doing this server-side instead of trusting the client made profile ownership actually mean something.
- The relay endpoint taught me to think about transactions as more than opaque blobs. Even without simulating execution, the backend can refuse to forward any tx that does not reference the deployed `packet_lock` cell_dep. That single structural check stops the relay from being used as a generic CKB transaction broadcaster, which is what it was effectively becoming.
- I also learned that if the client both supplies a body and supplies a hash for that body, the server has to recompute the hash. Otherwise the "hash" is just a name the client picked, and any integrity guarantee in front-end code is fictional.
- The reorg-rollback work clarified how careful the indexer has to be about not "forgetting" packets that briefly disappeared. A naive rollback can drop live packet rows that are still spendable on the canonical chain, which would make the UI look like a packet vanished. Re-pulling live rows from the indexer after a rollback keeps user-visible state aligned with chain truth.
- A meta-lesson across all four changes: trust-boundary work is easier to justify retroactively than to design upfront. Each fix this week was a place where the original code path looked fine in isolation but became suspect once I asked "what does the backend get to claim on the user's behalf, and what is it just relaying?"

## Brief
- I have been able to get a lot past the issues raised, and my execution of saving shortlinks without secrets touching the backend seems smooth.  I alsoo fixed share UX, lucky-split floor, notification noise. This week was almost entirely backend-internal: re-derivation, structural validation, and rollback correctness. You can now revisit and reshare your claim url. 
- I was also able to confirm the reclaim logic works, with reclaiming unclaimed packets I had sent earlier. The only fix on that front is to change the error message gracefullly to 'packets reclaimed' and something related for other dead cells.
- I also ran a bench on the `ckb-pop` project. I will share the details below. The results of the audit on on contracts across all projects as well....

## Practical Progress
- Added a new `ckb_address` module on the backend that decodes full-format CKB addresses (bech32m, version byte, hash-type validation) and re-derives the script hash via Blake2b-256 of `code_hash || hash_type || args`, with unit tests covering matching hashes, wrong HRP, wrong version byte, bad hash type, and garbage input.
- Wired that decoder into the profile upsert path so `/v1/profiles` now rejects requests where the supplied `sender_address` does not derive to the supplied `owner_lock_hash`. Profile ownership is now actually verifiable from the request payload rather than being whatever the client typed.
- Tightened the relay endpoint so the backend refuses to forward any signed transaction whose `cell_deps` do not reference the deployed `packet_lock` out-point at the expected index. Added unit tests covering the accept case, the empty-cell-deps case, and the foreign-cell-dep case.
- Added structural logging on successful relays (`tx_hash` only) so abuse patterns are visible without leaking transaction contents into logs.
- Added integrity enforcement on `/v1/messages`: the server now recomputes `blake160(body)` and rejects the request if it does not match the client-supplied `message_hash`, with case-insensitive hex compare and tests for the matching and mismatching cases.
- Patched the indexer runner so live packet rows are restored from the indexer after a reorg rollback instead of being silently dropped, keeping the UI's "live packets" view consistent with the canonical chain.
- Backfilled backend integration tests in `backend/tests/api.rs` for the profile upsert path so the new sender-address vs. owner-lock-hash invariant is locked in at the API surface.


## Proof of Participation
- `drive` link: [Source](https://drive.google.com/drive/folders/1A9lJi5cJcqLVY25JqXyCyM8g9pW0Gd3F)
- `pckt` source: [github.com/RobaireTH/pckt](https://github.com/RobaireTH/pckt)
- `pckt` backend: [pckt-backend.fly.dev](https://pckt-backend.fly.dev)
- `ckb-pop` :[PoP](https://ckb-pop.xyz)
- `pckt` live: [send pckt](https://sendpckt.robaireth.dev)
- Benchmark Report: [here](/dev-logs/BENCHMARK_REPORT_2026-05-11.md)

## Challenges
- Re-deriving the script hash from a CKB address is the kind of thing that is easy to almost get right. The version byte, the hash-type byte, and the bech32m variant all have to line up, and a wrong constant anywhere returns a plausible-looking but wrong hash. Writing the tests against an independent `script_hash` computation was the only reason I trusted the decoder.

> One thing worth nothing is that the CCC provided JoyID signing does not provide a deterministic signing and hence makes it harder for me to use EC2 or the likes. 
- Structural validation of a signed tx is necessarily partial. Checking the `cell_deps` reference does not prove the tx is a valid pckt claim. It only proves the tx is at least pretending to be one. Going further would require the backend to simulate or re-parse the script, which moves it back toward being authoritative. The current check is a deliberate compromise.
- Reorg handling is still the area I trust least. The fix this week restores live rows after rollback, but the broader question of how to expose chain reorgs to the UI honestly (without spamming users with "your packet briefly disappeared" notifications) is still open.
