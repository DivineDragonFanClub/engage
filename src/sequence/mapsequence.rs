use num_derive::{FromPrimitive, ToPrimitive};
use crate::proc::{Bindable, ProcInstFields};

pub mod human;
pub mod battle;

#[repr(i32)]
#[derive(PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum MapSequenceLabel {
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

#[unity::class("App", "MapSequence")]
pub struct MapSequence {
    pub proc: ProcInstFields,
    //
}

impl Bindable for MapSequence {}