pub use unity::prelude::*;

use crate::{gamedata::unit::Unit, proc::{Bindable, ProcInstFields}};

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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
    None = -1,
}

impl From<i32> for MapSequenceHumanLabel {
    fn from(value: i32) -> Self {
        match value {
            0 => MapSequenceHumanLabel::StartBranch, 
            1 => MapSequenceHumanLabel::FreeCursor, 
            2 => MapSequenceHumanLabel::PickCursor, 
            3 => MapSequenceHumanLabel::PickCursorResume, 
            4 => MapSequenceHumanLabel::PickCursorTick, 
            5 => MapSequenceHumanLabel::PickCursorWait, 
            6 => MapSequenceHumanLabel::PickFreeCursor, 
            7 => MapSequenceHumanLabel::PickMoveResume, 
            8 => MapSequenceHumanLabel::PickEngageStart, 
            9 => MapSequenceHumanLabel::PickEngageCancel, 
            10 => MapSequenceHumanLabel::SystemMenu, 
            11 => MapSequenceHumanLabel::TroopList, 
            12 => MapSequenceHumanLabel::RewindMenu, 
            13 => MapSequenceHumanLabel::SortiePositionChange, 
            14 => MapSequenceHumanLabel::SortieEndDialog, 
            15 => MapSequenceHumanLabel::UnitMove, 
            16 => MapSequenceHumanLabel::UnitCommand, 
            17 => MapSequenceHumanLabel::UnitSubCommand, 
            18 => MapSequenceHumanLabel::ItemMenuAttack, 
            19 => MapSequenceHumanLabel::ItemMenuEngageAttack, 
            20 => MapSequenceHumanLabel::ItemMenuEngageCharge, 
            21 => MapSequenceHumanLabel::ItemMenuEngageWait, 
            22 => MapSequenceHumanLabel::ItemMenuEngageRod, 
            23 => MapSequenceHumanLabel::ItemMenuCannon, 
            24 => MapSequenceHumanLabel::ItemMenuDestroy, 
            25 => MapSequenceHumanLabel::ItemMenuRod, 
            26 => MapSequenceHumanLabel::ItemMenuItem, 
            27 => MapSequenceHumanLabel::ItemMenuTrade, 
            28 => MapSequenceHumanLabel::ItemMenuFullBullet, 
            29 => MapSequenceHumanLabel::ItemMenuEnchantItem, 
            30 => MapSequenceHumanLabel::ItemMenuEnchantWeapon, 
            31 => MapSequenceHumanLabel::TransporterMenu, 
            32 => MapSequenceHumanLabel::TargetSelect, 
            33 => MapSequenceHumanLabel::WarpCursor, 
            34 => MapSequenceHumanLabel::RewarpCursor, 
            35 => MapSequenceHumanLabel::CreationCursor, 
            36 => MapSequenceHumanLabel::FireCannonCursor, 
            37 => MapSequenceHumanLabel::FullBulletCursor, 
            38 => MapSequenceHumanLabel::Talk, 
            39 => MapSequenceHumanLabel::EngageStart, 
            40 => MapSequenceHumanLabel::EngageLink, 
            41 => MapSequenceHumanLabel::EngageRewarp, 
            42 => MapSequenceHumanLabel::EngageRewarpCancel, 
            43 => MapSequenceHumanLabel::EngageSummonMenu, 
            44 => MapSequenceHumanLabel::EngageSummonBack, 
            45 => MapSequenceHumanLabel::GodChange, 
            46 => MapSequenceHumanLabel::Mind, 
            47 => MapSequenceHumanLabel::SaveMenu, 
            48 => MapSequenceHumanLabel::SuspendMenu, 
            49 => MapSequenceHumanLabel::TurnEnd, 
            50 => MapSequenceHumanLabel::JobIntro, 
            51 => MapSequenceHumanLabel::DirectAttack, 
            52 => MapSequenceHumanLabel::End, 
            _ => MapSequenceHumanLabel::None,
        }
    }
}


