use std::env;
use std::sync::OnceLock;
use once_cell::sync::Lazy;

static _OS_LOCK: OnceLock<String> = OnceLock::new();
pub static OS: Lazy<String> = Lazy::new(get_os);

pub fn get_os() -> String {
    _OS_LOCK.get_or_init(|| {
        if env::var("GRUB_DISTRIBUTOR").is_err() {
            let distro = whoami::distro();
            if distro.starts_with("Unknown") {
                "Linux".to_string()
            } else {
                distro
            }
        } else {
            format!("{} Linux", env::var("GRUB_DISTRIBUTOR").unwrap())
        }
    }).to_string()
}

trait AddTab {
    fn grub_add_tab(&self) -> String;
}

const GRUB_TAB: &str = "  ";

impl AddTab for String {
    fn grub_add_tab(&self) -> Self {
        let mut str = self.to_string();
        if str.chars().count() == 0 {
            return str;
        }
        str.insert_str(0, GRUB_TAB);
        str
    }
}

