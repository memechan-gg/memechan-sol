# Memechan.gg Developer Guide

This guide provides information for developers who want to contribute to the Memechan.gg Solana implementation, including development setup, coding standards, and contribution guidelines.

## Table of Contents

- [Development Environment Setup](#development-environment-setup)
- [Project Structure](#project-structure)
- [Building and Testing](#building-and-testing)
- [Contribution Guidelines](#contribution-guidelines)
- [Smart Contract Development](#smart-contract-development)
- [Client SDK Development](#client-sdk-development)
- [Debugging and Troubleshooting](#debugging-and-troubleshooting)

## Development Environment Setup

### Prerequisites

To develop for the Memechan.gg platform, you'll need:

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Solana CLI** (v1.17.31 or higher)
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.31/install)"
   ```

3. **Anchor Framework** (v0.29.0 or higher)
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked
   avm install 0.29.0
   avm use 0.29.0
   ```

4. **Node.js** (v14 or higher) and **Yarn**
   ```bash
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
   nvm install 14
   npm install -g yarn
   ```

5. **Solana Localnet**
   ```bash
   solana-test-validator
   ```

### Repository Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/memechan-gg/memechan-sol.git
   cd memechan-sol
   ```

2. Install dependencies:
   ```bash
   yarn install
   ```

3. Build the project:
   ```bash
   anchor build
   ```

## Project Structure

The Memechan.gg project follows a standard Anchor project structure with some additional organization:

```
memechan-sol/
├── programs/                  # Solana programs (smart contracts)
│   └── memechan-sol/          # Main program
│       ├── src/               # Source code
│       │   ├── consts.rs      # Constants
│       │   ├── endpoints/     # Program instruction handlers
│       │   ├── err.rs         # Error definitions
│       │   ├── libraries/     # Utility libraries
│       │   ├── math/          # Mathematical implementations
│       │   ├── models/        # Data structures
│       │   └── vesting.rs     # Vesting implementation
│       └── Cargo.toml         # Rust dependencies
├── tests/                     # Integration tests
│   ├── endpoints/             # Endpoint-specific tests
│   ├── common.ts              # Common test utilities
│   └── memechan-sol.ts        # Main test file
├── migrations/                # Database migrations
├── bin/                       # CLI tools and scripts
├── docs/                      # Documentation
├── Anchor.toml                # Anchor configuration
├── Cargo.toml                 # Workspace configuration
├── package.json               # JavaScript dependencies
└── tsconfig.json              # TypeScript configuration
```

## Building and Testing

### Building the Program

```bash
# Build with default features
anchor build

# Build with testing features
anchor build -- --features localnet-testing

# Build for mainnet
anchor build -- --features mainnet --no-default-features
```

### Running Tests

```bash
# Run all tests
anchor test

# Run specific test file
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/specific_test.ts

# Run with increased logging
RUST_LOG=debug anchor test
```

### Property-Based Testing

The project uses property-based testing for mathematical models:

```bash
# Run property tests
cargo test --package memechan-sol --lib --all-features -- models::bound::tests --show-output

# Run specific property test
cargo test --package memechan-sol --lib --all-features -- models::bound::tests::successfully_returns_positive_exponent --exact --show-output
```

## Contribution Guidelines

### Branching Strategy

We follow a feature branch workflow:

1. Create a branch from `main` for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes, commit, and push:
   ```bash
   git add .
   git commit -m "Description of changes"
   git push origin feature/your-feature-name
   ```

3. Create a pull request to merge your changes into `main`.

### Code Style

#### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Write documentation comments for public functions and types

#### TypeScript

- Follow the [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- Use `prettier` for formatting
- Use TypeScript types for all variables and function parameters

### Pull Request Process

1. Ensure your code passes all tests
2. Update documentation if necessary
3. Add tests for new functionality
4. Get at least one code review from a maintainer
5. Squash commits before merging

## Smart Contract Development

### Adding a New Instruction

To add a new instruction to the Memechan.gg program:

1. Create a new file in `programs/memechan-sol/src/endpoints/` for your instruction handler
2. Define the accounts struct with appropriate constraints
3. Implement the instruction handler function
4. Add the instruction to the program entry point in `lib.rs`
5. Add tests for the new instruction in `tests/endpoints/`

Example:

```rust
// In new_instruction.rs
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct NewInstruction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Other accounts...
}

pub fn handle(ctx: Context<NewInstruction>, param1: u64) -> Result<()> {
    // Implementation...
    Ok(())
}

// In lib.rs
pub fn new_instruction(ctx: Context<NewInstruction>, param1: u64) -> Result<()> {
    new_instruction::handle(ctx, param1)
}
```

### Security Best Practices

When developing smart contracts:

1. **Always validate inputs**: Check that inputs are within expected ranges
2. **Use checked arithmetic**: Prevent overflows with `checked_add`, `checked_sub`, etc.
3. **Validate account relationships**: Use constraints to ensure accounts have the correct relationships
4. **Minimize privileged operations**: Limit admin functions to only what's necessary
5. **Test edge cases**: Include tests for boundary conditions and error cases

## Client SDK Development

### TypeScript Client

The TypeScript client provides a convenient interface for interacting with the Memechan.gg program:

```typescript
// Example of extending the client SDK
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { MemechanSol } from '../target/types/memechan_sol';

export class NewFeatureWrapper {
  constructor(
    private program: Program<MemechanSol>,
    private poolAddress: PublicKey
  ) {}

  async performAction(param1: number): Promise<void> {
    await this.program.methods
      .newInstruction(new BN(param1))
      .accounts({
        // Account details
      })
      .rpc();
  }
}
```

## Debugging and Troubleshooting

### Common Issues

#### Program Build Failures

- Check that you have the correct Rust and Anchor versions
- Ensure all dependencies are correctly specified in `Cargo.toml`
- Look for syntax errors or type mismatches

#### Test Failures

- Use `console.log()` in TypeScript tests to debug
- Add `msg!()` statements in Rust code for on-chain logging
- Check account constraints and ensure all required accounts are provided

#### Transaction Errors

- Check for constraint violations in the program logs
- Verify that account owners and authorities are correct
- Ensure sufficient SOL for transaction fees

### Logging

- In Rust code, use `msg!()` for logging:
  ```rust
  msg!("Processing transaction with amount: {}", amount);
  ```

- In TypeScript tests, enable verbose logging:
  ```typescript
  // Enable verbose logging
  const txId = await program.methods
    .someInstruction()
    .accounts({...})
    .rpc({ skipPreflight: true });
  
  // Fetch transaction logs
  const tx = await provider.connection.getTransaction(txId, {
    commitment: 'confirmed',
  });
  console.log('Transaction logs:', tx?.meta?.logMessages);
  ```

### Using the Solana Explorer

For deployed programs, you can use the Solana Explorer to inspect transactions:

1. Go to [https://explorer.solana.com/](https://explorer.solana.com/) (or [https://explorer.solana.com/?cluster=devnet](https://explorer.solana.com/?cluster=devnet) for devnet)
2. Enter your transaction ID or program ID
3. View transaction details, including logs and account changes

## Advanced Topics

### Cross-Program Invocation (CPI)

To call other Solana programs from the Memechan.gg program:

```rust
// Example of calling the token program
pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.source.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, amount)
}
```

### Program Derived Addresses (PDAs)

To work with PDAs:

```rust
// Finding a PDA
let (pool_address, bump) = Pubkey::find_program_address(
    &[b"pool", user.key().as_ref()],
    program_id
);

// Signing with a PDA
let seeds = &[
    b"pool",
    user.key().as_ref(),
    &[bump],
];
let signer_seeds = &[&seeds[..]];

token::transfer(
    ctx.accounts.transfer_ctx().with_signer(signer_seeds),
    amount
)?;
```

## Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework Documentation](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Solana Program Library](https://spl.solana.com/)