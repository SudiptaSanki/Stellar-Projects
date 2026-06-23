# Stellar Journey to Mastery - Project Ideas by Level

Welcome, students! To help you achieve your belts and master Stellar & Soroban, we have categorized a wide range of project ideas based on the technical skills required for each level. Choose one that resonates with you or pitch your own idea following these guidelines.

---

## ⚪️ Level 1: White Belt
**Focus**: Wallets, basic transactions, network switching, and multi-wallet interactions. *No custom smart contracts required.*

### Project Ideas
1. **Crypto Tip Jar / Creator Tip Jar Wallet**
   - *Goal*: Build a simple web widget where users connect their Freighter wallet and send tips (XLM or custom assets) to a predefined creator address.
2. **Split Bill dApp / Expense Splitting App**
   - *Goal*: A basic frontend calculator that splits bills between friends and prepares direct peer-to-peer payment transactions via connected wallets.
3. **Savings Goal Tracker**
   - *Goal*: A simple frontend tracking a user's wallet balance against a self-defined savings goal without needing a smart contract.

---

## 🟡 Level 2: Yellow Belt
**Focus**: Basic smart contracts, events, on-chain state, and simple contract deployments.

### Project Ideas
1. **Proof of Completion / Digital Certificate Issuer**
   - *Goal*: A simple contract that registers completed course/module IDs and maps them to student public addresses on-chain.
2. **Skill Badge / Builder Achievement Tracker**
   - *Goal*: Create a simple contract that mints/records non-transferable digital badges to highlight student achievements.
3. **On-Chain Invoice Generator / Invoice Payment Tracker**
   - *Goal*: Write a contract that stores invoice structures (amount, payer, status) and emits events when the status updates to `paid`.
4. **On-Chain Quiz & Reward App (Basic)**
   - *Goal*: Store a hashed quiz answer on-chain. A user submitting the correct answer receives a small payout directly from the contract.
5. **Membership Pass Platform**
   - *Goal*: Mint a basic NFT or token representing community membership and map it to user addresses.
6. **Hackathon Participation Registry**
   - *Goal*: A simple registry contract logging wallet addresses of participants for specific hackathon events.

---

## 🟠 Level 3: Orange Belt
**Focus**: Complete mini dApp, robust unit tests, and seamless frontend integration.

### Project Ideas
1. **On-Chain Voting System / Community Poll Platform**
   - *Goal*: Build a secure voting contract where proposals are registered and users can cast weighted votes. Includes a React UI to display options and vote.
2. **Ticketing NFT (Basic)**
   - *Goal*: Mint NFTs representing event tickets, verify ownership on the frontend, and allow basic transfer of tickets between users.
3. **Token-Based Marketplace (Simple)**
   - *Goal*: List items for sale with prices. Allow buyers to purchase items, swapping custom tokens for registered ownership.
4. **Token Vesting Platform (Simple)**
   - *Goal*: Release locked project tokens linearly over time to team members or investors.
5. **Feature Request Board**
   - *Goal*: A dApp where users can submit feature requests and upvote them using tokens.
6. **Referral Reward Platform**
   - *Goal*: A smart contract that tracks user referrals and issues reward tokens when a new user completes a specific action.
7. **Project Funding Tracker**
   - *Goal*: A basic dApp tracking incoming funds to a specific project address and displaying progress towards a goal.
8. **Community Donation Pool**
   - *Goal*: A smart contract where users pool funds together for a cause, tracking total donations and individual contributions.

---

## 🟢 Level 4: Green Belt
**Focus**: Production-ready MVP on Testnet, backend integrations, secure authentication, and testing with at least 10 real users.

### Project Ideas
1. **Token-Gated Content / Content Access Gate**
   - *Goal*: Create a platform where content is encrypted/hidden and only unlocked if the user signs in with Freighter and holds a specific token.
2. **Task Bounty Board / Contribution Reward Platform**
   - *Goal*: A decentralized dashboard for coordinating community tasks. Users claim tasks, submit proof of work, and administrators release rewards.
3. **Mini DAO / DAO Voting Platform / Treasury Voting System**
   - *Goal*: Allow members to submit proposals, vote using governance tokens, and automatically execute state changes if the vote passes.
4. **Event Ticketing / NFT Ticketing Platform (Advanced)**
   - *Goal*: A comprehensive event ticketing platform with secure dynamic QR codes, resale price ceilings, and ticket check-in portals.
5. **Community Treasury**
   - *Goal*: A multi-user dApp to manage community funds, requiring consensus or admin roles to initiate payouts.
6. **Payroll Distribution Dashboard**
   - *Goal*: A dashboard allowing companies to batch pay employees in stablecoins or XLM with one signed transaction.
7. **Hackathon Prize Distribution Tool**
   - *Goal*: Manage complex prize pools and distribute payouts to multiple winning teams based on judge approvals.

---

## 🔵 Level 5: Blue Belt
**Focus**: Scaled MVP on Testnet (50+ users), advanced state workflows, user feedback loops, pitching, and demo readiness.

### Project Ideas
1. **Crowdfunding Platform**
   - *Goal*: A Kickstarter-style platform on Stellar. Campaigns set funding goals and deadlines. Manage state recovery and secure refunds.
2. **On-Chain Escrow Service / Escrow Payment Platform**
   - *Goal*: Secure payments between buyers and sellers with an arbiter signature required in case of dispute.
3. **Freelancer Milestone Escrow**
   - *Goal*: Hire freelancers with milestone-based payment release. Funds are locked at project start and released incrementally as milestones are verified.
4. **Decentralized Subscription Manager**
   - *Goal*: Enable users to manage all their recurring payments in one dashboard, setting limits and cancelling authorizations directly.
5. **Micro-Grant Distribution Platform**
   - *Goal*: Distribute grants based on milestones, allowing donors to track impact and releasing funds iteratively.
6. **Scholarship Distribution System**
   - *Goal*: Secure scholarship funds in escrow and distribute them to students conditionally over time based on academic performance verifications.

---

## ⚫️ Level 6: Black Belt
**Focus**: Mainnet deployment preparation, security audits, advanced cryptography, and real adoption strategies.

### Project Ideas
1. **On-Chain Reputation System**
   - *Goal*: Calculate user credit/reputation scores on-chain by aggregating their transaction history, voting consistency, and collateral health.
2. **Decentralized Identity (DID) System**
   - *Goal*: Build a W3C-compliant decentralized identifier registry on Stellar, enabling users to verify credentials without central authorities.
3. **Supply Chain Tracker**
   - *Goal*: Trace the journey of physical goods. Each step updates a tamper-proof product history.
4. **Decentralized Savings Pool (ROSCA Model)**
   - *Goal*: A rotating savings association where group members contribute periodically, and a different member receives the pool payout each round.
5. **Gasless Transaction Relayer (Meta Transactions)**
   - *Goal*: Allow users to send Stellar transactions without owning XLM via fee-bump transactions and relayers.
6. **Builder Incubator Dashboard**
   - *Goal*: A comprehensive platform tracking startup milestones, unlocking funding tranches, handling DAO voting, and emitting progress reports.

---

## 🏆 Level 7: Master Belt
**Focus**: Startup track, venture pitch preparation, Stellar Community Fund (SCF) grant readiness, and highly advanced financial architectures.

### Project Ideas
1. **On-Chain Leaderboard (Advanced)**
   - *Goal*: A global, highly scalable leaderboard for Web3 games featuring secure anti-cheat verification, optimized on-chain sorting, and SDK integration.
2. **Subscription Payment Manager / Web3 SaaS**
   - *Goal*: Build an automated recurring billing protocol for software access, utilizing advanced Stellar allowances and pre-authorized transactions.
3. **Gambling Website Basic (Provably Fair)**
   - *Goal*: Implement game rooms (dice, coin flip) using verified decentralized oracles or commit-reveal schemes for provably fair randomness.
4. **Peer-to-Peer Lending Protocol (Simple)**
   - *Goal*: Create a lending pool where depositors earn interest and borrowers can take out loans by depositing collateral with automated liquidation checks.
