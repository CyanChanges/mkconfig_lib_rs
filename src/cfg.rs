#![allow(dead_code)]

use std::env;
use std::path::Path;
use std::sync::OnceLock;
use once_cell::sync::Lazy;

pub const PREFIX: &str = "/usr";
pub const EXEC_PREFIX: &str = "/usr";

pub const DATA_ROOT_DIR: &str = "/usr/share";
pub const DATA_DIR: &str = DATA_ROOT_DIR;

pub const BIN_DIR: &str = "/usr/bin";
pub const SBIN_DIR: &str = "/usr/bin";

pub static PKG_DATA_DIR_CELL: OnceLock<String> = OnceLock::new();
pub static PKG_DATA_DIR: Lazy<String> = Lazy::new(get_pkg_data_dir);

pub static GRUB_PROBE_CELL: OnceLock<String> = OnceLock::new();
pub static GRUB_PROBE: Lazy<String> = Lazy::new(get_grub_probe);

pub static GRUB_FILE_CELL: OnceLock<String> = OnceLock::new();
pub static GRUB_FILE: Lazy<String> = Lazy::new(get_grub_file);

pub static GRUB_MK_RELPATH_CELL: OnceLock<String> = OnceLock::new();
pub static GRUB_MK_RELPATH: Lazy<String> = Lazy::new(get_mk_relpath);

pub static GRUB_SAVE_DEFAULT: Lazy<String> = Lazy::new(||env::var("GRUB_SAVEDEFAULT").unwrap_or("false".into()));

pub fn get_pkg_data_dir() -> String {
    PKG_DATA_DIR_CELL.get_or_init(|| env::var("pkgdatadir").unwrap_or(
        String::from(Path::new(DATA_DIR).join("grub").to_string_lossy())
    )).to_owned()
}

pub fn get_grub_probe() -> String {
    GRUB_PROBE_CELL.get_or_init(|| env::var("grub_probe").unwrap_or(
        String::from(Path::new(SBIN_DIR).join("grub").to_string_lossy())
    )).to_owned()
}

pub fn get_grub_file() -> String {
    GRUB_FILE_CELL.get_or_init(|| env::var("grub_file").unwrap_or(
        String::from(Path::new(BIN_DIR).join("grub-file").to_string_lossy())
    )).to_owned()
}

pub fn get_mk_relpath() -> String {
    GRUB_MK_RELPATH_CELL.get_or_init(|| env::var("grub_mkrelpath").unwrap_or(
        String::from(Path::new(BIN_DIR).join("grub-mkrelpath").to_string_lossy())
    )).to_owned()
}
