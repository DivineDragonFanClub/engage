use unity::il2cpp::object::Array;
use unity::prelude::*;
use crate::gamedata::{item::UnitItem, terrain::TerrainData, unit::Unit, WeaponMask, skill::SkillArray};

#[unity::class("App", "CapabilityInt")]
pub struct CapabilityInt {
    pub data: &'static mut Array<i32>,
}

#[unity::class("App", "BattleDetail")]
pub struct BattleDetail {
    pub capability: &'static mut CapabilityInt,
}

#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    //junk : [u8; 0x48],
    info: &'static mut BattleInfo,
    pub side_type : i32,
    __ : i32,
    pub unit: Option<&'static Unit>,
    pub unit_item: Option<&'static UnitItem>,
    pub specified_item: &'static UnitItem,
    pub x: i32,
    pub z: i32,
    pub terrain: &'static TerrainData,
    pub overlap: &'static TerrainData,
    pub status: &'static mut WeaponMask,
    pub detail: &'static BattleDetail,
    hierarchy: u64,
    support: u64,
    pub parent: &'static BattleInfoSide,
    pub reverse: &'static BattleInfoSide,
    destroy: *const u8,
    pub mask_skill: &'static SkillArray,
    pub level: i32,
    pub hp: i32,
    pub gain_exp: i32,
    pub gain_gold: i32,
    pub drop_item_ratio: f32,
    pub pick_up_item: i32,
    pub damage: i32,
    pub heal: i32,
    pub battle_times: i32,
    pub total_order: i32,
    pub total_action: i32,
    pub total_attack: i32,
    pub total_damage: i32,
    pub total_result: i32,
    pub temporary: i32,
    pub stun: i32,
    pub engage_count: i32,
    pub engage_first_count: i32,
    pub blown_distance: i32,
    pub weapon_expend: i32,
    pub expend_count: i32,
}

#[unity::class("App", "BattleInfo")]
pub struct BattleInfo {}

impl BattleInfo {
    pub fn get_unit(&self, side: i32) -> Option<&'static Unit> {
        unsafe {
            battle_info_get_unit(self, side, None)
        }
    }
}
#[unity::class("App", "BattleCalculator")]
pub struct BattleCalculator {
    pub mode: i32,
    pub info: &'static BattleInfo,
}
impl BattleCalculator {
    pub fn get_dead_side(&self) -> i32 { unsafe { battlecalcultor_get_deadside(self, None) }}
    pub fn get_side(&self, side: i32) -> Option<&'static mut BattleInfoSide> {
        unsafe { battle_calculator_get_side(self, side, None) }
    }
}

#[unity::from_offset("App", "BattleCalculator", "GetDeadSide")]
fn battlecalcultor_get_deadside(this: &BattleCalculator,method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01e7f750)]
pub fn battle_info_get_unit(this: &BattleInfo, index: i32,  method_info: OptionalMethod) -> Option<&'static Unit>;

#[skyline::from_offset(0x0246f1a0)]
pub fn battle_calculator_get_side(this: &BattleCalculator, side: i32, method_info: OptionalMethod) -> Option<&'static mut BattleInfoSide>;
