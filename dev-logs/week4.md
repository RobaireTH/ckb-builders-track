# CKBuilder Track Weekly Report — Week 
-  Name: **Mayowa Temitope AKINYELE**
-  Week Ending: **Feb 04, 2026**

## Courses Completed
### DApps tutorial
- [Transfer](https://docs.nervos.org/docs/dapp/transfer-ckb) CKB
- [Store data](https://docs.nervos.org/docs/dapp/store-data-on-cell) on cell
- Create a [Fungible Token](https://docs.nervos.org/docs/dapp/create-token)
- Create a [DOB](https://docs.nervos.org/docs/dapp/create-dob)
- Build a [simple lock](https://docs.nervos.org/docs/dapp/simple-lock)
- Into to wallets and [the CCC connector](https://docs.nervos.org/docs/integrate-wallets/ccc-wallet)
- The [How Tos](https://docs.nervos.org/docs/how-tos/how-to-sign-a-tx)

## Key Learnings

- State lives in immutable cells; updates happen by consuming old cells and creating new ones.
- Inputs, outputs, and dependencies are declared upfront—no hidden state or implicit execution.
- CKB locked in a cell pays for its data size, making storage a first-class economic concern.
- Lock scripts control spending authority; type scripts enforce how cells can change.
- Every valid update must destroy the previous state cell and replace it with a compliant new one.
- Fungible tokens (sUDT/xUDT) and DOBs are implemented via scripts and data structures.
- Wallets discover live cells, build transactions, and sign according to script requirements.
- The CCC connector provides an all-in-one required wallets integration.

## Brief
- Application logic is enforced through scripts, which validate whether a transaction is allowed rather than executing arbitrary logic or mutating global state.
- Wallets play a critical role as transaction constructors and signers, discovering live cells and authorizing state transitions without owning application state themselves.
- All state changes are expressed explicitly through transactions that consume existing cells and create new ones, making every transition transparent and deterministic.

## Practical Progress
![PoP Network UI](../public/PoPNet.PNG)
- I started building a Proof-of-Presence protocol (```ckb-pop```) and its reference dApp(```PoP Network```). The idea is that presence should be provable, unique, and permanent without requiring anyone’s permission or trust. Servers shouldn't hand me the ```You were there!```. Click to check the [preview](https://ckb-pop.vercel.app/#/) build.
- I designed the ```event-anchor``` such that an immutable cell proves the existence of an event and the event is tied to the creator address and hence making sure it is not modified or duplicated. No admin registry, just the uniqueness of the type script.
```rust
args = hash(event_id || creator_address)
```
- I built the event UX and presence UX for the reference dApp. 
- I hooked up the CCC connector for client signing, intergrating UTXO global wallet, JoyID, WalletConnect and UniPass. The experience with ```testnet.joyid.dev``` was nice considering I get to use the dev tools on there.

## Proof of Participation
You can find
- the reference dApp [here](https://ckb-pop.vercel.app/#/)
- the source code [here](https://github.com/RobaireTH/ckb-PoP)
- other proof of participation [here](https://drive.google.com/drive/folders/1G3dDoAgRpyLovuElT43hoFySoBCe6i82?usp=drive_link)

## Challenges
Considering I have been going from installations to installations for the past few weeks, my PC ran out of disk space and I couldn't really test the helpers via ```Axum``` . I only have the ```event-anchor``` and ```dob-badge``` scripts wired to the frontend yet. Note: Scripts are the UX helpers and the contracts are in ```/contracts```. I really can't find a better way to name the folders. So I guess that is one of my challenges. Using ```/backend``` makes it sound authoritative to me.
