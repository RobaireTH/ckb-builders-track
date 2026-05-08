# CKBuilder Track Weekly Report — Week 17
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **May 7, 2026**

## Artifacts Completed
- The main artifact this week was a security and trust-boundary hardening pass on `pckt` across the contract, backend, and frontend.
- The supporting research for the week lived inside `PCKT/NOTES/`, especially the security audit synthesis, backend boundary notes, contract notes, and the Talk Nervos draft that helped me explain the system more clearly.

## Issue-Focused Research Completed
- [Store Data on Cell](https://docs.nervos.org/docs/dapp/store-data-on-cell)
- [Intro to Script](https://docs.nervos.org/docs/script/intro-to-script)
- [ckb_occupied_capacity crate docs](https://docs.rs/ckb-occupied-capacity/latest/ckb_occupied_capacity/)
- [getrandom crate docs](https://docs.rs/getrandom/latest/getrandom/)
- [X-Forwarded-For header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Forwarded-For)
- [URI fragment](https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Fragment)
- [Referrer-Policy header](https://developer.mozilla.org/docs/Web/HTTP/Reference/Headers/Referrer-Policy)
- [Unvalidated Redirects and Forwards Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.html)
- [Forgot Password Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Forgot_Password_Cheat_Sheet.html)

## Key Learnings
- This week pushed `pckt` out of the "social transfer demo" mindset and into a stricter trust-model mindset. The product is only credible if the chain remains authoritative, the backend stays replaceable, and the frontend does not quietly smuggle risk back in through convenience features.
- The CKB-specific research this week was about cell viability, not chain basics. Looking again at cell storage and script behavior made the lucky-floor problem much clearer: the minimum is not only a product heuristic, it is a successor-cell survival constraint. Raising the lucky-slot floor, surfacing a live minimum hint, and rejecting recipient claim cells that carry data were all part of keeping future state transitions valid instead of only making the UI nicer.
- The `getrandom` and shortlink research made the slug issue straightforward. A share-link system does not get to use "random-looking" IDs; it needs cryptographically secure randomness from the OS if those links are going to act like capability URLs.
- The trusted-proxy and `X-Forwarded-For` research clarified that IP-based rate limiting is only meaningful if proxy trust is explicit and bounded. Otherwise the header is attacker-controlled input and the limiter is partly theater.
- The URI-fragment and referrer research clarified an important nuance for the claim-link design. The fragment itself is client-side and is not sent to the server, which supports the original `pckt` trust boundary. But OWASP's URL-token guidance also reinforced that link-secret flows still need `noreferrer`/`no-referrer`, careful share surfaces, and brute-force/rate-limit thinking if I do not want the social flow to leak secrets indirectly.
- The OWASP redirect guidance lined up directly with the shortlink issue I was hardening. If the backend accepts arbitrary redirect targets, the app's own domain can become part of a phishing path. Host allow-listing and server-side mapping are the right direction here.
- I also learned something important from the experiments I reverted. I briefly tried encrypted claim-secret storage with device-token pairing and a richer reshare flow. Both sounded useful, but they moved `pckt` toward a backend-assisted custody model and made the trust boundary less clean. Reverting them was not wasted work; it was architectural clarification.
- Another useful lesson was about product truth under asynchronous state. Social packet flows create notifications, reloads, claim races, and delayed visibility problems. Persisted notifications, better inbox behavior, friendlier errors, and less noisy reload sync are not polish-only. They are part of whether users trust what the app is telling them.
- Sender naming also became clearer this week. Local profile labels improve legibility, but they are still convenience data, not identity. Restricting those names to the owner’s own view was the correct move because it avoids pretending a mutable backend label is universal truth.

## Brief
- This week was mostly about hardening `pckt` after security review and product feedback.
- I was working at two levels at once. At the code level, I was fixing concrete issues in the contract, share flow, shortlink handling, rate limiting, notification UX, and packet-creation constraints. At the design level, I was checking whether those fixes still respected the original promise of the product: a friendly way to send CKB without quietly making the backend authoritative.
- The biggest pattern across the week was restraint. Several tempting features looked like "better UX" at first glance, but once I followed the trust assumptions all the way through, they created more confusion than value. That was especially true for claim-secret storage and reshare. I would rather keep the system simpler and more honest than add convenience by making the backend hold too much responsibility.
- By the end of the week, `pckt` felt more mature. It had stronger guardrails around claim outputs and shortlinks, a cleaner privacy story for share links, a more honest lucky-split floor, better notification and error surfaces, and a clearer sense of what should remain off-chain convenience versus on-chain truth.

## Practical Progress
- Raised the lucky-slot minimum payout to **63 CKB** and later added a live minimum hint so packet creators can see the real floor while choosing slot counts.
- Cleared packet lists when the wallet disconnects so stale sender or claimer state does not keep hanging around in the UI.
- Added friendlier error handling and richer share UX so failed claim or share paths explain themselves more cleanly.
- Added persisted notifications and an in-app notification panel, then suppressed the noisy notification dump that happened on reload.
- Restricted sender profile names so they are shown only in the owner's own view rather than being treated like public identity.
- Hardened shortlink generation by switching to OS CSPRNG-backed slugs.
- Restricted shortlink destinations to an allow-listed host set and updated deployment config so only intended `sendpckt` frontend hosts are accepted.
- Tightened backend abuse handling by rate-limiting by peer IP with a bounded bucket map and explicit proxy-trust configuration.
- Set `Referrer-Policy: no-referrer` and stripped URL fragments out of the feedback `mailto` flow so claim-link secrets do not leak through support/reporting paths.
- Hardened the contract by rejecting claim transactions whose recipient output carries data, then redeployed the testnet packet lock with the corrected deployment metadata.
- Re-enabled `lucky split` only after the updated deployment and revised floor handling made the mode honest enough to expose again.
- Simplified the share surface by dropping one-time-link warnings and removing the public shortlink from the main create-share flow.
- Prototyped encrypted claim-secret storage with device-token pairing and a reshare screen, then reverted both after deciding they blurred the non-authoritative backend boundary and added too much complexity for the current stage.
- Replaced the loud alert-style copy feedback with a smaller dark confirmation pill to make the share interaction feel calmer.

## Proof of Participation
- `pckt` source: [github.com/RobaireTH/pckt](https://github.com/RobaireTH/pckt)
- `pckt` backend: [pckt-backend.fly.dev](https://pckt-backend.fly.dev)
- `pckt` live: [send pckt](https://sendpckt.robaireth.dev)
- Talk update thread: [pckt on Talk](https://talk.nervos.org/t/pckt-a-friendly-way-to-send-ckb-pckt-ckb/10220/4)

## Challenges
- The main challenge this week was designing a safer share flow without turning the backend into a secret custodian. The product becomes easier to use if the server helps more, but the trust model becomes weaker just as quickly.
- Lucky-split UX is still constrained by CKB cell-capacity realities. The social metaphor says "fun random packet," but the chain says "you must preserve enough structure and capacity for every remaining state transition." The UI has to teach that honestly.
- Notification truth is also still a real challenge. Packet claims are asynchronous, wallet-dependent, and chain-backed, so it takes deliberate effort to keep inbox, activity, and packet views aligned with reality.
- Local sender profiles are useful, but they are still not a real identity layer. That part remains intentionally unfinished.
