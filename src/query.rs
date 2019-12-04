use crate::json::Json;
use crate::storage_query::{AugmentClap, MapQuery, StorageQuery, ValueQuery};
use core::fmt::Debug;
use core::ops::Deref;
use node_template_runtime::Runtime;
use sr_primitives::AccountId32;
use structopt::StructOpt;
use substrate_consensus_babe_primitives::BabeAuthorityWeight;
use substrate_primitives::H256;
use substrate_primitives_storage::{StorageData, StorageKey};

#[derive(StructOpt, Debug)]
/// Key arguements should be provided as json.
pub enum Key {
    /// Extrinsics nonce for accounts.
    /// example, the nonce expected in Alice's next signed transaction:
    /// "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    SystemAccountNonce(MapQuery<srml_system::AccountNonce<Runtime>, AccountId32, u32>),
    /// Map of block numbers to block hashes.
	SystemBlockHash(MapQuery<srml_system::BlockHash<Runtime>, u32, H256>),

    TimestampNow(ValueQuery<srml_timestamp::Now<Runtime>, u64>),

    BabeEpochIndex(ValueQuery<srml_babe::EpochIndex, u64>),
    BabeAuthorities(
        ValueQuery<srml_babe::Authorities, Vec<(srml_babe::AuthorityId, BabeAuthorityWeight)>>,
    ),
    BabeGenesisSlot(ValueQuery<srml_babe::GenesisSlot, u64>),
    BabeCurrentSlot(ValueQuery<srml_babe::CurrentSlot, u64>),
    BabeRandomness(ValueQuery<srml_babe::Randomness, [u8; 32]>),

    BalancesTotalIssuance(ValueQuery<srml_balances::TotalIssuance<Runtime>, u128>),
    // BalanceLock and VestingSchedule do not implement Serde so cannot be serialized to json.
    BalancesFreeBalance(MapQuery<srml_balances::FreeBalance<Runtime>, AccountId32, u128>),
    BalancesReservedBalance(MapQuery<srml_balances::ReservedBalance<Runtime>, AccountId32, u128>),
}

impl StorageQuery for Key {
    fn to_raw_key(&self) -> StorageKey {
        self.deref().to_raw_key()
    }

    fn raw_scale_to_json(&self, encoded: StorageData) -> Result<Json, parity_scale_codec::Error> {
        self.deref().raw_scale_to_json(encoded)
    }
}

impl Deref for Key {
    type Target = dyn StorageQuery;

    fn deref(&self) -> &(dyn StorageQuery + 'static) {
        match self {
            Self::SystemAccountNonce(q) => q,
            Self::SystemBlockHash(q) => q,
            Self::TimestampNow(q) => q,
            Self::BabeEpochIndex(q) => q,
            Self::BabeAuthorities(q) => q,
            Self::BabeGenesisSlot(q) => q,
            Self::BabeCurrentSlot(q) => q,
            Self::BabeRandomness(q) => q,
            Self::BalancesTotalIssuance(q) => q,
            Self::BalancesFreeBalance(q) => q,
            Self::BalancesReservedBalance(q) => q,
        }
    }
}
