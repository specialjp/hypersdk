
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCtx {
    pub day_ntl_vlm: Decimal,
    pub funding: Decimal,
    pub impact_pxs: Option<Vec<Decimal>>,
    pub mark_px: Decimal,
    pub mid_px: Option<Decimal>,
    pub open_interest: Decimal,
    pub oracle_px: Decimal,
    pub premium: Option<Decimal>,
    pub prev_day_px: Decimal,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetaAndAssetCtxsResponse(pub(crate) crate::hypercore::PerpTokens, pub Vec<AssetCtx>);
