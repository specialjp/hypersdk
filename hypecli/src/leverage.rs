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
