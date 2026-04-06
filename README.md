# 🎨 StellarNFT — NFT Minting Platform on Stellar

<p align="center">
  <img src="https://img.shields.io/badge/Stellar-Soroban-blueviolet?style=for-the-badge&logo=stellar" />
  <img src="https://img.shields.io/badge/Rust-1.78+-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Network-Testnet-yellow?style=for-the-badge" />
</p>

---

## 📖 Project Description

**StellarNFT** is a fully on-chain NFT Minting Platform built with **Soroban** — Stellar's native smart contract platform. It lets creators launch their own NFT collection, mint digital assets with rich metadata (stored on IPFS), and trade them on the Stellar network at near-zero fees (~0.00001 XLM per operation).

The contract is written in **Rust** and compiled to **WebAssembly (WASM)** for deployment on the Stellar blockchain. It follows best practices for access control, supply caps, and event emission — making it suitable for production collections.

---

## 🚀 What It Does

StellarNFT enables end-to-end NFT lifecycle management directly on Stellar:

| Action | Description |
|---|---|
| **Initialize** | Deploy a named collection with a symbol, mint price, and max supply cap |
| **Mint** | Any user can mint an NFT by providing name, description, IPFS URI, custom attributes, and royalty basis points |
| **Admin Mint** | Contract admin can airdrop NFTs without paying the mint price |
| **Transfer** | Move an NFT from one wallet to another, with full authorization checks |
| **Burn** | Permanently destroy an NFT (only the owner can do this) |
| **Approve** | Delegate transfer rights for a single token to another address |
| **Operator Approval** | Grant blanket transfer rights over all your tokens to an operator |
| **Query** | Read token metadata, owner, collection stats, balance, and approvals |
| **Pause / Unpause** | Admin can halt minting and transfers in an emergency |

---

## ✨ Features

### 🏗️ Core NFT Functionality
- **On-chain metadata** — Every NFT stores `name`, `description`, `URI`, `attributes` map, `minted_at` timestamp and `royalty_bps` directly in contract storage
- **Custom attributes** — Key/value `Map<String, String>` for traits, rarity scores, etc.
- **IPFS/HTTPS URI** — Link any off-chain asset (image, video, 3-D model) to the token

### 🔒 Access Control
- **Admin role** with `transfer_admin` for multisig handoff
- `require_auth()` on every state-changing call — no spoofing
- Single-token **Approve** + blanket **OperatorApproval** pattern (analogous to ERC-721)

### 🛡️ Safety Mechanisms
- **Max supply cap** — hard ceiling enforced at the contract level
- **Pause switch** — admin can freeze the contract instantly
- **Royalty basis points** — per-token royalty setting (0–10 000 bps) ready for marketplace integration

### 📡 Events
Three Soroban events are emitted for indexers and frontends:

```
MINT     (token_id, to)
TRANSFER (from, to, token_id)
BURN     (owner, token_id)
APPROVE  (owner, spender, token_id)
```

### 🧪 Test Suite
Full unit-test coverage inside `lib.rs`:
- Mint & query round-trip
- Transfer ownership
- Burn token
- Pause / unpause flow
- Max-supply enforcement (panic case)

---

## 📂 Project Structure

```
nft-minting-platform/
├── Cargo.toml                          # Workspace manifest
├── README.md
└── contracts/
    └── nft_platform/
        ├── Cargo.toml                  # Contract crate manifest
        └── src/
            └── lib.rs                  # ← All contract code + tests
```

---

## 🛠️ Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | ≥ 1.78 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| `wasm32` target | — | `rustup target add wasm32-unknown-unknown` |
| Stellar CLI | ≥ 22.x | `cargo install --locked stellar-cli --features opt` |

---

## ⚙️ Build & Test

```bash
# 1. Clone
git clone https://github.com/your-username/nft-minting-platform.git
cd nft-mintin


1. Contract address:CCQFN65UCJLKPWN4QRINL7SU4LCOV6ZAH4HB32E6SFTDP3YBUZ2NSA5E
	2. Wallet address:GD5BR5WNAOVWNY76GOYRQ24EOTCFJE33UBKYHFZ5D7BPUZ4ZYFPH4HFM
	3. Image:<img width="1917" height="1199" alt="image" src="https://github.com/user-attachments/assets/b07e009a-724c-4ab6-8f4f-9b5c454d0f5a" />

	4. Link:https://stellar.expert/explorer/testnet/contract/CCQFN65UCJLKPWN4QRINL7SU4LCOV6ZAH4HB32E6SFTDP3YBUZ2NSA5E
