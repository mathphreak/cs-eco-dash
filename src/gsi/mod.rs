extern crate nickel;
extern crate time;
extern crate crc;

use nickel::{Router, HttpRouter};
use std::sync::{Arc, Mutex};

pub use self::middleware::State;
pub use self::version::Versions;

pub mod version;
mod middleware;
mod paths;

pub fn router(state_mutex: Arc<Mutex<State>>) -> Router<()> {
    let mut router = Router::new();
    let post_handler = middleware::PostHandler::new(state_mutex);
    let installer = middleware::Installer::new();
    router.post("/", post_handler);
    router.post("/install-gsi", installer);
    router
}
