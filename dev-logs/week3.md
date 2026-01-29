# CKBuilder Track Weekly Report — Week 
-  Name: **Mayowa Temitope AKINYELE**
-  Week Ending: **Jan 28, 2026**

## Courses Completed
### [Less is more](https://xuejie.space/)
- Introduction to CKB Script Programming 3: [UDT
](https://xuejie.space/2019_09_06_introduction_to_ckb_script_programming_udt/)
- Introduction to CKB Script Programming 4: [WebAssembly on CKB](https://xuejie.space/2019_10_09_introduction_to_ckb_script_programming_wasm_on_ckb/)
- Introduction to CKB Script Programming 5: Debugging: [CKB-SDK-Rust](https://xuejie.space/2019_10_18_introduction_to_ckb_script_programming_debugging/)
- Let's Build a Minimal Blockchain 1: [Dawn](https://xuejie.space/2019_10_21_lets_build_a_minimal_blockchain_dawn/)

### [L1 Developer Training Course](https://nervos.gitbook.io/developer-training-course/)
- Transactions and Script Basics
- Managing the Script State

## Key Learnings
- How to send a basic transaction, and examining the components of a valid transaction.
- Learning the cell and how to handle the cell states and transitions.
- The CKB VM architecture.
- The uniqueness of the UDTs

## Brief
- Learned to implement custom tokens on CKB using Type Scripts and to enforce conservation rules to prevent unauthorized minting while utilizing CKB's unique storage model.
- Explored executing WebAssembly binaries on the CKB VM understanding how the architecture works and how best to work with it.
- Mastered the workflow for troubleshooting CKB scripts using tools like ckb-cli for transaction mocking and ckb-debugger for step-by-step inspection. Though I have trouble running the ckb-cli.
- Gained a deep understanding of the cell model, lock scripts (access) and type scripts (logic).

## Practical Progress
- Wrote a minimal CKB script and ran the ckb devnet; The scrit can be found in ```../ckb-scripts/```
- Ran the CKB node and followed the lab exercises in the L1 Dev Training
- Gained practical knowledge of a valid transaction and what goes behind the scenes.

## Proof of Participation
You can find the proof of participation [here](https://drive.google.com/drive/folders/1m2BOS6smeTUS68VPfZBjZ1nu-525JGdz?usp=drive_link)

## Challenges
I faced a couple challenges concerning runniing the script. It has to do with running the ckb-vm via the ckb-cli. I have reported this on the CKBuilders group. I will update this when help comes. This problem isn't crossing into next week. :smirk
