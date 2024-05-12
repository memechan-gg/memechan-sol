solana config set --url localhost

solana airdrop 1000

solana program deploy tests/programs/openbook.so  --keypair keypairs/kp.json # openbook: 6jXACiuWwGjc2Hq7rzz7mLGZZnAjAgS6noCYtB31xx4u
solana program deploy tests/programs/raydium.so  --keypair keypairs/kp.json # raydium: EA68RLWrCRtwbiudgb25mHrFzuLBfCEVVhHEeuFF9sC6

anchor deploy  --program-name memechan_sol --program-keypair keypairs/kp.json
