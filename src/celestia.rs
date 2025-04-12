use anyhow::Result;
use celestia_rpc::{Client, TxConfig, StateClient};
use celestia_types::{AppVersion, blob::{Blob, RawBlob}, state::RawTxResponse};
use serde::Serialize;
use base64::Engine;

use crate::constants::get_wordtia_namespace;

/// Encode a serializable object to JSON, base64 it, and prepare a Blob.
pub fn prepare_blob<T: Serialize>(data: &T) -> Result<RawBlob> {
    let json = serde_json::to_string(data)?;
    let encoded = base64::engine::general_purpose::STANDARD.encode(&json);
    let namespace = get_wordtia_namespace()?;

    // Create a proper Blob first, then convert to RawBlob
    let blob = Blob::new(
        namespace,
        encoded.into_bytes(),
        AppVersion::V2 // Use V2 as in the working example
    )?;

    // Convert to RawBlob
    Ok(blob.into())
}

/// Submit a blob to Celestia using the RPC client.
pub async fn submit_blob_to_celestia(
    client: &Client,
    blob: RawBlob,
    config: TxConfig,
) -> Result<RawTxResponse> {
    match client.state_submit_pay_for_blob(&[blob], config).await {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("❌ Celestia RPC error: {}", e);
            Err(e.into())
        }
    }
}

/// Display final result to the user via console.
pub fn log_submission_result(response: &RawTxResponse) {
    println!("✅ Game submitted to Celestia!");
    println!("🔗 Included in block: {}", response.height);
    println!(
        "🌐 View on Celenium: https://mocha-4.celenium.io/tx/{}",
        response.txhash
    );
}