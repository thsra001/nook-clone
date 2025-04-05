use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use super::{ButtonReflectSet, ButtonSet};
mod english;
mod german;
mod spanish;
mod italian;
#[derive(Resource, Default, Reflect)]
pub enum I18evyLang {
    #[default]
    English,
    French,
    Chinese,
    German,
    Italian,
    Spanish,
}
// transalation key
#[derive(Component)]
pub enum I18Key {
    AcPopulationGrowingGc,
    AcCityFolkWii,
    AcCityFolkWiiRainy,
    AcCityFolkWiiSnowy,
    AcNewLeaf3ds,
    AcNewLeaf3dsRainy,
    AcNewLeaf3dsSnowy,
    AcNewHorizonsSwitch,
    KkSlider,
    Random,
    MusicVolume,
    RainVolume,
    Settings,
    Home,
    MinimizeToTray,
    ExitNook,
    PatreonListAndSupportLinks,
    PlayerSettings,
    GrandfatherClockMode,
    UseGameRainSound,
    UseNoThunderRainSound,
    Language,
    ClearLocalFilesAndSettings,
    PatreonSupporters,
    OpensDevelopersPatreonPage,
    SupportMe,
    GoldSupporters,
    SilverSupporters,
    BronzeSupporters,
    Patreon,
    PlaysOnceNoLoop,
    AreYouSure,
    ClickOkToProceedAndDeleteAllLocalMusicFilesAndUserSettings,
    Customize,
    EnableTownTune,
    TuneSettings,
    Offline,
    OfflineFilesTotalFilesOfflineHourlyMusicFilesDownloaded,
    OfflineKkfilesTotalKkfilesOfflineKKMusicFilesDownloaded,
    DownloadAllHourlyMusic,
    DownloadAllKKMusic,
    SaveLovercase,
    PlayLowercase,
    FailedToLoadSound,
    FailedToLoadRainSound,
    Play,
    Pause,
    FailedToDownload,
    AcPopulationGrowingGcSnowy,
    AcPopulationGrowingGcSakura,
    AcNewHorizonsSwitchRainy,
    AcNewHorizonsSwitchSnowy,
    AcPocketCampMobile,
    PlayKkMusicOnSaturdayNights,
    OpenOnStartup,
    CustomizeKKPlaylist,
    CustomizeTownTune,
    KkPlaylist,
    CheckAll,
    UncheckAll,
    RadioOnly,
    LiveOnly,
    Downloading,
    DontDownloadMusic,
    SavesSpaceButNoOffline,
    TipYouCanUseTheMouseWheelToAdjustNotes,
    NookGitHub,
    Saved,
    Changelog,
    AcPopulationGrowingGcRainyDay,
}

pub struct I18evyImport;

impl Plugin for I18evyImport {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lang_update, text_follow_lang.after(ButtonReflectSet)))
            .init_resource::<I18evyLang>()
            .register_type::<I18evyLang>()
            .add_plugins(ResourceInspectorPlugin::<I18evyLang>::new().run_if(input_toggle_active(true, KeyCode::KeyO)));
    }
}

// change text function(i8key,lang)
fn find(key: &I18Key, lang: &I18evyLang) -> String {
    // im too lazy to add chinese and french, sorry! L france lololol
    // todo: add chinese from https://github.com/mn6/nook-desktop/tree/main/app/main/i18n
    match lang {
        I18evyLang::English => I18Key::english(&key),
        I18evyLang::German => I18Key::german(&key),
        I18evyLang::Spanish => I18Key::spanish(&key),
        I18evyLang::Italian => I18Key::italian(&key),
        _ => todo!("add languages"),
    }

    // match language and then vectorsearch and replace text
}
// system change all text on lang change
fn lang_update(lang: Res<I18evyLang>, mut q_tex: Query<(&mut Text, &I18Key)>) {
    if lang.is_changed() {
        info!("lang changed");
        for (mut tex, key) in &mut q_tex {
            //info!("text found:{}", tex.0);
            let res = find(key, &*lang);
            //info!("found key, transalating to:{}", &res);
            tex.0 = res;
        }
    }
}
// system change text on new text
fn text_follow_lang(
    lang: Res<I18evyLang>,
    mut q_tex: Query<(&mut Text, &I18Key), Changed<I18Key>>,
) {
    for (mut tex, key) in &mut q_tex {
        //info!("text found:{}", tex.0);
        let res = find(key, &*lang);
       // info!("found key, transalating to:{}", &res);
        tex.0 = res;
    }
}
// repeat both functions for tray menu
