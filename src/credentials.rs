use crate::defines::APP_SECRETS_PATH;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Secrets {
    pub session_token: Option<String>,
}

impl Secrets {
    pub fn load() -> Secrets {
        confy::load_path(&*APP_SECRETS_PATH).unwrap_or_default()
    }
    pub fn store(self) -> anyhow::Result<()> {
        confy::store_path(&*APP_SECRETS_PATH, self).map_err(Into::into)
    }
    pub fn get_session_token(&self) -> Option<&str> {
        Some(self.session_token.as_ref()?)
    }
}
