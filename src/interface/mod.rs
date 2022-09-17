use crate::config::AppConfig;

mod cli;

pub fn run(config: AppConfig) {
    // @todo implement other interfaces, e.g. through a Telegram bot
    cli::Cli::new(&config).run();
}
