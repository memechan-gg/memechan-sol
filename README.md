# Memechan.gg Solana implementation

# Mentions

Used math lib from Raydium, thanks for that.

`solana config set --keypair ${pwd}/keypairs/devnet-keypair.json`

`anchor deploy --program-name memechan_sol --program-keypair keypairs/kp.json`

`solana config set --url localhost`

`solana config set --url devnet`

`solana-keygen new --outfile kp.json`


# devnet deploy
- generate new keypar (gen_devkp.sh steps)
- replace old id everywhere
- anchor clean
- anchor build
- anchor deploy  --program-name memechan_sol --program-keypair keypairs/kp.json

# Proptesting
To collect the logs from proptests
`cargo test --package memechan-sol --lib --all-features -- models::bound::tests::successfully_returns_positive_exponent --exact --show-output >> output.txt`


# SDK
1. Create `.npmrc` file with the read token of `@avernikoz/memechan-sol-sdk` (should be something like `//registry.npmjs.org/:_authToken=[real npm auth token]`)
2. Run `yarn install` to install 