use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use i18n_embed::fluent::fluent_language_loader;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::LanguageLoader;
use rust_embed::RustEmbed;
use std::env;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

pub struct AppConfig {
    pub translator: FluentLanguageLoader,
    pub db: SqliteConnection,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            translator: init_translator(),
            db: init_db(),
        }
    }
}

fn init_translator() -> FluentLanguageLoader {
    let language_loader = fluent_language_loader!();
    language_loader
        .load_languages(&Localizations, &[language_loader.fallback_language()])
        .expect("error loading i18n");
    language_loader
}

fn init_db() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    SqliteConnection::establish(&database_url).expect("Can not establish database connection")
}
