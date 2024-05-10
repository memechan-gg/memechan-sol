export type MemechanSol = {
  "version": "0.1.0",
  "name": "memechan_sol",
  "instructions": [
    {
      "name": "new",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "adminSolVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "launchVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "swapX",
      "accounts": [
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "poolSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "coinInAmount",
          "type": "u64"
        },
        {
          "name": "coinYMinValue",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swapY",
      "accounts": [
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "poolSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "coinInAmount",
          "type": "u64"
        },
        {
          "name": "coinXMinValue",
          "type": "u64"
        }
      ]
    },
    {
      "name": "goLive",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "Signer"
          ]
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool account"
          ]
        },
        {
          "name": "boundPoolSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "poolMemeVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool Meme vault"
          ]
        },
        {
          "name": "poolWsolVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool WSOL vault"
          ]
        },
        {
          "name": "adminVaultSol",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool Admin Vault"
          ]
        },
        {
          "name": "memeMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Mint Account for Meme"
          ]
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Mint Account for WSOL"
          ]
        },
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Staking Pool Account"
          ]
        },
        {
          "name": "stakingPoolSignerPda",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Staking Pool Signer"
          ]
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Meme Ticket Account of Admin"
          ]
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Open Orders Account"
          ]
        },
        {
          "name": "targetOrders",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Target Orders Account"
          ]
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Market Orders Account"
          ]
        },
        {
          "name": "marketEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumAmm",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium AMM Account"
          ]
        },
        {
          "name": "raydiumAmmAuthority",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Raydium AMM Signer"
          ]
        },
        {
          "name": "raydiumLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium LP MinT"
          ]
        },
        {
          "name": "poolLpWallet",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium LP Token Account"
          ]
        },
        {
          "name": "raydiumMemeVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium Meme Token Account"
          ]
        },
        {
          "name": "raydiumWsolVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium WSOL Token Account"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u8"
        }
      ]
    },
    {
      "name": "addFees",
      "accounts": [
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "aldrinPoolAcc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolLpWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raydiumAmm",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumAmmAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raydiumMemeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumWsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "targetOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketCoinVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketPcVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketVaultSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMeme",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userWsol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "releaseAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawFees",
      "accounts": [
        {
          "name": "staking",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMeme",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userWsol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "boundMergeTickets",
      "accounts": [
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ticketInto",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ticketFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "stakingMergeTickets",
      "accounts": [
        {
          "name": "staking",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ticketInto",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ticketFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "closeTicket",
      "accounts": [
        {
          "name": "ticket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "boundPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "memeReserve",
            "type": {
              "defined": "Reserve"
            }
          },
          {
            "name": "solReserve",
            "type": {
              "defined": "Reserve"
            }
          },
          {
            "name": "adminFeesMeme",
            "type": "u64"
          },
          {
            "name": "adminFeesSol",
            "type": "u64"
          },
          {
            "name": "adminVaultSol",
            "type": "publicKey"
          },
          {
            "name": "fees",
            "type": {
              "defined": "Fees"
            }
          },
          {
            "name": "config",
            "type": {
              "defined": "Config"
            }
          },
          {
            "name": "locked",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "memeTicket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "pool",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "withdrawsMeme",
            "type": "u64"
          },
          {
            "name": "withdrawsWsol",
            "type": "u64"
          },
          {
            "name": "untilTimestamp",
            "type": "i64"
          },
          {
            "name": "vesting",
            "type": {
              "defined": "VestingData"
            }
          }
        ]
      }
    },
    {
      "name": "stakingPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "publicKey"
          },
          {
            "name": "memeVault",
            "type": "publicKey"
          },
          {
            "name": "memeMint",
            "type": "publicKey"
          },
          {
            "name": "wsolVault",
            "type": "publicKey"
          },
          {
            "name": "vestingConfig",
            "type": {
              "defined": "VestingConfig"
            }
          },
          {
            "name": "stakesTotal",
            "type": "u64"
          },
          {
            "name": "feesXTotal",
            "type": "u64"
          },
          {
            "name": "feesYTotal",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ammConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pnlOwner",
            "docs": [
              "withdraw pnl owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "cancelOwner",
            "docs": [
              "admin amm order owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "pending1",
            "docs": [
              "pending"
            ],
            "type": {
              "array": [
                "u64",
                28
              ]
            }
          },
          {
            "name": "pending2",
            "docs": [
              "pending"
            ],
            "type": {
              "array": [
                "u64",
                31
              ]
            }
          },
          {
            "name": "createPoolFee",
            "docs": [
              "init amm pool fee amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ammInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "status",
            "docs": [
              "Initialized status."
            ],
            "type": "u64"
          },
          {
            "name": "nonce",
            "docs": [
              "Nonce used in program address.",
              "The program address is created deterministically with the nonce,",
              "amm program id, and amm account pubkey.  This program address has",
              "authority over the amm's token coin account, token pc account, and pool",
              "token mint."
            ],
            "type": "u64"
          },
          {
            "name": "orderNum",
            "docs": [
              "max order count"
            ],
            "type": "u64"
          },
          {
            "name": "depth",
            "docs": [
              "within this range, 5 => 5% range"
            ],
            "type": "u64"
          },
          {
            "name": "coinDecimals",
            "docs": [
              "coin decimal"
            ],
            "type": "u64"
          },
          {
            "name": "pcDecimals",
            "docs": [
              "pc decimal"
            ],
            "type": "u64"
          },
          {
            "name": "state",
            "docs": [
              "amm machine state"
            ],
            "type": "u64"
          },
          {
            "name": "resetFlag",
            "docs": [
              "amm reset_flag"
            ],
            "type": "u64"
          },
          {
            "name": "minSize",
            "docs": [
              "min size 1->0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "volMaxCutRatio",
            "docs": [
              "vol_max_cut_ratio numerator, sys_decimal_value as denominator"
            ],
            "type": "u64"
          },
          {
            "name": "amountWave",
            "docs": [
              "amount wave numerator, sys_decimal_value as denominator"
            ],
            "type": "u64"
          },
          {
            "name": "coinLotSize",
            "docs": [
              "coinLotSize 1 -> 0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "pcLotSize",
            "docs": [
              "pcLotSize 1 -> 0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "minPriceMultiplier",
            "docs": [
              "min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier"
            ],
            "type": "u64"
          },
          {
            "name": "maxPriceMultiplier",
            "docs": [
              "max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier"
            ],
            "type": "u64"
          },
          {
            "name": "sysDecimalValue",
            "docs": [
              "system decimal value, used to normalize the value of coin and pc amount"
            ],
            "type": "u64"
          },
          {
            "name": "fees",
            "docs": [
              "All fee information"
            ],
            "type": {
              "defined": "RaydiumFees"
            }
          },
          {
            "name": "stateData",
            "docs": [
              "Statistical data"
            ],
            "type": {
              "defined": "StateData"
            }
          },
          {
            "name": "coinVault",
            "docs": [
              "Coin vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "pcVault",
            "docs": [
              "Pc vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "coinVaultMint",
            "docs": [
              "Coin vault mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "pcVaultMint",
            "docs": [
              "Pc vault mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "lpMint",
            "docs": [
              "lp mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "openOrders",
            "docs": [
              "open_orders key"
            ],
            "type": "publicKey"
          },
          {
            "name": "market",
            "docs": [
              "market key"
            ],
            "type": "publicKey"
          },
          {
            "name": "marketProgram",
            "docs": [
              "market program key"
            ],
            "type": "publicKey"
          },
          {
            "name": "targetOrders",
            "docs": [
              "target_orders key"
            ],
            "type": "publicKey"
          },
          {
            "name": "padding1",
            "docs": [
              "padding"
            ],
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "ammOwner",
            "docs": [
              "amm owner key"
            ],
            "type": "publicKey"
          },
          {
            "name": "lpAmount",
            "docs": [
              "pool lp amount"
            ],
            "type": "u64"
          },
          {
            "name": "clientOrderId",
            "docs": [
              "client order id"
            ],
            "type": "u64"
          },
          {
            "name": "padding2",
            "docs": [
              "padding"
            ],
            "type": {
              "array": [
                "u64",
                2
              ]
            }
          }
        ]
      }
    },
    {
      "name": "targetOrders",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "buyOrders",
            "type": {
              "array": [
                {
                  "defined": "TargetOrder"
                },
                50
              ]
            }
          },
          {
            "name": "padding1",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "targetX",
            "type": "u128"
          },
          {
            "name": "targetY",
            "type": "u128"
          },
          {
            "name": "planXBuy",
            "type": "u128"
          },
          {
            "name": "planYBuy",
            "type": "u128"
          },
          {
            "name": "planXSell",
            "type": "u128"
          },
          {
            "name": "planYSell",
            "type": "u128"
          },
          {
            "name": "placedX",
            "type": "u128"
          },
          {
            "name": "placedY",
            "type": "u128"
          },
          {
            "name": "calcPnlX",
            "type": "u128"
          },
          {
            "name": "calcPnlY",
            "type": "u128"
          },
          {
            "name": "sellOrders",
            "type": {
              "array": [
                {
                  "defined": "TargetOrder"
                },
                50
              ]
            }
          },
          {
            "name": "padding2",
            "type": {
              "array": [
                "u64",
                6
              ]
            }
          },
          {
            "name": "replaceBuyClientId",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "replaceSellClientId",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "lastOrderNumerator",
            "type": "u64"
          },
          {
            "name": "lastOrderDenominator",
            "type": "u64"
          },
          {
            "name": "planOrdersCur",
            "type": "u64"
          },
          {
            "name": "placeOrdersCur",
            "type": "u64"
          },
          {
            "name": "validBuyOrderNum",
            "type": "u64"
          },
          {
            "name": "validSellOrderNum",
            "type": "u64"
          },
          {
            "name": "padding3",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "freeSlotBits",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "openOrders",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "accountFlags",
            "type": "u64"
          },
          {
            "name": "market",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "owner",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "nativeCoinFree",
            "type": "u64"
          },
          {
            "name": "nativeCoinTotal",
            "type": "u64"
          },
          {
            "name": "nativePcFree",
            "type": "u64"
          },
          {
            "name": "nativePcTotal",
            "type": "u64"
          },
          {
            "name": "freeSlotBits",
            "type": "u128"
          },
          {
            "name": "isBidBits",
            "type": "u128"
          },
          {
            "name": "orders",
            "type": {
              "array": [
                "u128",
                128
              ]
            }
          },
          {
            "name": "clientOrderIds",
            "type": {
              "array": [
                "u64",
                128
              ]
            }
          },
          {
            "name": "referrerRebatesAccrued",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "marketState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "accountFlags",
            "type": "u64"
          },
          {
            "name": "ownAddress",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "vaultSignerNonce",
            "type": "u64"
          },
          {
            "name": "coinMint",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "pcMint",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinVault",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinDepositsTotal",
            "type": "u64"
          },
          {
            "name": "coinFeesAccrued",
            "type": "u64"
          },
          {
            "name": "pcVault",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "pcDepositsTotal",
            "type": "u64"
          },
          {
            "name": "pcFeesAccrued",
            "type": "u64"
          },
          {
            "name": "pcDustThreshold",
            "type": "u64"
          },
          {
            "name": "reqQ",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "eventQ",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "bids",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "asks",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinLotSize",
            "type": "u64"
          },
          {
            "name": "pcLotSize",
            "type": "u64"
          },
          {
            "name": "feeRateBps",
            "type": "u64"
          },
          {
            "name": "referrerRebatesAccrued",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "alphaAbs",
            "type": "u128"
          },
          {
            "name": "beta",
            "type": "u128"
          },
          {
            "name": "priceFactor",
            "type": "u64"
          },
          {
            "name": "gammaS",
            "type": "u64"
          },
          {
            "name": "gammaM",
            "type": "u64"
          },
          {
            "name": "omegaM",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Fees",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeInPercent",
            "type": "u64"
          },
          {
            "name": "feeOutPercent",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TokenLimit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "tokens",
            "type": {
              "defined": "TokenAmount"
            }
          }
        ]
      }
    },
    {
      "name": "TokenAmount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Reserve",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokens",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "vault",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "RaydiumFees",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "minSeparateNumerator",
            "docs": [
              "numerator of the min_separate"
            ],
            "type": "u64"
          },
          {
            "name": "minSeparateDenominator",
            "docs": [
              "denominator of the min_separate"
            ],
            "type": "u64"
          },
          {
            "name": "tradeFeeNumerator",
            "docs": [
              "numerator of the fee"
            ],
            "type": "u64"
          },
          {
            "name": "tradeFeeDenominator",
            "docs": [
              "denominator of the fee",
              "and 'trade_fee_denominator' must be equal to 'min_separate_denominator'"
            ],
            "type": "u64"
          },
          {
            "name": "pnlNumerator",
            "docs": [
              "numerator of the pnl"
            ],
            "type": "u64"
          },
          {
            "name": "pnlDenominator",
            "docs": [
              "denominator of the pnl"
            ],
            "type": "u64"
          },
          {
            "name": "swapFeeNumerator",
            "docs": [
              "numerator of the swap_fee"
            ],
            "type": "u64"
          },
          {
            "name": "swapFeeDenominator",
            "docs": [
              "denominator of the swap_fee"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StateData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "needTakePnlCoin",
            "docs": [
              "delay to take pnl coin"
            ],
            "type": "u64"
          },
          {
            "name": "needTakePnlPc",
            "docs": [
              "delay to take pnl pc"
            ],
            "type": "u64"
          },
          {
            "name": "totalPnlPc",
            "docs": [
              "total pnl pc"
            ],
            "type": "u64"
          },
          {
            "name": "totalPnlCoin",
            "docs": [
              "total pnl coin"
            ],
            "type": "u64"
          },
          {
            "name": "poolOpenTime",
            "docs": [
              "ido pool open time"
            ],
            "type": "u64"
          },
          {
            "name": "padding",
            "docs": [
              "padding for future updates"
            ],
            "type": {
              "array": [
                "u64",
                2
              ]
            }
          },
          {
            "name": "orderbookToInitTime",
            "docs": [
              "switch from orderbookonly to init"
            ],
            "type": "u64"
          },
          {
            "name": "swapCoinInAmount",
            "docs": [
              "swap coin in amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapPcOutAmount",
            "docs": [
              "swap pc out amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapAccPcFee",
            "docs": [
              "charge pc as swap fee while swap pc to coin"
            ],
            "type": "u64"
          },
          {
            "name": "swapPcInAmount",
            "docs": [
              "swap pc in amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapCoinOutAmount",
            "docs": [
              "swap coin out amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapAccCoinFee",
            "docs": [
              "charge coin as swap fee while swap coin to pc"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TargetOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "vol",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VestingConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "cliffTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "VestingData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "released",
            "type": "u64"
          },
          {
            "name": "notional",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAccountInput",
      "msg": "Provided account breaks some constraints, see logs for more info"
    },
    {
      "code": 6001,
      "name": "InvalidArg",
      "msg": "One of the provided input arguments is invalid"
    },
    {
      "code": 6002,
      "name": "SlippageExceeded",
      "msg": "Given amount of tokens to swap would result in \\\n        less than minimum requested tokens to receive"
    },
    {
      "code": 6003,
      "name": "InvariantViolation",
      "msg": "There's a bug in the program, see logs for more info"
    },
    {
      "code": 6004,
      "name": "InvalidTokenMints",
      "msg": "Provided mints are not available on the pool"
    },
    {
      "code": 6005,
      "name": "MulDivOverflow"
    },
    {
      "code": 6006,
      "name": "DivideByZero"
    },
    {
      "code": 6007,
      "name": "ZeroInAmt"
    },
    {
      "code": 6008,
      "name": "ZeroMemeVault"
    },
    {
      "code": 6009,
      "name": "InsufficientBalance"
    },
    {
      "code": 6010,
      "name": "PoolIsLocked",
      "msg": "Pool can't be interacted with until going into live phase"
    },
    {
      "code": 6011,
      "name": "NoZeroTokens",
      "msg": "Shouldn't provide zero tokens in"
    },
    {
      "code": 6012,
      "name": "NoTokensToWithdraw"
    },
    {
      "code": 6013,
      "name": "NotEnoughTicketTokens",
      "msg": "Amount of tokens in ticket is lower than needed to swap"
    },
    {
      "code": 6014,
      "name": "TicketTokensLocked",
      "msg": "Not enough time passed to unlock tokens bound to the ticket"
    },
    {
      "code": 6015,
      "name": "NonZeroAmountTicket",
      "msg": "Can't close ticket with non-zero bound token amount"
    },
    {
      "code": 6016,
      "name": "NotEnoughTokensToRelease",
      "msg": "Can't unstake the required amount of tokens"
    },
    {
      "code": 6017,
      "name": "BondingCurveMustBeNegativelySloped"
    },
    {
      "code": 6018,
      "name": "BondingCurveInterceptMustBePositive"
    }
  ]
};

export const IDL: MemechanSol = {
  "version": "0.1.0",
  "name": "memechan_sol",
  "instructions": [
    {
      "name": "new",
      "accounts": [
        {
          "name": "sender",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "adminSolVault",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "launchVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "swapX",
      "accounts": [
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "poolSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "coinInAmount",
          "type": "u64"
        },
        {
          "name": "coinYMinValue",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swapY",
      "accounts": [
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "solVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userSol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "poolSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "coinInAmount",
          "type": "u64"
        },
        {
          "name": "coinXMinValue",
          "type": "u64"
        }
      ]
    },
    {
      "name": "goLive",
      "accounts": [
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true,
          "docs": [
            "Signer"
          ]
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool account"
          ]
        },
        {
          "name": "boundPoolSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "poolMemeVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool Meme vault"
          ]
        },
        {
          "name": "poolWsolVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool WSOL vault"
          ]
        },
        {
          "name": "adminVaultSol",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Bonding Pool Admin Vault"
          ]
        },
        {
          "name": "memeMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Mint Account for Meme"
          ]
        },
        {
          "name": "solMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Mint Account for WSOL"
          ]
        },
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Staking Pool Account"
          ]
        },
        {
          "name": "stakingPoolSignerPda",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Staking Pool Signer"
          ]
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Meme Ticket Account of Admin"
          ]
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Open Orders Account"
          ]
        },
        {
          "name": "targetOrders",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Target Orders Account"
          ]
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Market Orders Account"
          ]
        },
        {
          "name": "marketEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumAmm",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium AMM Account"
          ]
        },
        {
          "name": "raydiumAmmAuthority",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Raydium AMM Signer"
          ]
        },
        {
          "name": "raydiumLpMint",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium LP MinT"
          ]
        },
        {
          "name": "poolLpWallet",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium LP Token Account"
          ]
        },
        {
          "name": "raydiumMemeVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium Meme Token Account"
          ]
        },
        {
          "name": "raydiumWsolVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Raydium WSOL Token Account"
          ]
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "clock",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketProgramId",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u8"
        }
      ]
    },
    {
      "name": "addFees",
      "accounts": [
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "aldrinPoolAcc",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "poolLpWallet",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "raydiumAmm",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumAmmAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "raydiumMemeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "raydiumWsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "openOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "targetOrders",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketEventQueue",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketCoinVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketPcVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketVaultSigner",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketBids",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "marketAsks",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "marketProgramId",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "unstake",
      "accounts": [
        {
          "name": "staking",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMeme",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userWsol",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "releaseAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawFees",
      "accounts": [
        {
          "name": "staking",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memeTicket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMeme",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "userWsol",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "memeVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "wsolVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakingSignerPda",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "boundMergeTickets",
      "accounts": [
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ticketInto",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ticketFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "stakingMergeTickets",
      "accounts": [
        {
          "name": "staking",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "ticketInto",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ticketFrom",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    },
    {
      "name": "closeTicket",
      "accounts": [
        {
          "name": "ticket",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "boundPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "memeReserve",
            "type": {
              "defined": "Reserve"
            }
          },
          {
            "name": "solReserve",
            "type": {
              "defined": "Reserve"
            }
          },
          {
            "name": "adminFeesMeme",
            "type": "u64"
          },
          {
            "name": "adminFeesSol",
            "type": "u64"
          },
          {
            "name": "adminVaultSol",
            "type": "publicKey"
          },
          {
            "name": "fees",
            "type": {
              "defined": "Fees"
            }
          },
          {
            "name": "config",
            "type": {
              "defined": "Config"
            }
          },
          {
            "name": "locked",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "memeTicket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "pool",
            "type": "publicKey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "withdrawsMeme",
            "type": "u64"
          },
          {
            "name": "withdrawsWsol",
            "type": "u64"
          },
          {
            "name": "untilTimestamp",
            "type": "i64"
          },
          {
            "name": "vesting",
            "type": {
              "defined": "VestingData"
            }
          }
        ]
      }
    },
    {
      "name": "stakingPool",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pool",
            "type": "publicKey"
          },
          {
            "name": "memeVault",
            "type": "publicKey"
          },
          {
            "name": "memeMint",
            "type": "publicKey"
          },
          {
            "name": "wsolVault",
            "type": "publicKey"
          },
          {
            "name": "vestingConfig",
            "type": {
              "defined": "VestingConfig"
            }
          },
          {
            "name": "stakesTotal",
            "type": "u64"
          },
          {
            "name": "feesXTotal",
            "type": "u64"
          },
          {
            "name": "feesYTotal",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ammConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pnlOwner",
            "docs": [
              "withdraw pnl owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "cancelOwner",
            "docs": [
              "admin amm order owner"
            ],
            "type": "publicKey"
          },
          {
            "name": "pending1",
            "docs": [
              "pending"
            ],
            "type": {
              "array": [
                "u64",
                28
              ]
            }
          },
          {
            "name": "pending2",
            "docs": [
              "pending"
            ],
            "type": {
              "array": [
                "u64",
                31
              ]
            }
          },
          {
            "name": "createPoolFee",
            "docs": [
              "init amm pool fee amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ammInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "status",
            "docs": [
              "Initialized status."
            ],
            "type": "u64"
          },
          {
            "name": "nonce",
            "docs": [
              "Nonce used in program address.",
              "The program address is created deterministically with the nonce,",
              "amm program id, and amm account pubkey.  This program address has",
              "authority over the amm's token coin account, token pc account, and pool",
              "token mint."
            ],
            "type": "u64"
          },
          {
            "name": "orderNum",
            "docs": [
              "max order count"
            ],
            "type": "u64"
          },
          {
            "name": "depth",
            "docs": [
              "within this range, 5 => 5% range"
            ],
            "type": "u64"
          },
          {
            "name": "coinDecimals",
            "docs": [
              "coin decimal"
            ],
            "type": "u64"
          },
          {
            "name": "pcDecimals",
            "docs": [
              "pc decimal"
            ],
            "type": "u64"
          },
          {
            "name": "state",
            "docs": [
              "amm machine state"
            ],
            "type": "u64"
          },
          {
            "name": "resetFlag",
            "docs": [
              "amm reset_flag"
            ],
            "type": "u64"
          },
          {
            "name": "minSize",
            "docs": [
              "min size 1->0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "volMaxCutRatio",
            "docs": [
              "vol_max_cut_ratio numerator, sys_decimal_value as denominator"
            ],
            "type": "u64"
          },
          {
            "name": "amountWave",
            "docs": [
              "amount wave numerator, sys_decimal_value as denominator"
            ],
            "type": "u64"
          },
          {
            "name": "coinLotSize",
            "docs": [
              "coinLotSize 1 -> 0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "pcLotSize",
            "docs": [
              "pcLotSize 1 -> 0.000001"
            ],
            "type": "u64"
          },
          {
            "name": "minPriceMultiplier",
            "docs": [
              "min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier"
            ],
            "type": "u64"
          },
          {
            "name": "maxPriceMultiplier",
            "docs": [
              "max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier"
            ],
            "type": "u64"
          },
          {
            "name": "sysDecimalValue",
            "docs": [
              "system decimal value, used to normalize the value of coin and pc amount"
            ],
            "type": "u64"
          },
          {
            "name": "fees",
            "docs": [
              "All fee information"
            ],
            "type": {
              "defined": "RaydiumFees"
            }
          },
          {
            "name": "stateData",
            "docs": [
              "Statistical data"
            ],
            "type": {
              "defined": "StateData"
            }
          },
          {
            "name": "coinVault",
            "docs": [
              "Coin vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "pcVault",
            "docs": [
              "Pc vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "coinVaultMint",
            "docs": [
              "Coin vault mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "pcVaultMint",
            "docs": [
              "Pc vault mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "lpMint",
            "docs": [
              "lp mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "openOrders",
            "docs": [
              "open_orders key"
            ],
            "type": "publicKey"
          },
          {
            "name": "market",
            "docs": [
              "market key"
            ],
            "type": "publicKey"
          },
          {
            "name": "marketProgram",
            "docs": [
              "market program key"
            ],
            "type": "publicKey"
          },
          {
            "name": "targetOrders",
            "docs": [
              "target_orders key"
            ],
            "type": "publicKey"
          },
          {
            "name": "padding1",
            "docs": [
              "padding"
            ],
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "ammOwner",
            "docs": [
              "amm owner key"
            ],
            "type": "publicKey"
          },
          {
            "name": "lpAmount",
            "docs": [
              "pool lp amount"
            ],
            "type": "u64"
          },
          {
            "name": "clientOrderId",
            "docs": [
              "client order id"
            ],
            "type": "u64"
          },
          {
            "name": "padding2",
            "docs": [
              "padding"
            ],
            "type": {
              "array": [
                "u64",
                2
              ]
            }
          }
        ]
      }
    },
    {
      "name": "targetOrders",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "buyOrders",
            "type": {
              "array": [
                {
                  "defined": "TargetOrder"
                },
                50
              ]
            }
          },
          {
            "name": "padding1",
            "type": {
              "array": [
                "u64",
                8
              ]
            }
          },
          {
            "name": "targetX",
            "type": "u128"
          },
          {
            "name": "targetY",
            "type": "u128"
          },
          {
            "name": "planXBuy",
            "type": "u128"
          },
          {
            "name": "planYBuy",
            "type": "u128"
          },
          {
            "name": "planXSell",
            "type": "u128"
          },
          {
            "name": "planYSell",
            "type": "u128"
          },
          {
            "name": "placedX",
            "type": "u128"
          },
          {
            "name": "placedY",
            "type": "u128"
          },
          {
            "name": "calcPnlX",
            "type": "u128"
          },
          {
            "name": "calcPnlY",
            "type": "u128"
          },
          {
            "name": "sellOrders",
            "type": {
              "array": [
                {
                  "defined": "TargetOrder"
                },
                50
              ]
            }
          },
          {
            "name": "padding2",
            "type": {
              "array": [
                "u64",
                6
              ]
            }
          },
          {
            "name": "replaceBuyClientId",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "replaceSellClientId",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "lastOrderNumerator",
            "type": "u64"
          },
          {
            "name": "lastOrderDenominator",
            "type": "u64"
          },
          {
            "name": "planOrdersCur",
            "type": "u64"
          },
          {
            "name": "placeOrdersCur",
            "type": "u64"
          },
          {
            "name": "validBuyOrderNum",
            "type": "u64"
          },
          {
            "name": "validSellOrderNum",
            "type": "u64"
          },
          {
            "name": "padding3",
            "type": {
              "array": [
                "u64",
                10
              ]
            }
          },
          {
            "name": "freeSlotBits",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "openOrders",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "accountFlags",
            "type": "u64"
          },
          {
            "name": "market",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "owner",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "nativeCoinFree",
            "type": "u64"
          },
          {
            "name": "nativeCoinTotal",
            "type": "u64"
          },
          {
            "name": "nativePcFree",
            "type": "u64"
          },
          {
            "name": "nativePcTotal",
            "type": "u64"
          },
          {
            "name": "freeSlotBits",
            "type": "u128"
          },
          {
            "name": "isBidBits",
            "type": "u128"
          },
          {
            "name": "orders",
            "type": {
              "array": [
                "u128",
                128
              ]
            }
          },
          {
            "name": "clientOrderIds",
            "type": {
              "array": [
                "u64",
                128
              ]
            }
          },
          {
            "name": "referrerRebatesAccrued",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "marketState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "accountFlags",
            "type": "u64"
          },
          {
            "name": "ownAddress",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "vaultSignerNonce",
            "type": "u64"
          },
          {
            "name": "coinMint",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "pcMint",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinVault",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinDepositsTotal",
            "type": "u64"
          },
          {
            "name": "coinFeesAccrued",
            "type": "u64"
          },
          {
            "name": "pcVault",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "pcDepositsTotal",
            "type": "u64"
          },
          {
            "name": "pcFeesAccrued",
            "type": "u64"
          },
          {
            "name": "pcDustThreshold",
            "type": "u64"
          },
          {
            "name": "reqQ",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "eventQ",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "bids",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "asks",
            "type": {
              "array": [
                "u64",
                4
              ]
            }
          },
          {
            "name": "coinLotSize",
            "type": "u64"
          },
          {
            "name": "pcLotSize",
            "type": "u64"
          },
          {
            "name": "feeRateBps",
            "type": "u64"
          },
          {
            "name": "referrerRebatesAccrued",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "Config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "alphaAbs",
            "type": "u128"
          },
          {
            "name": "beta",
            "type": "u128"
          },
          {
            "name": "priceFactor",
            "type": "u64"
          },
          {
            "name": "gammaS",
            "type": "u64"
          },
          {
            "name": "gammaM",
            "type": "u64"
          },
          {
            "name": "omegaM",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Fees",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeInPercent",
            "type": "u64"
          },
          {
            "name": "feeOutPercent",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TokenLimit",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "tokens",
            "type": {
              "defined": "TokenAmount"
            }
          }
        ]
      }
    },
    {
      "name": "TokenAmount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "Reserve",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokens",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "vault",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "RaydiumFees",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "minSeparateNumerator",
            "docs": [
              "numerator of the min_separate"
            ],
            "type": "u64"
          },
          {
            "name": "minSeparateDenominator",
            "docs": [
              "denominator of the min_separate"
            ],
            "type": "u64"
          },
          {
            "name": "tradeFeeNumerator",
            "docs": [
              "numerator of the fee"
            ],
            "type": "u64"
          },
          {
            "name": "tradeFeeDenominator",
            "docs": [
              "denominator of the fee",
              "and 'trade_fee_denominator' must be equal to 'min_separate_denominator'"
            ],
            "type": "u64"
          },
          {
            "name": "pnlNumerator",
            "docs": [
              "numerator of the pnl"
            ],
            "type": "u64"
          },
          {
            "name": "pnlDenominator",
            "docs": [
              "denominator of the pnl"
            ],
            "type": "u64"
          },
          {
            "name": "swapFeeNumerator",
            "docs": [
              "numerator of the swap_fee"
            ],
            "type": "u64"
          },
          {
            "name": "swapFeeDenominator",
            "docs": [
              "denominator of the swap_fee"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StateData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "needTakePnlCoin",
            "docs": [
              "delay to take pnl coin"
            ],
            "type": "u64"
          },
          {
            "name": "needTakePnlPc",
            "docs": [
              "delay to take pnl pc"
            ],
            "type": "u64"
          },
          {
            "name": "totalPnlPc",
            "docs": [
              "total pnl pc"
            ],
            "type": "u64"
          },
          {
            "name": "totalPnlCoin",
            "docs": [
              "total pnl coin"
            ],
            "type": "u64"
          },
          {
            "name": "poolOpenTime",
            "docs": [
              "ido pool open time"
            ],
            "type": "u64"
          },
          {
            "name": "padding",
            "docs": [
              "padding for future updates"
            ],
            "type": {
              "array": [
                "u64",
                2
              ]
            }
          },
          {
            "name": "orderbookToInitTime",
            "docs": [
              "switch from orderbookonly to init"
            ],
            "type": "u64"
          },
          {
            "name": "swapCoinInAmount",
            "docs": [
              "swap coin in amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapPcOutAmount",
            "docs": [
              "swap pc out amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapAccPcFee",
            "docs": [
              "charge pc as swap fee while swap pc to coin"
            ],
            "type": "u64"
          },
          {
            "name": "swapPcInAmount",
            "docs": [
              "swap pc in amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapCoinOutAmount",
            "docs": [
              "swap coin out amount"
            ],
            "type": "u128"
          },
          {
            "name": "swapAccCoinFee",
            "docs": [
              "charge coin as swap fee while swap coin to pc"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TargetOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "vol",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "VestingConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "cliffTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "VestingData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "released",
            "type": "u64"
          },
          {
            "name": "notional",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAccountInput",
      "msg": "Provided account breaks some constraints, see logs for more info"
    },
    {
      "code": 6001,
      "name": "InvalidArg",
      "msg": "One of the provided input arguments is invalid"
    },
    {
      "code": 6002,
      "name": "SlippageExceeded",
      "msg": "Given amount of tokens to swap would result in \\\n        less than minimum requested tokens to receive"
    },
    {
      "code": 6003,
      "name": "InvariantViolation",
      "msg": "There's a bug in the program, see logs for more info"
    },
    {
      "code": 6004,
      "name": "InvalidTokenMints",
      "msg": "Provided mints are not available on the pool"
    },
    {
      "code": 6005,
      "name": "MulDivOverflow"
    },
    {
      "code": 6006,
      "name": "DivideByZero"
    },
    {
      "code": 6007,
      "name": "ZeroInAmt"
    },
    {
      "code": 6008,
      "name": "ZeroMemeVault"
    },
    {
      "code": 6009,
      "name": "InsufficientBalance"
    },
    {
      "code": 6010,
      "name": "PoolIsLocked",
      "msg": "Pool can't be interacted with until going into live phase"
    },
    {
      "code": 6011,
      "name": "NoZeroTokens",
      "msg": "Shouldn't provide zero tokens in"
    },
    {
      "code": 6012,
      "name": "NoTokensToWithdraw"
    },
    {
      "code": 6013,
      "name": "NotEnoughTicketTokens",
      "msg": "Amount of tokens in ticket is lower than needed to swap"
    },
    {
      "code": 6014,
      "name": "TicketTokensLocked",
      "msg": "Not enough time passed to unlock tokens bound to the ticket"
    },
    {
      "code": 6015,
      "name": "NonZeroAmountTicket",
      "msg": "Can't close ticket with non-zero bound token amount"
    },
    {
      "code": 6016,
      "name": "NotEnoughTokensToRelease",
      "msg": "Can't unstake the required amount of tokens"
    },
    {
      "code": 6017,
      "name": "BondingCurveMustBeNegativelySloped"
    },
    {
      "code": 6018,
      "name": "BondingCurveInterceptMustBePositive"
    }
  ]
};
