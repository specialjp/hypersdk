
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_ctx_deserialization() {
        let json = r#"{
            "dayNtlVlm": "123456.78",
            "funding": "0.0001",
            "impactPxs": ["50000.0", "49999.0"],
            "markPx": "50000.5",
            "midPx": "50000.25",
            "openInterest": "999999.99",
            "oraclePx": "50001.0",
            "premium": "0.0005",
            "prevDayPx": "49500.0"
        }"#;

        let ctx: AssetCtx = serde_json::from_str(json).unwrap();
        assert_eq!(ctx.mark_px, rust_decimal::dec!(50000.5));
        assert_eq!(ctx.open_interest, rust_decimal::dec!(999999.99));
        assert!(ctx.mid_px.is_some());
        assert!(ctx.impact_pxs.is_some());
    }

    #[test]
    fn test_asset_ctx_optional_fields() {
        let json = r#"{
            "dayNtlVlm": "100.0",
            "funding": "0.0",
            "markPx": "1000.0",
            "openInterest": "500.0",
            "oraclePx": "1000.0",
            "prevDayPx": "999.0"
        }"#;

        let ctx: AssetCtx = serde_json::from_str(json).unwrap();
        assert!(ctx.mid_px.is_none());
        assert!(ctx.impact_pxs.is_none());
        assert!(ctx.premium.is_none());
    }
}
