# Memechan.gg Knowledge Graph

This document provides a comprehensive knowledge graph of the Memechan.gg platform, visualizing the relationships between different components, concepts, and processes.

## Core Components Relationship

```mermaid
graph TD
    A[Memechan.gg Platform] --> B[Smart Contracts]
    A --> C[Tokens]
    A --> D[User Interfaces]
    
    B --> B1[Bound Pool]
    B --> B2[Staking Pool]
    B --> B3[Chan Swap]
    B --> B4[Fee Management]
    
    C --> C1[MEME Token]
    C --> C2[CHAN Token]
    C --> C3[SOL]
    
    D --> D1[Web Interface]
    D --> D2[SDK]
    
    B1 --> E1[Bonding Curve]
    B1 --> E2[Token Swapping]
    B2 --> E3[Staking Mechanism]
    B2 --> E4[Reward Distribution]
    B3 --> E5[Fixed Price Swapping]
    B4 --> E6[Fee Collection]
    B4 --> E7[Fee Distribution]
    
    E1 --> F1[Price Calculation]
    E2 --> F2[Liquidity Management]
    E3 --> F3[Vesting]
    E4 --> F4[Yield Generation]
    E5 --> F5[CHAN Price Management]
    E6 --> F6[Admin Fees]
    E6 --> F7[LP Fees]
    E7 --> F8[Staker Rewards]
```

## Token Flow Diagram

```mermaid
graph LR
    A[User] -->|Deposits SOL| B[Bound Pool]
    B -->|Issues| C[MEME Tokens]
    C -->|Held by| A
    
    A -->|Stakes| D[Staking Pool]
    D -->|Issues| E[Staking Ticket]
    E -->|Represents| F[Vesting MEME]
    
    D -->|Distributes| G[Rewards]
    G -->|Received by| A
    
    A -->|Swaps MEME| H[Chan Swap]
    H -->|Issues| I[CHAN Tokens]
    I -->|Held by| A
    
    B -->|Collects| J[Swap Fees]
    J -->|Distributed to| K[Fee Recipients]
    K -->|Includes| L[Stakers]
    K -->|Includes| M[Admin]
```

## Process Flows

### Swap Process

```mermaid
sequenceDiagram
    participant User
    participant UI as User Interface
    participant BP as Bound Pool
    participant Wallet
    
    User->>UI: Enters swap amount
    UI->>BP: Calculates expected output
    BP->>UI: Returns quote
    UI->>User: Displays quote
    User->>UI: Confirms swap
    UI->>Wallet: Requests approval
    Wallet->>User: Prompts for confirmation
    User->>Wallet: Approves transaction
    Wallet->>BP: Executes swap
    BP->>BP: Updates reserves
    BP->>BP: Calculates fees
    BP->>User: Sends output tokens
```

### Staking Process

```mermaid
sequenceDiagram
    participant User
    participant UI as User Interface
    participant SP as Staking Pool
    participant Wallet
    
    User->>UI: Enters staking amount
    UI->>SP: Requests staking terms
    SP->>UI: Returns vesting schedule
    UI->>User: Displays terms
    User->>UI: Confirms staking
    UI->>Wallet: Requests approval
    Wallet->>User: Prompts for confirmation
    User->>Wallet: Approves transaction
    Wallet->>SP: Executes staking
    SP->>SP: Creates staking ticket
    SP->>SP: Initializes vesting
    SP->>User: Issues staking receipt
```

## Concept Map

```mermaid
graph TD
    A[Bonding Curve] -->|Determines| B[Token Price]
    C[Vesting] -->|Controls| D[Token Release]
    E[Staking] -->|Earns| F[Rewards]
    G[Swapping] -->|Generates| H[Fees]
    H -->|Distributed to| I[Stakers]
    H -->|Distributed to| J[Admin]
    
    K[MEME Token] -->|Can be| L[Swapped]
    K -->|Can be| M[Staked]
    N[CHAN Token] -->|Provides| O[Governance]
    N -->|Provides| P[Utility]
    
    Q[Liquidity Pool] -->|Contains| R[Token Reserves]
    R -->|Affects| B
    
    S[User] -->|Performs| G
    S -->|Performs| E
    S -->|Receives| T[Staking Ticket]
    T -->|Represents| U[Vesting Position]
```

## Technical Architecture

```mermaid
graph TD
    subgraph "Solana Blockchain"
        A[Memechan Program] --> B[Bound Pool Accounts]
        A --> C[Staking Pool Accounts]
        A --> D[Chan Swap Accounts]
        A --> E[Token Accounts]
        
        B --> B1[Meme Reserve]
        B --> B2[Quote Reserve]
        B --> B3[Config]
        
        C --> C1[Staking Tickets]
        C --> C2[Vesting Data]
        
        D --> D1[Chan Price]
        D --> D2[Chan Vault]
        
        E --> E1[Meme Token Mint]
        E --> E2[Chan Token Mint]
    end
    
    subgraph "Client Side"
        F[Web Interface] --> G[Anchor Client]
        H[SDK] --> G
        I[Mobile App] --> H
        
        G --> A
    end
```

## Token Economics

```mermaid
graph TD
    A[Token Supply] --> B[MEME Token]
    A --> C[CHAN Token]
    
    B -->|Max Supply| D[1 Trillion]
    B -->|Distribution| E[Bonding Curve]
    B -->|Utility| F[Staking]
    
    C -->|Supply| G[Limited]
    C -->|Acquisition| H[Swapping]
    C -->|Utility| I[Governance]
    C -->|Utility| J[Fee Discounts]
    
    K[Fee Structure] --> L[Swap Fee]
    L -->|Percentage| M[0.3%]
    L -->|Distribution| N[Stakers]
    L -->|Distribution| O[Admin]
    
    P[Vesting] --> Q[Cliff]
    P --> R[Linear Release]
    Q -->|Percentage| S[10%]
    R -->|Duration| T[Variable]
```

## User Journey Map

```mermaid
journey
    title Memechan.gg User Journey
    section Onboarding
        Connect Wallet: 5: User
        Explore Platform: 3: User
    section Trading
        Swap SOL for MEME: 5: User
        Review Transaction: 4: User
        Confirm Swap: 5: User
    section Staking
        Navigate to Staking: 4: User
        Review Staking Terms: 3: User
        Stake MEME Tokens: 5: User
    section Management
        Monitor Vesting: 3: User
        Claim Rewards: 5: User
        Unstake Vested Tokens: 4: User
    section Advanced
        Swap for CHAN: 4: User
        Participate in Governance: 3: User
```

## Integration Points

```mermaid
graph TD
    A[Memechan.gg] --> B[Solana Ecosystem]
    A --> C[External Platforms]
    
    B --> B1[Solana Wallets]
    B --> B2[Solana Explorer]
    B --> B3[Other Solana DeFi]
    
    C --> C1[Price Aggregators]
    C --> C2[Portfolio Trackers]
    C --> C3[DEX Aggregators]
    
    D[API Endpoints] --> D1[Price Data]
    D --> D2[Pool Information]
    D --> D3[User Positions]
    
    E[SDK Integration] --> E1[JavaScript/TypeScript]
    E --> E2[Python]
    E --> E3[Rust]
```

## Development Roadmap

```mermaid
gantt
    title Memechan.gg Development Roadmap
    dateFormat  YYYY-MM-DD
    section Core Platform
        Initial Development           :done,    des1, 2023-01-01, 2023-06-30
        Testnet Deployment            :done,    des2, 2023-07-01, 2023-08-31
        Mainnet Launch                :active,  des3, 2023-09-01, 2023-10-31
    section Ecosystem
        SDK Development               :         des4, 2023-10-01, 2023-11-30
        Mobile App Beta               :         des5, 2023-12-01, 2024-02-28
        Cross-chain Bridge            :         des6, 2024-03-01, 2024-05-31
    section Governance
        DAO Framework                 :         des7, 2024-06-01, 2024-08-31
        Community Voting              :         des8, 2024-09-01, 2024-10-31
```

## Glossary of Key Concepts

| Term | Definition |
|------|------------|
| **AMM** | Automated Market Maker - A type of decentralized exchange protocol that uses a mathematical formula to price assets |
| **Bonding Curve** | A mathematical curve that defines the price relationship between a token's supply and its price |
| **Vesting** | The process of gradually earning access to tokens over a predetermined period |
| **Cliff** | An initial period after which a portion of tokens becomes available |
| **Linear Vesting** | A vesting schedule where tokens are released at a constant rate over time |
| **Staking** | The process of locking up tokens to support network operations and earn rewards |
| **Liquidity Pool** | A collection of funds locked in a smart contract to facilitate trading |
| **Slippage** | The difference between expected price and execution price due to market movement |
| **PDA** | Program Derived Address - A deterministic address derived from a program ID and seeds |
| **CPI** | Cross-Program Invocation - When one Solana program calls another |
```