use std::collections::BTreeMap;

use anyhow::Result;

use crate::{
    config::target::{JournalConfig, JournalObserver},
    service::notify_service::Notifyer,
};

pub struct JournalAnalyzer {
    notifyer: Box<dyn Notifyer>,
}

pub struct JournalAnalyzerInput<'a> {
    jounral: BTreeMap<String, String>,
    jounal_config: &'a JournalConfig,
}

impl<'a> JournalAnalyzerInput<'a> {
    pub fn new(entry: BTreeMap<String, String>, config: &'a JournalConfig) -> Self {
        Self {
            jounral: entry,
            jounal_config: config,
        }
    }
}

impl JournalAnalyzer {
    const FIELD_MESSAGE: &str = "MESSAGE";
    const FIELD_SYSTEMD_UNIT: &str = "_SYSTEMD_UNIT";
    const FIELD_SYSTEMD_USER_UNIT: &str = "_SYSTEMD_USER_UNIT";

    pub fn new(notifyer: Box<dyn Notifyer>) -> Self {
        Self { notifyer: notifyer }
    }

    pub fn execute(&self, input: JournalAnalyzerInput) -> Result<()> {
        let line = match input.jounral.get(Self::FIELD_MESSAGE) {
            Some(line) => line,
            None => {
                return Ok(());
            }
        };
        log::debug!("{}", line);

        for target in input.jounal_config.services.iter() {
            if self.is_target_log(target, &input.jounral) && target.keywords_contain(&line) {
                let _ = self
                    .notifyer
                    .notify(&format!("[{}] {}", target.get_name(), line));
            }
        }
        Ok(())
    }

    fn is_target_log(&self, config: &JournalObserver, entry: &BTreeMap<String, String>) -> bool {
        match config.get_unit_type() {
            crate::config::target::JournalUnit::System => {
                entry.get(Self::FIELD_SYSTEMD_UNIT).map(|v| v.as_str()) == Some(config.get_name())
            }
            crate::config::target::JournalUnit::User => {
                entry.get(Self::FIELD_SYSTEMD_USER_UNIT).map(|v| v.as_str())
                    == Some(config.get_name())
            }
        }
    }
}
