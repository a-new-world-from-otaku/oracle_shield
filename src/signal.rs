use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal::ctrl_c;

pub async fn set_signal_handler(running: Arc<AtomicBool>) {
    let r = running.clone();
    ctrl_c().await.expect("Error setting Ctrl-C handler");
    r.store(false, Ordering::SeqCst);
}
