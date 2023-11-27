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
    pub Timestamp: Option<String>,
    pub pool: Option<String>,
    pub t0_price_usd: Option<f32>,
    pub t1_price_usd: Option<f32>,
    pub token_info: Option<String>,
}
