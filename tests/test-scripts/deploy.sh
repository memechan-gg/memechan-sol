#!/bin/bash

#solana program deploy ../../target/deploy/memechan_sol.so -k ../../target/deploy/memechan_sol-keypair.json
anchor build && anchor deploy
spl-token create-token tokenKey.json 
solana transfer 8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV 5 --allow-unfunded-recipient
spl-token authorize B6p1PUnkiJubxb6V4oioEsgVEEfrmuEpyf2J3C5dUzrK mint 8SvkUtJZCyJwSQGkiszwcRcPv7c8pPSr8GVEppGNN7DV

