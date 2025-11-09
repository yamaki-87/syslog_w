use anyhow::Result;
use flexi_logger::Logger;
use log::{Record, info};

use crate::{
    config::target,
    interfaces::ndis::NdisNotify,
    jornalctl::{WatchInput, watch::JournalWatcher},
    service::journal_analyzer::JournalAnalyzer,
};

pub mod config;
pub mod interfaces;
pub mod jornalctl;
pub mod service;
#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    logger_init();

    let env = config::env::get_env_cache();
    let target = target::load_config(env.get_target_yml())?;
    info!("config yml:{:?}", &target);

    let ndis_notifyer = NdisNotify::new();
    let journal_analyze = JournalAnalyzer::new(Box::new(ndis_notifyer));
    let journal_watcher = JournalWatcher::new(journal_analyze);
    journal_watcher
        .observe_loop(WatchInput { config: target })
        .await?;
    Ok(())
}

fn logger_init() {
    let level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    Logger::try_with_str(level)
        .unwrap()
        .format(custom_fmt)
        .duplicate_to_stdout(flexi_logger::Duplicate::All)
        .start()
        .unwrap();
}

fn custom_fmt(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &Record,
) -> std::io::Result<()> {
    write!(
        w,
        "[{}][{}][{}] {}",
        record.level(),
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.target(),
        &record.args()
    )
}
