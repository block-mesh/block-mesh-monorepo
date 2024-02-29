#[tracing::instrument(name = "Health check")]
pub async fn handler() -> &'static str {
    "OK"
}
