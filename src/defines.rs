use once_cell::sync::Lazy;
use std::path::PathBuf;

pub const APP_DIR: &str = "com.github.nozwock.yadv";
pub static APP_SECRETS_PATH: Lazy<PathBuf> =
    Lazy::new(|| app_config_dir().unwrap_or_default().join("secrets.ron"));

pub fn app_config_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(APP_DIR))
}
