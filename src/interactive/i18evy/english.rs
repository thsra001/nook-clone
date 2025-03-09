use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::egui::TextBuffer;

use super::I18Key;

impl I18Key {
    pub fn english(&self) -> String {
        match &self {
            I18Key::AcPopulationGrowingGc => "AC: Population Growing (GC)",
            I18Key::AcCityFolkWii => "AC: City Folk (Wii)",
            I18Key::AcCityFolkWiiRainy => "AC: City Folk (Wii) [Rainy]",
            I18Key::AcCityFolkWiiSnowy => "AC: City Folk (Wii) [Snowy]",
            I18Key::AcNewLeaf3ds => "AC: New Leaf (3DS)",
            I18Key::AcNewLeaf3dsRainy => "AC: New Leaf (3DS) [Rainy]",
            I18Key::AcNewLeaf3dsSnowy => "AC: New Leaf (3DS) [Snowy]",
            I18Key::AcNewHorizonsSwitch => "AC: New Horizons (Switch)",
            I18Key::KkSlider => "K.K. Slider",
            I18Key::Random => "Random",
            I18Key::MusicVolume => "Music Volume",
            I18Key::RainVolume => "Rain Volume",
            I18Key::Settings => "Settings",
            I18Key::Home => "Home",
            I18Key::MinimizeToTray => "Minimise to tray",
            I18Key::ExitNook => "Exit Nook",
            I18Key::PatreonListAndSupportLinks => "Patreon list and support links",
            I18Key::PlayerSettings => "player settings",
            I18Key::GrandfatherClockMode => "Grandfather clock mode",
            I18Key::UseGameRainSound => "Use game rain sound",
            I18Key::UseNoThunderRainSound => "Use no-thunder rain sound",
            I18Key::Language => "Language",
            I18Key::ClearLocalFilesAndSettings => "clear local files and settings",
            I18Key::PatreonSupporters => "patreon supporters",
            I18Key::OpensDevelopersPatreonPage => "Opens developer's Patreon page",
            I18Key::SupportMe => "(support me!)",
            I18Key::GoldSupporters => "gold supporters",
            I18Key::SilverSupporters => "sliver supporters",
            I18Key::BronzeSupporters => "bronze supporters",
            I18Key::Patreon => "patreon",
            I18Key::PlaysOnceNoLoop => "(plays once, doesn't loop)",
            I18Key::AreYouSure => "Are you sure?",
            I18Key::ClickOkToProceedAndDeleteAllLocalMusicFilesAndUserSettings => {
                "Click \"OK\" to proceed and delete all local music files and user settings."
            }
            I18Key::Customize => "customise",
            I18Key::EnableTownTune => "Enable town tune",
            I18Key::TuneSettings => "tune settings",
            I18Key::Offline => "offline",
            I18Key::OfflineFilesTotalFilesOfflineHourlyMusicFilesDownloaded => {
                "{{offlineFiles}}/{{totalFiles}} offline hourly music files downloaded"
            }
            I18Key::OfflineKkfilesTotalKkfilesOfflineKKMusicFilesDownloaded => {
                "{{offlineKKFiles}}/{{totalKKFiles}} offline k.k. music files downloaded"
            }
            I18Key::DownloadAllHourlyMusic => "download all hourly music",
            I18Key::DownloadAllKKMusic => "download all k.k. music",
            I18Key::SaveLovercase => "save",
            I18Key::PlayLowercase => "play",
            I18Key::FailedToLoadSound => "failed to load hourly music.",
            I18Key::FailedToLoadRainSound => "failed to load rain sound.",
            I18Key::Play => "Play",
            I18Key::Pause => "Pause",
            I18Key::FailedToDownload => "failed to download music.",
            I18Key::AcPopulationGrowingGcSnowy => "AC: Population Growing (GC) [Snowy]",
            I18Key::AcPopulationGrowingGcSakura => "AC: Population Growing (GC) [Sakura]",
            I18Key::AcNewHorizonsSwitchRainy => "AC: New Horizons (Switch) [Rainy]",
            I18Key::AcNewHorizonsSwitchSnowy => "AC: New Horizons (Switch) [Snowy]",
            I18Key::AcPocketCampMobile => "AC: Pocket Camp (Mobile)",
            I18Key::PlayKkMusicOnSaturdayNights => "Play K.K. music on Saturday nights",
            I18Key::OpenOnStartup => "Open on startup",
            I18Key::CustomizeKKPlaylist => "customise k.k playlist",
            I18Key::CustomizeTownTune => "customise town tune",
            I18Key::KkPlaylist => "k.k. playlist",
            I18Key::CheckAll => "check all",
            I18Key::UncheckAll => "uncheck all",
            I18Key::RadioOnly => "radio only",
            I18Key::LiveOnly => "live only",
            I18Key::Downloading => "downloading...",
            I18Key::DontDownloadMusic => "Don't download music",
            I18Key::SavesSpaceButNoOffline => "(saves space, but no offline)",
            I18Key::TipYouCanUseTheMouseWheelToAdjustNotes => {
                "tip: you can use the mouse wheel to adjust notes!"
            }
            I18Key::NookGitHub => "Nook GitHub",
            I18Key::Saved => "saved!",
            I18Key::Changelog => "changelog",
            I18Key::AcPopulationGrowingGcRainyDay => "AC: Population Growing (GC) [Rainy Day]",
        }
        .to_string()
    }
}
