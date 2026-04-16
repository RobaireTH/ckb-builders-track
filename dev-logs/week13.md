# CKBuilder Track Weekly Report — Week 13
- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Apr 18, 2026**

## Courses Completed
- [W3C DID Core](https://www.w3.org/TR/did-core/)
- [The did:web Method Specification](https://w3c-ccg.github.io/did-method-web/)
- [Well-Known DID Configuration](https://identity.foundation/well-known-did-configuration/resources/did-configuration)
- [did:webvh](https://github.com/decentralized-identity/didwebvh)
- [Web5: Extra Decentralized](https://www.nervos.org/knowledge-base/web5-extra-decentralized)
- [Where we're going, we don't need Accounts](https://www.nervos.org/knowledge-base/account_abstraction_where_were_going)
- [JoyID](https://joy.id/)
- [.bit Protocol](https://d.id/id-protocol/bit)
- [A New Chapter: .bit has successfully upgraded to RGB++ and DOB Protocol](https://blog.d.id/p/a-new-chapter-bit-has-successfully)
- [Spore Protocol Docs](https://docs.spore.pro/)

## Key Learnings
- I treated this week as `did:web` research. I did not more code changes as I await responses/feedback and directions.
- The [DID Core spec](https://www.w3.org/TR/did-core/) made one thing click for me immediately: DID documents are not only about keys. They also define service endpoints. The spec literally says services enable "trusted interactions", and that maps very cleanly onto what `ckb-pop` is already exposing through `/api/module/manifest`, `/api/claims/issue`, `/api/claims/verify`, and the event lookup routes.
- The [did:web method spec](https://w3c-ccg.github.io/did-method-web/) is practical as that pushed me away from treating `did:web` as a trust root and toward treating it as a discovery and reputation layer.
- The did:web spec's privacy warnings mattered more than I expected. Since resolution happens through DNS and HTTPS fetches, the DID method itself leaks infrastructure dependencies and request visibility. For `ckb-pop`, that means I should not confuse a resolvable web identity with censorship resistance or durable provenance.
- The [Nervos Web5 article](https://www.nervos.org/knowledge-base/web5-extra-decentralized) helped me see the bigger picture. It frames Web5 around self-owned DIDs, verifiable credentials, DWNs, peer-to-peer networking, PoW and UTXO-style settlement. That stack feels much closer to CKB than a lot of account-chain identity narratives do.
- The [Nervos account abstraction article](https://www.nervos.org/knowledge-base/account_abstraction_where_were_going) reinforced why CKB is unusual here. If accounts are not sacred objects and auth can be defined more flexibly at the protocol layer, then identity on CKB does not need to collapse into one wallet type or one signing scheme.

- [.bit](https://d.id/id-protocol/bit) was the strongest CKB-adjacent signal in this research. It is already treating identity as a self-sovereign data surface with names, records, subaccounts, achievements, and cross-chain interoperability. The [RGB++ and DOB upgrade note](https://blog.d.id/p/a-new-chapter-bit-has-successfully) made that even more relevant because it links identity more directly to programmable objects and Bitcoin-adjacent flows.
- [Spore](https://docs.spore.pro/) sharpened the object question again. If `ckb-pop` badges become richer digital objects instead of only "proof hashes plus a frontend interpretation", then DID-linked achievements begin to make much more sense than anonymous one-off mints.

## Brief
- This week was the identity week for `ckb-pop` and `ckb-pop-cli`. I was trying to answer a simple but important question: if I add a DID layer, what exactly should it do, and what should it never be allowed to replace?
- The answer is getting clearer. DIDs are very good for discovery, naming, service metadata, issuer keys, and portable identity references. They are not a substitute for the CKB guarantees I already care about: uniqueness, ownership, and durable artifact provenance.
- That distinction matters a lot for `ckb-pop`. The backend is already explicitly non-authoritative, the contracts remain the source of truth, and the module manifest already describes capabilities in a way that looks very close to DID-discoverable service metadata. So the DID layer should sit above the protocol as a legibility layer, not below it as the source of trust.
- For `ckb-pop-cli`, the implication is also practical. The CLI currently assumes a known backend and uses an external wallet flow through the browser signer and CCC. A DID-capable path could let the CLI resolve organizer or project identity first, discover the module manifest and claim surfaces second, and only then move into signing and transaction building.
- The most useful framing I got from this week is a layered one:
  - CKB address / JoyID / CCC signer for custody and transaction authority
  - `did:web` or `LinkedDomains` for project and service discovery
  - `.bit` for portable, human-readable, CKB-native identity
  - Spore / DOB style objects for richer achievement and proof surfaces
  - CKB contracts for final protocol enforcement

## Practical Progress
- Audited `ckb-pop` around the actual reusable boundary, not the old app boundary. The package, manifest route, proof drivers, artifact drivers, and policy extensions all made more sense once I looked at them as identity-discoverable surfaces rather than only internal module pieces.
- Mapped `assertionMethod` in DID thinking onto the organizer-issued signed claim path already present in `ckb-pop`. I do not need to force full VC machinery into v1 to see the shape. The current signed-claim path is already a minimal issuer-attestation primitive.
- Mapped DID `service` entries onto concrete `ckb-pop` routes:
  - module manifest discovery
  - event resolution
  - claim issuance / verification
  - badge observation
  - docs or integration surfaces
- Mapped the same logic onto `ckb-pop-cli`. The CLI can eventually accept an organizer DID or project DID, resolve its service document, pull the right backend manifest, and then continue into event creation, attendance, or badge verification against the discovered surface.
- Reframed `did:web` as the right place for project identity such as `ckb-pop.xyz`, not as the place where badge truth should live. If the domain changes or the hosting changes, the CKB anchors and badge cells should still remain the hard source of truth.
- Identified `LinkedDomains` as a strong next step if I want the web surface to certify another DID instead of making the web-hosted file the only controller.
- Identified `.bit` as the most CKB-native identity direction if I want organizer names, community presence, or user-facing identity to feel native to the ecosystem instead of bolted on from generic web DID tooling.
- Identified a possible longer path for richer PoP artifacts: organizer identity linked through DID, user identity expressed through CKB-native auth, and the proof artifact itself rendered as a richer object through Spore / DOB conventions.

## Challenges
- `did:web` is convenient, but it is still web-hosted, mutable, and infrastructure-visible. If I lean on it too heavily, I will accidentally move trust back toward the same web stack I am supposed to be escaping.
- `.bit`, JoyID, CKB addresses, and `did:web` all solve different identity problems. If I mix them carelessly, I will create identity sprawl instead of identity clarity.
- There is also a product-discipline risk here. `ckb-pop` is a presence primitive first. I do not want to bloat it into a full identity framework before the actual participation flows become sharper and more widely reusable.
- The final open question is whether I should ship service identity first, portable organizer identity first, or richer badge object identity first. I can see value in all three, but sequencing will matter.