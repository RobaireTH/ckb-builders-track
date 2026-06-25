# CKBuilder Track Weekly Report - Week 23
- Name: **Mayowa Temitope AKINYELE **
- Week Ending: **June 24, 2026**

This week was a focused `pckt` implementation week. After last week's security-model review, I moved from describing the remaining product and correctness edges to tightening the packet lifecycle itself. The work landed in three connected areas: removing the old 64-recipient ceiling, making the frontend understand the real packet slot range, and fixing the backend lookup path so claim links always resolve to the latest live successor cell instead of an older version with the same public key.

The through-line was simple: a packet is not a static object after it is sealed. It is a chain of successor cells. Every claim consumes one cell and creates the next one until the final claim, so the product has to be careful about which version it shows, which capacity floor it promises, and how many recipients the lock actually supports. This week was about making the contract, frontend, and backend tell the same story.

## Key Learnings

- **The old 64-slot cap was not a chain limitation.** The packet data stores `slots_total` as a byte, so the real upper bound is 255. The lock already has stronger state-machine constraints around `slots_claimed`, claimed-lock length, successor conservation, and final-claim behavior. Keeping a separate hardcoded 64-recipient cap in the contract made the product artificially smaller than the format allows.

- **Capacity, not slot count, is the real limiting factor.** Raising the slot ceiling does not mean every sender can cheaply create a 255-recipient packet. The lock still has to preserve enough capacity for future claim cells, and the frontend still has to prevent underfunded packets. That is the right boundary: the user should be limited by the economics of the packet they are trying to create, not by an arbitrary constant that sits below the data model.

- **The frontend needed to become explicit about packet bounds.** Before this pass, the UI slider and presets were still shaped around the smaller packet count. I added shared frontend constants for `MIN_PACKET_SLOTS = 1` and `MAX_PACKET_SLOTS = 255`, moved the slider to the real range, added 100 and 255 recipient presets, and made both the amount step and the review step reject invalid slot counts with the same message.

- **Transaction construction needs its own validation, even when the UI already validates.** The seal path now calls `assertValidSlots` before building the transaction. This is a small but important hardening point: UI controls are only one caller. The transaction builder should still refuse impossible packet parameters if a future screen, test, or integration calls it directly.

- **Claim links need the latest successor, not the most recently sealed packet.** `/v1/packets/by-pubkey/:hash` previously ordered by `sealed_at DESC`. Successor rows can share the same original seal timestamp, which means the API could return an older live version of the same packet chain. Ordering by `last_seen_block DESC, packets.rowid DESC` matches the way successor cells evolve on-chain and returns the newest indexed version.

- **The fee-completion order in claim transactions matters.** In the frontend claim path, the witness is now attached before `completeFeeBy`. That makes fee completion account for the actual witness payload that will be signed and relayed, instead of estimating before the claim witness is in place.

- **Live wallet refresh is now event-driven instead of only pull-driven.** The frontend subscribes to `/v1/events/stream` for the connected wallet and refreshes on seal, claim, and reclaim events that touch the wallet's owner or claimer lock hash. This should make sent and claimed packet views catch up faster after an on-chain state change.

## Brief
- `pckt` now accepts the full byte-sized recipient range: 1 to 255 slots.
- The shared Rust type constant moved from `MAX_SLOTS = 64` to `MAX_SLOTS = u8::MAX`.
- The lock invariant now rejects only zero slots instead of rejecting anything above 64.
- The frontend packet creation flow now exposes and validates the same 1-255 range.
- The claim transaction path now attaches the witness before fee completion.
- The backend packet-by-pubkey route now returns the latest successor cell by indexed block order.
- Regression tests were added for 255-slot reclaim acceptance and latest-successor lookup.

## Practical Progress

### Contract and shared types
- Removed the hardcoded `MAX_SLOTS: u8 = 64` constant from `contract/pckt-lock/src/main.rs`.
- Updated `enforce_invariants` so `slots_total == 0` is invalid, while any byte-sized nonzero slot count is allowed.
- Updated `shared/pckt-types/src/lib.rs` so `MAX_SLOTS` is `u8::MAX`.
- Added `accepts_max_byte_sized_slot_count` in `contract/tests/tests/integration.rs`. The test builds a 255-slot packet and verifies that the reclaim path succeeds after expiry, proving the lock accepts the maximum byte-sized slot count rather than silently preserving the old 64-slot assumption.

### Frontend packet creation
- Added `MIN_PACKET_SLOTS` and `MAX_PACKET_SLOTS` to `frontend/src/packets.ts`.
- Updated `frontend/src/screens/CreateAmount.tsx`:
  - recipient slider now spans 1 to 255
  - presets now include 100 and 255
  - validation rejects non-integer, below-min, and above-max slot counts
  - fixed-packet minimum amount validation still runs after slot validity is confirmed
- Updated `frontend/src/screens/CreateReview.tsx` with the same slot-count validation so an invalid draft cannot be sealed from the review step.
- Updated `frontend/src/tx.ts` with `assertValidSlots` so transaction building enforces the same slot range even if the UI is bypassed.

### Claim and refresh behavior
- Moved `completeFeeBy` in `buildAndRelayClaimTx` so it runs after the claim witness is inserted. This keeps fee calculation aligned with the transaction that is actually signed.
- Added `packetEventsUrl` and `PacketStreamEvent` in `frontend/src/api.ts`.
- Added an `EventSource` subscription in `frontend/src/App.tsx` for the connected wallet. The app now refreshes when seal, claim, or reclaim events involve that wallet as owner or claimer.

### Backend successor lookup
- Updated `backend/src/routes/packets.rs` so `/v1/packets/by-pubkey/:hash` orders by `last_seen_block DESC, packets.rowid DESC` instead of `sealed_at DESC`.
- Added `by_pubkey_returns_latest_successor_when_sealed_at_ties` in `backend/tests/api.rs`. The regression inserts two rows with the same `sealed_at` and same `claim_pubkey_hash`, then verifies the API returns the newer successor row.
- Adjusted the message store/fetch API test to compute the real `blake160(body)` hash, keeping the test aligned with the backend's hardened message validation.

## Reflections

The most useful part of this week was seeing how a small constant can hide a product assumption. `slots_total` was already byte-sized, but the contract, shared types, and UI still carried the older 64-recipient mental model. Removing that cap was not just a one-line contract change. It forced the rest of the stack to answer the same question: what does the packet actually support, and where should the product stop a user from building something invalid?

I also came away with a sharper understanding of the successor-cell model. A sealed packet is the start of a state chain, not the whole object forever. That matters for claim links. A claim page that resolves an old successor can show stale capacity, stale claim count, or a transaction target that has already been consumed. Ordering by `sealed_at` looked reasonable when thinking about packets as creation events, but it was wrong when thinking about packets as evolving cells. `last_seen_block` is the better proxy for the live chain tip that the user needs.

The frontend work was less glamorous, but it was necessary. If the lock accepts 255 recipients and the UI still caps the slider at 50, the product is lying by omission. If the review screen validates differently from the amount screen, the user can get blocked late. If the transaction builder trusts the UI, a later integration can bypass the guard. The shape I want to keep is validation at every boundary, with the same constants and the same language wherever possible.

## Proof of Participation
- `pckt` repo: `origin https://github.com/RobaireTH/pckt.git`
- Commits in scope for the week:
  - `c58fb100` - Remove 64-slot packet cap
  - `7670e509` - Update frontend packet slot handling
  - `058ea4f6` - Return latest packet successor by pubkey
- Primary files changed:
  - `contract/pckt-lock/src/main.rs`
  - `contract/tests/tests/integration.rs`
  - `shared/pckt-types/src/lib.rs`
  - `frontend/src/App.tsx`
  - `frontend/src/api.ts`
  - `frontend/src/packets.ts`
  - `frontend/src/screens/CreateAmount.tsx`
  - `frontend/src/screens/CreateReview.tsx`
  - `frontend/src/tx.ts`
  - `backend/src/routes/packets.rs`
  - `backend/tests/api.rs`

## Challenges
- The slot-count change looked smaller than it was. Removing the contract cap without aligning frontend validation would have produced a confusing half-feature where the chain supported 255 recipients but the product still behaved like it did not.
- The capacity floor remains the hard product constraint. A 255-recipient packet needs enough CKB to keep every future claim viable, so the UI has to keep explaining minimum totals clearly instead of treating a higher slot count as free.
- The successor lookup bug was subtle because ordering by `sealed_at` sounds natural for a packet lookup. It only becomes wrong once the packet is understood as a sequence of successor cells sharing the same original creation time.
- Event-driven refresh improves freshness, but it also makes the app more dependent on the backend stream behaving well. The fallback still has to be normal fetch refreshes, because SSE should make the UI faster, not become the only way it stays correct.
