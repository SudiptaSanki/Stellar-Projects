# Stellar Journey to Mastery - Project Ideas by Level

Welcome, students! To help you achieve your belts and master Stellar & Soroban, we have categorized 27 unique project ideas based on the technical skills required for each level. Choose one that resonates with you or pitch your own idea following these guidelines.

---

## ⚪️ Level 1: White Belt
**Focus**: Wallets, basic transactions, network switching, and multi-wallet interactions. *No custom smart contracts required.*

### Project Ideas
1. **Crypto Tip Jar Wallet** (Project #4)
   - *Goal*: Build a simple web widget where users can connect their Freighter wallet and send tips (XLM or custom assets) to a predefined creator address.
   - *Key Learnings*: Freighter installation, connection state, sending transactions.
2. **Split Bill dApp (Basic)** (Project #6)
   - *Goal*: A basic frontend calculator that splits bills between friends and prepares direct peer-to-peer payment transactions via connected Freighter wallets.
   - *Key Learnings*: Transaction building, multi-wallet addresses.

---

## 🟡 Level 2: Yellow Belt
**Focus**: Basic smart contracts, events, on-chain state, and simple contract deployments.

### Project Ideas
1. **Proof of Completion / Certificates** (Project #7)
   - *Goal*: A simple contract that registers completed course/module IDs and maps them to student public addresses on-chain.
   - *Key Learnings*: Mapping state storage, authorization checks.
2. **Skill Badge / Micro-Certificates** (Project #10)
   - *Goal*: Create a simple contract that mints/records non-transferable digital badges to highlight student achievements.
   - *Key Learnings*: Non-fungible/non-transferable token mappings on-chain.
3. **On-Chain Invoice Generator** (Project #13)
   - *Goal*: Write a contract that stores invoice structures (amount, payer, status) and emits events when the status updates to `paid`.
   - *Key Learnings*: Structures in Soroban, emitting contract events.
4. **On-Chain Quiz & Reward App (Basic)** (Project #24)
   - *Goal*: Store a hashed quiz answer on-chain. A user submitting the correct answer receives a small testnet token payout directly from the contract.
   - *Key Learnings*: Hashing, token transfers from contract balance.

---

## 🟠 Level 3: Orange Belt
**Focus**: Complete mini dApp, robust unit tests, and seamless frontend integration.

### Project Ideas
1. **On-Chain Voting System** (Project #9)
   - *Goal*: Build a secure voting contract where proposals are registered and users can cast weighted votes. Includes a React UI to display options and vote.
   - *Key Learnings*: Contract unit testing with mocked auth, complex mappings.
2. **Ticketing NFT (Basic)** (Project #3)
   - *Goal*: Mint NFTs representing event tickets, verify ownership on the frontend, and allow basic transfer of tickets between users.
   - *Key Learnings*: Minting logic, ownership verification.
3. **Token-Based Marketplace (Simple)** (Project #15)
   - *Goal*: List items for sale with prices. Allow buyers to purchase items, swapping custom tokens for registered ownership in contract state.
   - *Key Learnings*: Token interface integration, cross-contract calls.
4. **Token Vesting Platform (Simple)** (Project #26)
   - *Goal*: Release locked project tokens linearly over time to team members or investors.
   - *Key Learnings*: Time-locked transactions, linear release math.

---

## 🟢 Level 4: Green Belt
**Focus**: Production-ready MVP on Testnet, backend integrations, secure authentication, and testing with at least 10 real users.

### Project Ideas
1. **Token-Gated Content Platform** (Project #1)
   - *Goal*: Create a blogging or video platform where content is encrypted/hidden and only unlocked if the user signs in with Freighter and holds a specific token or NFT.
   - *Key Learnings*: Web3 Auth, cryptographic signatures, backend gating.
2. **Reward-Based Task App (Gamified)** (Project #5)
   - *Goal*: A decentralized dashboard for coordinating community tasks. Users claim tasks, submit proof of work, and administrators release rewards.
   - *Key Learnings*: Multi-role access control, escrowed payouts.
3. **Mini DAO for Communities** (Project #14)
   - *Goal*: Allow members to submit proposals, vote using governance tokens, and automatically execute state changes if the vote passes.
   - *Key Learnings*: Governance protocols, automated script execution.
4. **NFT Ticketing Platform (Advanced)** (Project #20)
   - *Goal*: A comprehensive event ticketing platform with secure dynamic QR codes linked to NFTs, resale price ceilings, and ticket check-in verification portals.
   - *Key Learnings*: Dynamic NFTs, marketplace rule enforcement.

---

## 🔵 Level 5: Blue Belt
**Focus**: Scaled MVP on Testnet (50+ users), advanced state workflows, user feedback loops, pitching, and demo readiness.

### Project Ideas
1. **Decentralized Crowdfunding Platform** (Project #16)
   - *Goal*: A Kickstarter-style platform on Stellar. Campaigns set funding goals and deadlines. If goals are met, funds release; if not, contributors can claim refunds.
   - *Key Learnings*: State recovery, time-locked deposits, secure refunds.
2. **On-Chain Escrow Service** (Project #17)
   - *Goal*: Secure payments between buyers and sellers with an arbiter signature required in case of dispute.
   - *Key Learnings*: Multi-signature contracts, dispute resolution flows.
3. **Freelancer Payment System (Milestone-Based)** (Project #18)
   - *Goal*: Hire freelancers with milestone-based payment release. Funds are locked at project start and released incrementally as milestones are verified.
   - *Key Learnings*: Escrow, progress-based state transitions.
4. **Decentralized Subscription Manager** (Project #25)
   - *Goal*: Enable users to manage all their recurring payments in one dashboard, setting monthly limits and cancelling authorizations directly.
   - *Key Learnings*: Allowance management, recurring transaction triggers.

---

## ⚫️ Level 6: Black Belt
**Focus**: Mainnet deployment preparation, security audits, advanced cryptography, and real adoption strategies.

### Project Ideas
1. **On-Chain Reputation System** (Project #2)
   - *Goal*: Calculate user credit/reputation scores on-chain by aggregating their transaction history, voting consistency, and collateral health.
   - *Key Learnings*: Complex mathematical calculations in Soroban, performance optimization.
2. **Decentralized Identity (DID) System** (Project #19)
   - *Goal*: Build a W3C-compliant decentralized identifier registry on Stellar, enabling users to verify credentials without central authorities.
   - *Key Learnings*: Cryptographic verification, metadata storage standards.
3. **Supply Chain Tracker** (Project #21)
   - *Goal*: Trace the journey of physical goods. Each step (manufacturing, shipping, customs, delivery) updates a tamper-proof product history.
   - *Key Learnings*: Multi-party transaction workflows, location/IoT data mapping.
4. **Decentralized Savings Pool (ROSCA Model)** (Project #22)
   - *Goal*: A rotating savings association where group members contribute periodically, and a different member receives the pool payout each round.
   - *Key Learnings*: Cycle math, member reliability tracking, group collateral.
5. **Gasless Transaction Relayer (Meta Transactions)** (Project #23)
   - *Goal*: Allow users to send Stellar transactions without owning XLM. Relayers pay the fee, and users reimburse them in stablecoins or other tokens.
   - *Key Learnings*: Fee-bump transactions, transaction envelopes, cryptographic signature relay.

---

## 🏆 Level 7: Master Belt
**Focus**: Startup track, venture pitch preparation, Stellar Community Fund (SCF) grant readiness, and highly advanced financial architectures.

### Project Ideas
1. **On-Chain Leaderboard (Advanced)** (Project #8)
   - *Goal*: A global, highly scalable leaderboard for Web3 games featuring secure anti-cheat verification, optimized on-chain sorting, and real-time SDK integration.
   - *Key Learnings*: Sorting algorithms in Rust/Soroban, high-throughput optimization.
2. **Subscription dApp (Web3 SaaS)** (Project #11)
   - *Goal*: Build an automated recurring billing protocol for software access, utilizing advanced Stellar allowances and pre-authorized transactions.
   - *Key Learnings*: Automated pull-payments, licensing models.
3. **Gambling Website Basic (Provably Fair)** (Project #12)
   - *Goal*: Implement game rooms (dice, coin flip) using verified decentralized oracles (e.g., Band Protocol) or commit-reveal schemes for provably fair randomness.
   - *Key Learnings*: Oracle integration, cryptographically secure randomness on-chain.
4. **Peer-to-Peer Lending Protocol (Simple)** (Project #27)
   - *Goal*: Create a lending pool where depositors earn interest and borrowers can take out loans by depositing collateral with automated liquidation checks.
   - *Key Learnings*: Collateralized Debt Positions (CDPs), oracle price feeds, interest math, liquidator triggers.
