# CKBuilder Track Weekly Report - Week 25
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **July 19, 2026**

This report covers everything I worked on after the Week 24 devlog. The work moved in three connected directions: closing the last `pckt` claim-badge cleanup, building Fiber Incident Recorder for the Fiber infrastructure hackathon, and preparing a Pocket Node implementation handoff for RJNR/RaheemJnr so the `pckt` giveaway model can be reused inside a CKB mobile wallet context. That handoff later narrowed into three days of implementation check-ins around Pocket Node's bulk airdrop send path.

The main shift this period was from building one application to extracting reusable infrastructure. With `pckt`, I had already built a claimable packet protocol on CKB. With FiberIR, the work was not another consumer product on top of Fiber, but a diagnostics layer around Fiber itself: a way for wallets, merchants, services, and node operators to capture payment failures, classify them, keep provenance, and debug them without reading raw node responses by hand. With Pocket Node, the work was to translate the `pckt` idea into a wallet-friendly first step: bulk/multi-address CKB sends now, with full packet-cell claims kept as later protocol work.

## Key Learnings

- **Fiber infrastructure needs failure memory, not only payment success paths.** Failures should become structured incidents with evidence, severity, remediation, retry state, and provenance.

- **The clean integration surface for FiberIR is event-first.** `POST /v1/events` lets wallets, merchants, node operators, and SDK wrappers submit observed payment attempts for classification.

- **Provenance labels are a product feature, not just documentation.** `live`, `inferred`, `fixture`, and `mock` labels keep FiberIR honest about what evidence each incident uses.

- **The boundary with upstream Fiber has to stay explicit.** FiberIR observes and explains Fiber JSON-RPC outcomes; it does not replace routing, retry logic, or the payment network.

- **A failed live payment can still prove the infrastructure.** A live `NO_ROUTE` result is useful evidence when the product is built to diagnose payment failures.

- **Pocket Node should start with multi-address transfers before full PCKT giveaways.** A simple multi-address CKB send gives immediate utility before adding the richer packet-cell claim flow.

- **PCKT can work without a backend if Pocket Node uses its light-client primitives well.** The wallet can scan packet cells by lock args, decode `PacketData`, and retry against the latest successor cell.

- **PCKT is not just a multi-output transaction.** It is a live state machine where each claim updates packet state and reclaim requires owner authorization.

## Brief

- Fixed the last `pckt` claim-badge clippy lint after the Week 24 integration work.
- Built FiberIR as a TypeScript monorepo for Fiber payment incident capture, classification, API access, dashboard triage, SDK integration, and demo replay.
- Added a canonical incident taxonomy for Fiber failures.
- Added a Fiber JSON-RPC collector/wrapper that can call Fiber, observe payment results, and turn failures/successes into `fiber-ir.event.v1` records.
- Added a Fastify API for health checks, event ingestion, incident listing/detail, status updates, summary stats, fixture replay, and live invoice demo.
- Added incident storage with in-memory and JSON-file persistence paths for low-cost deployment.
- Added a React dashboard for incident review, trends, settings, provenance, remediation, and hosted demo use.
- Added a guided demo path so testers do not need to run command-line Fiber operations.
- Packaged and deployed a hosted FiberIR app and a hosted Fiber sender node on Fly.io.
- Prepared a Pocket Node handoff for RJNR/RaheemJnr that narrowed `pckt` giveaway thinking into a practical bulk/multi-address CKB send path, with full PCKT claims left as later protocol work.
- Patched the Plan Agent handoff after the first generated plan contained placeholder review content instead of the actual implementation brief.

## Practical Progress

### PCKT cleanup

- Fixed the `pckt` claim-badge indexer clippy lint in `backend/src/indexer/runner.rs`.
- Kept the Week 24 claim-badge decision intact: the system can store, index, expose, and optionally construct PCKT claim receipts, but production badge minting should wait for the correct non-transferable/soulbound deployment instead of reusing a transferable DOB-style badge path and overclaiming.
- Preserved the difference between a `pckt-claim` proof and physical event attendance. The receipt proves a wallet claimed from a packet, in a slot, through a transaction. It should not pretend to prove real-world attendance unless a packet is explicitly tied to an event later.

### Fiber Incident Recorder

- Created the TypeScript workspace structure with `shared`, `classifier`, `collector`, `api`, `sdk/ts`, `dashboard`, and `examples`.
- Defined the shared incident model:
  - `FiberIncidentEventV1`
  - canonical incident classes
  - incident statuses
  - severity
  - remediation
  - field-level provenance
- Added the canonical failure classes:
  - `NO_ROUTE`
  - `PEER_OFFLINE`
  - `CHANNEL_NOT_READY`
  - `INSUFFICIENT_OUTBOUND_LIQUIDITY`
  - `INSUFFICIENT_INBOUND_LIQUIDITY`
  - `ASSET_MISMATCH`
  - `FEE_TOO_LOW`
  - `PAYMENT_TIMEOUT`
  - `INVOICE_EXPIRED`
  - `UNKNOWN_NODE_FAILURE`
- Built the deterministic classifier and remediation catalog, with explicit rule priority so an expired invoice does not get misclassified as a route problem just because the error text also mentions routing.
- Added a Fiber JSON-RPC HTTP client and collector for `node_info`, `list_peers`, `list_channels`, `get_payment`, `list_payments`, payment attempts, and terminal payment observation.
- Added wrapper logic so a Fiber RPC success can emit `payment_succeeded` and a failure can emit `payment_attempt_failed`.
- Added fixture replay for a failure -> retry -> success narrative, with provenance labels preserved.
- Added Fastify routes:
  - `GET /healthz`
  - `POST /v1/events`
  - `GET /v1/incidents`
  - `GET /v1/incidents/:id`
  - `PATCH /v1/incidents/:id`
  - `GET /v1/stats/summary`
  - `POST /v1/demo/peer-transfer?replay=1`
  - `POST /v1/demo/pay-invoice`
- Added event id deduplication so repeated event submissions do not create duplicate incidents.
- Added resolution behavior where a linked `payment_succeeded` event can resolve an open incident by payment id or invoice id.
- Added JSON-file persistence for the hosted deployment and in-memory storage for local/dev use.
- Added dashboard static serving from the API process so the hosted demo can be one public app.
- Built the dashboard sections for incident inspection, trend review, integration settings, and invoice sender demo.
- Added a guided invoice sender demo where a tester pastes a Fiber invoice and FiberIR calls `send_payment` from the hosted sender node.
- Added Fly packaging for the FiberIR app and a separate Fiber sender node running `nervos/fiber:0.9.0-rc7`.
- Kept the raw Fiber RPC private while exposing the FiberIR API and dashboard publicly.

### Hosted FiberIR demo

- Hosted dashboard: `https://fiber-ir-604bdd.fly.dev/`
- Hosted invoice sender: `https://fiber-ir-604bdd.fly.dev/?section=demo`
- Health check: `https://fiber-ir-604bdd.fly.dev/healthz`
- Incidents API: `https://fiber-ir-604bdd.fly.dev/v1/incidents`
- Hosted sender node app: `fiber-ir-fnn-a-604bdd`
- Hosted sender P2P address:
  - `/dns4/fiber-ir-fnn-a-604bdd.fly.dev/tcp/8228/p2p/Qmf7j9K5GUDsXjrMh86yfAmhhJFTxnNUUpnBRu9rnwN7Tc`
- Confirmed hosted app health with JSON-file persistence enabled.
- Confirmed the sender node runs Fiber `0.9.0-rc7`.
- Recorded a live invoice attempt as `NO_ROUTE` when Fiber returned `PathFind error: no path found`.

### Pocket Node / RJNR handoff

- Prepared a shareable handoff for adapting the `pckt` giveaway idea to Pocket Node.
- Plan title: **PCKT Giveaway Implementation for Pocket Node**.
- Plan URL:
  - `https://plan.agent-native.com/_agent-native/open?app=plan&view=plan&to=%2Fplans%2Fplan-076fe904144c4c38&planId=plan-076fe904144c4c38&agentSidebar=closed`
- Direct route:
  - `https://plan.agent-native.com/plans/plan-076fe904144c4c38`
- Pocket Node repo referenced:
  - `https://github.com/RaheemJnr/pocket-node`
- The handoff split the work into two tracks:
  - bulk/multi-address CKB transfer as the practical first step
  - full PCKT claimable giveaways as later protocol work
- Documented why Pocket Node already had enough wallet primitives for the first track:
  - Kotlin/Compose/Hilt/MVVM structure
  - local key storage and secp256k1 signing
  - CKB transaction models
  - `TransactionBuilder.kt`
  - `GatewayRepository.kt`
  - existing send, reservation, signing, and broadcast paths
- Recommended reusing Pocket Node's existing wallet, cell selection, pending broadcast reservation, signing, and broadcast paths instead of introducing remote custody.
- Treated RJNR's shipped bulk airdrop commit as the meaningful implementation evidence, not my PCKT lock deployment values or code hashes.
- Verified the matching upstream Pocket Node commit:
  - `761a3f02785d8313ae9d929749bec37c69415d13` - `feat: bulk airdrop send flow (Nairobi) (#417)`
- The commit added the practical adoption path: `buildMultiTransfer`, `prepareAndSendBulk`, bulk recipient parsing, conservative fee preview, batched broadcasts, retry of only unsent recipients after a failed batch, a hidden founder-only unlock, and tests.
- Kept full PCKT packet support as a design note for later, not as implementation proof for this week.

## Key Decisions


- **Failure is a valid demo outcome if it is live and classified honestly.** A `NO_ROUTE` result from a real Fiber call is not a failed submission story; it is exactly the kind of incident FiberIR is built to preserve.

- **Every shortcut must carry provenance.** Fixture replay is allowed for repeatable demos, but the UI and API must distinguish fixture evidence from live Fiber evidence.

- **Pocket Node should implement multi-address send first.** That path gives immediate giveaway utility without a custom contract or claim-key flow, and RJNR's upstream bulk airdrop commit is the right evidence to cite.

- **Pocket Node should treat PCKT as a state-machine integration, not a batch send.** Full PCKT support needs packet lock deps, Molecule encoding, packet witnesses, grouped signing, light-client state resolution, and successor retry behavior.

- **PCKT deployment values should stay out of the Pocket Node proof trail.** My lock code hashes and testnet metadata explain the later protocol option, but they do not prove RJNR's wallet implementation.


## Reflections

This period made the difference between product and infrastructure clearer for me. `pckt` is a product-shaped protocol: a sender creates a packet, recipients claim, and the UI makes the social action feel simple. FiberIR is different. Its value is not that it hides Fiber. Its value is that it gives Fiber integrators a way to preserve and explain what happened when a payment attempt succeeds, fails, retries, or gets resolved.

The most important discipline was not overclaiming. It would have been easy to describe FiberIR as a universal monitoring layer or to make the demo look stronger by blurring live data and fixture data. I chose the stricter story: this is an event-first incident recorder around explicit Fiber JSON-RPC calls. It classifies and stores what it observes. When data is inferred or fixture-backed, it says so.

The Pocket Node handoff also forced me to be stricter about what counts as evidence. My PCKT lock hashes and deployment details are useful context for a later packet-cell integration, but the adoption proof here is RJNR's bulk airdrop send work: the wallet gained a multi-recipient send path, batching, fee preview, retry behavior, hidden unlock, and tests.

The main CKB lesson here is that reusable protocol work requires exact state language. A PCKT packet is not a list of recipients. It is one live cell that evolves. A claim receipt is not an attendance badge. It is proof of a claim transaction. A Fiber payment failure is not just a log line. It can be a structured incident with provenance and remediation. Naming those boundaries correctly makes the systems easier to reuse.

## Proof of Participation

- `pckt` repo: `origin https://github.com/RobaireTH/pckt.git`
- `FiberIR` repo: `origin https://github.com/RobaireTH/fiber-ir.git`
- `Pocket Node` repo: `https://github.com/RaheemJnr/pocket-node`
- FiberIR hosted demo:
  - `https://fiber-ir-604bdd.fly.dev/`
  - `https://fiber-ir-604bdd.fly.dev/?section=demo`
- RJNR / Pocket Node implementation handoff:
  - `https://plan.agent-native.com/_agent-native/open?app=plan&view=plan&to=%2Fplans%2Fplan-076fe904144c4c38&planId=plan-076fe904144c4c38&agentSidebar=closed`
  - `https://plan.agent-native.com/plans/plan-076fe904144c4c38`
- Matching upstream Pocket Node implementation commit:
  - `https://github.com/RaheemJnr/pocket-node/commit/761a3f02785d8313ae9d929749bec37c69415d13`

## Challenges

- Real Fiber demos are sensitive to route graph, channel state, liquidity, and invoice conditions. The right response was to record the actual success or failure instead of forcing the UI to always show a success case.
- The hosted demo needed two deployment units: the public FiberIR app and a separate sender node with public P2P but private RPC.
- The incident dashboard had to stay useful without hiding provenance. A polished dashboard that lies about fixture data would be worse than a simpler one that tells the truth.
- The Plan Agent handoff for Pocket Node initially stored placeholder review content, so I had to patch the same plan with the real implementation body and confirm the link.
- The Pocket Node handoff needed a clean evidence boundary: the useful implementation proof is the upstream bulk airdrop commit, not my PCKT deployment constants.
- The first-step wallet problem was practical transaction UX: parse many recipients, estimate batched fees conservatively, and retry only unsent recipients if a mid-airdrop batch fails.

## Next

- Keep FiberIR focused on the integration path that matters most: explicit Fiber payment attempts in wallets, merchants, services, or operator tooling emitting `fiber-ir.event.v1`.
- Improve FiberIR with more live scenarios where Fiber exposes enough evidence to classify liquidity, peer, timeout, and asset failures without relying on fixtures.
- Keep the Pocket Node path split: cite the shipped bulk/multi-address send work as the first step, then revisit full PCKT packet support once the transaction builder, Molecule encoder, light-client scan, and grouped signing pieces are ready.
- Revisit PCKT mainnet readiness only after external contract review and clearer policy around non-transferable claim receipts.
