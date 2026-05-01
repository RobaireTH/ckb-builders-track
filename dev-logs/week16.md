# CKBuilder Track Weekly Report — Week 16
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Apr 28, 2026**

## Artifacts Completed
- The main artifact this week was `pckt`, a red-packet style CKB application that moved through real contract, backend, frontend, and deployment work on testnet.
- I also wrote a Talk Nervos draft for `pckt` that explains the architecture, design choices, deployment shape, and security boundaries in a way that is easier to share publicly.

## Courses / Readings Completed
- [How CKB Works](https://docs.nervos.org/docs/getting-started/how-ckb-works)
- [Intro to Script](https://docs.nervos.org/docs/script/intro-to-script)
- [Transfer CKB](https://docs.nervos.org/docs/dapp/transfer-ckb)
- [RFC0002 Nervos CKB: A Common Knowledge Base for Crypto-Economy](https://nervosnetwork.github.io/rfcs/rfcs/0002-ckb/0002-ckb.html)
- [Fly Volumes overview](https://fly.io/docs/volumes/overview/)
- [Create and manage volumes on Fly](https://fly.io/docs/volumes/volume-manage/)
- [Vercel Environment Variables](https://vercel.com/docs/environment-variables)
- [Vercel Project Environment Variables](https://vercel.com/docs/projects/environment-variables)
- [The “Lucky Money” That Started It All—The Reinvention of the Ancient Tradition “Red Packet” in Digital Times](https://journals.sagepub.com/doi/10.1177/20563051211041643)
- [Online Red Packets: A Large-scale Empirical Study of Gift Giving on WeChat](https://arxiv.org/abs/1712.02926)
- [Antom Red Packet QR Code POSM display guideline](https://global-agna.alipay.com/docs/ac/redpacket/redqrmaterials)
- [Alipay primary POSM display guideline](https://global-agna.alipay.com/docs/ac/redpacket/basicmaterials)

## Key Learnings
- This week reinforced that small social financial products still need hard protocol discipline. A playful UX does not reduce the need for correct capacity math, claim guards, expiry handling, or reclaim behavior.
- The CKB readings were a useful reset because they forced me to think again in live cells, dead cells, lock scripts, type scripts, and explicit state transitions instead of drifting into account-model assumptions. That matters a lot for a product like `pckt`, where every claim consumes and recreates state.
- Reading the official script and transaction material also made the reclaim path feel conceptually cleaner. The contract is not "editing a packet." It is validating two legitimate spend paths over the same evolving packet cell: claim before expiry, reclaim after expiry.
- I learned again that on-chain correctness and user-visible correctness are different jobs. The contract can be right while the UI still feels broken if sender activity, claimer history, explorer visibility, or packet lineage are displayed badly.
- The sender and claimer do not need the same view of truth. Splitting sent and claimed history, collapsing duplicated packet lineage, and showing the right totals per role matters a lot more than it first appears.
- Mobile responsiveness is not a polish-only issue in a product like this. If the packet cards, home screen, or activity surfaces stretch or overflow badly, the app starts to feel untrustworthy even when the chain logic is correct.
- I also got a sharper feel for what should remain convenience-only for now. Local sender profiles are useful, but they are not identity. They improve legibility without pretending to solve the DID question yet.
- The red-packet studies were surprisingly useful because they pushed me back toward the social meaning of the product. Red packets are not only about payment distribution. They are about ritual, reciprocity, luck, play, and group participation. That helped me see why the app should feel light and shareable even while the chain logic stays strict.
- The red-packet merchant material was useful in a more practical way too. It reminded me that these flows are not just digital curiosities. They often live in QR-first, mobile-first, glanceable environments where signage, simplicity, and fast recognition matter. That maps directly onto how `pckt` should behave if it ever grows beyond a small demo surface.
- The Fly and Vercel docs were also part of the learning this week. They were a reminder that operational details like persistent volumes, deployment environment boundaries, and config hygiene are part of product trust too, especially when the backend is indexing live chain state.
- The deployment docs also sharpened my thinking about replaceability. If the backend is meant to be non-authoritative, the deployment setup should reflect that discipline: explicit env-driven contract config, persistent but disposable indexed state, and a frontend that can swap environments without changing the whole trust model.
- Finally, the "lucky split" question became more concrete. I do not dislike the mode in theory. I dislike unsafe payout math and dust behavior in practice. If the contract can create outcomes I would not trust, then disabling or constraining the mode is the correct decision.

## Brief
- This week was the implementation and hardening week for `pckt`.
- Unlike the `Fiber Checkout` work, which stayed in product and architecture definition, `pckt` forced me back into the concrete details of contract behavior, transaction-building, live data, reclaim flow, frontend state, and deployment quality.
- It was also a week of broader product reading. I did not want `pckt` to become only a technically correct packet contract with a generic interface sitting on top. The extra reading helped me think more seriously about the social logic of red packets, the QR-first ritual around them, and the kind of lightness the product should preserve if it wants to feel culturally legible instead of merely functional.
- The app itself is intentionally small in concept: seal a packet, share a link, let people claim, and reclaim what remains after expiry. But getting that to feel honest required much more than only wiring a contract. I had to make the sender and claimer surfaces accurate, reduce duplicated state in the UI, expose explorer visibility, improve mobile behavior, and make the reclaim path real.
- By the end of the week, `pckt` was no longer just a contract plus a nice landing page. It had a stronger end-to-end story across backend indexing, frontend activity views, reclaim handling, testnet deployment, metadata polish, and public-facing documentation.

## Practical Progress
- Added a research layer around both the protocol and the social product itself, so this week was not only about fixing code but also about understanding the tradition and interaction model the product is borrowing from.
- Deployed the backend to Fly and wired the frontend to live testnet-backed data instead of only local or placeholder flows.
- Updated the testnet contract deployment metadata and then redeployed a corrected version when contract and claim-path issues surfaced.
- Fixed signing-flow errors and relay transaction RPC serialization issues that were blocking or confusing transaction submission.
- Fixed seal transaction reserve and capacity collection logic so packet creation behaves more honestly against actual wallet balances.
- Hardened the claim flow and added better protection around duplicate or invalid claim behavior after the corrected contract deployment.
- Implemented the reclaim flow for expired packets and wired it into the UI so the sender can actually recover remaining balance after expiry.
- Split sender and claimer history into the right views instead of mixing them together.
- Fixed blank or incorrect claimer activity surfaces so received packets and messages show up where they should.
- Collapsed duplicated packet lineage in sender-facing views so a packet does not keep multiplying visually as claims happen.
- Fixed claimed-packet timestamp formatting and then corrected sender and claimed packet dates again when inconsistencies showed up under real usage.
- Added explorer links so senders and claimers can track relevant transactions more conveniently.
- Improved packet-card behavior across desktop and mobile:
  - clamped sideways viewport overflow
  - fixed active packet card sizing
  - improved responsive layout on the home screen
  - reduced trashy duplication and stretched-card behavior
  - added stronger empty-state handling
- Added sender profile labels and working settings actions so addresses can be presented more legibly in the product.
- Updated metadata and product wording so the app presents itself more clearly as **A friendly way to send CKB**.
- Used the red-packet product and social readings to sharpen the public narrative around `pckt`, especially the difference between a plain transfer tool and a social, time-bound, multi-recipient ritual.
- Used the deployment and configuration readings to keep the implementation story cleaner:
  - Fly volume usage for indexed backend state
  - env-driven frontend and backend configuration
  - clearer separation between deployment environment and protocol truth
- Wrote the Talk Nervos article draft covering:
  - problem and solution
  - architecture
  - contract model
  - deployment architecture
  - security considerations
  - why `lucky split` remains constrained for now

## Proof of Participation
- `pckt` source: [github.com/RobaireTH/pckt](https://github.com/RobaireTH/pckt)
- `pckt` backend: [pckt-backend.fly.dev](https://pckt-backend.fly.dev)
- `pckt` live: [send pckt](https://sendpckt.robaireth.dev)
- Folow Talk updates: [pckt on Talk](https://talk.nervos.org/t/pckt-a-friendly-way-to-send-ckb-pckt-ckb/10220/4)
## Challenges
- The hardest challenge was keeping the app socially simple while the implementation details stayed financially honest. Claiming by link feels light, but the underlying contract and transaction flows are unforgiving.
- The UI truthfulness problem was real. Duplicated packet lines, wrong totals, bad timestamps, stretched cards, and blank claimant views all damage trust quickly, even when the chain state is technically fine.
- Reclaim and claim concurrency still demand careful UX messaging. Some failures are genuine protocol collisions or state races, not necessarily product bugs, but the product still has to explain them well.
- Identity remains deliberately unfinished. Sender labels help, but they are not the final answer. A more durable and user-owned identity layer still sits ahead if I want the social side of the app to feel truly native.
- `Lucky split` remains the clearest example of where product ambition has to yield to protocol safety. Until the payout math is something I trust fully, I would rather keep the mode constrained than ship a version that feels fun but can fail badly.
