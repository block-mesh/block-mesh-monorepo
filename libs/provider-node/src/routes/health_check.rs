#[tracing::instrument(name = "health_check")]
pub async fn health_check() -> &'static str {
    "OK"
}
