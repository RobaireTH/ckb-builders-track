# CKBuilder Track Weekly Report — Week
-  Name: **Mayowa Temitope AKINYELE**
-  Week Ending: **Feb 18, 2026**

## Courses Completed
- [First Class Assets](https://medium.com/nervosnetwork/first-class-asset-ff4feaf370c4)
- [Blockchain and Common Knowledge Base](https://medium.com/nervosnetwork/blockchain-and-common-knowledge-c07957374d9a)
- [PoW vs PoS 1](https://medium.com/nervosnetwork/recap-of-pow-vs-pos-discussion-on-fork-it-podcast-part-1-98b89cc9c5ee)
- [PoW vs PoS 2](https://medium.com/nervosnetwork/recap-of-pow-vs-pos-discussion-on-fork-it-podcast-part-2-db754acf30cd)
- [Nervos with the core team](https://medium.com/nervosnetwork/a-deep-dive-into-nervos-with-the-founding-team-3cdc71fc8615)

## Key Learnings
- CKB treats assets as first-class citizens — cells hold both data and value natively, unlike account-based chains where tokens are entries in a contract's storage. This is core to how ckb-pop badges work: each badge is a real cell, not a database row.
- The "Common Knowledge Base" framing — CKB is designed to store common knowledge (state everyone agrees on), not to execute arbitrary computation. This validates the ckb-pop design where the chain stores proofs and the backend handles convenience logic.
- The PoW vs PoS trade-offs and why CKB chose PoW for its base layer security model. PoW's permissionless nature aligns with ckb-pop's ethos of presence proofs that don't require anyone's permission or trust.
- The importance of chain rehydration for keeping backend state consistent with on-chain truth after restarts or DB wipes.
- Managing architectural trade-offs between querying the chain directly from the frontend versus routing through a backend.

## Brief
- This week was about making ckb-pop robust and production-ready. The biggest lesson was around data sourcing: I attempted to migrate the frontend to query the chain directly (removing the backend dependency), but reverted that after running into reliability issues. The backend remains the source of truth for convenience while the chain remains the source of truth for verification.
- Reading "First Class Assets" reinforced why ckb-pop's approach of making each badge a real cell (not a contract entry) is the right design for CKB. The cell model means badges are genuine first-class assets that users own and can verify independently.
- The founding team deep dive gave me a clearer picture of CKB's long-term vision and how the layered architecture is meant to scale — Layer 1 for preservation, Layer 2 for transaction throughput. This maps well to ckb-pop: the chain preserves badge proofs, the backend handles the interactive UX.

## Practical Progress
- I wired badge records to the CKB explorer — tx hashes and block height in badge cards now link directly to the testnet explorer so users can verify their badges on-chain.
- I built a chain rehydration system in the Rust backend: on startup, the server scans the chain for existing badge cells and rebuilds its local SQLite cache, making the backend resilient to database resets.
- I added badge confirmation polling — after a mint, the frontend notifies the backend which then polls the chain every 15 seconds until the transaction is confirmed and a block number is assigned.
- I implemented reliable badge persistence with retry logic (3 attempts with exponential backoff) and a localStorage fallback so badges are never silently dropped if the backend is unreachable.
- I migrated the kiosk live-QR component to generate HMAC-signed QR codes locally using the connected wallet's signature, removing the dependency on the backend for QR generation.
- I added event prefix lookup so manual check-in works with short 6–8 character event IDs instead of requiring the full 64-char SHA-256 hex ID.
- I cleaned up dead code and wired unused endpoints to pass clippy, keeping the Rust backend clean.
- I attempted a full chain-direct migration for the frontend but reverted it after evaluating reliability trade-offs, settling on the backend-mediated approach.

## Proof of Participation
You can find
- the reference dApp [here](https://ckb-pop.vercel.app/#/)
- the source code [here](https://github.com/RobaireTH/ckb-PoP.git)
- other proof of participation [here](https://drive.google.com/drive/folders/16e3GzwUzAqVeEWX_1F6OEZEgVDrczR95?usp=drive_link) or review my [commit history](https://github.com/robaireth/ckb-pop/commits).

## Challenges
- The chain-direct migration was the biggest challenge this week. Querying the chain from the frontend worked but introduced latency and error-handling complexity that hurt UX. Reverting was the right call but cost a day of work. The takeaway: the backend isn't authoritative, but it is necessary for a smooth experience. I will need a feedback on this though. 
