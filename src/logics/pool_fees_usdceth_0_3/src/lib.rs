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
    pub fees_24h_usd: Option<f32>,
    pub pool_summary_level_1: Option<String>,
    pub volume_24h_usd: Option<f32>,
}
