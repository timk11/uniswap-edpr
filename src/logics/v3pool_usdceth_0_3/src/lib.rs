// Auto-generated code from manifest.
// You update the structure as needed.
// The existence of the SnapshotValue structure must be maintained.
use candid::{Decode, Encode};
#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct SnapshotValue {
    pub meta: Meta,
}

#[derive(Debug, Clone, candid::CandidType, candid::Deserialize, serde::Serialize, chainsight_cdk_macros::StableMemoryStorable)]
pub struct Meta {
    pub address: Option<String>,
    pub block: Option<u32>,
    pub create_event: Option<String>,
    pub event_count: Option<u32>,
    pub fee: Option<u32>,
    pub fee_growth_global0_x128: Option<u32>,
    pub fee_growth_global1_x128: Option<u32>,
    pub liquidity: Option<u32>,
    pub log_index: Option<u32>,
    pub max_liquidity_per_tick: Option<u32>,
    pub positions: Option<String>,
    pub protocol_token0_fees: Option<u32>,
    pub protocol_token1_fees: Option<u32>,
    pub sqrt_ratio_x96: Option<u32>,
    pub tick_current: Option<i32>,
    pub tick_spacing: Option<i32>,
    pub ticks: Option<String>,
    pub timestamp: Option<u32>,
    pub token0: Option<String>,
    pub token0_amount: Option<u32>,
    pub token0_price: Option<u32>,
    pub token1: Option<String>,
    pub token1_amount: Option<u32>,
    pub token1_price: Option<u32>,
    pub txn_index: Option<u32>,
    pub uncollected_fees_token0: Option<u32>,
    pub uncollected_fees_token1: Option<u32>,
}
