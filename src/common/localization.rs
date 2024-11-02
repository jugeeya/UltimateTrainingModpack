use crate::common::MENU;
use crate::logging::*;
use crate::training::ui::notifications::notification;
use training_mod_consts::Locale;

#[repr(u8)]
#[derive(Debug)]
pub enum ModLanguageId {
    English,
    French,
}

impl From<u8> for ModLanguageId {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::English,
            1 => Self::French,
            _ => Self::English,
        }
    }
}

impl From<&str> for ModLanguageId {
    fn from(locale_code: &str) -> Self {
        match locale_code {
            "en_us" => Self::English,
            "fr" => Self::French,
            _ => Self::English,
        }
    }
}

impl From<Locale> for ModLanguageId {
    fn from(locale_code: Locale) -> Self {
        match locale_code {
            Locale::ENGLISH_US => Self::English,
            Locale::FRENCH => Self::French,
            _ => Self::English,
        }
    }
}

impl ModLanguageId {
    pub fn get_locale_code(&self) -> &str {
        match self {
            ModLanguageId::English => "en_us",
            ModLanguageId::French => "fr",
        }
    }
}

pub unsafe fn set_language_from_menu() {
    let locale = ModLanguageId::from(MENU.selected_locale);
    info!("Setting language to {:?}", locale);

    let locale_code = locale.get_locale_code();
    if rust_i18n::available_locales!().contains(&locale_code) {
        info!("Setting language to {:?}", locale_code);
        rust_i18n::set_locale(locale_code);

        notification("Language".to_string(), locale_code.to_string(), 8);
    } else {
        info!("{} not found using en_us instead.", locale_code);
        rust_i18n::set_locale("en_us");
    }
}
