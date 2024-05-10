solana config set --url localhost

solana airdrop 1000
solana airdrop 1000 GpZkFVSF2DBW7P4S3jDde7pydU2WBE3GiiHpRyqt1zcj

solana program deploy tests/programs/openbook.so  --keypair keypairs/kp.json # openbook: 6jXACiuWwGjc2Hq7rzz7mLGZZnAjAgS6noCYtB31xx4u
solana program deploy tests/programs/raydium.so  --keypair keypairs/kp.json # raydium: EA68RLWrCRtwbiudgb25mHrFzuLBfCEVVhHEeuFF9sC6

anchor deploy
