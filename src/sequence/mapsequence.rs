use crate::proc::{Bindable, ProcInstFields};

pub mod human;

#[unity::class("App", "MapSequence")]
pub struct MapSequence {
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
}
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapSequenceLabel {
    None = -1,
    Init = 0,
    Tick = 1,
    Sortie = 2,
    ResumeMap = 3,
    ResumeSortie = 4,
    SkipSortie = 5,
    MapStart = 6,
    MapBegin = 7,
    TurnBegin = 8,
    TurnBeginAfterRewind = 9,
    TurnBranch = 10,
    TurnBranchAfterRewind = 11,
    TurnHuman = 12,
    TurnAI = 13,
    TurnLink = 14,
    TurnReplay = 15,
    TurnEnd = 16,
    Complete = 17,
    GameOver = 18,
    RelayUnsettled = 19,
    SaveDataLoad = 20,
    RestartLoad = 21,
    RelayLoad = 22,
    RelayLoadError = 23,
    RelaySkipReplay = 24,
    RelayReplayToTakeOver = 25,
    VersusLoad = 26,
    VersusEdit = 27,
    End = 28,
    Tail = 29,

}

impl Bindable for MapSequence {}
impl AsRef<ProcInstFields> for MapSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MapSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
impl From<i32> for MapSequenceLabel {
    fn from(value: i32) -> Self {
        match value {
            0 => MapSequenceLabel::Init, 
            1 => MapSequenceLabel::Tick, 
            2 => MapSequenceLabel::Sortie, 
            3 => MapSequenceLabel::ResumeMap, 
            4 => MapSequenceLabel::ResumeSortie, 
            5 => MapSequenceLabel::SkipSortie, 
            6 => MapSequenceLabel::MapStart, 
            7 => MapSequenceLabel::MapBegin, 
            8 => MapSequenceLabel::TurnBegin, 
            9 => MapSequenceLabel::TurnBeginAfterRewind, 
            10 => MapSequenceLabel::TurnBranch, 
            11 => MapSequenceLabel::TurnBranchAfterRewind, 
            12 => MapSequenceLabel::TurnHuman, 
            13 => MapSequenceLabel::TurnAI, 
            14 => MapSequenceLabel::TurnLink, 
            15 => MapSequenceLabel::TurnReplay, 
            16 => MapSequenceLabel::TurnEnd, 
            17 => MapSequenceLabel::Complete, 
            18 => MapSequenceLabel::GameOver, 
            19 => MapSequenceLabel::RelayUnsettled, 
            20 => MapSequenceLabel::SaveDataLoad, 
            21 => MapSequenceLabel::RestartLoad, 
            22 => MapSequenceLabel::RelayLoad, 
            23 => MapSequenceLabel::RelayLoadError, 
            24 => MapSequenceLabel::RelaySkipReplay, 
            25 => MapSequenceLabel::RelayReplayToTakeOver, 
            26 => MapSequenceLabel::VersusLoad, 
            27 => MapSequenceLabel::VersusEdit, 
            28 => MapSequenceLabel::End, 
            _ => MapSequenceLabel::None,
        }
    }
}
