# 📄 On-Chain Resume — Soroban Smart Contract on Stellar

> **Decentralized, tamper-proof, peer-endorsed career credentials — permanently stored on the Stellar blockchain.**

---

## 🌐 Project Description

**On-Chain Resume** is a Soroban smart contract deployed on the Stellar network that lets anyone create, own, and share a verifiable professional profile stored entirely on-chain. No centralized server, no single point of failure, no middleman — just your credentials, cryptographically signed and permanently accessible.

In a world where credentials are easily forged and résumés live on company-controlled platforms (LinkedIn, Indeed), On-Chain Resume gives professionals full sovereignty over their career identity. Every update is authenticated by the owner's wallet, every endorsement is recorded immutably, and anyone can verify a profile without trusting a third party.

---
## 📡 Deployed Smart Contract

| Network | Contract ID |
|---|---|
| **Stellar Testnet** | `CDEPLOY_YOUR_CONTRACT_ID_HERE_AFTER_DEPLOYMENT` |
| **Stellar Mainnet** | Coming soon |

> **Note:** Deploy to testnet using the steps above and replace the contract ID in the table.  
> You can explore your deployed contract on [Stellar Expert](https://stellar.expert/explorer/testnet) or [Stellar Lab](https://lab.stellar.org/).
> 
   Contract Address- CDMTOY7IIQ64IB3CNEWWPFVCOAAE3BRB4T2YMY6FGS7IS4VUFFQ2OGWR
>
> 
> <img width="1890" height="898" alt="image" src="https://github.com/user-attachments/assets/c15e3b6d-58eb-4687-93e0-01abf6c45f2e" />

---

## ⚙️ What It Does

The contract allows any Stellar account to:

1. **Create a professional resume** on-chain with personal info, skills, and contact links.
2. **Add work experience** entries (company, role, years, description).
3. **Add education** records (institution, degree, field, graduation year).
4. **Add certifications** (name, issuer, year, credential ID).
5. **Receive peer endorsements** from other Stellar accounts — colleagues, managers, or clients who vouch for you.
6. **Read any resume publicly** — employers or collaborators can verify credentials directly from the blockchain.
7. **Delete / revoke** your resume if you choose to remove it.

All writes require the **owner's wallet signature** (`require_auth`), ensuring only you can modify your own profile.

---

## ✨ Features

| Feature | Description |
|---|---|
| 🔐 **Self-Sovereign Identity** | Only the wallet owner can create, update, or delete their resume |
| 📋 **Rich Profile Data** | Stores bio, title, skills, work history, education, certifications |
| 🤝 **Peer Endorsements** | Other Stellar accounts can endorse your profile — think on-chain LinkedIn recommendations |
| 🚫 **Anti-Gaming Rules** | Self-endorsements blocked; duplicate endorsements blocked |
| 📡 **On-Chain Events** | Contract emits events on create, endorse, and delete for easy off-chain indexing |
| 🔍 **Public Verifiability** | Anyone can read any resume — no login, no API key, no trust required |
| ♻️ **Persistent Storage** | Uses Soroban persistent storage so resumes survive ledger archiving |
| 🧪 **Fully Tested** | Unit tests for all core flows including failure paths |
| ⚡ **WASM-Optimized** | Built with `opt-level = "z"` and LTO for minimal contract footprint |

---

## 🏗️ Project Structure

```
on-chain-resume/
├── Cargo.toml                      # Workspace manifest
└── contracts/
    └── resume/
        ├── Cargo.toml              # Contract dependencies (soroban-sdk 21)
        └── src/
            └── lib.rs              # Smart contract source
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Build

```bash
# From project root
stellar contract build
```

The optimized `.wasm` will be at:
`target/wasm32-unknown-unknown/release/on_chain_resume.wasm`

### Run Tests

```bash
cargo test
```

### Deploy to Testnet

```bash
# 1. Generate a keypair and fund it
stellar keys generate --global alice --network testnet
stellar keys fund alice --network testnet

# 2. Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/on_chain_resume.wasm \
  --source alice \
  --network testnet
```

### Interact with the Contract

```bash
# Create a resume
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- create_resume \
  --owner <YOUR_ADDRESS> \
  --name "Alice Dev" \
  --title "Senior Rust Engineer" \
  --bio "Building the decentralized future." \
  --email "alice@example.com" \
  --github "github.com/alice" \
  --linkedin "linkedin.com/in/alice" \
  --skills '["Rust","Soroban","WebAssembly"]'

# Add work experience
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- add_experience \
  --owner <YOUR_ADDRESS> \
  --company "Stellar Foundation" \
  --role "Smart Contract Engineer" \
  --start_year 2022 \
  --end_year 0 \
  --description "Building production Soroban contracts."

# Endorse someone
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source bob \
  --network testnet \
  -- endorse \
  --endorser <BOB_ADDRESS> \
  --resume_owner <ALICE_ADDRESS>

# Read a resume (no auth needed)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_resume \
  --owner <ALICE_ADDRESS>
```

---


## 🗺️ Roadmap

- [ ] **NFT Badge Minting** — mint a Stellar NFT representing certifications
- [ ] **Multi-sig Endorsements** — require N-of-M endorsements to unlock a credential badge  
- [ ] **Frontend dApp** — React + Freighter wallet integration for a full resume UI
- [ ] **IPFS Integration** — store extended profile data (portfolio links, images) on IPFS, hash on-chain
- [ ] **Recruiter Allowlisting** — opt-in to let specific wallet addresses contact you

---

## 🛠️ Tech Stack

- **Blockchain:** [Stellar](https://stellar.org/)
- **Smart Contract Runtime:** [Soroban](https://soroban.stellar.org/)
- **Language:** Rust (`no_std`)
- **SDK:** `soroban-sdk v21`
- **CLI:** `stellar-cli`

---

## 📜 License

MIT License — free to fork, deploy, and build on top of.

---

## 🙌 Contributing

PRs welcome! Open an issue to discuss new features or improvements.

```bash
git clone https://github.com/YOUR_USERNAME/on-chain-resume
cd on-chain-resume
cargo test
```
