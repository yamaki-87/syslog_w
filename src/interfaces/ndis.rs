use crate::{config, service::notify_service::Notifyer};
use anyhow::Result;
use log::error;
use serde::Serialize;
const SERVICE_NAME: &str = "syslogW";
#[derive(Serialize)]
struct NdisData<'a> {
    webhook: &'a str,
    message: &'a str,
    service: &'a str,
}

pub struct NdisNotify {}
impl NdisNotify {
    pub fn new() -> Self {
        Self {}
    }
}

impl Notifyer for NdisNotify {
    fn notify(&self, msg: &str) -> Result<()> {
        let env = config::env::get_env_cache();
        let max_retry = env.get_max_retry();
        let data = NdisData {
            webhook: env.get_webhock(),
            message: msg,
            service: SERVICE_NAME,
        };

        let mut i = 0u32;
        while i < max_retry {
            i += 1;
            let r = ureq::post(env.get_ndis_url())
                .header("Content-Type", "application/json")
                .send_json(&data);
            match r {
                Ok(_r) => break,
                Err(e) => {
                    error!("NDIS連携失敗:{}", e);
                    continue;
                }
            }
        }

        Ok(())
    }
}
