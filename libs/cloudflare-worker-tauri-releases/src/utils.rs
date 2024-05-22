use crate::types::LatestRelease;
use crate::LATEST_RELEASE;

#[tracing::instrument(name = "get_release", err)]
pub async fn get_release() -> anyhow::Result<LatestRelease> {
    Ok(reqwest::Client::new()
        .get(LATEST_RELEASE)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "block-mesh-monorepo")
        .send()
        .await?
        .json()
        .await?)
}

#[tracing::instrument(name = "get_json", err)]
pub async fn get_json(url: &str) -> anyhow::Result<serde_json::Value> {
    Ok(reqwest::Client::new()
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "block-mesh-monorepo")
        .send()
        .await?
        .json()
        .await?)
}
