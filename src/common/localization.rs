use crate::common::MENU;
use crate::logging::*;
use crate::training::ui::notifications::notification;
use training_mod_consts::Locale;
use training_mod_sync::*;

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

pub fn init() {
    info!("Initializing localization");
    handle_language_change();
    info!(
        "Initialized localization with {:#?}",
        ModLanguageId::from(read(&MENU).selected_locale)
    );
}

pub fn set_language() {
    let locale: ModLanguageId = ModLanguageId::from(read(&MENU).selected_locale);
    info!("Setting language to {:?}", locale);

    let locale_code = locale.get_locale_code();

    if rust_i18n::available_locales!().contains(&locale_code) {
        rust_i18n::set_locale(locale_code);

        notification("Language".to_string(), locale_code.to_string(), 360);
    } else {
        info!("{} not found using en_us instead.", locale_code);
        rust_i18n::set_locale("en_us");
    }
}

pub fn handle_language_change() {
    let has_locale_changed =
        *ModLanguageId::from(read(&MENU).selected_locale).get_locale_code() != *rust_i18n::locale();

    if has_locale_changed {
        set_language();
    }
}
