use i18n_codegen::i18n;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

i18n!("src/locales");

impl Default for Locale {
    fn default() -> Self {
        Self::En
    }
}

impl From<Option<String>> for Locale {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(str) if str.as_str() == "pl" => Self::Pl,
            _ => Default::default(),
        }
    }
}

pub trait ToLocaleString {
    fn to_locale_string(&self, locale: Locale) -> String;
}

pub trait ToLocaleEmbed {
    fn to_locale_embed(&self, locale: Locale) -> EmbedBuilder;
}

pub trait ToLocaleEmbedField {
    fn to_locale_embed_field(&self, locale: Locale) -> EmbedFieldBuilder;
}
