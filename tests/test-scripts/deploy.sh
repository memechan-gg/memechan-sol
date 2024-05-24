#!/bin/bash

#solana program deploy ../../target/deploy/memechan_sol.so -k ../../target/deploy/memechan_sol-keypair.json
anchor build && anchor deploy
