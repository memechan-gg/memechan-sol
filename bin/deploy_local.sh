solana config set --url localhost

solana airdrop 1000

# solana program deploy tests/programs/openbook.so --program-id keypairs/openbook.json # openbook: 5LwV4JK4ExCPJGJqhJLEQcHPeVsUX1omRo97jH6BvymW
# solana program deploy tests/programs/raydium.so --program-id keypairs/raydium.json # raydium: FszPLAESehPmaW69NAGBLzRLFNFNjPQXNCfSEac8nst

solana-test-validator --bpf-program 5LwV4JK4ExCPJGJqhJLEQcHPeVsUX1omRo97jH6BvymW tests/programs/devnet/openbook.so --bpf-program FszPLAESehPmaW69NAGBLzRLFNFNjPQXNCfSEac8nsA tests/devnet/programs/raydium.so

anchor deploy  --program-name memechan_sol --program-keypair keypairs/kp.json