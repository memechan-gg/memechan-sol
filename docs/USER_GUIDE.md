# Memechan.gg User Guide

This guide provides step-by-step instructions for using the Memechan.gg platform, including token swapping, staking, and other key features.

## Table of Contents

- [Getting Started](#getting-started)
- [Token Swapping](#token-swapping)
- [Staking](#staking)
- [Vesting and Rewards](#vesting-and-rewards)
- [CHAN Token](#chan-token)
- [Frequently Asked Questions](#frequently-asked-questions)

## Getting Started

### Prerequisites

Before using the Memechan.gg platform, you'll need:

1. A Solana wallet (such as [Phantom](https://phantom.app/), [Solflare](https://solflare.com/), or [Backpack](https://www.backpack.app/))
2. Some SOL for transaction fees and swapping
3. Access to the Memechan.gg web interface or compatible dApp

### Connecting Your Wallet

1. Visit the Memechan.gg platform
2. Click the "Connect Wallet" button in the top right corner
3. Select your wallet provider from the list
4. Approve the connection request in your wallet

## Token Swapping

Memechan.gg allows you to swap between SOL, MEME, and CHAN tokens through its custom AMM (Automated Market Maker).

### Swapping SOL for MEME Tokens

1. Navigate to the "Swap" tab
2. Select SOL as the input token and MEME as the output token
3. Enter the amount of SOL you want to swap
4. The interface will display the estimated MEME tokens you'll receive
5. Click "Swap" to initiate the transaction
6. Confirm the transaction in your wallet

### Swapping MEME for SOL

1. Navigate to the "Swap" tab
2. Select MEME as the input token and SOL as the output token
3. Enter the amount of MEME tokens you want to swap
4. The interface will display the estimated SOL you'll receive
5. Click "Swap" to initiate the transaction
6. Confirm the transaction in your wallet

### Understanding Slippage

Slippage refers to the difference between the expected price of a trade and the actual price when the trade executes. The Memechan.gg platform uses a bonding curve mechanism that may result in price changes based on the size of your swap:

- Smaller swaps typically have lower slippage
- Larger swaps may have higher slippage due to their impact on the pool's reserves

You can adjust your slippage tolerance in the swap interface settings.

## Staking

Staking your MEME tokens allows you to earn rewards over time.

### Staking MEME Tokens

1. Navigate to the "Stake" tab
2. Enter the amount of MEME tokens you want to stake
3. Review the staking terms, including the vesting period
4. Click "Stake" to initiate the transaction
5. Confirm the transaction in your wallet

Once staked, your tokens will be subject to the vesting schedule, which includes:
- An initial cliff period where a percentage of tokens is immediately available
- A linear vesting period for the remaining tokens

### Viewing Your Staked Tokens

1. Navigate to the "Portfolio" tab
2. The "Staked" section displays your currently staked tokens
3. You can view details including:
   - Total staked amount
   - Vested amount (available for withdrawal)
   - Remaining vesting period

### Unstaking Tokens

1. Navigate to the "Stake" tab
2. Select the "Unstake" option
3. Enter the amount you wish to unstake (limited to your vested amount)
4. Click "Unstake" to initiate the transaction
5. Confirm the transaction in your wallet

## Vesting and Rewards

### Understanding the Vesting Schedule

When you stake MEME tokens, they follow a vesting schedule:

1. **Cliff Period**: Initially, 10% of your staked tokens are immediately available for withdrawal
2. **Linear Vesting**: The remaining 90% of tokens vest linearly over the vesting period
3. **Full Vesting**: After the vesting period ends, 100% of your tokens are available for withdrawal

### Reward Distribution

Staking rewards are distributed based on:

1. **Fee Sharing**: A portion of swap fees is distributed to stakers
2. **Staking Duration**: Longer staking periods may result in higher rewards
3. **Stake Amount**: Larger stakes receive proportionally more rewards

To claim rewards:
1. Navigate to the "Rewards" tab
2. View your available rewards
3. Click "Claim" to receive your rewards
4. Confirm the transaction in your wallet

## CHAN Token

The CHAN token is the governance and utility token of the Memechan.gg ecosystem.

### Acquiring CHAN Tokens

There are several ways to acquire CHAN tokens:

1. **Swapping**: Exchange MEME tokens for CHAN through the platform
2. **Rewards**: Earn CHAN tokens through staking rewards
3. **Airdrops**: Participate in community events for potential airdrops

### Using CHAN Tokens

CHAN tokens can be used for:

1. **Governance**: Vote on platform proposals and changes
2. **Fee Discounts**: Reduced trading fees when holding CHAN
3. **Special Features**: Access to exclusive platform features

## Frequently Asked Questions

### General Questions

**Q: What is Memechan.gg?**  
A: Memechan.gg is a decentralized finance (DeFi) platform built on the Solana blockchain, offering token swapping, staking, and other financial services.

**Q: How are token prices determined?**  
A: Token prices are determined by a custom bonding curve mechanism that adjusts prices based on supply and demand within the liquidity pools.

### Swapping

**Q: Why did I receive fewer tokens than expected?**  
A: This could be due to slippage, which occurs when the price changes between the time you submit a transaction and when it's processed. You can adjust your slippage tolerance in the settings.

**Q: Are there fees for swapping?**  
A: Yes, there is a small fee for each swap. A portion of these fees goes to stakers, and another portion goes to platform development.

### Staking

**Q: Can I unstake my tokens at any time?**  
A: You can unstake vested tokens at any time. However, tokens that are still in the vesting period cannot be unstaked until they vest.

**Q: What happens if I unstake before the vesting period ends?**  
A: You can only unstake tokens that have already vested according to the vesting schedule. The remaining tokens will continue to vest over time.

### Technical Issues

**Q: What should I do if a transaction fails?**  
A: If a transaction fails, check:
1. That you have enough SOL for transaction fees
2. That you're not trying to swap or unstake more than you have available
3. That the network isn't congested (which may require increasing your transaction priority fee)

**Q: How do I report a bug or issue?**  
A: You can report bugs or issues through:
1. The "Support" section on the platform
2. The official Discord community
3. The GitHub repository's issue tracker

## Support and Resources

- **Official Website**: [Memechan.gg](https://memechan.gg)
- **Documentation**: [docs.memechan.gg](https://docs.memechan.gg)
- **Discord Community**: [discord.gg/memechan](https://discord.gg/memechan)
- **Twitter**: [@memechan_gg](https://twitter.com/memechan_gg)