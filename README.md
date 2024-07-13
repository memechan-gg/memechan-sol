# Memechan.gg Solana implementation

# Mentions

Used math lib from Raydium, thanks for that.

# mainnet testing deploy
- "yarn bt"
- "yarn wbt" until buffer gets written fully
- "yarn dt

# mainnet deploy
- generate new keypair corresponding to pubkey you want to deploy your program to (and save private key to kp.json) or create random one using "solana-keygen new -o kp.json"
- in lib.rs on line 15 change address that you pass to macros declare_id! to the pubkey of the keypair you'll be using to deploy
- check if lock/vesting constants in consts.rs are okay
- "anchor clean"
- "anchor build -- --features mainnet"
- go to ./target/deploy
- generate new keypair that you'll be using for deployment buffer with "solana-keygen new -o bufkey.json"
- "solana program write-buffer --buffer bufkey.json memechan_sol.so \<connection config>"
  - config can be null if you already set up your current solana config to mainnet
  - if you don't have access to a private RPC node, you can use public mainnet one via "-u m" parameter
- deployment may get stuck randomly; you may want to cancel it and continue via different connection config or to wait for a bit
  - don't worry; all the data is safe, and you'll be able to continue deployment from where you stopped
  - still not proven whether cancelling yourself is a thing or just a placebo; use as you deem fit
- if the process is stuck you can continue by executing "solana program write-buffer --buffer bufkey.json memechan_sol.so \<connection config>"
- after the process is finished deploy by executing "solana program deploy --buffer bufkey.json --program-id kp.json \<connection config>"
  - this one can also be retried; when deploy succeeds, the buffer used will be consumed, and you cannot overwrite anything this way
- if you made a mistake with program address or want to deploy it somewhere else, you can refund program's account using " solana program close \<program address> --bypass-warning \<connection config>"
  - beware that if you lose key with pubkey of the program you are going to close, you won't be able to re-deploy your program to that pubkey
# Proptesting
To collect the logs from proptests
`cargo test --package memechan-sol --lib --all-features -- models::bound::tests::successfully_returns_positive_exponent --exact --show-output >> output.txt`
