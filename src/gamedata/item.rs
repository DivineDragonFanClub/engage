pub use unity::prelude::*;
use super::{Gamedata, GodData, unit::GodUnit, skill::SkillArray};

#[unity::class("App", "UnitItemList")]
pub struct UnitItemList {}

#[unity::class("App", "ItemData")]
pub struct ItemData {
	structbase: [u8; 0x10],
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub usetype: u32,
	pub weaponattr: u32,
	pub icon: &'static Il2CppString,
	pub endurance: u8,
	pub power: u8,
}
impl Gamedata for ItemData { }

#[unity::class("App", "UnitItem")]
pub struct UnitItem {
    pub m_index: i32,
    pub m_item: &'static ItemData,
    pub m_endurance: u8,
    pub m_RefineLevel: u8,
    pub m_Flags :i32,
    pub m_Engrave: Option<&'static GodData>,
    pub m_GodUnit: Option<&'static GodUnit>,
}

impl UnitItem {
	pub fn get_equipped_skills(&self) ->  Option<&SkillArray> { unsafe { UnitItem_GetEquipSkills(self, None)}}
	pub fn get_power(&self) -> i32 { unsafe { UnitItem_GetPower(self, None)}}

	pub fn is_weapon(&self) -> bool { { unsafe { UnitItem_IsWeapon(self, None) }}}
	pub fn is_drop(&self) -> bool { { unsafe {  UnitItem_get_is_drop(self, None)}}}
	pub fn refine_data_exist(&self) -> bool { {unsafe { UnitItem_IsExistRefineData(self, None)}}}

	pub fn set_engrave(&self, engrave: &GodData) -> bool { unsafe { UnitItem_SetEngrave(self, engrave, None)}}
	pub fn set_refine_level(&self, level: i32) { unsafe { UnitItem_Set_RefineLevel(self, level, None); }}
}

impl UnitItemList {
	pub fn get_count(&self) -> i32 { unsafe { UnitItemList_Get_Count(self, None)}}
	pub fn add(&self, item: &ItemData) { unsafe { UnitItemList_Add(self, item, None); }}
	pub fn get_item(&self, index: i32) -> Option<&'static mut UnitItem> { unsafe { UnitItemList_Get_Item(self, index, None)}}
}


#[unity::from_offset("App", "UnitItemList", "get_Count")]
pub fn UnitItemList_Get_Count(this: &UnitItemList, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "get_Item")]
pub fn UnitItemList_Get_Item(this: &UnitItemList, index: i32, method_info: OptionalMethod) ->  Option<&'static mut UnitItem>;

#[skyline::from_offset(0x01fb3ab0)]
pub fn UnitItemList_Add(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "set_RefineLevel")]
pub fn UnitItem_Set_RefineLevel(this: &UnitItem, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsExistRefineData")]
pub fn UnitItem_IsExistRefineData(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "SetEngrave")]
pub fn UnitItem_SetEngrave(this: &UnitItem, data: &GodData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fad9e0)]
pub fn UnitItem_ctor(this: &UnitItem, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsWeapon")]
pub fn UnitItem_IsWeapon(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "get_IsDrop")]
pub fn UnitItem_get_is_drop(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "GetPower")]
pub fn UnitItem_GetPower(this: &UnitItem, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "GetEquipSkills")]
pub fn UnitItem_GetEquipSkills(this: &UnitItem, method_info: OptionalMethod) -> Option<&SkillArray>;