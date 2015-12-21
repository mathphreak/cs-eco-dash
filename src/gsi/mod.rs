extern crate nickel;
extern crate time;
extern crate crc;

use nickel::{Router, HttpRouter};
use std::sync::{Arc, Mutex};

pub use self::middleware::Player;
pub use self::version::Versions;

pub mod version;
mod middleware;
mod paths;

pub fn router(player_mutex: Arc<Mutex<Player>>) -> Router<()> {
    let mut router = Router::new();
    let post_handler = middleware::PostHandler::new(player_mutex);
    let installer = middleware::Installer::new();
    router.post("/", post_handler);
    router.post("/install-gsi", installer);
    router
}
