use anyhow::Result;

pub trait Notifyer {
    fn notify(&self, msg: &str) -> Result<()>;
}
