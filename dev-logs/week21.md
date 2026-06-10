# CKBuilder Track Weekly Report — Week 21
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **June 10, 2026**

This week was a full product-direction rewrite of `NERVE` rather than an incremental feature week. The underlying repository is still `NERVE`, a CKB-native agent/job execution engine with Fiber payment support, but the work this week was to repackage that engine into **Fiber Payroll**, a deployable contractor payout stack for small teams. The key decision was to avoid building a second backend from scratch. Instead, I treated the existing on-chain job lifecycle and Fiber primitives as the base layer, then built a thin payroll façade on top of them: contractor records, payout creation/execution, a normalized payroll ledger, and a lightweight contractor portal. The result is a product surface that speaks in employers, contractors, payouts, and settlement status, while still reusing the underlying `post_job -> reserve_job -> claim_job -> complete_job` flow that already existed in `NERVE`.

This was also a week of deliberate product restraint. The easy thing would have been to rebrand the README and stop there, or to over-correct and invent a whole new backend abstraction disconnected from the chain model. I did neither. The architectural line I held all week was: **NERVE remains the engine; Fiber Payroll becomes the product surface**. That one sentence drove almost every code decision that followed.

## Issue-Focused Research Completed
- Internal architecture review of `mitosis`'s existing execution model: `packages/core`, `packages/mcp`, Fiber helpers, and the on-chain `job_cell` lifecycle.
- Re-read of the current `jobs` route and transaction builder flow to confirm that payroll execution could honestly map to `post_job -> reserve_job -> claim_job -> complete_job`, rather than faking a separate status model.
- Review of existing Fiber capabilities already present in the repo:
  - direct invoice/payment flow
  - hold-invoice escrow setup
  - worker lock-arg mapping
  - MCP-level Fiber settlement hooks
- Internal product research against the codebase itself:
  - what already existed and could be reused
  - what was still "marketplace vocabulary" and needed translation
  - where the backend was thin enough to extend without rewriting contracts

## Key Learnings
- **The strongest move was not a rewrite; it was a translation layer.** `nerve` already had the hard primitives: Rust transaction building, CKB job lifecycle, identity/reputation context, Fiber helper routes, and payout-capable settlement logic. The missing piece was product language and product shape. Reframing those primitives as payroll operations turned out to be cheaper and cleaner than inventing a new ledger model from nothing.
- **The existing NERVE job model is flexible enough to serve as a payroll execution engine if the UI and API stop exposing it raw.** This was the decisive architectural point of the week. A payout is still a job under the hood, but an employer should never need to think in `Open`, `Reserved`, `Claimed`, or `Completed`; they should see `draft`, `queued`, `processing`, `paid`, or `payment_failed`. Once I committed to that translation boundary, the implementation got much clearer.
- **File-backed state is the right v1 trade-off here.** `nerve` did not already have an application database layer, and spending the week bolting on Postgres or SQLite would have burned time on infrastructure instead of product shape. A JSON-backed contractor/payout store inside `packages/mcp` is enough to prove the product and keeps the migration path to a real database obvious later.
- **The employer and contractor experiences need different truth surfaces.** Employers need operational visibility: who is payout-ready, what failed, and whether Fiber settlement actually completed. Contractors need only a narrow surface: update payout details, see payout history, and know whether action is required. That asymmetry made the portal token approach the right v1 move instead of full auth.
- **The Rust signer work still mattered even though the visible product work lived mostly in TypeScript and React.** Repackaging the product only works if the transaction/signing layer remains coherent. Tightening `signer.rs` and identity derivation in the same window made the new product surface less likely to rest on brittle internals.
- **Documentation became part of the product, not a postscript.** The repo still points to `RobaireTH/NERVE`, but the README, compose flow, and discovery manifest needed to lead with Fiber Payroll if the direction was going to be legible to anyone outside the codebase.

## Brief
- Primary work this week was transforming `nerve` from a generic NERVE marketplace stack into a payroll-shaped product.
- The central architectural decision was to keep NERVE as the execution engine and add a payroll façade in `packages/mcp` rather than invent a separate backend.
- The second major thrust was building the product surface: a new React/Vite web app under `packages/web` with an employer dashboard, payroll ledger, and contractor portal.
- The third layer was infrastructure and product packaging: discovery route updates, compose-level web service, signer cleanup, integration docs, limitations, and submission materials.

## Practical Progress

### Product framing and architecture
- Wrote and committed the design spec.
- Defined the product line clearly:
  - `Fiber Payroll` is the user-facing product
  - `NERVE` remains the underlying execution engine
- Chose the thin-facade architecture explicitly:
  - contractor record = payout destination metadata + optional lock args / Fiber routing info
  - payout = existing NERVE job plus embedded Fiber payment metadata
  - payroll ledger = normalized view over job state and Fiber settlement state
  - contractor portal = tokenized update/status view rather than a full account system

### Infrastructure and product rebrand
- Reframed the repository entrypoint around Fiber Payroll in `README.md` (`11889b7 chore: rebrand to Fiber Payroll and update infrastructure`).
- Added the web service to `docker-compose.yml` so the stack is no longer just `core + mcp + fiber + agent`; it now has a product UI surface as part of the deployment story.
- Updated startup/testing scripts and OpenClaw config in the same pass so the supporting infrastructure did not lag behind the new product narrative.

### Core/signer layer
- Tightened signer behavior in `packages/core/src/signer.rs` and related identity handling (`0c0440c feat(core): improve signer and identity management`).
- Added MCP tool-call support in the Superise signer path and cleaned up identity derivation / signing compatibility checks. This matters because the payroll surface still ultimately resolves into the same transaction-building and signing path as the original NERVE flows.

### MCP application layer
- Refactored MCP startup to use a separate `app.ts` and updated package/dependency wiring (`6ee4149 feat(mcp): update infrastructure and discovery routes`).
- Reworked the discovery route so the public entrypoint speaks in payroll language first, while still retaining access to the engine routes underneath.
- Implemented the payroll state layer (`596761e feat(mcp): implement payroll management features`):
  - `packages/mcp/src/payroll/types.ts`
  - `packages/mcp/src/payroll/store.ts`
  - `packages/mcp/src/payroll/executor.ts`
- Added contractor and payroll routes:
  - `GET/POST/PATCH /payroll/contractors`
  - `GET/POST /payroll/payouts`
  - `POST /payroll/payouts/:id/execute`
  - `GET /payroll/ledger`
- Implemented payout execution as a real translation into the existing job lifecycle:
  1. validate contractor payout readiness
  2. `post_job`
  3. `reserve_job`
  4. `claim_job`
  5. `complete_job`
  6. persist normalized payroll state
- Added the contractor portal routes and in-process route test harness (`92a1190 feat(mcp): add contractor portal and update CLI scripts`):
  - `GET /portal/:token`
  - `PATCH /portal/:token/profile`
  - `GET /portal/:token/payouts`
- The important design choice here was to keep the façade stateful and product-shaped, but to let execution continue to flow through the pre-existing CKB/Fiber rails rather than creating duplicate orchestration logic.

### Web product surface
- Added a full React/Vite web package under `packages/web` (`db1cde3 feat: add web dashboard and comprehensive documentation`).
- Built the employer dashboard:
  - contractor listing
  - contractor creation flow
  - payout creation flow
  - payout execution trigger
  - readiness indicator (`Ready` vs `Needs payout details`)
  - portal entry link per contractor
- Built the payroll ledger:
  - normalized payout rows
  - status and Fiber settlement columns
  - inline failure reason rendering for operational debugging
- Built the contractor portal:
  - token-based entry
  - editable payout details
  - payout history surface
  - save confirmation state
- Important UI decision: I intentionally removed the dependency on a full router runtime after the local install proved unreliable in this environment. Instead, I used a minimal pathname-based view switch inside `App.tsx` so the built app remained deterministic. The product behavior was the goal; the routing abstraction was not worth blocking the week.

### Documentation and support artifacts
- Added:
  - `INTEGRATION_SUMMARY.md`
  - `LIMITATIONS.md`
  - `TEST.md`
  - `implementation-plan.md`
  - `submission-draft.md`
  - `submission-draft.zh.md`
  - `hackathon-ckb.md`
  - `docs/superpowers/plans/2026-06-09-fiber-payroll.md`
- These were not busywork. They captured the exact execution path, open constraints, and packaging story needed to make the repo legible as a shipped project instead of a one-off code dump.

## Reflections: why this shape, and what it was built upon
The most important design decision this week was refusing both extremes:

1. **Not stopping at a rebrand.** A renamed README without a payroll façade would still have been a marketplace engine wearing a payroll costume.
2. **Not overbuilding a new backend.** A brand-new payroll service would have ignored the fact that `nerve` already had a working execution substrate worth preserving.

The work was built on three pre-existing foundations inside `nerve`:

- **NERVE's execution engine**
  `packages/core` already knew how to build, sign, and broadcast the transaction flows that matter.

- **The job lifecycle**
  `post_job`, `reserve_job`, `claim_job`, and `complete_job` already formed a strong state machine for escrowed work/result settlement. Payroll could be treated as a constrained specialization of that lifecycle rather than a whole new protocol.

- **Fiber integration**
  The project already had enough Fiber capability that it would have been wasteful to throw it away and call out to some unrelated payment abstraction.

So the practical thesis of the week became:
**turn existing protocol machinery into a narrower product instead of inventing new machinery.**

That is also why I stayed with file-backed storage and a tokenized portal. Both are obviously incomplete in the abstract, but they are the right v1 choices when the real work is proving the product shape and the API translation layer.

## Proof of Participation
- `mitosis` repo / NERVE base: `origin https://github.com/RobaireTH/NERVE.git`
- Commits in scope for the week:
  - `580a9bc` — design spec
  - `11889b7` — product rebrand + infrastructure
  - `0c0440c` — signer / identity improvements
  - `6ee4149` — MCP app + discovery refactor
  - `596761e` — payroll storage / executor / routes
  - `92a1190` — contractor portal + route test harness
  - `db1cde3` — web dashboard + docs + submission material
- Primary files/surfaces built on top of the original engine:
  - `packages/mcp/src/payroll/`
  - `packages/mcp/src/routes/payroll.ts`
  - `packages/mcp/src/routes/portal.ts`
  - `packages/web/`


## Challenges
- **Direction-setting was the hard part, not just implementation.** The repo had enough existing capability that it would have been easy to rationalize almost any product story. The real challenge was choosing one and forcing the codebase to align around it.
- **The existing repository still carries the NERVE identity everywhere.** README, badges, remote URL, docs, and architectural language are now partially product-first and partially engine-first. That is fine for one week of packaging work, but it means the repo currently tells two truths at once.
- **The local frontend dependency environment was unreliable.** React Router’s install was corrupted in a way that broke both test and build resolution, so I had to switch to a smaller local pathname router to keep forward motion. That was an implementation concession to the environment, not a product decision.
- **MCP testability required an in-process harness because the sandbox blocks some socket patterns.** Pulling the Express app into `app.ts` and invoking it directly was the correct move, but it was still extra plumbing that had to exist before the route work could be validated cleanly.
- **The current product depth is still thinner than the framing.** By the end of the week, the contractor creation, payout execution, ledger, and portal flows exist. But deeper operational polish — better in-app navigation, browser-level QA, live data states, and more deliberate responsive/visual verification — still needs another pass before I would call the product surface mature.

