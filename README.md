# ⚡ VoltChain

**Empowering community microgrids through blockchain-based energy trading on Stellar**

![License](https://img.shields.io/badge/license-MIT-blue)
![Stellar](https://img.shields.io/badge/Stellar-Ready-blue)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)

---

## 🎯 The Vision

VoltChain is a decentralized platform designed to bridge the **$850 billion energy access gap** for underserved communities. By leveraging the **Stellar** blockchain and **Soroban** smart contracts, we enable a transparent, efficient, and low-cost marketplace for peer-to-peer (P2P) energy trading.

### Key Pillars:

- **Democratized Energy**: Direct sales between prosumers (households with solar/wind) and consumers.

* **Trustless Verification**: Energy production is verified via on-chain oracles linked to IoT smart meters.
* **Extreme Efficiency**: Transaction costs under **$0.0001** with **~5s finality**.
* **Sustainability**: Automated minting of Renewable Energy Certificates (RECs).

---

## 🖼️ Dashboard Preview

![VoltChain Dashboard](file:///home/escelit/.gemini/antigravity/brain/e41eed47-23cd-44c4-939a-bc83f67cc1ee/updated_voltchain_dashboard_1777908491240.png)

---

## 🏗️ Architecture Stack

### Why this stack?

- **Soroban (Stellar)**: Chosen for its WASM-based execution environment which offers predictable fees and high performance compared to EVM.
- **Rust (Actix-Web)**: Provides memory safety and high concurrency for handling thousands of IoT meter data points.
- **Next.js 14**: Enables server-side rendering for SEO and a premium, responsive UI experience.

### System Topology

```mermaid
graph TD
    subgraph "IoT Layer"
        A[Smart Meter] -->|Encrypted kWh Data| B(IoT Gateway)
    end

    subgraph "Backend Layer (Rust)"
        B -->|REST/Websocket| C{Actix-Web API}
        C -->|CRUD| D[(PostgreSQL)]
        C -->|Cache| E[(Redis)]
    end

    subgraph "Blockchain Layer (Stellar)"
        C -->|Submit Transaction| F[Soroban Smart Contract]
        F -->|Emit Events| G[Event Indexer]
        G -->|Update Feed| C
        F -->|Settle Asset| H{Stellar Ledger}
    end

    subgraph "Frontend Layer"
        I[User Dashboard] -->|Interact| C
        I -->|Sign Transaction| H
    end
```

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: `rustup default stable`
- **Node.js**: v18 or v20
- **Stellar CLI**: `cargo install --locked stellar-cli`
- **Docker**: (Optional, for PostgreSQL/Redis)

### 🛠️ Detailed Setup

#### 1. Smart Contracts

```bash
cd contracts
stellar contract build
# Run unit tests
stellar contract test
```

#### 2. Backend API

1. Create a `.env` file in `/backend`:
   ```env
   DATABASE_URL=postgres://user:pass@localhost/voltchain
   REDIS_URL=redis://127.0.0.1/
   RUST_LOG=info
   ```
2. Run migrations and start:
   ```bash
   cd backend
   cargo run
   ```

#### 3. Frontend Dashboard

```bash
cd frontend
npm install
npm run dev
```

---

## 🔌 API Reference

### Trades

- `GET /trades`: List all historical trades.
- `POST /trades`: Record a new energy transaction.
- `GET /trades/{id}`: Fetch details of a specific trade.

### Health

- `GET /health`: Check API and Database connectivity.

---

## 📜 Smart Contract Interface

### `EnergyTradeContract`

- `trade(prosumer, consumer, amount_kwh, price_per_kwh)`: Executes and records a trade.
- `get_trades()`: Returns a vector of all trade records on-chain.

---

## 🚀 Deployment

### Testnet Deployment

The energy-trade contract is deployed on Stellar testnet at:

- **Contract ID**: `CBQMTUYO22PJOU5PTJLUCAAYUU22FHHMIJ3BW6FQO2FJMI4NTAUVOEOV`
- **Stellar Lab**: [View Contract](https://lab.stellar.org/r/testnet/contract/CBQMTUYO22PJOU5PTJLUCAAYUU22FHHMIJ3BW6FQO2FJMI4NTAUVOEOV)
- **Explorer**: [Transaction History](https://stellar.expert/explorer/testnet/contract/CBQMTUYO22PJOU5PTJLUCAAYUU22FHHMIJ3BW6FQO2FJMI4NTAUVOEOV)

### Environment Configuration

To interact with the deployed contract, ensure your backend `.env` file contains:

```env
CONTRACT_ID=CBQMTUYO22PJOU5PTJLUCAAYUU22FHHMIJ3BW6FQO2FJMI4NTAUVOEOV
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK=testnet
```

---

## 🤝 Contributing

We welcome contributions from the community! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, branching strategy, and the process for submitting pull requests.

---

---

## 🛠️ Project Roadmap

- [x] **Milestone 1**: Initial Scaffolding & Monorepo Setup.
- [x] **Milestone 2**: Stateful Smart Contracts & Event Emission.
- [x] **Milestone 3**: Backend Persistence Layer (Diesel/Postgres).
- [ ] **Milestone 4**: Stellar Wallet Integration (Freighter).
- [ ] **Milestone 5**: Real Asset Settlement (XLM/Tokens).
- [ ] **Milestone 6**: IoT Oracle & Real-time Meter Data.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
