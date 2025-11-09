use std::sync::OnceLock;

pub struct EnvCache {
    target_yml: String,
    webhock: String,
    ndis_url: String,
    max_retry: u32,
    log_level: String,
    read_interval: u64,
}

impl EnvCache {
    fn new() -> Self {
        Self {
            target_yml: std::env::var("TARGET_YML").expect("環境変数TARGETYMLが存在しません"),
            webhock: std::env::var("WEBHOCK").expect("環境変数WEBHOCKが存在しません"),
            ndis_url: std::env::var("NDIS_URL").expect("環境変数NDIS_URLが存在しません"),
            max_retry: std::env::var("MAX_RETRY")
                .expect("環境変数MAX_RETRYが存在しません")
                .parse::<u32>()
                .expect("数値に変換できません"),
            log_level: std::env::var("LOG_LEVEL").unwrap_or("info".to_string()),
            read_interval: std::env::var("JOURNAL_INTERVAL")
                .expect("環境変数JOURNAL_INTERVALが存在しません")
                .parse::<u64>()
                .expect("数値に変換できません"),
        }
    }

    pub fn get_target_yml(&self) -> &str {
        &self.target_yml
    }
    pub fn get_webhock(&self) -> &str {
        &self.webhock
    }
    pub fn get_ndis_url(&self) -> &str {
        &self.ndis_url
    }

    pub fn get_log_level(&self) -> &str {
        &self.log_level
    }

    pub fn get_max_retry(&self) -> u32 {
        self.max_retry
    }

    pub fn get_journal_read_interval(&self) -> u64 {
        self.read_interval
    }
}

static ENV_CACHE: OnceLock<EnvCache> = OnceLock::new();

pub fn get_env_cache() -> &'static EnvCache {
    ENV_CACHE.get_or_init(|| EnvCache::new())
}
