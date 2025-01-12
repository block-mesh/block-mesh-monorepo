use libmimalloc_sys::mi_collect;
use std::time::Duration;

#[tracing::instrument(name = "memory_loop", skip_all)]
pub async fn memory_loop() -> Result<(), anyhow::Error> {
    loop {
        unsafe {
            mi_collect(true);
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
