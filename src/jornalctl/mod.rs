use crate::config::target::JournalConfig;

pub mod watch;

pub struct WatchInput {
    pub config: JournalConfig,
}
