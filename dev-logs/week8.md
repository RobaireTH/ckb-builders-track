# CKBuilder Track Weekly Report — Week 8

- Name: **Mayowa Temitope AKINYELE**
- Week Ending: **Mar 4, 2026**

## Courses Completed

- [Type ID mechanics — ckb-std source](https://github.com/nervosnetwork/ckb-std/blob/master/src/type_id.rs)
- [RFC 0022 — Type ID section](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md#type-id)
- Went over the Cell Model again to confirm my understanding of it isn't faulty.

## Key Learnings

- The in-transaction UTXO count I was using for badge uniqueness was completely illusory. A contract can only see cells in the current transaction, not live UTXOs in the UTXO set from a previous one. A second mint in a separate transaction would pass every check I had written. I had a uniqueness constraint with no actual enforcement.
- Type ID is the right primitive for cell-level singleton enforcement. The ID is derived from `blake2b(first_input_outpoint || output_index)`. Because a UTXO can only be consumed once, that input outpoint can never appear in any future transaction, so the type_id is globally unique across all time. This is not a soft constraint — it is enforced by the cell model itself.
- Type ID solves the wrong half of the problem for ckb-pop specifically. It guarantees that a minted badge cell is a singleton, but it does not prevent two separate mint transactions — each with different inputs — from producing two different type_ids for the same `(event_id, recipient)` pair. The chain would accept both. The protocol-level uniqueness constraint (one badge per attendee per event) is still unresolved and needs a different mechanism.
- Cell capacity is sensitive to args size. Reviewer feedback to shrink the two hash fields from 32 to 20 bytes each reduced args from 96 bytes to 60 bytes, saving 36 bytes directly off the minimum cell capacity every badge must carry. At scale that matters.
- Disabling `badgeExistsHint` exposed a real indexer limitation. With type_id at bytes 0–31, a prefix search by `(event_id, recipient)` through the CKB indexer is no longer possible. The indexer operates on raw args bytes; it cannot skip the type_id prefix to search by the fields behind it. A backend-side lookup is the only practical replacement which I have not implemented. I await feedback.

## Brief

- This week, I was correcting a fundamental contract bug. The mentor and reviewer @XuJiandong pointed out that the `dob-badge` uniqueness check was unenforceable, and suggested Type ID from ckb-std as the replacement. I went through the implementation, understood the mechanics and rewrote the contract. I also updated the CLI to match.I redeployed to testnet after shrinking the hash fields in response to review feedback.

- I wrote the PR description explaining exactly why the old check failed made me realise the boundary of what Type ID actually guarantees. It gives you a singleton cell. It does not give you a singleton badge per attendee per event. Those are different things and I had conflated them. The remaining uniqueness problem is a protocol design question, not a contract implementation question, and I want a clear answer before I close out this contract.

- On the CLI side, the week was mostly sync work. I kept args serialisation and deploy hashes consistent with the contract changes. I added a README so the tool is actually usable by someone finding it for the first time.

## Practical Progress

- Diagnosed the root cause of Issue: `dob-badge` counted input/output cells with matching args within the current transaction, which cannot see prior live UTXOs, making the check meaningless for cross-transaction re-minting.
- Replaced the broken uniqueness check with `check_type_id(0)` from `ckb-std`. The args layout changed from 64 bytes to 96 bytes: `bytes 0–31: type_id | bytes 32–63: SHA256(event_id) | bytes 64–95: SHA256(recipient_address)`.
- Applied reviewer feedback from @XuJiandong: shrunk both hash fields from 32 to 20 bytes, reducing total args from 96 to 60 bytes and lowering minimum badge cell capacity. New layout: `bytes 0–31: type_id | bytes 32–51: SHA256(event_id)[:20] | bytes 52–71: SHA256(recipient_address)[:20]`.
- Updated frontend badge minting and chain sync logic for the new 60-byte args layout (PR #71).
- Redeployed contracts to Pudge testnet twice: v3 for the initial Type ID switch, v4 after the hash field reduction. Updated `DEP_TX_HASH` in both the frontend config and the CLI contract registry.
- Disabled `badgeExistsHint` in the frontend (it now always returns false). With type_id occupying bytes 0–31, the indexer cannot prefix-scan by `(event_id, recipient)`. A backend query is needed as a replacement and is not yet implemented.

## Proof of Participation

You can find

- the reference dApp [here](https://ckb-pop.vercel.app/#/)
- the ckb-pop source code [here](https://github.com/RobaireTH/ckb-PoP.git)
- the CLI source code [here](https://github.com/RobaireTH/ckb-pop-cli)
- other proof of participation [here](https://drive.google.com/drive/folders/1WCzOK6gJx_UT5DY7IwmYEaMaPn2PO54B?usp=drive_link) or review my [ckb-pop commit history](https://github.com/robaireth/ckb-pop/commits) and [ckb-pop-cli commit history](https://github.com/RobaireTH/ckb-pop-cli/commits).

## Challenges

- Protocol-level uniqueness is still open. Type ID makes each badge cell a singleton but two separate transactions can mint two distinct type_id cells for the same `(event_id, recipient)` pair and the chain accepts both. I need a mechanism that enforces one badge per attendee per event at the protocol level, not just at the cell level. I do not have a clear answer yet and would like guidance on whether this is the right layer to solve it or whether it belongs off-chain.
- `badgeExistsHint` being disabled is a real regression. Before, the indexer could do a prefix search to check whether a badge already existed. That path is now broken and I have not built the backend replacement. Until it is built, duplicate mint prevention falls entirely to the attendance window check in the event-anchor contract. I will explore better ways to do this.
