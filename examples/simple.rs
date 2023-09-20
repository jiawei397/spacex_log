use log::{debug, error, info, warn};
use spacex_log::init_log;

fn main() {
    init_log("INFO");
    // init_log("info");

    debug!("debug info");

    info!("test info");

    warn!("warn info");

    error!("error");
}
