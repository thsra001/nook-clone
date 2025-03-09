use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::egui::TextBuffer;

use super::I18Key;

impl I18Key {
    pub fn italian(&self) -> String {
        match &self {
            I18Key::AcPopulationGrowingGc => "AC: Population Growing (GC)",
            I18Key::AcCityFolkWii => "AC: Wild World/City Folk (Ds/Wii)",
            I18Key::AcCityFolkWiiRainy => "AC: Wild World/City Folk (Ds/Wii) [Rainy]",
            I18Key::AcCityFolkWiiSnowy => "AC: Wild World/City Folk (Ds/Wii) [Snowy]",
            I18Key::AcNewLeaf3ds => "AC: New Leaf (3DS)",
            I18Key::AcNewLeaf3dsRainy => "AC: New Leaf (3DS) [Rainy]",
            I18Key::AcNewLeaf3dsSnowy => "AC: New Leaf (3DS) [Snowy]",
            I18Key::AcNewHorizonsSwitch => "AC: New Horizons (Switch)",
            I18Key::KkSlider => "K.K. Slider",
            I18Key::Random => "Casuale",
            I18Key::MusicVolume => "Volume della musica",
            I18Key::RainVolume => "Volume della pioggia",
            I18Key::Settings => "Impostazioni",
            I18Key::Home => "Home",
            I18Key::MinimizeToTray => "Riduci a icona nella barra",
            I18Key::ExitNook => "Esci da Nook",
            I18Key::PatreonListAndSupportLinks =>  "Elenco Patreon e link di supporto",
            I18Key::PlayerSettings => "impostazioni del lettore",
            I18Key::GrandfatherClockMode => "Modalità orologio a pendolo",
            I18Key::UseGameRainSound => "Usa il suono della pioggia del gioco",
            I18Key::UseNoThunderRainSound => "Usa il suono della pioggia senza tuoni",
            I18Key::Language => "lingua",
            I18Key::ClearLocalFilesAndSettings => "cancellare i file e le impostazioni locali",
            I18Key::PatreonSupporters => "patreon supporters",
            I18Key::OpensDevelopersPatreonPage => "Apre la pagina Patreon dello sviluppatore",
            I18Key::SupportMe => "(supportami!)",
            I18Key::GoldSupporters => "gold supporters",
            I18Key::SilverSupporters => "silver supporters",
            I18Key::BronzeSupporters => "bronze supporters",
            I18Key::Patreon => "patreon",
            I18Key::PlaysOnceNoLoop =>  "(suona una volta, nessun loop)",
            I18Key::AreYouSure =>  "Sei sicuro?",
            I18Key::ClickOkToProceedAndDeleteAllLocalMusicFilesAndUserSettings => {
                "Fare clic su \"OK\" per procedere ed eliminare tutti i file musicali locali e le impostazioni utente."
            }
            I18Key::Customize => "Personalizzata",
            I18Key::EnableTownTune => "Abilita la melodia della città",
            I18Key::TuneSettings =>  "impostazioni di sintonia",
            I18Key::Offline => "offline",
            I18Key::OfflineFilesTotalFilesOfflineHourlyMusicFilesDownloaded => {
               "{{offlineFiles}}/{{totalFiles}} file musicali orari offline scaricati"
            }
            I18Key::OfflineKkfilesTotalKkfilesOfflineKKMusicFilesDownloaded => {
               "{{offlineKKFiles}}/{{totalKKFiles}} offline k.k. file musicali scaricati"
            }
            I18Key::DownloadAllHourlyMusic => "scarica tutta la musica oraria",
            I18Key::DownloadAllKKMusic => "scarica tutta la musica k.k",
            I18Key::SaveLovercase => "salva",
            I18Key::PlayLowercase => "avvia",
            I18Key::FailedToLoadSound => "Impossibile caricare la musica oraria.",
            I18Key::FailedToLoadRainSound => "Impossibile caricare il suono della pioggia.",
            I18Key::Play => "Avvia",
            I18Key::Pause => "Pausa",
            I18Key::FailedToDownload => "Impossibile scaricare la musica.",
            I18Key::AcPopulationGrowingGcSnowy => "AC: Population Growing (GC) [Snowy]",
            I18Key::AcPopulationGrowingGcSakura => "AC: Population Growing (GC) [Sakura]",
            I18Key::AcNewHorizonsSwitchRainy => "AC: New Horizons (Switch) [Rainy]",
            I18Key::AcNewHorizonsSwitchSnowy => "AC: New Horizons (Switch) [Snowy]",
            I18Key::AcPocketCampMobile => "AC: Pocket Camp (Mobile)",
            I18Key::PlayKkMusicOnSaturdayNights => "Suona K.K. musica il sabato sera",
            // why was this transalation missing? machine transalated
            I18Key::OpenOnStartup => "Aprire all'avvio",

            I18Key::CustomizeKKPlaylist => "personalizza k.k. playlist",
            I18Key::CustomizeTownTune => "personalizza la melodia della città",
            I18Key::KkPlaylist => "k.k. playlist",
            I18Key::CheckAll =>  "controlla tutto",
            I18Key::UncheckAll => "Deseleziona tutto",
            I18Key::RadioOnly =>  "solo radio",
            I18Key::LiveOnly =>  "solo live",
            I18Key::Downloading => "scaricamento in corso...",
            I18Key::DontDownloadMusic => "Non scaricare musica",
            I18Key::SavesSpaceButNoOffline => "(risparmia spazio, ma non offline)",
            I18Key::TipYouCanUseTheMouseWheelToAdjustNotes => {
               "consiglio: puoi usare la rotellina del mouse per regolare le note!"
            }
            I18Key::NookGitHub => "Nook GitHub",
            I18Key::Saved => "salvato!",
            // automatic transalations
            I18Key::Changelog => "changelog",
            I18Key::AcPopulationGrowingGcRainyDay => "AC: Population Growing (GC) [Rainy Day]",
        }
        .to_string()
    }
}
