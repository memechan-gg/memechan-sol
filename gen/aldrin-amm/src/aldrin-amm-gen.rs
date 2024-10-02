#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use solana_program::declare_id;
use anchor_lang::prelude::*;
pub mod typedefs {
    //! User-defined types.
    use super::*;
    pub struct Fees {
        pub trade_fee_numerator: u64,
        pub trade_fee_denominator: u64,
        pub owner_trade_fee_numerator: u64,
        pub owner_trade_fee_denominator: u64,
        pub owner_withdraw_fee_numerator: u64,
        pub owner_withdraw_fee_denominator: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for Fees {
        #[inline]
        fn default() -> Fees {
            Fees {
                trade_fee_numerator: ::core::default::Default::default(),
                trade_fee_denominator: ::core::default::Default::default(),
                owner_trade_fee_numerator: ::core::default::Default::default(),
                owner_trade_fee_denominator: ::core::default::Default::default(),
                owner_withdraw_fee_numerator: ::core::default::Default::default(),
                owner_withdraw_fee_denominator: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Fees {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "trade_fee_numerator",
                "trade_fee_denominator",
                "owner_trade_fee_numerator",
                "owner_trade_fee_denominator",
                "owner_withdraw_fee_numerator",
                "owner_withdraw_fee_denominator",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.trade_fee_numerator,
                &self.trade_fee_denominator,
                &self.owner_trade_fee_numerator,
                &self.owner_trade_fee_denominator,
                &self.owner_withdraw_fee_numerator,
                &&self.owner_withdraw_fee_denominator,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Fees", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Fees {}
    impl borsh::ser::BorshSerialize for Fees
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.trade_fee_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.trade_fee_denominator, writer)?;
            borsh::BorshSerialize::serialize(&self.owner_trade_fee_numerator, writer)?;
            borsh::BorshSerialize::serialize(&self.owner_trade_fee_denominator, writer)?;
            borsh::BorshSerialize::serialize(
                &self.owner_withdraw_fee_numerator,
                writer,
            )?;
            borsh::BorshSerialize::serialize(
                &self.owner_withdraw_fee_denominator,
                writer,
            )?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Fees
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                trade_fee_numerator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                trade_fee_denominator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                owner_trade_fee_numerator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                owner_trade_fee_denominator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                owner_withdraw_fee_numerator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                owner_withdraw_fee_denominator: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Fees {
        #[inline]
        fn clone(&self) -> Fees {
            Fees {
                trade_fee_numerator: ::core::clone::Clone::clone(
                    &self.trade_fee_numerator,
                ),
                trade_fee_denominator: ::core::clone::Clone::clone(
                    &self.trade_fee_denominator,
                ),
                owner_trade_fee_numerator: ::core::clone::Clone::clone(
                    &self.owner_trade_fee_numerator,
                ),
                owner_trade_fee_denominator: ::core::clone::Clone::clone(
                    &self.owner_trade_fee_denominator,
                ),
                owner_withdraw_fee_numerator: ::core::clone::Clone::clone(
                    &self.owner_withdraw_fee_numerator,
                ),
                owner_withdraw_fee_denominator: ::core::clone::Clone::clone(
                    &self.owner_withdraw_fee_denominator,
                ),
            }
        }
    }
    pub struct AttachedFarmingState {
        pub farming_state: Pubkey,
        pub last_withdraw_time: i64,
        pub last_vested_withdraw_time: i64,
    }
    #[automatically_derived]
    impl ::core::default::Default for AttachedFarmingState {
        #[inline]
        fn default() -> AttachedFarmingState {
            AttachedFarmingState {
                farming_state: ::core::default::Default::default(),
                last_withdraw_time: ::core::default::Default::default(),
                last_vested_withdraw_time: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AttachedFarmingState {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "AttachedFarmingState",
                "farming_state",
                &self.farming_state,
                "last_withdraw_time",
                &self.last_withdraw_time,
                "last_vested_withdraw_time",
                &&self.last_vested_withdraw_time,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AttachedFarmingState {}
    impl borsh::ser::BorshSerialize for AttachedFarmingState
    where
        Pubkey: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
            borsh::BorshSerialize::serialize(&self.last_withdraw_time, writer)?;
            borsh::BorshSerialize::serialize(&self.last_vested_withdraw_time, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AttachedFarmingState
    where
        Pubkey: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                farming_state: borsh::BorshDeserialize::deserialize_reader(reader)?,
                last_withdraw_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                last_vested_withdraw_time: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AttachedFarmingState {
        #[inline]
        fn clone(&self) -> AttachedFarmingState {
            AttachedFarmingState {
                farming_state: ::core::clone::Clone::clone(&self.farming_state),
                last_withdraw_time: ::core::clone::Clone::clone(
                    &self.last_withdraw_time,
                ),
                last_vested_withdraw_time: ::core::clone::Clone::clone(
                    &self.last_vested_withdraw_time,
                ),
            }
        }
    }
    pub struct Snapshot {
        pub is_initialized: bool,
        pub tokens_frozen: u64,
        pub farming_tokens: u64,
        pub time: i64,
    }
    #[automatically_derived]
    impl ::core::default::Default for Snapshot {
        #[inline]
        fn default() -> Snapshot {
            Snapshot {
                is_initialized: ::core::default::Default::default(),
                tokens_frozen: ::core::default::Default::default(),
                farming_tokens: ::core::default::Default::default(),
                time: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Snapshot {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Snapshot",
                "is_initialized",
                &self.is_initialized,
                "tokens_frozen",
                &self.tokens_frozen,
                "farming_tokens",
                &self.farming_tokens,
                "time",
                &&self.time,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Snapshot {}
    impl borsh::ser::BorshSerialize for Snapshot
    where
        bool: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.is_initialized, writer)?;
            borsh::BorshSerialize::serialize(&self.tokens_frozen, writer)?;
            borsh::BorshSerialize::serialize(&self.farming_tokens, writer)?;
            borsh::BorshSerialize::serialize(&self.time, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Snapshot
    where
        bool: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                is_initialized: borsh::BorshDeserialize::deserialize_reader(reader)?,
                tokens_frozen: borsh::BorshDeserialize::deserialize_reader(reader)?,
                farming_tokens: borsh::BorshDeserialize::deserialize_reader(reader)?,
                time: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Snapshot {
        #[inline]
        fn clone(&self) -> Snapshot {
            Snapshot {
                is_initialized: ::core::clone::Clone::clone(&self.is_initialized),
                tokens_frozen: ::core::clone::Clone::clone(&self.tokens_frozen),
                farming_tokens: ::core::clone::Clone::clone(&self.farming_tokens),
                time: ::core::clone::Clone::clone(&self.time),
            }
        }
    }
    pub enum Side {
        Bid,
        Ask,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Side {}
    impl borsh::ser::BorshSerialize for Side {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 = match self {
                Side::Bid => 0u8,
                Side::Ask => 1u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                Side::Bid => {}
                Side::Ask => {}
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Side {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for Side {
        fn deserialize_variant<R: borsh::maybestd::io::Read>(
            reader: &mut R,
            variant_idx: u8,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let mut return_value = match variant_idx {
                0u8 => Side::Bid,
                1u8 => Side::Ask,
                _ => {
                    return Err(
                        borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Unexpected variant index: {0:?}", variant_idx),
                                );
                                res
                            },
                        ),
                    );
                }
            };
            Ok(return_value)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Side {
        #[inline]
        fn clone(&self) -> Side {
            match self {
                Side::Bid => Side::Bid,
                Side::Ask => Side::Ask,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Side {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Side::Bid => "Bid",
                    Side::Ask => "Ask",
                },
            )
        }
    }
    impl Default for Side {
        fn default() -> Self {
            Self::Bid
        }
    }
    pub enum TradeDirection {
        AtoB,
        BtoA,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TradeDirection {}
    impl borsh::ser::BorshSerialize for TradeDirection {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            let variant_idx: u8 = match self {
                TradeDirection::AtoB => 0u8,
                TradeDirection::BtoA => 1u8,
            };
            writer.write_all(&variant_idx.to_le_bytes())?;
            match self {
                TradeDirection::AtoB => {}
                TradeDirection::BtoA => {}
            }
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TradeDirection {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(reader)?;
            <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
        }
    }
    impl borsh::de::EnumExt for TradeDirection {
        fn deserialize_variant<R: borsh::maybestd::io::Read>(
            reader: &mut R,
            variant_idx: u8,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            let mut return_value = match variant_idx {
                0u8 => TradeDirection::AtoB,
                1u8 => TradeDirection::BtoA,
                _ => {
                    return Err(
                        borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("Unexpected variant index: {0:?}", variant_idx),
                                );
                                res
                            },
                        ),
                    );
                }
            };
            Ok(return_value)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TradeDirection {
        #[inline]
        fn clone(&self) -> TradeDirection {
            match self {
                TradeDirection::AtoB => TradeDirection::AtoB,
                TradeDirection::BtoA => TradeDirection::BtoA,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TradeDirection {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TradeDirection::AtoB => "AtoB",
                    TradeDirection::BtoA => "BtoA",
                },
            )
        }
    }
    impl Default for TradeDirection {
        fn default() -> Self {
            Self::AtoB
        }
    }
}
pub mod state {
    //! Structs of accounts which hold state.
    use super::*;
    /// Account: Pool
    pub struct Pool {
        pub lp_token_freeze_vault: Pubkey,
        pub pool_mint: Pubkey,
        pub base_token_vault: Pubkey,
        pub base_token_mint: Pubkey,
        pub quote_token_vault: Pubkey,
        pub quote_token_mint: Pubkey,
        pub pool_signer: Pubkey,
        pub pool_signer_nonce: u8,
        pub authority: Pubkey,
        pub initializer_account: Pubkey,
        pub fee_base_account: Pubkey,
        pub fee_quote_account: Pubkey,
        pub fee_pool_token_account: Pubkey,
        pub fees: Fees,
        pub curve_type: u8,
        pub curve: Pubkey,
    }
    #[automatically_derived]
    impl ::core::default::Default for Pool {
        #[inline]
        fn default() -> Pool {
            Pool {
                lp_token_freeze_vault: ::core::default::Default::default(),
                pool_mint: ::core::default::Default::default(),
                base_token_vault: ::core::default::Default::default(),
                base_token_mint: ::core::default::Default::default(),
                quote_token_vault: ::core::default::Default::default(),
                quote_token_mint: ::core::default::Default::default(),
                pool_signer: ::core::default::Default::default(),
                pool_signer_nonce: ::core::default::Default::default(),
                authority: ::core::default::Default::default(),
                initializer_account: ::core::default::Default::default(),
                fee_base_account: ::core::default::Default::default(),
                fee_quote_account: ::core::default::Default::default(),
                fee_pool_token_account: ::core::default::Default::default(),
                fees: ::core::default::Default::default(),
                curve_type: ::core::default::Default::default(),
                curve: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Pool {}
    impl borsh::ser::BorshSerialize for Pool
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Fees: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.lp_token_freeze_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.base_token_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.quote_token_mint, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
            borsh::BorshSerialize::serialize(&self.pool_signer_nonce, writer)?;
            borsh::BorshSerialize::serialize(&self.authority, writer)?;
            borsh::BorshSerialize::serialize(&self.initializer_account, writer)?;
            borsh::BorshSerialize::serialize(&self.fee_base_account, writer)?;
            borsh::BorshSerialize::serialize(&self.fee_quote_account, writer)?;
            borsh::BorshSerialize::serialize(&self.fee_pool_token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.fees, writer)?;
            borsh::BorshSerialize::serialize(&self.curve_type, writer)?;
            borsh::BorshSerialize::serialize(&self.curve, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Pool
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Fees: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                lp_token_freeze_vault: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                pool_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                base_token_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                base_token_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                quote_token_vault: borsh::BorshDeserialize::deserialize_reader(reader)?,
                quote_token_mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                pool_signer: borsh::BorshDeserialize::deserialize_reader(reader)?,
                pool_signer_nonce: borsh::BorshDeserialize::deserialize_reader(reader)?,
                authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initializer_account: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                fee_base_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_quote_account: borsh::BorshDeserialize::deserialize_reader(reader)?,
                fee_pool_token_account: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                fees: borsh::BorshDeserialize::deserialize_reader(reader)?,
                curve_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
                curve: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Pool {
        #[inline]
        fn clone(&self) -> Pool {
            Pool {
                lp_token_freeze_vault: ::core::clone::Clone::clone(
                    &self.lp_token_freeze_vault,
                ),
                pool_mint: ::core::clone::Clone::clone(&self.pool_mint),
                base_token_vault: ::core::clone::Clone::clone(&self.base_token_vault),
                base_token_mint: ::core::clone::Clone::clone(&self.base_token_mint),
                quote_token_vault: ::core::clone::Clone::clone(&self.quote_token_vault),
                quote_token_mint: ::core::clone::Clone::clone(&self.quote_token_mint),
                pool_signer: ::core::clone::Clone::clone(&self.pool_signer),
                pool_signer_nonce: ::core::clone::Clone::clone(&self.pool_signer_nonce),
                authority: ::core::clone::Clone::clone(&self.authority),
                initializer_account: ::core::clone::Clone::clone(
                    &self.initializer_account,
                ),
                fee_base_account: ::core::clone::Clone::clone(&self.fee_base_account),
                fee_quote_account: ::core::clone::Clone::clone(&self.fee_quote_account),
                fee_pool_token_account: ::core::clone::Clone::clone(
                    &self.fee_pool_token_account,
                ),
                fees: ::core::clone::Clone::clone(&self.fees),
                curve_type: ::core::clone::Clone::clone(&self.curve_type),
                curve: ::core::clone::Clone::clone(&self.curve),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Pool {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[241, 154, 109, 4, 17, 177, 109, 188]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Pool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [241, 154, 109, 4, 17, 177, 109, 188].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[241, 154, 109, 4, 17, 177, 109, 188] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("Pool"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Pool {
        const DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for Pool {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: ProductCurve
    pub struct ProductCurve {
        pub no_value: u8,
    }
    #[automatically_derived]
    impl ::core::default::Default for ProductCurve {
        #[inline]
        fn default() -> ProductCurve {
            ProductCurve {
                no_value: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ProductCurve {}
    impl borsh::ser::BorshSerialize for ProductCurve
    where
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.no_value, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ProductCurve
    where
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                no_value: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProductCurve {
        #[inline]
        fn clone(&self) -> ProductCurve {
            ProductCurve {
                no_value: ::core::clone::Clone::clone(&self.no_value),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ProductCurve {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[233, 17, 103, 168, 109, 121, 94, 133]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ProductCurve {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [233, 17, 103, 168, 109, 121, 94, 133].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[233, 17, 103, 168, 109, 121, 94, 133] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("ProductCurve"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ProductCurve {
        const DISCRIMINATOR: [u8; 8] = [233, 17, 103, 168, 109, 121, 94, 133];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for ProductCurve {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: StableCurve
    pub struct StableCurve {
        pub amp: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for StableCurve {
        #[inline]
        fn default() -> StableCurve {
            StableCurve {
                amp: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StableCurve {}
    impl borsh::ser::BorshSerialize for StableCurve
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.amp, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for StableCurve
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                amp: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StableCurve {
        #[inline]
        fn clone(&self) -> StableCurve {
            StableCurve {
                amp: ::core::clone::Clone::clone(&self.amp),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for StableCurve {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[160, 34, 225, 172, 72, 171, 72, 146]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for StableCurve {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [160, 34, 225, 172, 72, 171, 72, 146].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[160, 34, 225, 172, 72, 171, 72, 146] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("StableCurve"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for StableCurve {
        const DISCRIMINATOR: [u8; 8] = [160, 34, 225, 172, 72, 171, 72, 146];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for StableCurve {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: FarmingState
    pub struct FarmingState {
        pub tokens_unlocked: u64,
        pub tokens_per_period: u64,
        pub tokens_total: u64,
        pub period_length: u64,
        pub no_withdrawal_time: i64,
        pub vesting_type: u8,
        pub vesting_period: i64,
        pub start_time: i64,
        pub current_time: i64,
        pub pool: Pubkey,
        pub farming_token_vault: Pubkey,
        pub farming_snapshots: Pubkey,
    }
    #[automatically_derived]
    impl ::core::default::Default for FarmingState {
        #[inline]
        fn default() -> FarmingState {
            FarmingState {
                tokens_unlocked: ::core::default::Default::default(),
                tokens_per_period: ::core::default::Default::default(),
                tokens_total: ::core::default::Default::default(),
                period_length: ::core::default::Default::default(),
                no_withdrawal_time: ::core::default::Default::default(),
                vesting_type: ::core::default::Default::default(),
                vesting_period: ::core::default::Default::default(),
                start_time: ::core::default::Default::default(),
                current_time: ::core::default::Default::default(),
                pool: ::core::default::Default::default(),
                farming_token_vault: ::core::default::Default::default(),
                farming_snapshots: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FarmingState {}
    impl borsh::ser::BorshSerialize for FarmingState
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.tokens_unlocked, writer)?;
            borsh::BorshSerialize::serialize(&self.tokens_per_period, writer)?;
            borsh::BorshSerialize::serialize(&self.tokens_total, writer)?;
            borsh::BorshSerialize::serialize(&self.period_length, writer)?;
            borsh::BorshSerialize::serialize(&self.no_withdrawal_time, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_type, writer)?;
            borsh::BorshSerialize::serialize(&self.vesting_period, writer)?;
            borsh::BorshSerialize::serialize(&self.start_time, writer)?;
            borsh::BorshSerialize::serialize(&self.current_time, writer)?;
            borsh::BorshSerialize::serialize(&self.pool, writer)?;
            borsh::BorshSerialize::serialize(&self.farming_token_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.farming_snapshots, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FarmingState
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                tokens_unlocked: borsh::BorshDeserialize::deserialize_reader(reader)?,
                tokens_per_period: borsh::BorshDeserialize::deserialize_reader(reader)?,
                tokens_total: borsh::BorshDeserialize::deserialize_reader(reader)?,
                period_length: borsh::BorshDeserialize::deserialize_reader(reader)?,
                no_withdrawal_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                vesting_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
                vesting_period: borsh::BorshDeserialize::deserialize_reader(reader)?,
                start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                current_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                pool: borsh::BorshDeserialize::deserialize_reader(reader)?,
                farming_token_vault: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                farming_snapshots: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FarmingState {
        #[inline]
        fn clone(&self) -> FarmingState {
            FarmingState {
                tokens_unlocked: ::core::clone::Clone::clone(&self.tokens_unlocked),
                tokens_per_period: ::core::clone::Clone::clone(&self.tokens_per_period),
                tokens_total: ::core::clone::Clone::clone(&self.tokens_total),
                period_length: ::core::clone::Clone::clone(&self.period_length),
                no_withdrawal_time: ::core::clone::Clone::clone(
                    &self.no_withdrawal_time,
                ),
                vesting_type: ::core::clone::Clone::clone(&self.vesting_type),
                vesting_period: ::core::clone::Clone::clone(&self.vesting_period),
                start_time: ::core::clone::Clone::clone(&self.start_time),
                current_time: ::core::clone::Clone::clone(&self.current_time),
                pool: ::core::clone::Clone::clone(&self.pool),
                farming_token_vault: ::core::clone::Clone::clone(
                    &self.farming_token_vault,
                ),
                farming_snapshots: ::core::clone::Clone::clone(&self.farming_snapshots),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FarmingState {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[132, 243, 130, 58, 15, 125, 164, 123]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FarmingState {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [132, 243, 130, 58, 15, 125, 164, 123].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[132, 243, 130, 58, 15, 125, 164, 123] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("FarmingState"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FarmingState {
        const DISCRIMINATOR: [u8; 8] = [132, 243, 130, 58, 15, 125, 164, 123];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for FarmingState {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: FarmingTicket
    pub struct FarmingTicket {
        pub tokens_frozen: u64,
        pub start_time: i64,
        pub end_time: i64,
        pub user_key: Pubkey,
        pub pool: Pubkey,
        pub next_attached: u64,
        pub states_attached: [AttachedFarmingState; 10],
    }
    #[automatically_derived]
    impl ::core::default::Default for FarmingTicket {
        #[inline]
        fn default() -> FarmingTicket {
            FarmingTicket {
                tokens_frozen: ::core::default::Default::default(),
                start_time: ::core::default::Default::default(),
                end_time: ::core::default::Default::default(),
                user_key: ::core::default::Default::default(),
                pool: ::core::default::Default::default(),
                next_attached: ::core::default::Default::default(),
                states_attached: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FarmingTicket {}
    impl borsh::ser::BorshSerialize for FarmingTicket
    where
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        [AttachedFarmingState; 10]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.tokens_frozen, writer)?;
            borsh::BorshSerialize::serialize(&self.start_time, writer)?;
            borsh::BorshSerialize::serialize(&self.end_time, writer)?;
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.pool, writer)?;
            borsh::BorshSerialize::serialize(&self.next_attached, writer)?;
            borsh::BorshSerialize::serialize(&self.states_attached, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FarmingTicket
    where
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        [AttachedFarmingState; 10]: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                tokens_frozen: borsh::BorshDeserialize::deserialize_reader(reader)?,
                start_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                end_time: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                pool: borsh::BorshDeserialize::deserialize_reader(reader)?,
                next_attached: borsh::BorshDeserialize::deserialize_reader(reader)?,
                states_attached: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FarmingTicket {
        #[inline]
        fn clone(&self) -> FarmingTicket {
            FarmingTicket {
                tokens_frozen: ::core::clone::Clone::clone(&self.tokens_frozen),
                start_time: ::core::clone::Clone::clone(&self.start_time),
                end_time: ::core::clone::Clone::clone(&self.end_time),
                user_key: ::core::clone::Clone::clone(&self.user_key),
                pool: ::core::clone::Clone::clone(&self.pool),
                next_attached: ::core::clone::Clone::clone(&self.next_attached),
                states_attached: ::core::clone::Clone::clone(&self.states_attached),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FarmingTicket {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[190, 26, 211, 184, 8, 108, 35, 231]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FarmingTicket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [190, 26, 211, 184, 8, 108, 35, 231].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[190, 26, 211, 184, 8, 108, 35, 231] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("FarmingTicket"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FarmingTicket {
        const DISCRIMINATOR: [u8; 8] = [190, 26, 211, 184, 8, 108, 35, 231];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for FarmingTicket {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: SnapshotQueue
    pub struct SnapshotQueue {
        pub next_index: u64,
        pub snapshots: [Snapshot; 1500],
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SnapshotQueue {}
    impl borsh::ser::BorshSerialize for SnapshotQueue
    where
        u64: borsh::ser::BorshSerialize,
        [Snapshot; 1500]: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.next_index, writer)?;
            borsh::BorshSerialize::serialize(&self.snapshots, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SnapshotQueue
    where
        u64: borsh::BorshDeserialize,
        [Snapshot; 1500]: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                next_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                snapshots: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SnapshotQueue {
        #[inline]
        fn clone(&self) -> SnapshotQueue {
            SnapshotQueue {
                next_index: ::core::clone::Clone::clone(&self.next_index),
                snapshots: ::core::clone::Clone::clone(&self.snapshots),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SnapshotQueue {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[43, 17, 98, 203, 103, 179, 122, 134]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SnapshotQueue {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [43, 17, 98, 203, 103, 179, 122, 134].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[43, 17, 98, 203, 103, 179, 122, 134] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("SnapshotQueue"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SnapshotQueue {
        const DISCRIMINATOR: [u8; 8] = [43, 17, 98, 203, 103, 179, 122, 134];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for SnapshotQueue {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    /// Account: FarmingCalc
    pub struct FarmingCalc {
        pub farming_state: Pubkey,
        pub user_key: Pubkey,
        pub initializer: Pubkey,
        pub token_amount: u64,
    }
    #[automatically_derived]
    impl ::core::default::Default for FarmingCalc {
        #[inline]
        fn default() -> FarmingCalc {
            FarmingCalc {
                farming_state: ::core::default::Default::default(),
                user_key: ::core::default::Default::default(),
                initializer: ::core::default::Default::default(),
                token_amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FarmingCalc {}
    impl borsh::ser::BorshSerialize for FarmingCalc
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
            borsh::BorshSerialize::serialize(&self.user_key, writer)?;
            borsh::BorshSerialize::serialize(&self.initializer, writer)?;
            borsh::BorshSerialize::serialize(&self.token_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for FarmingCalc
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                farming_state: borsh::BorshDeserialize::deserialize_reader(reader)?,
                user_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                initializer: borsh::BorshDeserialize::deserialize_reader(reader)?,
                token_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FarmingCalc {
        #[inline]
        fn clone(&self) -> FarmingCalc {
            FarmingCalc {
                farming_state: ::core::clone::Clone::clone(&self.farming_state),
                user_key: ::core::clone::Clone::clone(&self.user_key),
                initializer: ::core::clone::Clone::clone(&self.initializer),
                token_amount: ::core::clone::Clone::clone(&self.token_amount),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FarmingCalc {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[144, 111, 215, 93, 70, 59, 32, 196]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FarmingCalc {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [144, 111, 215, 93, 70, 59, 32, 196].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[144, 111, 215, 93, 70, 59, 32, 196] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("FarmingCalc"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FarmingCalc {
        const DISCRIMINATOR: [u8; 8] = [144, 111, 215, 93, 70, 59, 32, 196];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for FarmingCalc {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
}
pub mod ix_accounts {
    //! Accounts used in instructions.
    use super::*;
    pub struct InitializeConstProductCurve<'info> {
        #[account(mut)]
        pub curve: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, InitializeConstProductCurveBumps>
    for InitializeConstProductCurve<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut InitializeConstProductCurveBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let curve: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("curve"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&curve.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("curve"),
                );
            }
            Ok(InitializeConstProductCurve {
                curve,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeConstProductCurve<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.curve.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeConstProductCurve<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.curve.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeConstProductCurve<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.curve, program_id)
                .map_err(|e| e.with_account_name("curve"))?;
            Ok(())
        }
    }
    pub struct InitializeConstProductCurveBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InitializeConstProductCurveBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InitializeConstProductCurveBumps")
        }
    }
    impl Default for InitializeConstProductCurveBumps {
        fn default() -> Self {
            InitializeConstProductCurveBumps {
            }
        }
    }
    impl<'info> anchor_lang::Bumps for InitializeConstProductCurve<'info>
    where
        'info: 'info,
    {
        type Bumps = InitializeConstProductCurveBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize_const_product_curve {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`InitializeConstProductCurve`].
        pub struct InitializeConstProductCurve {
            pub curve: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeConstProductCurve
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.curve, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeConstProductCurve {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.curve,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize_const_product_curve {
        use super::*;
        /// Generated CPI struct of the accounts for [`InitializeConstProductCurve`].
        pub struct InitializeConstProductCurve<'info> {
            pub curve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeConstProductCurve<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.curve),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info>
        for InitializeConstProductCurve<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.curve));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct InitializeStableCurve<'info> {
        #[account(mut)]
        pub curve: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, InitializeStableCurveBumps>
    for InitializeStableCurve<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut InitializeStableCurveBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let curve: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("curve"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&curve.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("curve"),
                );
            }
            Ok(InitializeStableCurve {
                curve,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeStableCurve<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.curve.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeStableCurve<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.curve.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeStableCurve<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.curve, program_id)
                .map_err(|e| e.with_account_name("curve"))?;
            Ok(())
        }
    }
    pub struct InitializeStableCurveBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InitializeStableCurveBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InitializeStableCurveBumps")
        }
    }
    impl Default for InitializeStableCurveBumps {
        fn default() -> Self {
            InitializeStableCurveBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for InitializeStableCurve<'info>
    where
        'info: 'info,
    {
        type Bumps = InitializeStableCurveBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize_stable_curve {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`InitializeStableCurve`].
        pub struct InitializeStableCurve {
            pub curve: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeStableCurve
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.curve, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeStableCurve {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.curve,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize_stable_curve {
        use super::*;
        /// Generated CPI struct of the accounts for [`InitializeStableCurve`].
        pub struct InitializeStableCurve<'info> {
            pub curve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeStableCurve<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.curve),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeStableCurve<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.curve));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct Initialize<'info> {
        #[account(mut)]
        pub pool: AccountInfo<'info>,
        pub pool_mint: AccountInfo<'info>,
        pub lp_token_freeze_vault: AccountInfo<'info>,
        pub base_token_vault: AccountInfo<'info>,
        pub quote_token_vault: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        pub initializer: Signer<'info>,
        pub pool_authority: AccountInfo<'info>,
        pub fee_base_account: AccountInfo<'info>,
        pub fee_quote_account: AccountInfo<'info>,
        pub fee_pool_token_account: AccountInfo<'info>,
        pub curve: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut InitializeBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let lp_token_freeze_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let initializer: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("initializer"))?;
            let pool_authority: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_authority"))?;
            let fee_base_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_base_account"))?;
            let fee_quote_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_quote_account"))?;
            let fee_pool_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_pool_token_account"))?;
            let curve: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("curve"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&pool.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("pool"),
                );
            }
            Ok(Initialize {
                pool,
                pool_mint,
                lp_token_freeze_vault,
                base_token_vault,
                quote_token_vault,
                pool_signer,
                initializer,
                pool_authority,
                fee_base_account,
                fee_quote_account,
                fee_pool_token_account,
                curve,
                token_program,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.lp_token_freeze_vault.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.initializer.to_account_infos());
            account_infos.extend(self.pool_authority.to_account_infos());
            account_infos.extend(self.fee_base_account.to_account_infos());
            account_infos.extend(self.fee_quote_account.to_account_infos());
            account_infos.extend(self.fee_pool_token_account.to_account_infos());
            account_infos.extend(self.curve.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.lp_token_freeze_vault.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.initializer.to_account_metas(None));
            account_metas.extend(self.pool_authority.to_account_metas(None));
            account_metas.extend(self.fee_base_account.to_account_metas(None));
            account_metas.extend(self.fee_quote_account.to_account_metas(None));
            account_metas.extend(self.fee_pool_token_account.to_account_metas(None));
            account_metas.extend(self.curve.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Initialize<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.pool, program_id)
                .map_err(|e| e.with_account_name("pool"))?;
            Ok(())
        }
    }
    pub struct InitializeBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InitializeBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InitializeBumps")
        }
    }
    impl Default for InitializeBumps {
        fn default() -> Self {
            InitializeBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for Initialize<'info>
    where
        'info: 'info,
    {
        type Bumps = InitializeBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`Initialize`].
        pub struct Initialize {
            pub pool: Pubkey,
            pub pool_mint: Pubkey,
            pub lp_token_freeze_vault: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub pool_signer: Pubkey,
            pub initializer: Pubkey,
            pub pool_authority: Pubkey,
            pub fee_base_account: Pubkey,
            pub fee_quote_account: Pubkey,
            pub fee_pool_token_account: Pubkey,
            pub curve: Pubkey,
            pub token_program: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for Initialize
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_token_freeze_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.initializer, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_base_account, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_quote_account, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_pool_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.curve, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Initialize {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.lp_token_freeze_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.initializer,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_authority,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.fee_base_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.fee_quote_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.fee_pool_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.curve,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize {
        use super::*;
        /// Generated CPI struct of the accounts for [`Initialize`].
        pub struct Initialize<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub lp_token_freeze_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub initializer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_base_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_quote_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_pool_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub curve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Initialize<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.lp_token_freeze_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.initializer),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_authority),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.fee_base_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.fee_quote_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.fee_pool_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.curve),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.lp_token_freeze_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.initializer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.pool_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_base_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_quote_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_pool_token_account,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.curve));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct GetCreationBasket<'info> {
        pub pool: AccountInfo<'info>,
        pub base_token_vault: AccountInfo<'info>,
        pub quote_token_vault: AccountInfo<'info>,
        pub pool_mint: AccountInfo<'info>,
        #[account(mut)]
        pub retbuf_account: AccountInfo<'info>,
        pub retbuf_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, GetCreationBasketBumps>
    for GetCreationBasket<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut GetCreationBasketBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let retbuf_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("retbuf_account"))?;
            let retbuf_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("retbuf_program"))?;
            if !&retbuf_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("retbuf_account"),
                );
            }
            Ok(GetCreationBasket {
                pool,
                base_token_vault,
                quote_token_vault,
                pool_mint,
                retbuf_account,
                retbuf_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for GetCreationBasket<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.retbuf_account.to_account_infos());
            account_infos.extend(self.retbuf_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for GetCreationBasket<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.retbuf_account.to_account_metas(None));
            account_metas.extend(self.retbuf_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for GetCreationBasket<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.retbuf_account, program_id)
                .map_err(|e| e.with_account_name("retbuf_account"))?;
            Ok(())
        }
    }
    pub struct GetCreationBasketBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for GetCreationBasketBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "GetCreationBasketBumps")
        }
    }
    impl Default for GetCreationBasketBumps {
        fn default() -> Self {
            GetCreationBasketBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for GetCreationBasket<'info>
    where
        'info: 'info,
    {
        type Bumps = GetCreationBasketBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_get_creation_basket {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`GetCreationBasket`].
        pub struct GetCreationBasket {
            pub pool: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub pool_mint: Pubkey,
            pub retbuf_account: Pubkey,
            pub retbuf_program: Pubkey,
        }
        impl borsh::ser::BorshSerialize for GetCreationBasket
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.retbuf_account, writer)?;
                borsh::BorshSerialize::serialize(&self.retbuf_program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for GetCreationBasket {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.retbuf_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.retbuf_program,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_get_creation_basket {
        use super::*;
        /// Generated CPI struct of the accounts for [`GetCreationBasket`].
        pub struct GetCreationBasket<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub retbuf_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub retbuf_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GetCreationBasket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.retbuf_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.retbuf_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GetCreationBasket<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.retbuf_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.retbuf_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct GetRedemptionBasket<'info> {
        pub pool: AccountInfo<'info>,
        pub base_token_vault: AccountInfo<'info>,
        pub quote_token_vault: AccountInfo<'info>,
        pub pool_mint: AccountInfo<'info>,
        #[account(mut)]
        pub retbuf_account: AccountInfo<'info>,
        pub retbuf_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, GetRedemptionBasketBumps>
    for GetRedemptionBasket<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut GetRedemptionBasketBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let retbuf_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("retbuf_account"))?;
            let retbuf_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("retbuf_program"))?;
            if !&retbuf_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("retbuf_account"),
                );
            }
            Ok(GetRedemptionBasket {
                pool,
                base_token_vault,
                quote_token_vault,
                pool_mint,
                retbuf_account,
                retbuf_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for GetRedemptionBasket<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.retbuf_account.to_account_infos());
            account_infos.extend(self.retbuf_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for GetRedemptionBasket<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.retbuf_account.to_account_metas(None));
            account_metas.extend(self.retbuf_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for GetRedemptionBasket<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.retbuf_account, program_id)
                .map_err(|e| e.with_account_name("retbuf_account"))?;
            Ok(())
        }
    }
    pub struct GetRedemptionBasketBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for GetRedemptionBasketBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "GetRedemptionBasketBumps")
        }
    }
    impl Default for GetRedemptionBasketBumps {
        fn default() -> Self {
            GetRedemptionBasketBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for GetRedemptionBasket<'info>
    where
        'info: 'info,
    {
        type Bumps = GetRedemptionBasketBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_get_redemption_basket {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`GetRedemptionBasket`].
        pub struct GetRedemptionBasket {
            pub pool: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub pool_mint: Pubkey,
            pub retbuf_account: Pubkey,
            pub retbuf_program: Pubkey,
        }
        impl borsh::ser::BorshSerialize for GetRedemptionBasket
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.retbuf_account, writer)?;
                borsh::BorshSerialize::serialize(&self.retbuf_program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for GetRedemptionBasket {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.retbuf_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.retbuf_program,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_get_redemption_basket {
        use super::*;
        /// Generated CPI struct of the accounts for [`GetRedemptionBasket`].
        pub struct GetRedemptionBasket<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub retbuf_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub retbuf_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for GetRedemptionBasket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.retbuf_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.retbuf_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for GetRedemptionBasket<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.retbuf_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.retbuf_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct CreateBasket<'info> {
        pub pool: AccountInfo<'info>,
        #[account(mut)]
        pub pool_mint: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub user_base_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub user_quote_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub base_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub quote_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub user_pool_token_account: AccountInfo<'info>,
        pub wallet_authority: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, CreateBasketBumps> for CreateBasket<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut CreateBasketBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let user_base_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            let user_quote_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let user_pool_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&pool_mint.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("pool_mint"),
                );
            }
            if !&user_base_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_base_token_account"),
                );
            }
            if !&user_quote_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_quote_token_account"),
                );
            }
            if !&base_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("base_token_vault"),
                );
            }
            if !&quote_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("quote_token_vault"),
                );
            }
            if !&user_pool_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_pool_token_account"),
                );
            }
            Ok(CreateBasket {
                pool,
                pool_mint,
                pool_signer,
                user_base_token_account,
                user_quote_token_account,
                base_token_vault,
                quote_token_vault,
                user_pool_token_account,
                wallet_authority,
                token_program,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateBasket<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.user_base_token_account.to_account_infos());
            account_infos.extend(self.user_quote_token_account.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.user_pool_token_account.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateBasket<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.user_base_token_account.to_account_metas(None));
            account_metas.extend(self.user_quote_token_account.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.user_pool_token_account.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CreateBasket<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)
                .map_err(|e| e.with_account_name("pool_mint"))?;
            anchor_lang::AccountsExit::exit(&self.user_base_token_account, program_id)
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_quote_token_account, program_id)
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.base_token_vault, program_id)
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.quote_token_vault, program_id)
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.user_pool_token_account, program_id)
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            Ok(())
        }
    }
    pub struct CreateBasketBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CreateBasketBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "CreateBasketBumps")
        }
    }
    impl Default for CreateBasketBumps {
        fn default() -> Self {
            CreateBasketBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for CreateBasket<'info>
    where
        'info: 'info,
    {
        type Bumps = CreateBasketBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_create_basket {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`CreateBasket`].
        pub struct CreateBasket {
            pub pool: Pubkey,
            pub pool_mint: Pubkey,
            pub pool_signer: Pubkey,
            pub user_base_token_account: Pubkey,
            pub user_quote_token_account: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub user_pool_token_account: Pubkey,
            pub wallet_authority: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for CreateBasket
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.user_base_token_account, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.user_quote_token_account,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.user_pool_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CreateBasket {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_base_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_quote_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_pool_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_create_basket {
        use super::*;
        /// Generated CPI struct of the accounts for [`CreateBasket`].
        pub struct CreateBasket<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_base_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_quote_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_pool_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateBasket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_base_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_quote_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_pool_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateBasket<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_base_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_quote_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_pool_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct RedeemBasket<'info> {
        pub pool: AccountInfo<'info>,
        #[account(mut)]
        pub pool_mint: AccountInfo<'info>,
        #[account(mut)]
        pub base_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub quote_token_vault: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub user_pool_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub user_base_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub user_quote_token_account: AccountInfo<'info>,
        pub wallet_authority: Signer<'info>,
        #[account(mut)]
        pub user_sol_account: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        #[account(mut)]
        pub fee_base_account: AccountInfo<'info>,
        #[account(mut)]
        pub fee_quote_account: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, RedeemBasketBumps> for RedeemBasket<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut RedeemBasketBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let user_pool_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            let user_base_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            let user_quote_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let user_sol_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_sol_account"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let fee_base_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_base_account"))?;
            let fee_quote_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_quote_account"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            if !&pool_mint.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("pool_mint"),
                );
            }
            if !&base_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("base_token_vault"),
                );
            }
            if !&quote_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("quote_token_vault"),
                );
            }
            if !&user_pool_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_pool_token_account"),
                );
            }
            if !&user_base_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_base_token_account"),
                );
            }
            if !&user_quote_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_quote_token_account"),
                );
            }
            if !&user_sol_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_sol_account"),
                );
            }
            if !&fee_base_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("fee_base_account"),
                );
            }
            if !&fee_quote_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("fee_quote_account"),
                );
            }
            Ok(RedeemBasket {
                pool,
                pool_mint,
                base_token_vault,
                quote_token_vault,
                pool_signer,
                user_pool_token_account,
                user_base_token_account,
                user_quote_token_account,
                wallet_authority,
                user_sol_account,
                token_program,
                fee_base_account,
                fee_quote_account,
                clock,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for RedeemBasket<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.user_pool_token_account.to_account_infos());
            account_infos.extend(self.user_base_token_account.to_account_infos());
            account_infos.extend(self.user_quote_token_account.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.user_sol_account.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.fee_base_account.to_account_infos());
            account_infos.extend(self.fee_quote_account.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for RedeemBasket<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.user_pool_token_account.to_account_metas(None));
            account_metas.extend(self.user_base_token_account.to_account_metas(None));
            account_metas.extend(self.user_quote_token_account.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.user_sol_account.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.fee_base_account.to_account_metas(None));
            account_metas.extend(self.fee_quote_account.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for RedeemBasket<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)
                .map_err(|e| e.with_account_name("pool_mint"))?;
            anchor_lang::AccountsExit::exit(&self.base_token_vault, program_id)
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.quote_token_vault, program_id)
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.user_pool_token_account, program_id)
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_base_token_account, program_id)
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_quote_token_account, program_id)
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_sol_account, program_id)
                .map_err(|e| e.with_account_name("user_sol_account"))?;
            anchor_lang::AccountsExit::exit(&self.fee_base_account, program_id)
                .map_err(|e| e.with_account_name("fee_base_account"))?;
            anchor_lang::AccountsExit::exit(&self.fee_quote_account, program_id)
                .map_err(|e| e.with_account_name("fee_quote_account"))?;
            Ok(())
        }
    }
    pub struct RedeemBasketBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for RedeemBasketBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "RedeemBasketBumps")
        }
    }
    impl Default for RedeemBasketBumps {
        fn default() -> Self {
            RedeemBasketBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for RedeemBasket<'info>
    where
        'info: 'info,
    {
        type Bumps = RedeemBasketBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_redeem_basket {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`RedeemBasket`].
        pub struct RedeemBasket {
            pub pool: Pubkey,
            pub pool_mint: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub pool_signer: Pubkey,
            pub user_pool_token_account: Pubkey,
            pub user_base_token_account: Pubkey,
            pub user_quote_token_account: Pubkey,
            pub wallet_authority: Pubkey,
            pub user_sol_account: Pubkey,
            pub token_program: Pubkey,
            pub fee_base_account: Pubkey,
            pub fee_quote_account: Pubkey,
            pub clock: Pubkey,
        }
        impl borsh::ser::BorshSerialize for RedeemBasket
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.user_pool_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.user_base_token_account, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.user_quote_token_account,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.user_sol_account, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_base_account, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_quote_account, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for RedeemBasket {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_pool_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_base_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_quote_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_sol_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.fee_base_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.fee_quote_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_redeem_basket {
        use super::*;
        /// Generated CPI struct of the accounts for [`RedeemBasket`].
        pub struct RedeemBasket<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_pool_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_base_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_quote_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_sol_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_base_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_quote_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for RedeemBasket<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_pool_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_base_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_quote_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_sol_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.fee_base_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.fee_quote_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for RedeemBasket<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_pool_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_base_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_quote_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_sol_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_base_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_quote_account,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
            }
        }
    }
    pub struct Swap<'info> {
        pub pool: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub pool_mint: AccountInfo<'info>,
        #[account(mut)]
        pub base_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub quote_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub fee_pool_token_account: AccountInfo<'info>,
        pub wallet_authority: Signer<'info>,
        #[account(mut)]
        pub user_base_token_account: AccountInfo<'info>,
        #[account(mut)]
        pub user_quote_token_account: AccountInfo<'info>,
        pub curve: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, SwapBumps> for Swap<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut SwapBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let pool_mint: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_mint"))?;
            let base_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            let quote_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            let fee_pool_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("fee_pool_token_account"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let user_base_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            let user_quote_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            let curve: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("curve"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            if !&pool_mint.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("pool_mint"),
                );
            }
            if !&base_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("base_token_vault"),
                );
            }
            if !&quote_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("quote_token_vault"),
                );
            }
            if !&fee_pool_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("fee_pool_token_account"),
                );
            }
            if !&user_base_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_base_token_account"),
                );
            }
            if !&user_quote_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_quote_token_account"),
                );
            }
            Ok(Swap {
                pool,
                pool_signer,
                pool_mint,
                base_token_vault,
                quote_token_vault,
                fee_pool_token_account,
                wallet_authority,
                user_base_token_account,
                user_quote_token_account,
                curve,
                token_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Swap<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.pool_mint.to_account_infos());
            account_infos.extend(self.base_token_vault.to_account_infos());
            account_infos.extend(self.quote_token_vault.to_account_infos());
            account_infos.extend(self.fee_pool_token_account.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.user_base_token_account.to_account_infos());
            account_infos.extend(self.user_quote_token_account.to_account_infos());
            account_infos.extend(self.curve.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Swap<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.pool_mint.to_account_metas(None));
            account_metas.extend(self.base_token_vault.to_account_metas(None));
            account_metas.extend(self.quote_token_vault.to_account_metas(None));
            account_metas.extend(self.fee_pool_token_account.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.user_base_token_account.to_account_metas(None));
            account_metas.extend(self.user_quote_token_account.to_account_metas(None));
            account_metas.extend(self.curve.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for Swap<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.pool_mint, program_id)
                .map_err(|e| e.with_account_name("pool_mint"))?;
            anchor_lang::AccountsExit::exit(&self.base_token_vault, program_id)
                .map_err(|e| e.with_account_name("base_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.quote_token_vault, program_id)
                .map_err(|e| e.with_account_name("quote_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.fee_pool_token_account, program_id)
                .map_err(|e| e.with_account_name("fee_pool_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_base_token_account, program_id)
                .map_err(|e| e.with_account_name("user_base_token_account"))?;
            anchor_lang::AccountsExit::exit(&self.user_quote_token_account, program_id)
                .map_err(|e| e.with_account_name("user_quote_token_account"))?;
            Ok(())
        }
    }
    pub struct SwapBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for SwapBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "SwapBumps")
        }
    }
    impl Default for SwapBumps {
        fn default() -> Self {
            SwapBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for Swap<'info>
    where
        'info: 'info,
    {
        type Bumps = SwapBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_swap {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`Swap`].
        pub struct Swap {
            pub pool: Pubkey,
            pub pool_signer: Pubkey,
            pub pool_mint: Pubkey,
            pub base_token_vault: Pubkey,
            pub quote_token_vault: Pubkey,
            pub fee_pool_token_account: Pubkey,
            pub wallet_authority: Pubkey,
            pub user_base_token_account: Pubkey,
            pub user_quote_token_account: Pubkey,
            pub curve: Pubkey,
            pub token_program: Pubkey,
        }
        impl borsh::ser::BorshSerialize for Swap
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_mint, writer)?;
                borsh::BorshSerialize::serialize(&self.base_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.quote_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.fee_pool_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.user_base_token_account, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.user_quote_token_account,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.curve, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for Swap {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.pool_mint,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.base_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.quote_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.fee_pool_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_base_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_quote_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.curve,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_swap {
        use super::*;
        /// Generated CPI struct of the accounts for [`Swap`].
        pub struct Swap<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub base_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub quote_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub fee_pool_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_base_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_quote_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub curve: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Swap<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.pool_mint),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.base_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.quote_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.fee_pool_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_base_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_quote_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.curve),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Swap<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_mint),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.base_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.quote_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.fee_pool_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_base_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_quote_token_account,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.curve));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct InitializeFarming<'info> {
        pub pool: AccountInfo<'info>,
        #[account(mut)]
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub snapshots: AccountInfo<'info>,
        #[account(mut)]
        pub farming_token_vault: AccountInfo<'info>,
        pub farming_authority: Signer<'info>,
        pub wallet_authority: Signer<'info>,
        #[account(mut)]
        pub farming_token_account: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, InitializeFarmingBumps>
    for InitializeFarming<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut InitializeFarmingBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let snapshots: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("snapshots"))?;
            let farming_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            let farming_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_authority"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let farming_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_account"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&farming_state.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_state"),
                );
            }
            if !&snapshots.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("snapshots"),
                );
            }
            if !&farming_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_vault"),
                );
            }
            if !&farming_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_account"),
                );
            }
            Ok(InitializeFarming {
                pool,
                farming_state,
                snapshots,
                farming_token_vault,
                farming_authority,
                wallet_authority,
                farming_token_account,
                token_program,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeFarming<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.snapshots.to_account_infos());
            account_infos.extend(self.farming_token_vault.to_account_infos());
            account_infos.extend(self.farming_authority.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.farming_token_account.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeFarming<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.snapshots.to_account_metas(None));
            account_metas.extend(self.farming_token_vault.to_account_metas(None));
            account_metas.extend(self.farming_authority.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.farming_token_account.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeFarming<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_state, program_id)
                .map_err(|e| e.with_account_name("farming_state"))?;
            anchor_lang::AccountsExit::exit(&self.snapshots, program_id)
                .map_err(|e| e.with_account_name("snapshots"))?;
            anchor_lang::AccountsExit::exit(&self.farming_token_vault, program_id)
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.farming_token_account, program_id)
                .map_err(|e| e.with_account_name("farming_token_account"))?;
            Ok(())
        }
    }
    pub struct InitializeFarmingBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InitializeFarmingBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InitializeFarmingBumps")
        }
    }
    impl Default for InitializeFarmingBumps {
        fn default() -> Self {
            InitializeFarmingBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for InitializeFarming<'info>
    where
        'info: 'info,
    {
        type Bumps = InitializeFarmingBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize_farming {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`InitializeFarming`].
        pub struct InitializeFarming {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub snapshots: Pubkey,
            pub farming_token_vault: Pubkey,
            pub farming_authority: Pubkey,
            pub wallet_authority: Pubkey,
            pub farming_token_account: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeFarming
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.snapshots, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeFarming {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.snapshots,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize_farming {
        use super::*;
        /// Generated CPI struct of the accounts for [`InitializeFarming`].
        pub struct InitializeFarming<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub snapshots: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeFarming<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.snapshots),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeFarming<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.snapshots),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct WithdrawFarmingVaultTokens<'info> {
        pub pool: AccountInfo<'info>,
        pub pool_authority: Signer<'info>,
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub farming_token_vault: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub target_token_wallet: AccountInfo<'info>,
        pub token_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, WithdrawFarmingVaultTokensBumps>
    for WithdrawFarmingVaultTokens<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut WithdrawFarmingVaultTokensBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let pool_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_authority"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let target_token_wallet: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("target_token_wallet"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            if !&farming_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_vault"),
                );
            }
            if !&target_token_wallet.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("target_token_wallet"),
                );
            }
            Ok(WithdrawFarmingVaultTokens {
                pool,
                pool_authority,
                farming_state,
                farming_token_vault,
                pool_signer,
                target_token_wallet,
                token_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawFarmingVaultTokens<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.pool_authority.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_token_vault.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.target_token_wallet.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WithdrawFarmingVaultTokens<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.pool_authority.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.target_token_wallet.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for WithdrawFarmingVaultTokens<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_token_vault, program_id)
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.target_token_wallet, program_id)
                .map_err(|e| e.with_account_name("target_token_wallet"))?;
            Ok(())
        }
    }
    pub struct WithdrawFarmingVaultTokensBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for WithdrawFarmingVaultTokensBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "WithdrawFarmingVaultTokensBumps")
        }
    }
    impl Default for WithdrawFarmingVaultTokensBumps {
        fn default() -> Self {
            WithdrawFarmingVaultTokensBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for WithdrawFarmingVaultTokens<'info>
    where
        'info: 'info,
    {
        type Bumps = WithdrawFarmingVaultTokensBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_withdraw_farming_vault_tokens {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`WithdrawFarmingVaultTokens`].
        pub struct WithdrawFarmingVaultTokens {
            pub pool: Pubkey,
            pub pool_authority: Pubkey,
            pub farming_state: Pubkey,
            pub farming_token_vault: Pubkey,
            pub pool_signer: Pubkey,
            pub target_token_wallet: Pubkey,
            pub token_program: Pubkey,
        }
        impl borsh::ser::BorshSerialize for WithdrawFarmingVaultTokens
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.target_token_wallet, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for WithdrawFarmingVaultTokens {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.target_token_wallet,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_withdraw_farming_vault_tokens {
        use super::*;
        /// Generated CPI struct of the accounts for [`WithdrawFarmingVaultTokens`].
        pub struct WithdrawFarmingVaultTokens<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub pool_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub target_token_wallet: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawFarmingVaultTokens<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.target_token_wallet),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info>
        for WithdrawFarmingVaultTokens<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.pool_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.target_token_wallet,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
            }
        }
    }
    pub struct StartFarming<'info> {
        pub pool: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub farming_ticket: AccountInfo<'info>,
        #[account(mut)]
        pub lp_token_freeze_vault: AccountInfo<'info>,
        #[account(mut)]
        pub user_lp_token_account: AccountInfo<'info>,
        pub wallet_authority: Signer<'info>,
        pub user_key: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, StartFarmingBumps> for StartFarming<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut StartFarmingBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let lp_token_freeze_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            let user_lp_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_lp_token_account"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let user_key: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_key"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&farming_ticket.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_ticket"),
                );
            }
            if !&lp_token_freeze_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("lp_token_freeze_vault"),
                );
            }
            if !&user_lp_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_lp_token_account"),
                );
            }
            Ok(StartFarming {
                pool,
                farming_state,
                farming_ticket,
                lp_token_freeze_vault,
                user_lp_token_account,
                wallet_authority,
                user_key,
                token_program,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for StartFarming<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.lp_token_freeze_vault.to_account_infos());
            account_infos.extend(self.user_lp_token_account.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.user_key.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for StartFarming<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.lp_token_freeze_vault.to_account_metas(None));
            account_metas.extend(self.user_lp_token_account.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.user_key.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for StartFarming<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_ticket, program_id)
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            anchor_lang::AccountsExit::exit(&self.lp_token_freeze_vault, program_id)
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            anchor_lang::AccountsExit::exit(&self.user_lp_token_account, program_id)
                .map_err(|e| e.with_account_name("user_lp_token_account"))?;
            Ok(())
        }
    }
    pub struct StartFarmingBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for StartFarmingBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "StartFarmingBumps")
        }
    }
    impl Default for StartFarmingBumps {
        fn default() -> Self {
            StartFarmingBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for StartFarming<'info>
    where
        'info: 'info,
    {
        type Bumps = StartFarmingBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_start_farming {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`StartFarming`].
        pub struct StartFarming {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_ticket: Pubkey,
            pub lp_token_freeze_vault: Pubkey,
            pub user_lp_token_account: Pubkey,
            pub wallet_authority: Pubkey,
            pub user_key: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for StartFarming
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_token_freeze_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.user_lp_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.user_key, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for StartFarming {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.lp_token_freeze_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_lp_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_key,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_start_farming {
        use super::*;
        /// Generated CPI struct of the accounts for [`StartFarming`].
        pub struct StartFarming<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub lp_token_freeze_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_lp_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_key: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for StartFarming<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.lp_token_freeze_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_lp_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_key),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for StartFarming<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.lp_token_freeze_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_lp_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_key),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct InitializeFarmingCalc<'info> {
        #[account(mut)]
        pub farming_calc: AccountInfo<'info>,
        pub farming_ticket: AccountInfo<'info>,
        pub user_key: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        pub initializer: Signer<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, InitializeFarmingCalcBumps>
    for InitializeFarmingCalc<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut InitializeFarmingCalcBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let farming_calc: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_calc"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let user_key: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_key"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let initializer: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("initializer"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&farming_calc.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_calc"),
                );
            }
            Ok(InitializeFarmingCalc {
                farming_calc,
                farming_ticket,
                user_key,
                farming_state,
                initializer,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeFarmingCalc<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.farming_calc.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.user_key.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.initializer.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for InitializeFarmingCalc<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.farming_calc.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.user_key.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.initializer.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for InitializeFarmingCalc<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_calc, program_id)
                .map_err(|e| e.with_account_name("farming_calc"))?;
            Ok(())
        }
    }
    pub struct InitializeFarmingCalcBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for InitializeFarmingCalcBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "InitializeFarmingCalcBumps")
        }
    }
    impl Default for InitializeFarmingCalcBumps {
        fn default() -> Self {
            InitializeFarmingCalcBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for InitializeFarmingCalc<'info>
    where
        'info: 'info,
    {
        type Bumps = InitializeFarmingCalcBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_initialize_farming_calc {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`InitializeFarmingCalc`].
        pub struct InitializeFarmingCalc {
            pub farming_calc: Pubkey,
            pub farming_ticket: Pubkey,
            pub user_key: Pubkey,
            pub farming_state: Pubkey,
            pub initializer: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for InitializeFarmingCalc
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.farming_calc, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.user_key, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.initializer, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for InitializeFarmingCalc {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_calc,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_key,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.initializer,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_initialize_farming_calc {
        use super::*;
        /// Generated CPI struct of the accounts for [`InitializeFarmingCalc`].
        pub struct InitializeFarmingCalc<'info> {
            pub farming_calc: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_key: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub initializer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for InitializeFarmingCalc<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_calc),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_key),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.initializer),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for InitializeFarmingCalc<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.farming_calc),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_key),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.initializer),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct CloseFarmingCalc<'info> {
        #[account(mut)]
        pub farming_calc: AccountInfo<'info>,
        pub farming_ticket: AccountInfo<'info>,
        pub signer: Signer<'info>,
        #[account(mut)]
        pub initializer: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, CloseFarmingCalcBumps>
    for CloseFarmingCalc<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut CloseFarmingCalcBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let farming_calc: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_calc"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let signer: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("signer"))?;
            let initializer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("initializer"))?;
            if !&farming_calc.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_calc"),
                );
            }
            if !&initializer.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("initializer"),
                );
            }
            Ok(CloseFarmingCalc {
                farming_calc,
                farming_ticket,
                signer,
                initializer,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CloseFarmingCalc<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.farming_calc.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.signer.to_account_infos());
            account_infos.extend(self.initializer.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CloseFarmingCalc<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.farming_calc.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.signer.to_account_metas(None));
            account_metas.extend(self.initializer.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CloseFarmingCalc<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_calc, program_id)
                .map_err(|e| e.with_account_name("farming_calc"))?;
            anchor_lang::AccountsExit::exit(&self.initializer, program_id)
                .map_err(|e| e.with_account_name("initializer"))?;
            Ok(())
        }
    }
    pub struct CloseFarmingCalcBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CloseFarmingCalcBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "CloseFarmingCalcBumps")
        }
    }
    impl Default for CloseFarmingCalcBumps {
        fn default() -> Self {
            CloseFarmingCalcBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for CloseFarmingCalc<'info>
    where
        'info: 'info,
    {
        type Bumps = CloseFarmingCalcBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_close_farming_calc {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`CloseFarmingCalc`].
        pub struct CloseFarmingCalc {
            pub farming_calc: Pubkey,
            pub farming_ticket: Pubkey,
            pub signer: Pubkey,
            pub initializer: Pubkey,
        }
        impl borsh::ser::BorshSerialize for CloseFarmingCalc
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.farming_calc, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.signer, writer)?;
                borsh::BorshSerialize::serialize(&self.initializer, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CloseFarmingCalc {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_calc,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.signer,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.initializer,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_close_farming_calc {
        use super::*;
        /// Generated CPI struct of the accounts for [`CloseFarmingCalc`].
        pub struct CloseFarmingCalc<'info> {
            pub farming_calc: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub initializer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CloseFarmingCalc<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_calc),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.signer),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.initializer),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CloseFarmingCalc<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.farming_calc),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.signer));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.initializer),
                    );
                account_infos
            }
        }
    }
    pub struct CalculateFarmed<'info> {
        pub pool: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        pub farming_snapshots: AccountInfo<'info>,
        #[account(mut)]
        pub farming_calc: AccountInfo<'info>,
        #[account(mut)]
        pub farming_ticket: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, CalculateFarmedBumps>
    for CalculateFarmed<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut CalculateFarmedBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_snapshots: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_snapshots"))?;
            let farming_calc: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_calc"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            if !&farming_calc.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_calc"),
                );
            }
            if !&farming_ticket.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_ticket"),
                );
            }
            Ok(CalculateFarmed {
                pool,
                farming_state,
                farming_snapshots,
                farming_calc,
                farming_ticket,
                clock,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CalculateFarmed<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_snapshots.to_account_infos());
            account_infos.extend(self.farming_calc.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CalculateFarmed<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_snapshots.to_account_metas(None));
            account_metas.extend(self.farming_calc.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CalculateFarmed<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_calc, program_id)
                .map_err(|e| e.with_account_name("farming_calc"))?;
            anchor_lang::AccountsExit::exit(&self.farming_ticket, program_id)
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            Ok(())
        }
    }
    pub struct CalculateFarmedBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CalculateFarmedBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "CalculateFarmedBumps")
        }
    }
    impl Default for CalculateFarmedBumps {
        fn default() -> Self {
            CalculateFarmedBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for CalculateFarmed<'info>
    where
        'info: 'info,
    {
        type Bumps = CalculateFarmedBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_calculate_farmed {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`CalculateFarmed`].
        pub struct CalculateFarmed {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_snapshots: Pubkey,
            pub farming_calc: Pubkey,
            pub farming_ticket: Pubkey,
            pub clock: Pubkey,
        }
        impl borsh::ser::BorshSerialize for CalculateFarmed
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_snapshots, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_calc, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CalculateFarmed {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_snapshots,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_calc,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_calculate_farmed {
        use super::*;
        /// Generated CPI struct of the accounts for [`CalculateFarmed`].
        pub struct CalculateFarmed<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_snapshots: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_calc: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CalculateFarmed<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_snapshots),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_calc),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CalculateFarmed<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_snapshots,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.farming_calc),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
            }
        }
    }
    pub struct WithdrawFarmed<'info> {
        pub pool: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub farming_calc: AccountInfo<'info>,
        #[account(mut)]
        pub farming_token_vault: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub user_farming_token_account: AccountInfo<'info>,
        pub user_key: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, WithdrawFarmedBumps>
    for WithdrawFarmed<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut WithdrawFarmedBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_calc: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_calc"))?;
            let farming_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let user_farming_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_farming_token_account"))?;
            let user_key: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_key"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            if !&farming_calc.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_calc"),
                );
            }
            if !&farming_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_vault"),
                );
            }
            if !&user_farming_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_farming_token_account"),
                );
            }
            Ok(WithdrawFarmed {
                pool,
                farming_state,
                farming_calc,
                farming_token_vault,
                pool_signer,
                user_farming_token_account,
                user_key,
                token_program,
                clock,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawFarmed<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_calc.to_account_infos());
            account_infos.extend(self.farming_token_vault.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.user_farming_token_account.to_account_infos());
            account_infos.extend(self.user_key.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for WithdrawFarmed<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_calc.to_account_metas(None));
            account_metas.extend(self.farming_token_vault.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.user_farming_token_account.to_account_metas(None));
            account_metas.extend(self.user_key.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for WithdrawFarmed<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_calc, program_id)
                .map_err(|e| e.with_account_name("farming_calc"))?;
            anchor_lang::AccountsExit::exit(&self.farming_token_vault, program_id)
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.user_farming_token_account, program_id)
                .map_err(|e| e.with_account_name("user_farming_token_account"))?;
            Ok(())
        }
    }
    pub struct WithdrawFarmedBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for WithdrawFarmedBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "WithdrawFarmedBumps")
        }
    }
    impl Default for WithdrawFarmedBumps {
        fn default() -> Self {
            WithdrawFarmedBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for WithdrawFarmed<'info>
    where
        'info: 'info,
    {
        type Bumps = WithdrawFarmedBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_withdraw_farmed {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`WithdrawFarmed`].
        pub struct WithdrawFarmed {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_calc: Pubkey,
            pub farming_token_vault: Pubkey,
            pub pool_signer: Pubkey,
            pub user_farming_token_account: Pubkey,
            pub user_key: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
        }
        impl borsh::ser::BorshSerialize for WithdrawFarmed
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_calc, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.user_farming_token_account,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.user_key, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for WithdrawFarmed {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_calc,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_farming_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_key,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_withdraw_farmed {
        use super::*;
        /// Generated CPI struct of the accounts for [`WithdrawFarmed`].
        pub struct WithdrawFarmed<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_calc: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_farming_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_key: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for WithdrawFarmed<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_calc),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_farming_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_key),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for WithdrawFarmed<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.farming_calc),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_farming_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_key),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
            }
        }
    }
    pub struct CheckFarmed<'info> {
        pub pool: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        pub farming_snapshots: AccountInfo<'info>,
        pub farming_ticket: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, CheckFarmedBumps> for CheckFarmed<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut CheckFarmedBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_snapshots: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_snapshots"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            Ok(CheckFarmed {
                pool,
                farming_state,
                farming_snapshots,
                farming_ticket,
                pool_signer,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CheckFarmed<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_snapshots.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CheckFarmed<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_snapshots.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for CheckFarmed<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            Ok(())
        }
    }
    pub struct CheckFarmedBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for CheckFarmedBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "CheckFarmedBumps")
        }
    }
    impl Default for CheckFarmedBumps {
        fn default() -> Self {
            CheckFarmedBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for CheckFarmed<'info>
    where
        'info: 'info,
    {
        type Bumps = CheckFarmedBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_check_farmed {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`CheckFarmed`].
        pub struct CheckFarmed {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_snapshots: Pubkey,
            pub farming_ticket: Pubkey,
            pub pool_signer: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for CheckFarmed
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_snapshots, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for CheckFarmed {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_snapshots,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_check_farmed {
        use super::*;
        /// Generated CPI struct of the accounts for [`CheckFarmed`].
        pub struct CheckFarmed<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_snapshots: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CheckFarmed<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_snapshots),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CheckFarmed<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_snapshots,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct EndFarming<'info> {
        pub pool: AccountInfo<'info>,
        pub farming_state: AccountInfo<'info>,
        pub farming_snapshots: AccountInfo<'info>,
        #[account(mut)]
        pub farming_ticket: AccountInfo<'info>,
        #[account(mut)]
        pub lp_token_freeze_vault: AccountInfo<'info>,
        pub pool_signer: AccountInfo<'info>,
        #[account(mut)]
        pub user_pool_token_account: AccountInfo<'info>,
        pub user_key: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, EndFarmingBumps> for EndFarming<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut EndFarmingBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_snapshots: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_snapshots"))?;
            let farming_ticket: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            let lp_token_freeze_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            let pool_signer: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool_signer"))?;
            let user_pool_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            let user_key: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("user_key"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&farming_ticket.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_ticket"),
                );
            }
            if !&lp_token_freeze_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("lp_token_freeze_vault"),
                );
            }
            if !&user_pool_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("user_pool_token_account"),
                );
            }
            Ok(EndFarming {
                pool,
                farming_state,
                farming_snapshots,
                farming_ticket,
                lp_token_freeze_vault,
                pool_signer,
                user_pool_token_account,
                user_key,
                token_program,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for EndFarming<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_snapshots.to_account_infos());
            account_infos.extend(self.farming_ticket.to_account_infos());
            account_infos.extend(self.lp_token_freeze_vault.to_account_infos());
            account_infos.extend(self.pool_signer.to_account_infos());
            account_infos.extend(self.user_pool_token_account.to_account_infos());
            account_infos.extend(self.user_key.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for EndFarming<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_snapshots.to_account_metas(None));
            account_metas.extend(self.farming_ticket.to_account_metas(None));
            account_metas.extend(self.lp_token_freeze_vault.to_account_metas(None));
            account_metas.extend(self.pool_signer.to_account_metas(None));
            account_metas.extend(self.user_pool_token_account.to_account_metas(None));
            account_metas.extend(self.user_key.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for EndFarming<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_ticket, program_id)
                .map_err(|e| e.with_account_name("farming_ticket"))?;
            anchor_lang::AccountsExit::exit(&self.lp_token_freeze_vault, program_id)
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            anchor_lang::AccountsExit::exit(&self.user_pool_token_account, program_id)
                .map_err(|e| e.with_account_name("user_pool_token_account"))?;
            Ok(())
        }
    }
    pub struct EndFarmingBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for EndFarmingBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "EndFarmingBumps")
        }
    }
    impl Default for EndFarmingBumps {
        fn default() -> Self {
            EndFarmingBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for EndFarming<'info>
    where
        'info: 'info,
    {
        type Bumps = EndFarmingBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_end_farming {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`EndFarming`].
        pub struct EndFarming {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_snapshots: Pubkey,
            pub farming_ticket: Pubkey,
            pub lp_token_freeze_vault: Pubkey,
            pub pool_signer: Pubkey,
            pub user_pool_token_account: Pubkey,
            pub user_key: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for EndFarming
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_snapshots, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_ticket, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_token_freeze_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.pool_signer, writer)?;
                borsh::BorshSerialize::serialize(&self.user_pool_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.user_key, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for EndFarming {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.farming_snapshots,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_ticket,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.lp_token_freeze_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool_signer,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.user_pool_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.user_key,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_end_farming {
        use super::*;
        /// Generated CPI struct of the accounts for [`EndFarming`].
        pub struct EndFarming<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_snapshots: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_ticket: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub lp_token_freeze_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub pool_signer: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_pool_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub user_key: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for EndFarming<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.farming_snapshots),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_ticket),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.lp_token_freeze_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool_signer),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.user_pool_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.user_key),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for EndFarming<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_snapshots,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_ticket,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.lp_token_freeze_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.pool_signer),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.user_pool_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.user_key),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct TakeFarmingSnapshot<'info> {
        pub pool: AccountInfo<'info>,
        #[account(mut)]
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub farming_snapshots: AccountInfo<'info>,
        pub lp_token_freeze_vault: AccountInfo<'info>,
        pub authority: Signer<'info>,
        pub token_program: AccountInfo<'info>,
        pub clock: AccountInfo<'info>,
        pub rent: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, TakeFarmingSnapshotBumps>
    for TakeFarmingSnapshot<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut TakeFarmingSnapshotBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_snapshots: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_snapshots"))?;
            let lp_token_freeze_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("lp_token_freeze_vault"))?;
            let authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("authority"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            let clock: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("clock"))?;
            let rent: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
            if !&farming_state.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_state"),
                );
            }
            if !&farming_snapshots.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_snapshots"),
                );
            }
            Ok(TakeFarmingSnapshot {
                pool,
                farming_state,
                farming_snapshots,
                lp_token_freeze_vault,
                authority,
                token_program,
                clock,
                rent,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for TakeFarmingSnapshot<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_snapshots.to_account_infos());
            account_infos.extend(self.lp_token_freeze_vault.to_account_infos());
            account_infos.extend(self.authority.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos.extend(self.clock.to_account_infos());
            account_infos.extend(self.rent.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for TakeFarmingSnapshot<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_snapshots.to_account_metas(None));
            account_metas.extend(self.lp_token_freeze_vault.to_account_metas(None));
            account_metas.extend(self.authority.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas.extend(self.clock.to_account_metas(None));
            account_metas.extend(self.rent.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for TakeFarmingSnapshot<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_state, program_id)
                .map_err(|e| e.with_account_name("farming_state"))?;
            anchor_lang::AccountsExit::exit(&self.farming_snapshots, program_id)
                .map_err(|e| e.with_account_name("farming_snapshots"))?;
            Ok(())
        }
    }
    pub struct TakeFarmingSnapshotBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for TakeFarmingSnapshotBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "TakeFarmingSnapshotBumps")
        }
    }
    impl Default for TakeFarmingSnapshotBumps {
        fn default() -> Self {
            TakeFarmingSnapshotBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for TakeFarmingSnapshot<'info>
    where
        'info: 'info,
    {
        type Bumps = TakeFarmingSnapshotBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_take_farming_snapshot {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`TakeFarmingSnapshot`].
        pub struct TakeFarmingSnapshot {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_snapshots: Pubkey,
            pub lp_token_freeze_vault: Pubkey,
            pub authority: Pubkey,
            pub token_program: Pubkey,
            pub clock: Pubkey,
            pub rent: Pubkey,
        }
        impl borsh::ser::BorshSerialize for TakeFarmingSnapshot
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_snapshots, writer)?;
                borsh::BorshSerialize::serialize(&self.lp_token_freeze_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                borsh::BorshSerialize::serialize(&self.clock, writer)?;
                borsh::BorshSerialize::serialize(&self.rent, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for TakeFarmingSnapshot {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_snapshots,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.lp_token_freeze_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.clock,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_take_farming_snapshot {
        use super::*;
        /// Generated CPI struct of the accounts for [`TakeFarmingSnapshot`].
        pub struct TakeFarmingSnapshot<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_snapshots: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub lp_token_freeze_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub clock: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for TakeFarmingSnapshot<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_snapshots),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.lp_token_freeze_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.clock),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for TakeFarmingSnapshot<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_snapshots,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.lp_token_freeze_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(&self.authority),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.clock));
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.rent));
                account_infos
            }
        }
    }
    pub struct IncreaseFarmingTotal<'info> {
        pub pool: AccountInfo<'info>,
        #[account(mut)]
        pub farming_state: AccountInfo<'info>,
        #[account(mut)]
        pub farming_token_vault: AccountInfo<'info>,
        #[account(mut)]
        pub farming_token_account: AccountInfo<'info>,
        pub wallet_authority: Signer<'info>,
        pub initializer_account: Signer<'info>,
        pub token_program: AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::Accounts<'info, IncreaseFarmingTotalBumps>
    for IncreaseFarmingTotal<'info>
    where
        'info: 'info,
    {
        #[inline(never)]
        fn try_accounts(
            __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >],
            __ix_data: &[u8],
            __bumps: &mut IncreaseFarmingTotalBumps,
            __reallocs: &mut std::collections::BTreeSet<
                anchor_lang::solana_program::pubkey::Pubkey,
            >,
        ) -> anchor_lang::Result<Self> {
            let pool: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("pool"))?;
            let farming_state: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_state"))?;
            let farming_token_vault: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            let farming_token_account: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("farming_token_account"))?;
            let wallet_authority: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("wallet_authority"))?;
            let initializer_account: Signer = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("initializer_account"))?;
            let token_program: AccountInfo = anchor_lang::Accounts::try_accounts(
                    __program_id,
                    __accounts,
                    __ix_data,
                    __bumps,
                    __reallocs,
                )
                .map_err(|e| e.with_account_name("token_program"))?;
            if !&farming_state.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_state"),
                );
            }
            if !&farming_token_vault.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_vault"),
                );
            }
            if !&farming_token_account.is_writable {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintMut,
                        )
                        .with_account_name("farming_token_account"),
                );
            }
            Ok(IncreaseFarmingTotal {
                pool,
                farming_state,
                farming_token_vault,
                farming_token_account,
                wallet_authority,
                initializer_account,
                token_program,
            })
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseFarmingTotal<'info>
    where
        'info: 'info,
    {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos.extend(self.pool.to_account_infos());
            account_infos.extend(self.farming_state.to_account_infos());
            account_infos.extend(self.farming_token_vault.to_account_infos());
            account_infos.extend(self.farming_token_account.to_account_infos());
            account_infos.extend(self.wallet_authority.to_account_infos());
            account_infos.extend(self.initializer_account.to_account_infos());
            account_infos.extend(self.token_program.to_account_infos());
            account_infos
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for IncreaseFarmingTotal<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas.extend(self.pool.to_account_metas(None));
            account_metas.extend(self.farming_state.to_account_metas(None));
            account_metas.extend(self.farming_token_vault.to_account_metas(None));
            account_metas.extend(self.farming_token_account.to_account_metas(None));
            account_metas.extend(self.wallet_authority.to_account_metas(None));
            account_metas.extend(self.initializer_account.to_account_metas(None));
            account_metas.extend(self.token_program.to_account_metas(None));
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::AccountsExit<'info> for IncreaseFarmingTotal<'info>
    where
        'info: 'info,
    {
        fn exit(
            &self,
            program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        ) -> anchor_lang::Result<()> {
            anchor_lang::AccountsExit::exit(&self.farming_state, program_id)
                .map_err(|e| e.with_account_name("farming_state"))?;
            anchor_lang::AccountsExit::exit(&self.farming_token_vault, program_id)
                .map_err(|e| e.with_account_name("farming_token_vault"))?;
            anchor_lang::AccountsExit::exit(&self.farming_token_account, program_id)
                .map_err(|e| e.with_account_name("farming_token_account"))?;
            Ok(())
        }
    }
    pub struct IncreaseFarmingTotalBumps {}
    #[automatically_derived]
    impl ::core::fmt::Debug for IncreaseFarmingTotalBumps {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "IncreaseFarmingTotalBumps")
        }
    }
    impl Default for IncreaseFarmingTotalBumps {
        fn default() -> Self {
            IncreaseFarmingTotalBumps {}
        }
    }
    impl<'info> anchor_lang::Bumps for IncreaseFarmingTotal<'info>
    where
        'info: 'info,
    {
        type Bumps = IncreaseFarmingTotalBumps;
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
    /// instead of an `AccountInfo`. This is useful for clients that want
    /// to generate a list of accounts, without explicitly knowing the
    /// order all the fields should be in.
    ///
    /// To access the struct in this module, one should use the sibling
    /// `accounts` module (also generated), which re-exports this.
    pub(crate) mod __client_accounts_increase_farming_total {
        use super::*;
        use anchor_lang::prelude::borsh;
        /// Generated client accounts for [`IncreaseFarmingTotal`].
        pub struct IncreaseFarmingTotal {
            pub pool: Pubkey,
            pub farming_state: Pubkey,
            pub farming_token_vault: Pubkey,
            pub farming_token_account: Pubkey,
            pub wallet_authority: Pubkey,
            pub initializer_account: Pubkey,
            pub token_program: Pubkey,
        }
        impl borsh::ser::BorshSerialize for IncreaseFarmingTotal
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.pool, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_state, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_vault, writer)?;
                borsh::BorshSerialize::serialize(&self.farming_token_account, writer)?;
                borsh::BorshSerialize::serialize(&self.wallet_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.initializer_account, writer)?;
                borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::ToAccountMetas for IncreaseFarmingTotal {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.pool,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_state,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_vault,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            self.farming_token_account,
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.wallet_authority,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.initializer_account,
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                account_metas
            }
        }
    }
    /// An internal, Anchor generated module. This is used (as an
    /// implementation detail), to generate a CPI struct for a given
    /// `#[derive(Accounts)]` implementation, where each field is an
    /// AccountInfo.
    ///
    /// To access the struct in this module, one should use the sibling
    /// [`cpi::accounts`] module (also generated), which re-exports this.
    pub(crate) mod __cpi_client_accounts_increase_farming_total {
        use super::*;
        /// Generated CPI struct of the accounts for [`IncreaseFarmingTotal`].
        pub struct IncreaseFarmingTotal<'info> {
            pub pool: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            pub farming_state: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_token_vault: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub farming_token_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub wallet_authority: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub initializer_account: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
            pub token_program: anchor_lang::solana_program::account_info::AccountInfo<
                'info,
            >,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IncreaseFarmingTotal<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.pool),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_state),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_vault),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new(
                            anchor_lang::Key::key(&self.farming_token_account),
                            false,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.wallet_authority),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.initializer_account),
                            true,
                        ),
                    );
                account_metas
                    .push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseFarmingTotal<'info> {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos
                    .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.pool));
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_state,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_vault,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.farming_token_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.wallet_authority,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.initializer_account,
                        ),
                    );
                account_infos
                    .extend(
                        anchor_lang::ToAccountInfos::to_account_infos(
                            &self.token_program,
                        ),
                    );
                account_infos
            }
        }
    }
}
use ix_accounts::*;
pub use state::*;
pub use typedefs::*;
use self::mm_farming_pool::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct MmFarmingPool;
    #[automatically_derived]
    impl ::core::clone::Clone for MmFarmingPool {
        #[inline]
        fn clone(&self) -> MmFarmingPool {
            MmFarmingPool
        }
    }
    impl anchor_lang::Id for MmFarmingPool {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one type. "global" for a
/// regular instruction.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::InitializeConstProductCurve::DISCRIMINATOR => {
            __private::__global::initialize_const_product_curve(
                program_id,
                accounts,
                ix_data,
            )
        }
        instruction::InitializeStableCurve::DISCRIMINATOR => {
            __private::__global::initialize_stable_curve(program_id, accounts, ix_data)
        }
        instruction::Initialize::DISCRIMINATOR => {
            __private::__global::initialize(program_id, accounts, ix_data)
        }
        instruction::GetCreationBasket::DISCRIMINATOR => {
            __private::__global::get_creation_basket(program_id, accounts, ix_data)
        }
        instruction::GetRedemptionBasket::DISCRIMINATOR => {
            __private::__global::get_redemption_basket(program_id, accounts, ix_data)
        }
        instruction::CreateBasket::DISCRIMINATOR => {
            __private::__global::create_basket(program_id, accounts, ix_data)
        }
        instruction::RedeemBasket::DISCRIMINATOR => {
            __private::__global::redeem_basket(program_id, accounts, ix_data)
        }
        instruction::Swap::DISCRIMINATOR => {
            __private::__global::swap(program_id, accounts, ix_data)
        }
        instruction::InitializeFarming::DISCRIMINATOR => {
            __private::__global::initialize_farming(program_id, accounts, ix_data)
        }
        instruction::WithdrawFarmingVaultTokens::DISCRIMINATOR => {
            __private::__global::withdraw_farming_vault_tokens(
                program_id,
                accounts,
                ix_data,
            )
        }
        instruction::StartFarming::DISCRIMINATOR => {
            __private::__global::start_farming(program_id, accounts, ix_data)
        }
        instruction::InitializeFarmingCalc::DISCRIMINATOR => {
            __private::__global::initialize_farming_calc(program_id, accounts, ix_data)
        }
        instruction::CloseFarmingCalc::DISCRIMINATOR => {
            __private::__global::close_farming_calc(program_id, accounts, ix_data)
        }
        instruction::CalculateFarmed::DISCRIMINATOR => {
            __private::__global::calculate_farmed(program_id, accounts, ix_data)
        }
        instruction::WithdrawFarmed::DISCRIMINATOR => {
            __private::__global::withdraw_farmed(program_id, accounts, ix_data)
        }
        instruction::CheckFarmed::DISCRIMINATOR => {
            __private::__global::check_farmed(program_id, accounts, ix_data)
        }
        instruction::EndFarming::DISCRIMINATOR => {
            __private::__global::end_farming(program_id, accounts, ix_data)
        }
        instruction::TakeFarmingSnapshot::DISCRIMINATOR => {
            __private::__global::take_farming_snapshot(program_id, accounts, ix_data)
        }
        instruction::IncreaseFarmingTotal::DISCRIMINATOR => {
            __private::__global::increase_farming_total(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch<'info>(
            program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123, 158]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "gen/aldirn-amm/src/lib.rs",
                                        line: 2u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] = [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps>
        for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !&to.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.base = __bump;
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !&program.executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdlCreateAccountsBumps",
                    "base",
                    &&self.base,
                )
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self {
                IdlCreateAccountsBumps {
                    base: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self {
                IdlAccountsBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps>
        for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlResizeAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self {
                IdlResizeAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlResizeAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps>
        for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self {
                IdlCreateBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps>
        for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlSetBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self {
                IdlSetBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlSetBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps>
        for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCloseAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !&sol_destination.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self {
                IdlCloseAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCloseAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space = idl_ref
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(idl_ref.data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > idl_ref.data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info().clone(),
                        },
                    ),
                    new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap(),
                )?;
                idl_ref.realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts.idl.data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "gen/aldirn-amm/src/lib.rs",
                                    line: 2u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn initialize_const_product_curve<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeConstProductCurve");
            let ix = instruction::InitializeConstProductCurve::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeConstProductCurve = ix;
            let mut __bumps = <InitializeConstProductCurve as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeConstProductCurve::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::initialize_const_product_curve(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_stable_curve<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeStableCurve");
            let ix = instruction::InitializeStableCurve::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeStableCurve { _amp } = ix;
            let mut __bumps = <InitializeStableCurve as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeStableCurve::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::initialize_stable_curve(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _amp,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Initialize");
            let ix = instruction::Initialize::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Initialize { _signer_nonce, _curve_type } = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::initialize(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _signer_nonce,
                _curve_type,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn get_creation_basket<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: GetCreationBasket");
            let ix = instruction::GetCreationBasket::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::GetCreationBasket { _tokens_requested } = ix;
            let mut __bumps = <GetCreationBasket as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = GetCreationBasket::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::get_creation_basket(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _tokens_requested,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn get_redemption_basket<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: GetRedemptionBasket");
            let ix = instruction::GetRedemptionBasket::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::GetRedemptionBasket { _tokens_requested } = ix;
            let mut __bumps = <GetRedemptionBasket as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = GetRedemptionBasket::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::get_redemption_basket(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _tokens_requested,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn create_basket<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateBasket");
            let ix = instruction::CreateBasket::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CreateBasket {
                _creation_size,
                _base_token_used_max,
                _quote_token_used_max,
            } = ix;
            let mut __bumps = <CreateBasket as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CreateBasket::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::create_basket(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _creation_size,
                _base_token_used_max,
                _quote_token_used_max,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn redeem_basket<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: RedeemBasket");
            let ix = instruction::RedeemBasket::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::RedeemBasket {
                _redemption_size,
                _base_token_returned_min,
                _quote_token_returned_min,
            } = ix;
            let mut __bumps = <RedeemBasket as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = RedeemBasket::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::redeem_basket(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _redemption_size,
                _base_token_returned_min,
                _quote_token_returned_min,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn swap<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Swap");
            let ix = instruction::Swap::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Swap { _tokens, _min_tokens, _side } = ix;
            let mut __bumps = <Swap as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Swap::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::swap(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _tokens,
                _min_tokens,
                _side,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_farming<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeFarming");
            let ix = instruction::InitializeFarming::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeFarming {
                _token_amount,
                _tokens_per_period,
                _period_length,
                _no_withdraw_period_seconds,
                _vesting_period_seconds,
            } = ix;
            let mut __bumps = <InitializeFarming as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeFarming::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::initialize_farming(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _token_amount,
                _tokens_per_period,
                _period_length,
                _no_withdraw_period_seconds,
                _vesting_period_seconds,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_farming_vault_tokens<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawFarmingVaultTokens");
            let ix = instruction::WithdrawFarmingVaultTokens::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::WithdrawFarmingVaultTokens = ix;
            let mut __bumps = <WithdrawFarmingVaultTokens as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = WithdrawFarmingVaultTokens::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::withdraw_farming_vault_tokens(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn start_farming<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: StartFarming");
            let ix = instruction::StartFarming::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::StartFarming { _pool_token_amount } = ix;
            let mut __bumps = <StartFarming as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = StartFarming::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::start_farming(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _pool_token_amount,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn initialize_farming_calc<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: InitializeFarmingCalc");
            let ix = instruction::InitializeFarmingCalc::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::InitializeFarmingCalc = ix;
            let mut __bumps = <InitializeFarmingCalc as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = InitializeFarmingCalc::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::initialize_farming_calc(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn close_farming_calc<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CloseFarmingCalc");
            let ix = instruction::CloseFarmingCalc::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CloseFarmingCalc = ix;
            let mut __bumps = <CloseFarmingCalc as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CloseFarmingCalc::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::close_farming_calc(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn calculate_farmed<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CalculateFarmed");
            let ix = instruction::CalculateFarmed::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CalculateFarmed { _max_snapshots } = ix;
            let mut __bumps = <CalculateFarmed as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CalculateFarmed::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::calculate_farmed(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _max_snapshots,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn withdraw_farmed<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: WithdrawFarmed");
            let ix = instruction::WithdrawFarmed::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::WithdrawFarmed = ix;
            let mut __bumps = <WithdrawFarmed as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = WithdrawFarmed::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::withdraw_farmed(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn check_farmed<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CheckFarmed");
            let ix = instruction::CheckFarmed::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CheckFarmed { _max_snapshots } = ix;
            let mut __bumps = <CheckFarmed as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CheckFarmed::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::check_farmed(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _max_snapshots,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn end_farming<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: EndFarming");
            let ix = instruction::EndFarming::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::EndFarming = ix;
            let mut __bumps = <EndFarming as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = EndFarming::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::end_farming(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn take_farming_snapshot<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: TakeFarmingSnapshot");
            let ix = instruction::TakeFarmingSnapshot::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::TakeFarmingSnapshot = ix;
            let mut __bumps = <TakeFarmingSnapshot as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = TakeFarmingSnapshot::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::take_farming_snapshot(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn increase_farming_total<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IncreaseFarmingTotal");
            let ix = instruction::IncreaseFarmingTotal::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::IncreaseFarmingTotal { _increase_amount } = ix;
            let mut __bumps = <IncreaseFarmingTotal as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = IncreaseFarmingTotal::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mm_farming_pool::increase_farming_total(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                _increase_amount,
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod mm_farming_pool {
    //! Anchor CPI crate generated from mm_farming_pool v0.0.0 using [anchor-gen](https://crates.io/crates/anchor-gen) v0.3.1.
    use super::*;
    pub fn initialize_const_product_curve(
        _ctx: Context<InitializeConstProductCurve>,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn initialize_stable_curve(
        _ctx: Context<InitializeStableCurve>,
        _amp: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn initialize(
        _ctx: Context<Initialize>,
        _signer_nonce: u8,
        _curve_type: u8,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn get_creation_basket(
        _ctx: Context<GetCreationBasket>,
        _tokens_requested: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn get_redemption_basket(
        _ctx: Context<GetRedemptionBasket>,
        _tokens_requested: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn create_basket(
        _ctx: Context<CreateBasket>,
        _creation_size: u64,
        _base_token_used_max: u64,
        _quote_token_used_max: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn redeem_basket(
        _ctx: Context<RedeemBasket>,
        _redemption_size: u64,
        _base_token_returned_min: u64,
        _quote_token_returned_min: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn swap(
        _ctx: Context<Swap>,
        _tokens: u64,
        _min_tokens: u64,
        _side: Side,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn initialize_farming(
        _ctx: Context<InitializeFarming>,
        _token_amount: u64,
        _tokens_per_period: u64,
        _period_length: u64,
        _no_withdraw_period_seconds: i64,
        _vesting_period_seconds: i64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn withdraw_farming_vault_tokens(
        _ctx: Context<WithdrawFarmingVaultTokens>,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn start_farming(
        _ctx: Context<StartFarming>,
        _pool_token_amount: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn initialize_farming_calc(_ctx: Context<InitializeFarmingCalc>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn close_farming_calc(_ctx: Context<CloseFarmingCalc>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn calculate_farmed(
        _ctx: Context<CalculateFarmed>,
        _max_snapshots: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn withdraw_farmed(_ctx: Context<WithdrawFarmed>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn check_farmed(_ctx: Context<CheckFarmed>, _max_snapshots: u64) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn end_farming(_ctx: Context<EndFarming>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn take_farming_snapshot(_ctx: Context<TakeFarmingSnapshot>) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
    pub fn increase_farming_total(
        _ctx: Context<IncreaseFarmingTotal>,
        _increase_amount: u64,
    ) -> Result<()> {
        {
            ::core::panicking::panic_fmt(
                format_args!(
                    "not implemented: {0}",
                    format_args!("This program is a wrapper for CPI."),
                ),
            );
        }
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct InitializeConstProductCurve;
    impl borsh::ser::BorshSerialize for InitializeConstProductCurve {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeConstProductCurve {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for InitializeConstProductCurve {
        const DISCRIMINATOR: [u8; 8] = [208, 105, 224, 205, 146, 224, 21, 189];
    }
    impl anchor_lang::InstructionData for InitializeConstProductCurve {}
    impl anchor_lang::Owner for InitializeConstProductCurve {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeStableCurve {
        pub _amp: u64,
    }
    impl borsh::ser::BorshSerialize for InitializeStableCurve
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._amp, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeStableCurve
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _amp: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeStableCurve {
        const DISCRIMINATOR: [u8; 8] = [233, 39, 188, 215, 250, 137, 155, 144];
    }
    impl anchor_lang::InstructionData for InitializeStableCurve {}
    impl anchor_lang::Owner for InitializeStableCurve {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Initialize {
        pub _signer_nonce: u8,
        pub _curve_type: u8,
    }
    impl borsh::ser::BorshSerialize for Initialize
    where
        u8: borsh::ser::BorshSerialize,
        u8: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._signer_nonce, writer)?;
            borsh::BorshSerialize::serialize(&self._curve_type, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Initialize
    where
        u8: borsh::BorshDeserialize,
        u8: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _signer_nonce: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _curve_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    }
    impl anchor_lang::InstructionData for Initialize {}
    impl anchor_lang::Owner for Initialize {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct GetCreationBasket {
        pub _tokens_requested: u64,
    }
    impl borsh::ser::BorshSerialize for GetCreationBasket
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._tokens_requested, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for GetCreationBasket
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _tokens_requested: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for GetCreationBasket {
        const DISCRIMINATOR: [u8; 8] = [171, 193, 72, 164, 85, 158, 80, 44];
    }
    impl anchor_lang::InstructionData for GetCreationBasket {}
    impl anchor_lang::Owner for GetCreationBasket {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct GetRedemptionBasket {
        pub _tokens_requested: u64,
    }
    impl borsh::ser::BorshSerialize for GetRedemptionBasket
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._tokens_requested, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for GetRedemptionBasket
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _tokens_requested: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for GetRedemptionBasket {
        const DISCRIMINATOR: [u8; 8] = [61, 105, 56, 131, 232, 206, 128, 190];
    }
    impl anchor_lang::InstructionData for GetRedemptionBasket {}
    impl anchor_lang::Owner for GetRedemptionBasket {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CreateBasket {
        pub _creation_size: u64,
        pub _base_token_used_max: u64,
        pub _quote_token_used_max: u64,
    }
    impl borsh::ser::BorshSerialize for CreateBasket
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._creation_size, writer)?;
            borsh::BorshSerialize::serialize(&self._base_token_used_max, writer)?;
            borsh::BorshSerialize::serialize(&self._quote_token_used_max, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateBasket
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _creation_size: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _base_token_used_max: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                _quote_token_used_max: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Discriminator for CreateBasket {
        const DISCRIMINATOR: [u8; 8] = [47, 105, 155, 148, 15, 169, 202, 211];
    }
    impl anchor_lang::InstructionData for CreateBasket {}
    impl anchor_lang::Owner for CreateBasket {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct RedeemBasket {
        pub _redemption_size: u64,
        pub _base_token_returned_min: u64,
        pub _quote_token_returned_min: u64,
    }
    impl borsh::ser::BorshSerialize for RedeemBasket
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._redemption_size, writer)?;
            borsh::BorshSerialize::serialize(&self._base_token_returned_min, writer)?;
            borsh::BorshSerialize::serialize(&self._quote_token_returned_min, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for RedeemBasket
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _redemption_size: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _base_token_returned_min: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                _quote_token_returned_min: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Discriminator for RedeemBasket {
        const DISCRIMINATOR: [u8; 8] = [37, 133, 222, 57, 189, 160, 151, 41];
    }
    impl anchor_lang::InstructionData for RedeemBasket {}
    impl anchor_lang::Owner for RedeemBasket {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Swap {
        pub _tokens: u64,
        pub _min_tokens: u64,
        pub _side: Side,
    }
    impl borsh::ser::BorshSerialize for Swap
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Side: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._tokens, writer)?;
            borsh::BorshSerialize::serialize(&self._min_tokens, writer)?;
            borsh::BorshSerialize::serialize(&self._side, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Swap
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Side: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _tokens: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _min_tokens: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _side: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Swap {
        const DISCRIMINATOR: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
    }
    impl anchor_lang::InstructionData for Swap {}
    impl anchor_lang::Owner for Swap {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeFarming {
        pub _token_amount: u64,
        pub _tokens_per_period: u64,
        pub _period_length: u64,
        pub _no_withdraw_period_seconds: i64,
        pub _vesting_period_seconds: i64,
    }
    impl borsh::ser::BorshSerialize for InitializeFarming
    where
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
        i64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._token_amount, writer)?;
            borsh::BorshSerialize::serialize(&self._tokens_per_period, writer)?;
            borsh::BorshSerialize::serialize(&self._period_length, writer)?;
            borsh::BorshSerialize::serialize(&self._no_withdraw_period_seconds, writer)?;
            borsh::BorshSerialize::serialize(&self._vesting_period_seconds, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeFarming
    where
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
        i64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _token_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _tokens_per_period: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _period_length: borsh::BorshDeserialize::deserialize_reader(reader)?,
                _no_withdraw_period_seconds: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
                _vesting_period_seconds: borsh::BorshDeserialize::deserialize_reader(
                    reader,
                )?,
            })
        }
    }
    impl anchor_lang::Discriminator for InitializeFarming {
        const DISCRIMINATOR: [u8; 8] = [20, 125, 200, 32, 235, 51, 109, 72];
    }
    impl anchor_lang::InstructionData for InitializeFarming {}
    impl anchor_lang::Owner for InitializeFarming {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct WithdrawFarmingVaultTokens;
    impl borsh::ser::BorshSerialize for WithdrawFarmingVaultTokens {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawFarmingVaultTokens {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for WithdrawFarmingVaultTokens {
        const DISCRIMINATOR: [u8; 8] = [168, 180, 25, 171, 154, 214, 38, 124];
    }
    impl anchor_lang::InstructionData for WithdrawFarmingVaultTokens {}
    impl anchor_lang::Owner for WithdrawFarmingVaultTokens {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct StartFarming {
        pub _pool_token_amount: u64,
    }
    impl borsh::ser::BorshSerialize for StartFarming
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._pool_token_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for StartFarming
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _pool_token_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for StartFarming {
        const DISCRIMINATOR: [u8; 8] = [150, 205, 185, 109, 97, 202, 68, 110];
    }
    impl anchor_lang::InstructionData for StartFarming {}
    impl anchor_lang::Owner for StartFarming {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct InitializeFarmingCalc;
    impl borsh::ser::BorshSerialize for InitializeFarmingCalc {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for InitializeFarmingCalc {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for InitializeFarmingCalc {
        const DISCRIMINATOR: [u8; 8] = [143, 65, 168, 123, 247, 38, 168, 93];
    }
    impl anchor_lang::InstructionData for InitializeFarmingCalc {}
    impl anchor_lang::Owner for InitializeFarmingCalc {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CloseFarmingCalc;
    impl borsh::ser::BorshSerialize for CloseFarmingCalc {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CloseFarmingCalc {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CloseFarmingCalc {
        const DISCRIMINATOR: [u8; 8] = [200, 99, 247, 252, 122, 122, 160, 174];
    }
    impl anchor_lang::InstructionData for CloseFarmingCalc {}
    impl anchor_lang::Owner for CloseFarmingCalc {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CalculateFarmed {
        pub _max_snapshots: u64,
    }
    impl borsh::ser::BorshSerialize for CalculateFarmed
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._max_snapshots, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CalculateFarmed
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _max_snapshots: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for CalculateFarmed {
        const DISCRIMINATOR: [u8; 8] = [159, 148, 91, 176, 198, 108, 27, 148];
    }
    impl anchor_lang::InstructionData for CalculateFarmed {}
    impl anchor_lang::Owner for CalculateFarmed {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct WithdrawFarmed;
    impl borsh::ser::BorshSerialize for WithdrawFarmed {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for WithdrawFarmed {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for WithdrawFarmed {
        const DISCRIMINATOR: [u8; 8] = [175, 95, 99, 74, 63, 66, 237, 61];
    }
    impl anchor_lang::InstructionData for WithdrawFarmed {}
    impl anchor_lang::Owner for WithdrawFarmed {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct CheckFarmed {
        pub _max_snapshots: u64,
    }
    impl borsh::ser::BorshSerialize for CheckFarmed
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._max_snapshots, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CheckFarmed
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _max_snapshots: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for CheckFarmed {
        const DISCRIMINATOR: [u8; 8] = [150, 144, 156, 13, 122, 84, 19, 227];
    }
    impl anchor_lang::InstructionData for CheckFarmed {}
    impl anchor_lang::Owner for CheckFarmed {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct EndFarming;
    impl borsh::ser::BorshSerialize for EndFarming {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for EndFarming {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for EndFarming {
        const DISCRIMINATOR: [u8; 8] = [49, 90, 68, 217, 222, 198, 89, 21];
    }
    impl anchor_lang::InstructionData for EndFarming {}
    impl anchor_lang::Owner for EndFarming {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct TakeFarmingSnapshot;
    impl borsh::ser::BorshSerialize for TakeFarmingSnapshot {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for TakeFarmingSnapshot {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for TakeFarmingSnapshot {
        const DISCRIMINATOR: [u8; 8] = [42, 72, 81, 36, 91, 173, 245, 2];
    }
    impl anchor_lang::InstructionData for TakeFarmingSnapshot {}
    impl anchor_lang::Owner for TakeFarmingSnapshot {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct IncreaseFarmingTotal {
        pub _increase_amount: u64,
    }
    impl borsh::ser::BorshSerialize for IncreaseFarmingTotal
    where
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self._increase_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for IncreaseFarmingTotal
    where
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                _increase_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for IncreaseFarmingTotal {
        const DISCRIMINATOR: [u8; 8] = [22, 151, 57, 111, 66, 5, 7, 61];
    }
    impl anchor_lang::InstructionData for IncreaseFarmingTotal {}
    impl anchor_lang::Owner for IncreaseFarmingTotal {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_increase_farming_total::*;
    pub use crate::__client_accounts_create_basket::*;
    pub use crate::__client_accounts_initialize::*;
    pub use crate::__client_accounts_check_farmed::*;
    pub use crate::__client_accounts_withdraw_farmed::*;
    pub use crate::__client_accounts_take_farming_snapshot::*;
    pub use crate::__client_accounts_get_redemption_basket::*;
    pub use crate::__client_accounts_redeem_basket::*;
    pub use crate::__client_accounts_initialize_stable_curve::*;
    pub use crate::__client_accounts_get_creation_basket::*;
    pub use crate::__client_accounts_withdraw_farming_vault_tokens::*;
    pub use crate::__client_accounts_initialize_farming::*;
    pub use crate::__client_accounts_swap::*;
    pub use crate::__client_accounts_initialize_farming_calc::*;
    pub use crate::__client_accounts_start_farming::*;
    pub use crate::__client_accounts_initialize_const_product_curve::*;
    pub use crate::__client_accounts_calculate_farmed::*;
    pub use crate::__client_accounts_end_farming::*;
    pub use crate::__client_accounts_close_farming_calc::*;
}
/// The const program ID.
pub const ID: ::solana_program::pubkey::Pubkey = ::solana_program::pubkey::Pubkey::new_from_array([
    5u8,
    69u8,
    227u8,
    101u8,
    190u8,
    242u8,
    113u8,
    173u8,
    117u8,
    53u8,
    3u8,
    103u8,
    86u8,
    93u8,
    164u8,
    13u8,
    163u8,
    54u8,
    220u8,
    28u8,
    135u8,
    155u8,
    177u8,
    84u8,
    138u8,
    122u8,
    252u8,
    197u8,
    90u8,
    169u8,
    57u8,
    30u8,
]);
/// Returns `true` if given pubkey is the program ID.
pub fn check_id(id: &::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID.
pub const fn id() -> ::solana_program::pubkey::Pubkey {
    ID
}
