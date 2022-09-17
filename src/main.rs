#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

use config::AppConfig;

mod config;
mod interface;
mod persistence;

fn main() {
    let config = AppConfig::new();
    interface::run(config);
}
