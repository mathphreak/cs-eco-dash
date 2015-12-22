extern crate nickel;
extern crate time;
extern crate crc;

use nickel::{Router, HttpRouter};
use std::sync::{Arc, Mutex};

pub use self::message::Message;
pub use self::version::Versions;
use super::game;

pub mod version;
mod message;
mod middleware;
mod paths;

pub fn router(state_mutex: Arc<Mutex<game::State>>) -> Router<()> {
    let mut router = Router::new();
    let post_handler = middleware::PostHandler::new(state_mutex);
    let installer = middleware::Installer::new();
    router.post("/", post_handler);
    router.post("/install-gsi", installer);
    router
}
