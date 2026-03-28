# CKBuilder Track Weekly Report — Week 9 through 11

- Name: **Mayowa Temitope AKINYELE**
- Week 9 Ending: **Mar 11, 2026**
- Week 10 Ending: **Mar 18, 2026**
- Week 11 Ending: **Mar 25, 2026**

## Courses Completed

- [CCC - CKBer's Codebase](https://github.com/ckb-devrel/ccc)
- [Awesome CKB](https://cookbook.ckbdapps.com/awesome/)
- [Fiber Docs](https://www.fiber.world/docs)
- [CKB Script Error Codes Wiki](https://nervosnetwork.github.io/ckb-script-error-codes/)
- [UTXOSwap SDK](https://github.com/UTXOSwap/utxoswap-sdk-js)
- [DOB Cookbook](https://github.com/sporeprotocol/dob-cookbook)
- [CKB Test tool for Rust](https://github.com/nervosnetwork/ckb-testtool)

## Key Learnings

- Reputation makes more sense on CKB as a dispute-windowed state transition than as a mutable score. That gives a protocol object you can inspect and reason about instead of another backend number.
- If the layouts drift even slightly, the protocol stops being coherent.
- A prompt-level rule is not protection. A type script that refuses the transaction is protection.
- Identity, jobs, capability claims, and reputation all need their own explicit state transitions.
- Fixed sleeps are not a real synchronisation strategy on CKB testnet. If the next step depends on a live cell, the only reliable approach is to poll the chain or indexer until the state actually exists.
- Payment flows are not just about moving value. On CKB they are also about sequencing, cell ownership, and where responsibility lives: on-chain settlement, off-chain routing, or explicit escrow.
- Result verification and result storage are different concerns. I do not need to store a worker's full result in a new cell to prove that it is tied to the original job. Binding it through the witness is enough.
- Fiber flows worked best when the explicit flow came first. `setup escrow -> complete with preimage -> settle` was much easier to reason about than automating too early and hiding important state transitions.
- Alternate signing backends are much more fragile than they look. If a signer is not byte-faithful for a 32-byte digest, it is not just inconvenient. It is unsafe for CKB signing.
- A protocol can be technically correct and still fail the handoff if the setup path is unclear.

## Brief

- The week that transitioned out of `ckb-pop` thinking and into NERVE came. I had not started the hackathon proper yet, but I was already shaping the protocol and the repo around the idea of an agent marketplace on CKB.
- I treated this week as the architecture week for the project. The work was less about polish and more about making sure the system would have the right primitives before I got pulled into hackathon speed.
- The main goal was to define the protocol shape early instead of letting the implementation sprawl: agent identity, job lifecycle, reputation updates, capability proofs, and the bridge routes that a real agent or external client would use.
- Then came the hackathon week. [NERVE](https://github.com/RobaireTH/NERVE) stopped feeling like a protocol sketch and started becoming a system I could actually run.
- I expanded the build in several directions at once: onboarding, demos, tests, reputation and capability flows, sub-agents, Fiber support, and the bridge APIs an external agent would need if it was not running the bundled stack.
- A lot of the value this week came from correcting wrong assumptions quickly. The more I pushed the system through testnet paths, the more the real protocol constraints became obvious.
- Then came the last week. I used this week for system hardening and to prove that flows matter.
- I spent most of the week on payment behavior, identity relationships, result binding, deployability, and demo reliability. A lot of the important work came from bugs that only showed up when I forced the system through live testnet paths.
- By the end of the week, the main NERVE demo path was real: identity, jobs, reward settlement, badge minting, reputation updates, sub-agent spawning, direct Fiber payments, hold-invoice escrow, and the restored mock AMM swap path all had live-tested flows.

## Practical Progress

The whole 3 weeks were practical based and I must say I learned a lot, than I have learned so far. Or was I able to say that because it's in speed? The following are my practical progres:

- Scaffolded or sketched the NERVE monorepo in with plans for a Rust core service, TypeScript MCP bridge, contracts workspace, scripts, and OpenClaw agent workspace.
- Added the initial contract set:
  - `agent_identity`
  - `job_cell`
  - `reputation`
  - `capability_nft`
- Implemented the first version of the `agent_identity` contract so identity becomes an enforceable on-chain object instead of only an application concept.
- Added the first `job_cell` lifecycle in the contract layer so a job can move through protocol-defined states instead of arbitrary backend flags.
- Added the first `reputation` state machine with a dispute-windowed flow.
- Built the early `nerve-core` structure for transaction building, signing, and broadcasting.
- Added the early `nerve-mcp` structure for reading chain state and exposing routes agents can call.
- Added the first deployment flow and environment handling for testnet contracts.
- Set up the initial OpenClaw workspace so NERVE had an actual agent runtime context rather than just contract code.
- Added the `nerve` CLI flows and expanded the demo/test scripts into something usable on testnet.
- Added capability NFT minting with signed attestation proofs plus the related builder and route flows.
- Added the full dispute-windowed reputation flow:
  - create
  - propose
  - finalize
- Added Fiber integration work:
  - Fiber setup scripts
  - Fiber bridge routes
  - direct payment support
  - `pay-agent` support
- Added sub-agent spawning, parent-child identity relationships, and revenue share fields.
- Added onboarding / discovery / status routes so an external agent can discover and participate in the same marketplace.
- Added badge minting support and expanded the on-chain proof story around completed work.
- Tightened contract behavior around state transitions, TTL checks, and destruction rules.
- Reworked the docs and bridge surfaces so the system could be reasoned about from outside the bundled local stack.
- Reintroduced the mock AMM direction as a deterministic DeFi demo path after recognizing that a stable demo mattered more than pretending a third-party swap route was ready when it was not.
- Polished Fiber node setup for main and worker flows, including announcement configuration support and better environment handling.
- Investigated SupeRISE signing deeply enough to isolate the real failure mode and concluded that it should be skipped for now because the upstream signing path is not safe for CKB transaction signing.
- Completed end-to-end live tests for:
  - full happy-path marketplace flow
  - badge minting
  - reputation propose/finalize
  - direct Fiber payment
  - `pay-agent`
  - hold-invoice escrow
  - mock AMM pool creation and swap

## Proof of Participation

You can find

- the NERVE source code [here](https://github.com/RobaireTH/NERVE)
- the live docs [here](https://nerve-docs.vercel.app)
- the protocol / flow diagrams [here](https://excalidraw.com/#json=vXxT0PwGFgISrZIAW17nd,K-Lo5qEvbGGsin8YOHm2cw)

## Challenges

- In the first week, The biggest challenge was architectural coherence. Every time identity layout, job data layout, or reputation structure changed, it affected contracts, and the bridge layer at the same time. Starting a protocol project from scratch made it obvious how easy it is to accidentally create three different versions of the same system. A lot of the week went into preventing that drift.
- Then, the DeFi path needed a clearer decision. UTXOSwap was ecosystem-aligned, but the deterministic mock AMM path was the one I could actually make reliable for a demo. It currently isn't the support path.
- Interface churn was constant. Identity layout, reputation handling, sub-agent support, and payment flows all touched one another, so stabilising one part of the stack kept forcing changes in another.
- Then, the hardest decision to make, having the build focus on the happy path because in the real sense, I only have the result compare to description paylod, social trust and offline proof of a well done jobs. The bad actors or agents that abandoned jobs currently only get a reputation duck. That is one of the best things to do, but I am sure there is a better way around it. Left to v2.
