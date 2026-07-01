# CKBuilder Track Weekly Report - Week 24
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **July 1, 2026**

This week was about turning the `ckb-pop` idea for `pckt` into something concrete, while being careful not to overclaim what the current ckb-pop contracts can guarantee. The direction I settled on is that every successful `pckt` claim can become a proof event: the sender creates the packet, and the claimers can receive claim receipts/badges connected to that packet claim.

The important distinction this week was that the badge should represent the claiming event itself, not a physical event. A `pckt` claim badge says: this wallet claimed from this packet, in this slot, through this transaction. That is a useful proof for activity history, future profile surfaces, and ckb-pop compatibility. It should not pretend to be event attendance unless the packet itself is explicitly tied to an event later.

## Key Learnings

- **The sender maps cleanly to the creator, but the badge belongs to the claimer.** In the `pckt` model, the sender creates the packet cell and controls the initial gift. The claimers are the people who prove knowledge of the claim secret and produce valid claim transactions. For ckb-pop semantics, this means the sender can be treated as the creator of the proof context, while each claimer is the recipient of their own claim receipt.

- **The badge is for the claim event, not necessarily a physical event.** I initially had to separate two ideas that sound similar: ckb-pop badges for real-world participation and `pckt` badges for on-chain participation. The safer product shape is a `pckt-claim` proof type. It can later be used by event organizers if a packet is created for an event, but the base integration should only claim what it can prove: a successful on-chain claim.

- **Existing ckb-pop DOB badge deployment is useful, but not enough for soulbound receipts.** I checked the `ckb-pop` deployment metadata and contract behavior. The deployed DOB badge type script enforces structure and Type ID uniqueness, but its own design does not make it a strict non-transferable asset. That means it should not be wired into `pckt` production as a soulbound receipt without another script or policy layer.

- **The right implementation posture is gated readiness.** Rather than force badge minting before the on-chain primitive is ready, I added the storage, indexing, API, and frontend transaction construction needed for claim receipts, but kept minting behind explicit env. This lets the code path be reviewed and tested now while preventing production from creating transferable "soulbound" badges by mistake.

- **Claim receipt indexing belongs beside claim indexing.** The backend already follows packet state through the indexer. A claim badge is only meaningful when it is connected to a real claim transaction, so I added a `claim_badges` table keyed around the packet out point, claim transaction hash, and claimer. That keeps the receipt attached to the event it is supposed to prove.

- **Deployment env should follow deployment reality.** The most practical decision this week was not to add production badge env until there is an actual deployed non-transferable badge script. Placeholder env would make the product look more deployed than it is. For now, the code is ready and activation waits on the correct on-chain deployment.

## Brief
- Investigated the best way to integrate `ckb-pop` with `pckt` claim flows.
- Decided that the first badge should be a `pckt-claim` receipt, not a physical-event attendance badge.
- Confirmed that the current ckb-pop DOB badge contract should not be treated as a strict soulbound token.
- Added backend storage for claim badge records.
- Added indexer support to detect claim badge outputs connected to claim transactions.
- Added API fields so claimed packets can expose badge receipt metadata.
- Added frontend support to optionally mint claim badge outputs during the claim transaction.
- Kept claim badge minting disabled until a real non-transferable badge deployment exists.

## Practical Progress

### ckb-pop integration review
- Checked the local `ckb-pop` repository and verified it was available for review.
- Reviewed the DOB badge deployment metadata under `contracts/deploy-info*.json`.
- Checked the ckb-pop frontend testnet env and confirmed the deployed DOB badge values exist on testnet.
- Read the DOB badge contract behavior and README enough to confirm that it enforces type structure and uniqueness, not strict non-transferability.
- Rejected the unsafe shortcut of turning on `pckt` badge minting with those values and calling it soulbound.

### Backend storage
- Added a new `claim_badges` table through a backend migration.
- Added `backend/src/db/claim_badges.rs` for recording and fetching badge rows.
- Stored badge rows by badge out point, packet out point, claim transaction hash, claimer lock hash, claim public key hash, slot index, slot amount, and metadata JSON.
- Added indexes for looking up claim badges by claimer and packet.

### Indexer and API
- Added claim witness parsing so the backend can extract the claimer lock hash from the claim witness.
- Updated the indexer to connect `ckb-pop` / `pckt-claim` badge outputs to the claim transaction that produced them.
- Added API fields on claimed-packet responses for badge out point, badge scope id, and badge metadata.
- Added tests for witness parsing, badge storage, indexer badge recording, and claimed-packet API output.

### Frontend transaction path
- Added `frontend/src/claimBadge.ts` to construct optional claim badge metadata, type args, output cell, and cell dep.
- Added frontend config entries for claim badge support, gated by `VITE_CLAIM_BADGE_ENABLED`.
- Updated the claim transaction builder so badge minting can be included only when configured.
- Updated the claim success UI to mention a PCKT claim receipt only when a badge was actually minted.


## Reflections

The useful part of this week was refusing the easy integration. There was a deployed ckb-pop badge contract available, and it would have been simple to wire its code hash and cell dep into `pckt`. But doing that would have blurred an important line: a typed badge cell is not automatically a soulbound credential. If the product says claimers receive a soulbound receipt, the script has to enforce that property or the UI has to be honest that the receipt is only a badge-like proof cell.

I also clarified the product meaning of the badge. `pckt` is not primarily an events app. It is a gift/claim protocol. So the native badge should represent a claim, not an event. That makes the integration smaller and more truthful. If someone later creates a packet for a physical meetup, the claim receipt can become part of that event story. But the base protocol should stay grounded in the fact it can verify on-chain.

The implementation shape now feels like the right compromise. The backend can store and expose claim receipts. The indexer can recognize them. The frontend can build them. But production does not turn them on until the on-chain soulbound piece is real. That is slower than forcing the feature live, but it keeps the trust boundary clean.

## Proof of Participation
- `pckt` repo: `origin https://github.com/RobaireTH/pckt.git`
- Commits in scope for the week:
  - `cf4c1e04` - Add claim badge storage
  - `7d2d993b` - Index PCKT claim badges
  - `b7cd1a45` - Mint optional claim badges
- Primary files changed:
  - `backend/migrations/20260501000000_claim_badges.sql`
  - `backend/src/db/claim_badges.rs`
  - `backend/src/indexer/claim.rs`
  - `backend/src/indexer/runner.rs`
  - `backend/src/routes/packets.rs`
  - `backend/tests/api.rs`
  - `backend/tests/indexer.rs`
  - `frontend/src/claimBadge.ts`
  - `frontend/src/config.ts`
  - `frontend/src/tx.ts`
  - `frontend/src/screens/Claim.tsx`

## Challenges
- The biggest challenge was separating "ckb-pop compatible" from "soulbound." The current DOB badge path can help express a receipt, but it should not be sold as non-transferable without stronger enforcement.
- The indexer work needed to connect badge outputs to the right claim event instead of just noticing that a badge-like output existed somewhere in a transaction.
- Backend deployment was blocked by missing Fly authentication on the machine due to the pausing I reported the last time I mentioned the FLY.IO billing.
