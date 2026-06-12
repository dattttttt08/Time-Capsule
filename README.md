# TimeCapsule Contract

## Project Title
TimeCapsule Contract

## Project Description
TimeCapsule Contract is a decentralized smart contract platform designed to package, store, and lock digital assets or proofs of Real-World Assets (RWA) into time-bound "capsules" on-chain. Built using Soroban on the Stellar blockchain, it provides a tamper-proof mechanism to enforce time constraints on digital ownership, ensuring that assets can only be unlocked, managed, or transferred after a specific ledger timestamp countdown has officially elapsed.

## Project Vision
The vision of TimeCapsule Contract is to provide a completely trustless, transparent, and automated solution for time-locked asset management and decentralized custody. By removing centralized intermediaries, it empowers users to execute digital asset inheritance, scheduled distributions, and automated escrow-like time locks with absolute cryptographic security, ensuring no third party can alter or prematurely access the asset before its maturity date.

## Key Features
- **Capsule Management:** Users can initialize digital capsules tied to unique asset tokens (`item_id`).
- **Immutable Time-locking:** Lock parameters are anchored directly to the immutable real-time `timestamp` of the Stellar ledger.
- **Ownership Transfer:** Features a built-in function to safely hand over ownership of active or unlocked capsules to new recipient addresses.
- **Decentralized Security:** Built using Soroban's persistent storage state architecture to defend records against data manipulation or retroactive changes.
- **Transparent Querying:** Publicly accessible methods allow gas-free state verification to see if a time capsule's unlock schedule has been cleared.

## Usage Instructions
1. **Deploy Contract:** Deploy the compiled WASM code onto the Stellar Testnet network.
2. **Create Capsule:** Call the `create_capsule` function, supplying the target asset `item_id`, the initial `owner` address, and the countdown duration in seconds (`duration_seconds`).
3. **Query Lock Status:** Invoke `is_accessible` with your asset ID. It evaluates the current ledger time against the unlock deadline to return `false` (locked) or `true` (unlocked).
4. **Transfer Ownership:** Once parameters align, current owners can seamlessly assign full asset claim rights over to a `new_owner` profile.

## Future Scope
- **Asset Token Escrow:** Integrate native Soroban token standard contracts (SAC) to physically hold multi-asset tokens inside the time-capsule escrow pool.
- **Multi-Signature Triggers:** Introduce multi-sig approvals allowing specialized emergency legal handlers or oracles to safely trigger a backup release.
- **Advanced UI/UX Dashboards:** Implement visual frontend applications for tracking active countdown progress indicators alongside decentralized wallet connectors.
- **Dynamic Lock Extenders:** Allow designated owners to dynamically top-up or push forward lock durations if asset protection terms change.

## Technology Stack
- Rust and Soroban SDK for secure, memory-safe smart contract architecture.
- Stellar blockchain for decentralized, fast, and immutable global ledger state tracking.
- Cryptographic keys and integrated ledger clock mechanisms to enforce un-biasable lock expiration.

## Contribution
Community contributions are welcomed from blockchain core developers and RWA asset tokenization specialists. Fork the project repository and submit pull requests to assist in expanding features.

## License
This project is licensed under the MIT License.

### Contract Detail
ID: CDEHXC2JFHCHIGNEQ6LLS7LLUHQS5L5AAKURZFHFOV76FSPNRDLD5YGQ
![alt text](image.png)