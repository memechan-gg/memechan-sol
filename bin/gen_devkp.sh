#!/bin/sh

# Delete a file
rm keypairs/kp.json

solana-keygen new --outfile keypairs/kp.json

solana-keygen pubkey keypairs/kp.json