use num_derive::{FromPrimitive, ToPrimitive};
pub use unity::prelude::*;

use crate::{gamedata::unit::Unit, proc::{Bindable, ProcInstFields}};
#[repr(i32)]
#[derive(PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum MapSequenceHumanLabel {
    StartBranch = 0,
    FreeCursor = 1,
    PickCursor = 2,
    PickCursorResume = 3,
    PickCursorTick = 4,
    PickCursorWait = 5,
    PickFreeCursor = 6,
    PickMoveResume = 7,
    PickEngageStart = 8,
    PickEngageCancel = 9,
    SystemMenu = 10,
    TroopList = 11,
    RewindMenu = 12,
    SortiePositionChange = 13,
    SortieEndDialog = 14,
    UnitMove = 15,
    UnitCommand = 16,
    UnitSubCommand = 17,
    ItemMenuAttack = 18,
    ItemMenuEngageAttack = 19,
    ItemMenuEngageCharge = 20,
    ItemMenuEngageWait = 21,
    ItemMenuEngageRod = 22,
    ItemMenuCannon = 23,
    ItemMenuDestroy = 24,
    ItemMenuRod = 25,
    ItemMenuItem = 26,
    ItemMenuTrade = 27,
    ItemMenuFullBullet = 28,
    ItemMenuEnchantItem = 29,
    ItemMenuEnchantWeapon = 30,
    TransporterMenu = 31,
    TargetSelect = 32,
    WarpCursor = 33,
    RewarpCursor = 34,
    CreationCursor = 35,
    FireCannonCursor = 36,
    FullBulletCursor = 37,
    Talk = 38,
    EngageStart = 39,
    EngageLink = 40,
    EngageRewarp = 41,
    EngageRewarpCancel = 42,
    EngageSummonMenu = 43,
    EngageSummonBack = 44,
    GodChange = 45,
    Mind = 46,
    SaveMenu = 47,
    SuspendMenu = 48,
    TurnEnd = 49,
    JobIntro = 50,
    DirectAttack = 51,
    End = 52,
}
#[repr(C)]
#[unity::class("App", "MapSequenceHuman")]
pub struct MapSequenceHuman {
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    job_intro_unit: Option<&'static Unit>,
    job_intro_keyhelp_type: i32,
    return_label: i32,
    old_unit_x: i32,
    old_unit_z: i32,
    old_cursor_x: i32,
    old_cursor_z: i32,
    old_pickup_x: i32,
    old_pickup_z: i32,
    engage_x: i32,
    engage_z: i32,
    enter_x: i32,
    enter_z: i32,
    is_enemy_attack_range: bool,
    is_update_support_skill: bool,
    update_support_skill_unit: Option<&'static Unit>,
    operate_mode: i32,
}

impl Bindable for MapSequenceHuman { }

impl AsRef<ProcInstFields> for MapSequenceHuman {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MapSequenceHuman {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
