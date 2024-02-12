#![allow(dead_code)]

use std::sync::Arc;
use shell_quote::Sh;
use crate::cfg::GRUB_SAVE_DEFAULT;
use crate::misc::OS;

type Statement = String;

pub struct MenuEntry {
    name: String,
    mods: Box<Vec<Arc<str>>>,
    statements: Box<Vec<Statement>>,
}

impl MenuEntry {
    pub fn builder() -> Self {
        MenuEntry {
            name: OS.clone(),
            mods: Box::from(Vec::new()),
            statements: Box::from(Vec::new()),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.into();
        self
    }

    pub fn booster(&mut self) -> &mut Self {
        self.name.push_str(" (booster initramfs)");
        self
    }

    pub fn fallback(&mut self) -> &mut Self {
        self.name.push_str(" (fallback initramfs)");
        self
    }

    pub fn recovery(&mut self) -> &mut Self {
        self.name.push_str(" (recovery mode)");
        self
    }

    pub fn insmod(&mut self, mod_name: &str) -> &mut Self {
        self.mods.push(Arc::from(mod_name));
        self
    }
    pub fn chainloader(&mut self, path: &str) -> &Self {
        match self.mods.iter().find(|m| m.as_ref() == "chain") {
            None => { self.insmod("chain"); }
            _ => {}
        };
        if path.starts_with("/") {
            self.statements.push(format!(r"chainloader {}", path));
        } else {
            self.statements.push(format!("chainloader /{}", path))
        }
        self
    }

    pub fn save_default(&mut self) -> &mut Self {
        self.statements.insert(0, save_default_entry().to_string());
        self
    }

    pub fn generate(&self) -> String {
        let mut vec = Vec::with_capacity(self.statements.len() + self.mods.len());
        for m in self.mods.iter() {
            vec.push(format!("insmod {}", m))
        }
        vec.extend((*self.statements).clone());
        format!(
            "menuentry {} {{\n\
            \t{}\n\
            }}\n",
            unsafe { std::str::from_utf8_unchecked(&Sh::quote(&self.name)) },
            indent::indent_with("\t", vec.join("\n"))
        )
    }
}


pub fn save_default_entry() -> &'static str {
    if GRUB_SAVE_DEFAULT.eq("true") {
        "savedefault"
    } else {""}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_entry() {
        assert_eq!(
            MenuEntry::builder()
                .name("Linux")
                .insmod("fat")
                .chainloader("/EFI/Linux/BOOTX.efi")
                .generate(), format!("menuentry {} {{\n\
                 \tinsmod fat\n\
                 \tinsmod chain\n\
                 \tchainloader /EFI/Linux/BOOTX.efi\n\
                }}\n", "Linux")
        )
    }
}

