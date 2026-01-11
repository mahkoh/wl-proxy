use log::LevelFilter;

pub mod proxy;
pub mod server;

pub fn install_logger() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .try_init();
}
