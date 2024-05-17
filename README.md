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