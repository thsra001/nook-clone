use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::egui::TextBuffer;

use super::I18Key;

impl I18Key {
    pub fn german(&self) -> String {
        match &self {
            I18Key::AcPopulationGrowingGc => "AC: Population Growing (GC)",
            I18Key::AcCityFolkWii => "AC: City Folk (Wii)",
            I18Key::AcCityFolkWiiRainy => "AC: City Folk (Wii) [Regen]",
            I18Key::AcCityFolkWiiSnowy => "AC: City Folk (Wii) [Schnee]",
            I18Key::AcNewLeaf3ds => "AC: New Leaf (3DS)",
            I18Key::AcNewLeaf3dsRainy => "AC: New Leaf (3DS) [Regen]",
            I18Key::AcNewLeaf3dsSnowy => "AC: New Leaf (3DS) [Schnee]",
            I18Key::AcNewHorizonsSwitch => "AC: New Horizons (Switch)",
            I18Key::KkSlider => "K.K.",
            I18Key::Random => "Zufällig",
            I18Key::MusicVolume => "Musiklautstärke",
            I18Key::RainVolume => "Regenlautstärke",
            I18Key::Settings => "Einstellungen",
            I18Key::Home => "Startseite",
            I18Key::MinimizeToTray => "In Taskleiste minimieren",
            I18Key::ExitNook => "Nook beenden",
            I18Key::PatreonListAndSupportLinks => "Patreon Unterstützer und Support Links",
            I18Key::PlayerSettings => "Audioeinstellungen",
            I18Key::GrandfatherClockMode => "Großvater Uhrmodus",
            I18Key::UseGameRainSound => "Regengeräusche des Spiels verwenden",
            I18Key::UseNoThunderRainSound => "Regengeräusche ohne Donner verwenden",
            I18Key::Language => "Sprache",
            I18Key::ClearLocalFilesAndSettings => "Lokale Dateien und Einstellungen löschen",
            I18Key::PatreonSupporters => "Patreon Unterstützer",
            I18Key::OpensDevelopersPatreonPage => "Öffnet Patreon Seite des Entwicklers",
            I18Key::SupportMe => "(Unterstütze mich!)",
            I18Key::GoldSupporters => "Gold Unterstützer",
            I18Key::SilverSupporters => "Silber Unterstützer",
            I18Key::BronzeSupporters => "Bronze Unterstützer",
            I18Key::Patreon => "Patreon",
            I18Key::PlaysOnceNoLoop => "(spielt einmal, ohne Wiederholung)",
            I18Key::AreYouSure => "Bist du sicher?",
            I18Key::ClickOkToProceedAndDeleteAllLocalMusicFilesAndUserSettings => {
                "Wähle \"OK\" um alle lokalen Musikdateien und Nutzereinstellungen zu löschen."
            }
            I18Key::Customize => "Anpassen",
            I18Key::EnableTownTune => "Stadtmelodie aktivieren",
            I18Key::TuneSettings => "Melodieeinstellungen",
            I18Key::Offline => "Offline",
            I18Key::OfflineFilesTotalFilesOfflineHourlyMusicFilesDownloaded => {
                "{{offlineFiles}}/{{totalFiles}} stündliche Musikdateien heruntergeladen"
            }
            I18Key::OfflineKkfilesTotalKkfilesOfflineKKMusicFilesDownloaded => {
                "{{offlineKKFiles}}/{{totalKKFiles}} offline K.K. Musikdateien heruntergeladen"
            }
            I18Key::DownloadAllHourlyMusic => "stündliche Musik herunterladen",
            I18Key::DownloadAllKKMusic => "K.K. Musik herunterladen",
            I18Key::SaveLovercase => "speichern",
            I18Key::PlayLowercase => "abspielen",
            I18Key::FailedToLoadSound => "Herunterladen von stündlicher Musik fehlgeschlagen.",
            I18Key::FailedToLoadRainSound => "Herunterladen von Regengeräuschen fehlgeschlagen.",
            I18Key::Play => "Abspielen",
            I18Key::Pause => "Pause",
            I18Key::FailedToDownload => "Herunterladen von Musik fehlgeschlagen.",
            I18Key::AcPopulationGrowingGcSnowy => "AC: Population Growing (GC) [Schnee]",
            I18Key::AcPopulationGrowingGcSakura => "AC: Population Growing (GC) [Blütenfest]",
            I18Key::AcNewHorizonsSwitchRainy => "AC: New Horizons (Switch) [Regen]",
            I18Key::AcNewHorizonsSwitchSnowy => "AC: New Horizons (Switch) [Schnee]",
            I18Key::AcPocketCampMobile => "AC: Pocket Camp (Smartphone)",
            I18Key::PlayKkMusicOnSaturdayNights => "K.K. Musik Samstag nachts abspielen",
            I18Key::OpenOnStartup => "Beim Start öffnen",
            I18Key::CustomizeKKPlaylist => "K.K. Playlist anpassen",
            I18Key::CustomizeTownTune => "Stadtmelodie anpassen",
            I18Key::KkPlaylist => "K.K. Playlist",
            I18Key::CheckAll => "alle auswählen",
            I18Key::UncheckAll => "alle abwählen",
            I18Key::RadioOnly => "nur Radio",
            I18Key::LiveOnly => "nur Live",
            I18Key::Downloading => "Lade herunter...",
            I18Key::DontDownloadMusic => "Musik nicht herunterladen",
            I18Key::SavesSpaceButNoOffline => "(spart Platz, keine offline Nutzung möglich)",
            I18Key::TipYouCanUseTheMouseWheelToAdjustNotes => {
                "Tipp: Benutze das Mausrad zur Änderung der Noten!"
            }
            I18Key::NookGitHub => "Nook GitHub",
            I18Key::Saved => "gerettet!",
            // automatic transalations
            I18Key::Changelog => "Änderungsliste",
            I18Key::AcPopulationGrowingGcRainyDay => "AC: Population Growing (GC) [Regentag]",
        }
        .to_string()
    }
}
