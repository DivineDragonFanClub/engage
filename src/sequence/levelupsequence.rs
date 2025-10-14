use unity::prelude::*;

use crate::{
    gamedata::unit::Unit,
    proc::{Bindable, ProcInstFields}
};

#[unity::class("App", "LevelUpSequnece")]
pub struct LevelUpSequence {
    pub base: ProcInstFields,
    pub res_name_level_up: &'static Il2CppString,
    pub res_name_class_change: &'static Il2CppString,
    pub unit: &'static mut Unit,
    pub grow: &'static mut Unit,
    pub level: i32,
    pub is_class_change: bool,
}

impl Bindable for LevelUpSequence {}