use anyhow::Result;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, DesktopLanguageRequester, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localization::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localization::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}
pub(crate) use fl;

pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

pub fn setup_localization() -> Result<()> {
    let library_localizer = localizer();
    let requested_languages = DesktopLanguageRequester::requested_languages();
    library_localizer.select(&requested_languages)?;
    Ok(())
}
