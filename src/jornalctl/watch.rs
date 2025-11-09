use std::time::Duration;

use anyhow::Result;
use log::{error, info};
use systemd::{Journal, journal};

use crate::{
    config::target::JournalConfig,
    jornalctl::WatchInput,
    service::journal_analyzer::{JournalAnalyzer, JournalAnalyzerInput},
};

pub struct JournalWatcher {
    service: JournalAnalyzer,
}
impl JournalWatcher {
    pub fn new(service: JournalAnalyzer) -> Self {
        Self { service: service }
    }

    pub async fn observe_loop(self, input: WatchInput) -> Result<()> {
        info!("journal監視ログ START");
        let mut jouranl = journal::OpenOptions::default().open()?;
        self.target_set_journal(&input.config, &mut jouranl)?;

        loop {
            log::debug!("loop ----");
            let entry_result: std::result::Result<
                Option<std::collections::BTreeMap<String, String>>,
                std::io::Error,
            > = jouranl.await_next_entry(Some(Duration::from_secs(5)));
            let entry_some = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    error!("jounal 監視処理にて ERROR:{}", e);
                    continue;
                }
            };

            if entry_some.is_none() {
                log::debug!("entryが存在しませんのでskip");
                continue;
            }

            let entry = entry_some.unwrap();
            log::debug!("{:?}", &entry);
            if let Err(e) = self
                .service
                .execute(JournalAnalyzerInput::new(entry, &input.config))
            {
                error!("解析エラー {}", e);
            }
        }
        Ok(())
    }

    fn target_set_journal(&self, target: &JournalConfig, journal: &mut Journal) -> Result<()> {
        journal.seek_tail()?;
        journal.previous()?;

        for target in target.services.iter() {
            journal.match_add(target.get_unit_type().as_str(), target.get_name())?;
        }
        Ok(())
    }
}
