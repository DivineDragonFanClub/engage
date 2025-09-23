use crate::battle::BattleCalculator;
use crate::proc::{Bindable, ProcInstFields};

#[unity::class("Combat", "CombatSequence")]
pub struct CombatSequence {
    pub proc: ProcInstFields,
    pub is_resume: bool,
    pub is_load: bool,
    pub calculator: &'static BattleCalculator,
    pub sim_calculator: &'static BattleCalculator,
    camera: *const u8,
}
impl Bindable for CombatSequence {}