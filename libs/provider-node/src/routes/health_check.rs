#[tracing::instrument(name = "health_check")]
pub async fn handler() -> &'static str {
    "OK"
}
