# CKBuilder Track Weekly Report — Week 12
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Apr 01, 2026**

## Courses Completed
- [Omnilock: Universal Lock Script](https://pocket-node-learn-ckb.vercel.app/lessons/15-omnilock-wallet)
- [Building a Token DEX](https://pocket-node-learn-ckb.vercel.app/lessons/22-token-dex)
- [NFT Marketplace](https://pocket-node-learn-ckb.vercel.app/lessons/23-nft-marketplace)
- [Mainnet Deployment](https://pocket-node-learn-ckb.vercel.app/lessons/24-mainnet-deployment)
- [Spore Protocol Docs](https://docs.spore.pro/)
- [DOB Decoder Standalone Server](https://github.com/sporeprotocol/dob-decoder-standalone-server)
- [Web5: Extra Decentralized](https://www.nervos.org/knowledge-base/web5-extra-decentralized)
- [Common Knowledge Base (CKB): Understanding Our Ethos](https://www.nervos.org/knowledge-base/ckb_understanding_our_ethos)
- [ZK-Rollups vs. Optimistic Rollups](https://www.nervos.org/knowledge-base/zk_rollup_vs_optimistic_rollup)

## Key Learnings
- This week gave me a cleaner way to think about provenance on CKB. Provenance covers ownership, authorization, object definition, [meaning](https://talk.nervos.org/t/building-ckb-pop-a-reusable-participation-primitive-on-ckb/10136/2?u=mayowa_akinyele), rendering, transfer rules and the deployment guarantees that make the object trustworthy.
- `Omnilock` made the wallet story clearer to me. A single lock script supporting multiple auth modes means onboarding does not have to break around wallet type. This matters for both `ckb-pop` and `NERVE` if I want them to be usable outside a narrow CKB-native flow.
- The `order cell` pattern in the Token DEX lesson reinforced something I keep seeing on CKB. Many things that look like application logic elsewhere are better understood here as explicit cell lifecycles. Atomic exchange, partial fills, remainder cells and even front-running resistance come from the transaction model itself.
- The NFT marketplace and Spore readings made me think harder about the object itself. A Spore is an on-chain object with its own content and behavior. That is closer to how I want to think about presence proofs, capability proofs and other artifacts I am creating.
- The `dob-decoder-standalone-server` was especially useful because it pushed the rendering question into protocol territory. If a digital object needs a decoder, then provenance also includes a verifiable path from bytes to human-readable meaning.
- The mainnet deployment lesson reminded me that provenance also depends on operational discipline. Code hash continuity, deployment records, hardware wallets, multisig, monitoring and an emergency response plan are part of whether the protocol can be trusted.
- The Nervos ethos and Web5 readings connected this to a wider direction: low time preference, PoW, UTXO-style state, first-class assets and peer-to-peer rails. That stack is much closer to the kind of systems I want `ckb-pop` and `NERVE` to become.
- The rollup comparison helped me separate settlement from execution more clearly. Faster interaction layers may matter later, but the provenance layer still needs to stay inspectable and final.

## Brief
- This week was my road to provenance. I was trying to understand what kind of objects `ckb-pop` and `NERVE` are actually producing, what guarantees those objects need, and what CKB-native path gives those guarantees honestly.
- For `ckb-pop`, the original idea was simple: prove that someone was somewhere. But the more I read, the less sufficient a bare proof feels. If the badge is going to matter, then it should be legible, ownable, renderable and durable. That pushed me toward thinking more seriously about `Spore`, `DOB`, decoder infrastructure and what it means for a proof to become a real object with provenance.
- For `NERVE`, the same line of thought became even clearer. The protocol is about agent identity, jobs, payments, results and reputation. That is really a provenance system for work. Who existed, who accepted the job, what result was tied to it, how value moved and how the state changed after that.
- `Omnilock` also mattered more than I expected. If I want either system to move past a demo for people already inside the ecosystem, then wallet and auth flexibility cannot be an afterthought. Passkeys, Ethereum-style auth, multisig, ACP patterns and time-locks all begin to feel like infrastructure.
- The mainnet lesson closed the loop. If provenance is the direction, then deployment quality belongs inside the protocol story itself and not outside it as an ops concern.

## Practical Progress
- Reframed `ckb-pop` more explicitly as a provenance protocol, with badge minting as one part of the flow.
- Mapped `Spore` and `DOB` ideas onto `ckb-pop` badges as a possible next step for making proofs more legible and portable.
- Studied the standalone DOB decoder server to understand how renderability can be handled without depending on one frontend’s interpretation.
- Tightened my mental model for `NERVE` as provenance for agent work: identity, capability, result binding, settlement and reputation as connected state transitions.
- Clarified that `Omnilock` is likely part of the onboarding path if I want broader wallet compatibility for both projects.
- Built a more serious picture of what the next step means operationally: code hash verification, deployment records, multisig governance, monitoring and soft-launch discipline before anything mainnet-facing.

## Proof of Participation
You can find
- the `ckb-pop` source code [here](https://github.com/RobaireTH/ckb-PoP)
- the `NERVE` source code [here](https://github.com/RobaireTH/NERVE)
- the `ckb-pop` build [here](https://ckb-pop.vercel.app/#/)
- the `NERVE` docs [here](https://nerve-docs.vercel.app)
- the study materials in the links above

## Challenges
- The biggest open question now is object standardisation. I can see why `Spore` and `DOB` make sense for provenance, but I do not want to force `ckb-pop` into a richer object model too early if that adds complexity without improving the actual proof.
- For `NERVE`, I still need a sharper boundary between what should live on-chain as durable state, what should stay in witnesses, and what should only exist as decoded or rendered views.
- Mainnet thinking introduces governance tension immediately. `data1` immutability is the cleanest trust story, but `type`-based upgradeability is the safer early-stage engineering path. Choosing that boundary well matters.
- The Web5 and Nervos ethos direction is compelling, but the hard part is keeping the system genuinely peer-to-peer while still delivering an experience that normal users can survive. That is still the balance I am trying to get right.
