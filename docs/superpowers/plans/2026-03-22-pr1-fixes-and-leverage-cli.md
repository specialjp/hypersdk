# PR #1 Fixes, Tests, and Leverage CLI Command Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix PR #1 issues (.DS_Store, breaking changes), add tests for new SDK features, and wire up a `leverage` CLI command in hypecli.

**Architecture:** Work on the `backup` branch. Fix hygiene issues first, then add SDK tests for the new types/actions, then add the CLI leverage command following existing patterns (SignerArgs + asset resolution + sign-and-send).

**Tech Stack:** Rust, clap 4.5, hypersdk, serde, tokio, alloy

---

## File Structure

| File | Action | Responsibility |
|------|--------|----------------|
| `.gitignore` | Modify | Add `.DS_Store` pattern |
| `.DS_Store`, `examples/.DS_Store`, `skills/.DS_Store`, `src/.DS_Store` | Delete | Remove committed macOS artifacts |
| `src/hypercore/types/api.rs` | Modify (on backup branch) | Add `UpdateLeverage` to signing match arms — already in PR, add test |
| `src/hypercore/types/mod.rs` | Modify (on backup branch) | `Builder`, `BatchOrder.builder`, `Dex.assets` — already in PR, add tests |
| `src/hypercore/types/asset_ctx.rs` | Already in PR | `AssetCtx`, `MetaAndAssetCtxsResponse` — add test |
| `src/hypercore/signing.rs` | Modify (on backup branch) | Add test for `UpdateLeverage` signing |
| `src/hypercore/mod.rs` | Modify (on backup branch) | `PerpTokens` pub visibility, `perp_meta_and_asset_ctxs` — already in PR |
| `hypecli/src/leverage.rs` | Create | New CLI command for updating leverage |
| `hypecli/src/main.rs` | Modify | Register leverage command |

---

### Task 1: Switch to backup branch and clean up .DS_Store files

**Files:**
- Modify: `.gitignore`
- Delete: `.DS_Store`, `examples/.DS_Store`, `skills/.DS_Store`, `src/.DS_Store`

- [ ] **Step 1: Switch to the backup branch**

```bash
git checkout backup
```

- [ ] **Step 2: Add .DS_Store to .gitignore**

Append to `.gitignore`:
```
.DS_Store
```

- [ ] **Step 3: Remove .DS_Store files from git tracking**

```bash
git rm --cached .DS_Store examples/.DS_Store skills/.DS_Store src/.DS_Store
```

- [ ] **Step 4: Commit**

```bash
git add .gitignore
git commit -m "chore: remove .DS_Store files and add to .gitignore"
```

---

### Task 2: Add serialization test for Builder struct

**Files:**
- Modify: `src/hypercore/types/mod.rs` (test module at bottom of file)

- [ ] **Step 1: Write the failing test**

Add to the existing `#[cfg(test)] mod tests` block in `src/hypercore/types/mod.rs`:

```rust
#[test]
fn test_builder_serialization() {
    let builder = Builder {
        b: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
        f: 10,
    };
    let json = serde_json::to_string(&builder).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["b"], "0x1234567890abcdef1234567890abcdef12345678");
    assert_eq!(parsed["f"], 10);

    // Round-trip
    let deserialized: Builder = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.b, builder.b);
    assert_eq!(deserialized.f, builder.f);
}
```

- [ ] **Step 2: Run test to verify it passes**

```bash
cargo test -p hypersdk test_builder_serialization -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/types/mod.rs
git commit -m "test: add Builder serialization test"
```

---

### Task 3: Add serialization test for BatchOrder with builder field

**Files:**
- Modify: `src/hypercore/types/mod.rs` (test module)

- [ ] **Step 1: Write the test**

Add to the test module:

```rust
#[test]
fn test_batch_order_builder_field_serialization() {
    use rust_decimal::dec;

    // Without builder — "builder" key should be absent
    let batch = BatchOrder {
        orders: vec![OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: dec!(50000),
            sz: dec!(0.1),
            reduce_only: false,
            order_type: OrderTypePlacement::Limit {
                tif: TimeInForce::Gtc,
            },
            cloid: Default::default(),
        }],
        grouping: OrderGrouping::Na,
        builder: None,
    };
    let json = serde_json::to_string(&batch).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.get("builder").is_none(), "builder should be absent when None");

    // With builder — "builder" key should be present
    let batch_with_builder = BatchOrder {
        orders: vec![],
        grouping: OrderGrouping::Na,
        builder: Some(Builder {
            b: "0xabc".to_string(),
            f: 10,
        }),
    };
    let json2 = serde_json::to_string(&batch_with_builder).unwrap();
    let parsed2: serde_json::Value = serde_json::from_str(&json2).unwrap();
    assert_eq!(parsed2["builder"]["b"], "0xabc");
    assert_eq!(parsed2["builder"]["f"], 10);
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p hypersdk test_batch_order_builder_field_serialization -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/types/mod.rs
git commit -m "test: add BatchOrder builder field serialization test"
```

---

### Task 4: Add serialization test for UpdateLeverage

**Files:**
- Modify: `src/hypercore/types/api.rs` (test module at bottom)

- [ ] **Step 1: Write the test**

Add to `#[cfg(test)] mod tests` in `src/hypercore/types/api.rs`:

```rust
#[test]
fn update_leverage_serialization() {
    let action = Action::UpdateLeverage(UpdateLeverage {
        asset: 3,
        is_cross: true,
        leverage: 20,
    });

    let json = serde_json::to_string(&action).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["type"], "updateLeverage");
    assert_eq!(parsed["asset"], 3);
    assert_eq!(parsed["isCross"], true);
    assert_eq!(parsed["leverage"], 20);
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p hypersdk update_leverage_serialization -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/types/api.rs
git commit -m "test: add UpdateLeverage serialization test"
```

---

### Task 5: Add signing test for UpdateLeverage action

**Files:**
- Modify: `src/hypercore/signing.rs` (test module)

- [ ] **Step 1: Write the test**

Add to the existing test module in `src/hypercore/signing.rs`:

```rust
#[test]
fn test_sign_update_leverage() {
    use types::api::UpdateLeverage;

    let signer = get_signer();
    let expected_address = signer.address();

    let action = Action::UpdateLeverage(UpdateLeverage {
        asset: 3,
        is_cross: true,
        leverage: 20,
    });

    let nonce = chrono::Utc::now().timestamp_millis() as u64;
    let action_request = action
        .sign_sync(&signer, nonce, None, None, Chain::Mainnet)
        .unwrap();

    // Recover and verify
    let recovered = Action::UpdateLeverage(UpdateLeverage {
        asset: 3,
        is_cross: true,
        leverage: 20,
    })
    .recover(&action_request.signature, nonce, None, None, Chain::Mainnet)
    .unwrap();

    assert_eq!(
        recovered, expected_address,
        "Recovered address should match for UpdateLeverage action"
    );
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p hypersdk test_sign_update_leverage -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/signing.rs
git commit -m "test: add UpdateLeverage signing/recovery test"
```

---

### Task 6: Add deserialization test for AssetCtx

**Files:**
- Modify: `src/hypercore/types/asset_ctx.rs`

- [ ] **Step 1: Write the test**

Add at the bottom of `src/hypercore/types/asset_ctx.rs`:

```rust
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
```

- [ ] **Step 2: Run tests**

```bash
cargo test -p hypersdk test_asset_ctx -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/types/asset_ctx.rs
git commit -m "test: add AssetCtx deserialization tests"
```

---

### Task 7: Add MetaAndAssetCtxs InfoRequest serialization test

**Files:**
- Modify: `src/hypercore/types/mod.rs` (test module)

- [ ] **Step 1: Write the test**

Add to the test module in `src/hypercore/types/mod.rs`:

```rust
#[test]
fn test_meta_and_asset_ctxs_info_request_serialization() {
    // Without dex
    let req = super::InfoRequest::MetaAndAssetCtxs { dex: None };
    let json = serde_json::to_string(&req).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["type"], "metaAndAssetCtxs");
    assert!(parsed.get("dex").is_none());

    // With dex
    let req_with_dex = super::InfoRequest::MetaAndAssetCtxs {
        dex: Some("xyz".to_string()),
    };
    let json2 = serde_json::to_string(&req_with_dex).unwrap();
    let parsed2: serde_json::Value = serde_json::from_str(&json2).unwrap();
    assert_eq!(parsed2["type"], "metaAndAssetCtxs");
    assert_eq!(parsed2["dex"], "xyz");
}
```

- [ ] **Step 2: Run test**

```bash
cargo test -p hypersdk test_meta_and_asset_ctxs_info_request -- --nocapture
```
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/hypercore/types/mod.rs
git commit -m "test: add MetaAndAssetCtxs info request serialization test"
```

---

### Task 8: Create the leverage CLI command

**Files:**
- Create: `hypecli/src/leverage.rs`
- Modify: `hypecli/src/main.rs`

- [ ] **Step 1: Create `hypecli/src/leverage.rs`**

Follow the existing pattern from `send.rs` (signed action with SignerArgs + asset resolution):

```rust
use crate::utils::{find_signer_sync, resolve_asset};
use crate::SignerArgs;
use clap::Args;
use hypersdk::hypercore::http::Client as HttpClient;
use hypersdk::hypercore::NonceHandler;

/// Update leverage for a perpetual position.
///
/// Sets the leverage and margin mode (cross or isolated) for a given asset.
///
/// # Examples
///
/// Set 10x cross leverage on BTC:
///   hypecli leverage --keystore my_wallet --asset BTC --leverage 10 --cross
///
/// Set 5x isolated leverage on ETH:
///   hypecli leverage --keystore my_wallet --asset ETH --leverage 5
#[derive(Args)]
pub struct LeverageCmd {
    #[command(flatten)]
    signer: SignerArgs,

    /// Asset name (e.g., "BTC", "ETH", or "xyz:BTC" for HIP-3 DEX).
    #[arg(long)]
    asset: String,

    /// Leverage value (e.g., 1, 5, 10, 20, 50).
    #[arg(long)]
    leverage: u32,

    /// Use cross margin mode. If omitted, uses isolated margin.
    #[arg(long, default_value_t = false)]
    cross: bool,
}

impl LeverageCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        let signer = find_signer_sync(&self.signer)?;
        let client = HttpClient::new(self.signer.chain);

        let asset_index = resolve_asset(&client, &self.asset).await?;

        let nonce = NonceHandler::default().next();

        client
            .update_leverage(
                &signer,
                asset_index,
                self.cross,
                self.leverage,
                nonce,
                None,
                None,
            )
            .await?;

        println!(
            "Leverage updated: {} {}x {}",
            self.asset,
            self.leverage,
            if self.cross { "cross" } else { "isolated" }
        );

        Ok(())
    }
}
```

- [ ] **Step 2: Register the command in `hypecli/src/main.rs`**

Add `mod leverage;` to the module list at the top.

Add `use leverage::LeverageCmd;` to the imports.

Add to the `Command` enum:
```rust
/// Update leverage for a perpetual position
Leverage(LeverageCmd),
```

Add to the `Command::run` match:
```rust
Self::Leverage(cmd) => cmd.run().await,
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo build -p hypecli
```
Expected: Compiles successfully

- [ ] **Step 4: Verify help text appears**

```bash
cargo run -p hypecli -- leverage --help
```
Expected: Shows usage with `--asset`, `--leverage`, `--cross`, and signer args

- [ ] **Step 5: Commit**

```bash
git add hypecli/src/leverage.rs hypecli/src/main.rs
git commit -m "feat: add leverage CLI command for updating asset leverage"
```

---

### Task 9: Run all tests and verify

- [ ] **Step 1: Run full SDK test suite**

```bash
cargo test -p hypersdk
```
Expected: All tests pass (existing + new)

- [ ] **Step 2: Run full workspace build**

```bash
cargo build --workspace
```
Expected: Clean build with no errors

- [ ] **Step 3: Verify examples compile**

```bash
cargo build --examples
```
Expected: Examples compile with the new `builder: None` field

---

### Task 10: Clean up stale TODO comment

**Files:**
- Modify: `src/hypercore/mod.rs`

- [ ] **Step 1: Remove the stale `// TODO: perpDexs` comment**

The comment `// TODO: perpDexs` appears after `perp_meta_and_asset_ctxs` but `perp_dexs` is already implemented. Remove the line.

- [ ] **Step 2: Commit**

```bash
git add src/hypercore/mod.rs
git commit -m "chore: remove stale TODO comment (perpDexs already implemented)"
```
