use unity::prelude::*;

use crate::proc::{Bindable, ProcInstFields};

#[repr(C)]
#[unity::class("App", "MainSequence")]
pub struct MainSequence {
    // Start SingletonProcInst here
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    // End here
    pub scene_name: &'static mut Il2CppString,
    pub scene_mode: i32,
}

#[repr(C)]
pub struct MainSequenceStaticFields {
    pub jump_label: MainSequenceLabel,
    pub fake_label: i32,
    pub initialized: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainSequenceLabel {
    None = 0,
    Startup = 1,
    TitleLoop = 2,
    TitleLoopFromMainMenu = 3,
    MainMenu = 4,
    Chapter = 5,
    Gmap = 6,
    Kizuna = 7,
    Hub = 8,
    HubToSavePosition = 9,
    Ending = 10,
    NextChapter = 11,
    Map = 12,
    Complete = 13,
    GameOver = 14,
    ChapterSave = 15,
    AfterChapterSave = 16,
    SetSaveDataLoadTarget = 17,
    SaveDataLoad = 18,
    SaveDataLoadFailed = 19,
    SaveDataVersionFailed = 20,
    DataLoadFailed = 21,
    AfterLoadFailed = 22,
    ContentsResume = 23,
    RelayDebug = 24,
    Relay = 25,
    Versus = 26,
    Challenge = 27,
    BackToTitle = 28,
    End = 29,
}
impl Bindable for MainSequence {}
impl AsRef<ProcInstFields> for MainSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MainSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl From<i32> for MainSequenceLabel {
    fn from(value: i32) -> Self {
        match value {
            0 => MainSequenceLabel::None, 
            1 => MainSequenceLabel::Startup, 
            2 => MainSequenceLabel::TitleLoop, 
            3 => MainSequenceLabel::TitleLoopFromMainMenu, 
            4 => MainSequenceLabel::MainMenu, 
            5 => MainSequenceLabel::Chapter, 
            6 => MainSequenceLabel::Gmap, 
            7 => MainSequenceLabel::Kizuna, 
            8 => MainSequenceLabel::Hub, 
            9 => MainSequenceLabel::HubToSavePosition, 
            10 => MainSequenceLabel::Ending, 
            11 => MainSequenceLabel::NextChapter, 
            12 => MainSequenceLabel::Map, 
            13 => MainSequenceLabel::Complete, 
            14 => MainSequenceLabel::GameOver, 
            15 => MainSequenceLabel::ChapterSave, 
            16 => MainSequenceLabel::AfterChapterSave, 
            17 => MainSequenceLabel::SetSaveDataLoadTarget, 
            18 => MainSequenceLabel::SaveDataLoad, 
            19 => MainSequenceLabel::SaveDataLoadFailed, 
            20 => MainSequenceLabel::SaveDataVersionFailed, 
            21 => MainSequenceLabel::DataLoadFailed, 
            22 => MainSequenceLabel::AfterLoadFailed, 
            23 => MainSequenceLabel::ContentsResume, 
            24 => MainSequenceLabel::RelayDebug, 
            25 => MainSequenceLabel::Relay, 
            26 => MainSequenceLabel::Versus, 
            27 => MainSequenceLabel::Challenge, 
            28 => MainSequenceLabel::BackToTitle, 
            29 => MainSequenceLabel::End, 
            _ => MainSequenceLabel::None,
        }
    }
}
