#![allow(dead_code)]

use std::sync::Arc;
use shell_quote::Sh;
use crate::cfg::GRUB_SAVE_DEFAULT;
use crate::indent;
use crate::misc::OS;
use crate::types::Statement;

pub struct MenuEntry {
    name: String,
    prepare: Vec<Statement>,
    mods: Vec<Arc<str>>,
    statements: Vec<Statement>,
}

impl MenuEntry {
    pub fn builder() -> Self {
        MenuEntry {
            name: OS.clone(),
            prepare: Vec::new(),
            mods: Vec::new(),
            statements: Vec::with_capacity(1),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn booster(mut self) -> Self {
        self.name.push_str(" (booster initramfs)");
        self
    }

    pub fn fallback(mut self) -> Self {
        self.name.push_str(" (fallback initramfs)");
        self
    }

    pub fn recovery(mut self) -> Self {
        self.name.push_str(" (recovery mode)");
        self
    }

    pub fn insmod(mut self, mod_name: &str) -> Self {
        self.mods.push(Arc::from(mod_name));
        self
    }

    pub fn chainloader(mut self, path: &str) -> Self {
        if !self.mods.iter().any(|m| m.as_ref() == "chain") {
            self = self.insmod("chain");
        }
        Self::_st_chainloader(path, &mut self);
        self
    }

    fn _st_chainloader(path: &str, borrow: &mut MenuEntry) {
        if path.starts_with("/") {
            borrow.statements.push(format!(r"chainloader {}", path));
        } else {
            borrow.statements.push(format!("chainloader /{}", path))
        }
    }

    pub fn save_default(&mut self) -> &mut Self {
        self.prepare.insert(0, save_default_entry().to_string());
        self
    }

    pub fn generate(&self) -> String {
        let mut vec = Vec::with_capacity(self.statements.len() + 1 + self.mods.len());
        for m in self.mods.iter() {
            vec.push(format!("insmod {}", m))
        }
        vec.push(String::from(""));
        vec.extend(self.statements.clone());
        format!(
            "menuentry {} {{\n\
            {}\
            {}\n\
            }}\n",
            unsafe { std::str::from_utf8_unchecked(&Sh::quote(&self.name)) },
            indent!(&self.prepare),
            indent!(&vec, false)
        )
    }
}


pub fn save_default_entry() -> &'static str {
    if GRUB_SAVE_DEFAULT.eq("true") {
        "savedefault"
    } else { "" }
}

#[cfg(test)]
mod tests {
    use crate::misc::GRUB_TAB;
    use super::*;

    #[test]
    fn menu_entry() {
        assert_eq!(
            MenuEntry::builder()
                .name("Linux")
                .insmod("fat")
                .chainloader("/EFI/Linux/BOOTX.efi")
                .save_default()
                .generate(),
            format!("menuentry {} {{\n\
                     {}\
                     {GRUB_TAB}insmod fat\n\
                     {GRUB_TAB}insmod chain\n\
                     \n\
                     {GRUB_TAB}chainloader /EFI/Linux/BOOTX.efi\n\
                     }}\n", "Linux", indent!(&vec![save_default_entry().to_string()]))
        )
    }
}

