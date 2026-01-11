# CKBuilder Track Weekly Report — Week 1
-  Name: **Mayowa Temitope AKINYELE**
-  Week Ending: **Jan 14, 2026**

## Courses Completed
-  The Learn module of the [Nervos Docs](https://docs.nervos.org/)
-  CKB basic theoretical knowledge
-  CKB basic practical operation

## Key Learnings
By going through the Nervos docs I have learned the following:
- How the Nervos blockchain works and compare to other blockchains.
- How to build a CKB transaction.
- The Quantum Resistant Nature of the CKB blockchain. 
- The CKB Cell Model and other core structures of the CKB blockchain.

## Brief
- The Nervos Network (CKB) is the Layer 1 blockchain of the Nervos ecosystem. It helps keep the blockchain secure and provides trust for the Nervos ecosystem. The blockchain is by design Multi-Layer. But, the CKB is there only to provide the base layer.
- The CKB makes storage a precious digital asset and hence, eliminating the problem of the commons. 
- 1 CKB = 10^8 shannons = 1 Byte of the storage on the blockchain.
- If a user have 1000 bytes of storage on the CKB, then they would need at least 1000 CKBS to maintain that storage. And, once it's deleted, the CKBytes are made available for other purposes.
- The cell model of the Nervos blockchain makes transaction the consumption and creation of new cells. The difference between the input cells and output cells is the transaction fee for miners.
- The cells are protected by two types of scripts. The lock script which determines access to the cell and the optional type script which determines how the cell can be modified.

```bash
Script: {
Code hash: hexString
Hash type: Uint8
Args: hexString}
```

## Practical Progress
### **First Transaction**
I sent my first transaction using the one-button click setup provided by the CKB Academy practical operation. This gave me a good understanding of how the transactions happen on the blockhchain, from creation to confirmation.

> Create > Sign > Broadcast > Validate > Propagate > Confirm

You can view the transaction on the [CKB Explorer](https://testnet.explorer.nervos.org/transaction/0x19b3b93e0e2b02ed94f050d988a5186aec3f224f41b83752a6c754cc58cb3e26)

## Proof of Participation
You can find the proof of participation [here](https://drive.google.com/drive/folders/1-A_dqp2pOULM0iGPVfbGE-vmvifc7hmT)

## Challenges
- I find it difficult to install the offckb cli on my PC even though I have installed the VS Build tools ( 2026 & 2019(assuming no support for latest versions)), downgraded my node to v22.21.1. I still cotinue to get errors with the node-gyp dep.