use num_derive::{FromPrimitive, ToPrimitive};
pub use unity::prelude::*;
use crate::battle::{BattleCalculator, BattleInfo};
use crate::proc::{Bindable, ProcInstFields};
use crate::sequence::mapsequence::MapSequence;

#[unity::class("App", "MapSequenceBattle")]
pub struct MapSequenceBattle {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub info: &'static BattleInfo,
    pub sum_info: &'static BattleInfo,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    reliance: *const u8,

}

impl Bindable for MapSequenceBattle {}

#[unity::class("App", "MapSequenceBattleAction")]
pub struct MapSequenceBattleAction {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_loaded: bool,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    signal: *const u8,
    pub scene_index: i32,
    pub battle_count: i32,
    pub attack_count: i32,
    pub info_wait: f32,
    pub attack_side: i32,
}

impl Bindable for MapSequenceBattleAction {}