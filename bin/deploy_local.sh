solana config set --url localhost

solana airdrop 1000

solana program deploy tests/programs/openbook.so  --keypair keypairs/kp.json # openbook: 5LwV4JK4ExCPJGJqhJLEQcHPeVsUX1omRo97jH6BvymW
solana program deploy tests/programs/raydium.so  --keypair keypairs/kp.json # raydium: FszPLAESehPmaW69NAGBLzRLFNFNjPQXNCfSEac8nst

anchor deploy  --program-name memechan_sol --program-keypair keypairs/kp.json
