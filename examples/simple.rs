use log::{error, info, warn};

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    info!("test info");

    warn!("warn info");

    error!("error");
}
