use anyhow::*;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use reqwest::Client;
use std::fs::{File, remove_dir_all, remove_file};
use std::io::Write;
use std::path::PathBuf;
use tar::Archive;
use transcribe_rs::TranscriptionEngine;
use transcribe_rs::engines::parakeet::{ParakeetEngine, ParakeetModelParams};

pub async fn ensure_model_exists(uri: &str, path: &str) -> Result<()> {
    if !PathBuf::from(path).exists() {
        download_model(uri, path).await?;
        extract_archive(&PathBuf::from(path)).await?;
    } else {
        if try_load_model(path.into()).await {
            return Ok(());
        } else {
            remove_dir_all(path)?;
            download_model(uri, path).await?;
            extract_archive(&path.into())
                .await
                .context("Failed to extract archive")?;
        }
    }

    Ok(())
}

pub async fn extract_archive(source: &PathBuf) -> Result<()> {
    let tar_gz = File::open(source)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(".")?;

    remove_file(source)?;

    Ok(())
}

pub async fn try_load_model(path: PathBuf) -> bool {
    let mut engine = ParakeetEngine::new();

    let res = engine.load_model_with_params(&path, ParakeetModelParams::int8());

    res.is_ok()
}

pub async fn download_model(uri: &str, path: &str) -> Result<()> {
    let client = Client::new();
    let res = client
        .get(uri)
        .send()
        .await
        .context("failed to send request to client")?;

    if !res.status().is_success() {
        bail!("failed to download model: {}", res.status());
    }

    let mut file = File::create(path).context("failed to create file")?;

    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| anyhow::anyhow!("Error while downloading chunk: {}", e))?;
        file.write_all(&chunk).context("Failed to write chunk")?;
    }

    Ok(())
}
