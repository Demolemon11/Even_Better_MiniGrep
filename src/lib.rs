use config::Config;

pub mod config;
pub fn run() {
    let config = Config::default();
    config.search()
}
