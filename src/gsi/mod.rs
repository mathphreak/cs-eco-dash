extern crate nickel;
extern crate time;
extern crate crc;

use nickel::{Router, HttpRouter};
use std::sync::{Arc, Mutex};

pub use self::version::Versions;
use super::game;

pub mod version;
pub mod message;
mod middleware;
mod paths;

pub fn router(state_mutex: Arc<Mutex<game::State>>) -> Router<()> {
    let mut router = Router::new();
    let post_handler = middleware::GsiDataHandler::new(state_mutex.clone());
    let installer = middleware::Installer::new(state_mutex.clone());
    let prefs_handler = middleware::PrefsHandler::new();
    router.post("/", post_handler);
    router.post("/install-gsi", installer);
    router.post("/update-prefs", prefs_handler);
    router
}
