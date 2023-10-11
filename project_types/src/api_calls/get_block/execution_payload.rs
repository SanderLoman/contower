use derivative::Derivative;
use ethereum_types::Address;
use serde_derive::{Serialize, Deserialize};
use ssz_derive::Encode;
use ssz_types::{FixedVector, VariableList};
use tree_hash_derive::TreeHash;

use crate::{EthSpec, execution_block_hash::ExecutionBlockHash, Hash256, Uint256};

use super::{Transactions, withdrawal::Withdrawal};

pub type Withdrawals<T> = VariableList<Withdrawal, <T as EthSpec>::MaxWithdrawalsPerPayload>;

#[derive(
    Debug, Clone, Serialize, Encode, Deserialize, TreeHash, Derivative, 
    // arbitrary::Arbitrary,
)]
// #[arbitrary(bound = "T: EthSpec")]
pub struct ExecutionPayload<T: EthSpec> {
    pub parent_hash: ExecutionBlockHash,
    pub fee_recipient: Address,
    pub state_root: Hash256,
    pub receipts_root: Hash256,
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, T::BytesPerLogsBloom>,
    pub prev_randao: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub block_number: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub gas_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub gas_used: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub timestamp: u64,
    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub extra_data: VariableList<u8, T::MaxExtraDataBytes>,
    #[serde(with = "serde_utils::quoted_u256")]
    pub base_fee_per_gas: Uint256,
    pub block_hash: ExecutionBlockHash,
    #[serde(with = "ssz_types::serde_utils::list_of_hex_var_list")]
    pub transactions: Transactions<T>,
    pub withdrawals: Withdrawals<T>,
}
