use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub redis: Arc<Mutex<redis::Client>>
}
