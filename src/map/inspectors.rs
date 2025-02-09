use std::iter::Map;

use unity::prelude::*;
use unity::system::List;
use crate::script::DynValue;
use crate::util::get_instance;
use unity::il2cpp::object::Array;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapInspectorKind {
	None = 0,
	Turn = 1,
	TurnAfter = 2,
	TurnEnd = 3,
	Area = 4,
	Tbox = 5,
	Door = 6,
	Torch = 7,
	Visit = 8,
	Escape = 9,
	Destroy = 10,
	Breakdown = 11,
	BreakdownEnemy = 12,
	Waypoint = 13,
	Command = 14,
	Die = 15,
	ReviveBefore = 16,
	ReviveAfter = 17,
	Fixed = 18,
	Talk = 19,
	BattleBefore = 20,
	BattleTalk = 21,
	BattleAfter = 22,
	Pickup = 23,
	TargetSelect = 24,
	UnitCommandPrepare = 25,
	UnitCommandInterrupt = 26,
	EngageBefore = 27,
	EngageAfter = 28,
	Cannon = 29,
	HelpSpot = 30,
}


/// Singleton Class that contains all MapInspectors
#[unity::class("App", "MapInspectors")]
pub struct MapInspectors {
    parent: [u8; 0x10],
    pub inspectors: &'static mut List<MapInspector>,
    pub kind_inspectors: &'static mut Array<&'static mut List<MapInspector>>,
}

impl MapInspectors {
    pub fn get_instance() -> &'static mut MapInspectors { get_instance::<Self>() }
    pub fn add<T: Inspector>(inspector: &T) { unsafe { mapinspectors_add(inspector, None); } }
    pub fn get_kind_inspectors<T: Inspector>(kind: MapInspectorKind) -> Option<&'static mut List<T>> { 
        unsafe { std::mem::transmute( mapinspectors_get_kind(kind, None) ) }
    }
}

/// Base MapInspector Class
#[unity::class("App", "MapInspector")]
pub struct MapInspector {
    pub kind: MapInspectorKind,
    pub condition: Option<&'static DynValue>,
    pub function: Option<&'static DynValue>,
    pub args: Option<&'static Array<&'static DynValue>>,
}

impl MapInspector {
    pub fn cast<T: Inspector>(&self) -> &T {
        unsafe { std::mem::transmute::<&MapInspector, &T>(self) }
    }

    pub fn cast_mut<T: Inspector>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut MapInspector, &mut T>(self) }
    }
}
pub trait Inspector: Il2CppClassData + Sized {}

#[unity::class("App", "PokeInspector")]
pub struct PokeInspector {
    pub parent: MapInspectorFields,
    pub x: i32,
    pub z: i32,
    pub w: i32,
    pub h: i32,
    pub max_hp: i32,
    pub person: i32,
    pub hp_label: Option<&'static Il2CppString>,
}
impl Inspector for PokeInspector {}

#[unity::class("App", "UnitInspector")]
pub struct UnitInspector {
    pub parent: MapInspectorFields,
    pub person: i32,
    pub force: i32,
}
// Same as FixedInspector (Kind 18)
impl Inspector for UnitInspector {}

#[unity::class("App", "TalkInspector")]
pub struct TalkInspector {
    pub parent: MapInspectorFields,
    pub from_person: i32,
    pub from_force: i32,
    pub to_person: i32,
    pub to_force: i32,
    pub both: bool,
}
// Same as EachInspector
impl Inspector for TalkInspector {}

#[unity::from_offset("App", "MapInspectors", "Add")]
fn mapinspectors_add<T: Inspector>(inspector: &T, method_info: OptionalMethod);

#[unity::from_offset("App", "MapInspectors", "GetKindInspector")]
fn mapinspectors_get_kind(kind: MapInspectorKind, method_info: OptionalMethod) -> Option<&'static mut List<MapInspector>>;

