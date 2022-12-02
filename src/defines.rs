use once_cell::sync::Lazy;
use std::path::PathBuf;

pub const APP_DIR: &str = "com.github.nozwock.yadv";
pub static APP_SECRETS_PATH: Lazy<PathBuf> =
    Lazy::new(|| app_config_dir().unwrap_or_default().join("secrets.ron"));

pub static API_HEADER_USER_AGENT: [&str; 2] = [
    "User-Agent",
    "github.com/nozwock/yaadv; magenta-duck.ezffr@slmail.me",
];
pub static API_HEADER_FROM: [&str; 2] = ["From", "magenta-duck.ezffr@slmail.me"];

pub fn app_config_dir() -> Option<PathBuf> {
    Some(dirs::config_dir()?.join(APP_DIR))
}
