use std::result;

pub use unity::prelude::*;
pub use unity::engine::Color;
pub use unity::system::*;
use crate::bit::*;
use super::{*, unit::*, item::ItemData};

#[unity::class("App", "AssetTable")]
pub struct AssetTable {
    pub parent: StructBaseFields,
    pub preset_name: Option<&'static Il2CppString>,
    pub mode: i32,
    __: i32,
    pub conditions: Option<&'static mut Array<&'static mut Il2CppString>>,
    pub body_model: Option<&'static Il2CppString>,
    pub dress_model: Option<&'static Il2CppString>,
    pub head_model: Option<&'static Il2CppString>,
    pub hair_model: Option<&'static Il2CppString>,
    pub ride_model: Option<&'static Il2CppString>,
    pub ride_dress_model: Option<&'static Il2CppString>,
    pub left_hand: Option<&'static Il2CppString>,
    pub right_hand: Option<&'static Il2CppString>,
    pub trail: Option<&'static Il2CppString>,
    pub magic: Option<&'static Il2CppString>,
    pub body_anim: Option<&'static Il2CppString>, 
    pub ride_anim: Option<&'static Il2CppString>,
    pub info_anim: Option<&'static Il2CppString>,
    pub talk_anim: Option<&'static Il2CppString>,
    pub demo_anim: Option<&'static Il2CppString>,
    pub hub_anim: Option<&'static Il2CppString>,
    pub hair_r: u8,
    pub hair_g: u8,
    pub hair_b: u8,
    pub grad_r: u8,
    pub grad_g: u8,
    pub grad_b: u8,
    pub skin_r: u8,
    pub skin_g: u8,
    pub skin_b: u8,
    pub toon_r: u8,
    pub toon_g: u8,
    pub toon_b: u8,
    pub mask_color_100_r: u8,
    pub mask_color_100_g: u8,
    pub mask_color_100_b: u8,
    pub mask_color_075_r: u8,
    pub mask_color_075_g: u8,
    pub mask_color_075_b: u8,
    pub mask_color_050_r: u8,
    pub mask_color_050_g: u8,
    pub mask_color_050_b: u8,
    pub mask_color_025_r: u8,
    pub mask_color_025_g: u8,
    pub mask_color_025_b: u8,
    pub unity_colors: [Color; 8],
    pub accessories: [&'static mut AssetTableAccessory; 8],
    pub accessory_list: &'static AssetTableAccessoryList,
    pub scale_stuff: [f32; 19], 
    ___: i32,
    pub voice: Option<&'static Il2CppString>,
    pub foot_steps: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
    pub comment: Option<&'static Il2CppString>,
    pub condition_indexes: &'static mut AssetTableConditionIndexes,
}
impl Gamedata for AssetTable {}

impl AssetTable {
    pub fn add_condition_key<'a>(key: impl Into<&'a Il2CppString>) {
        Self::class().get_static_fields::<AssetTableStaticFields>().condition_flags.add_by_key(key.into());
    }
}

#[unity::class("App", "AsssetTable.ConditionFlags")]
pub struct AssetTableConditionFlags {
    pub bits: BitStructFields,
    pub keys: &'static List<Il2CppString>,
    pub hits: &'static SimpleList<i32>,
    pub dic: &'static Dictionary<'static, &'static Il2CppString, i32>,

}

impl AssetTableConditionFlags {
    pub fn add_by_key<'a>(&self, key: impl Into<&'a Il2CppString>) { unsafe { condition_add_by_key(self, key.into(), None);}  }
    pub fn add_unit(&self, unit: &Unit ) { unsafe { condition_add_unit(self, unit, None);}  }
}

#[repr(C)]
pub struct AssetTableStaticFields { 
    pub preset_name: &'static List<Il2CppString>,
    pub search_lists: &'static mut Array<&'static mut List<AssetTable>>,
    pub condition_indexes: &'static Dictionary<'static, &'static Il2CppString, i32>,
    pub condition_flags: &'static mut AssetTableConditionFlags,
    pub null_sound: AssetTableSound,
    pub null_color: Color,
    pub shared: &'static AssetTableResult,
}

impl AssetTableStaticFields {
    pub fn get() -> &'static mut Self {
        AssetTable::class().get_static_fields_mut::<AssetTableStaticFields>()
    }
    pub fn get_condition_index(key: impl Into<&'static Il2CppString> ) -> i32 {
        let mut return_value = 0;
        let found =  Self::get().condition_indexes.try_get_value(key.into(), &mut return_value);
        if found { return_value.clone() } else { -1 }
    }
}


pub struct AssetTableSound {
    pub voice: Option<&'static Il2CppString>,
    pub footstep: Option<&'static Il2CppString>,
    pub material: Option<&'static Il2CppString>,
}

#[unity::class("App", "AssetTable.Result")]
pub struct AssetTableResult {
    pub pid: &'static Il2CppString,
    pub jid: &'static Il2CppString,
    pub body_model: &'static Il2CppString,
    pub dress_model: &'static Il2CppString,
    pub head_model: &'static Il2CppString,
    pub hair_model: &'static Il2CppString,
    pub ride_model: &'static Il2CppString,
    pub ride_dress_model: &'static Il2CppString,
    pub left_hand: &'static Il2CppString,
    pub right_hand: &'static Il2CppString,
    pub trail: &'static Il2CppString,
    pub magic: &'static Il2CppString,
    pub body_anim: Option<&'static Il2CppString>,
    pub ride_anim: Option<&'static Il2CppString>,
    pub unity_colors: [Color; 8],
    pub scale_stuff: [f32; 19], 
    __ : i32,
    pub sound: AssetTableSound,
    pub info_anims: Option<&'static Il2CppString>,
    pub talk_anims: Option<&'static Il2CppString>,
    pub demo_anims: Option<&'static Il2CppString>,
    pub hub_anims: Option<&'static Il2CppString>,
    pub force_id: Option<&'static Il2CppString>,
    pub weapon_id: Option<&'static Il2CppString>,
    pub body_anims: &'static mut List<Il2CppString>,
    pub accessory_list: &'static mut AssetTableAccessoryList,
    pub accessory_dictionary: &'static Dictionary<'static, &'static Il2CppString, &'static AssetTableAccessory>,
}

impl AssetTableResult {
    pub fn get_from_pid<'a>(mode: i32, pid: impl Into<&'a Il2CppString>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult { 
        unsafe { result_get_from_pid(mode, pid.into(), conditions, None) } 
    }
    /// Generates a new result from GodUnit
    /// 
    /// Calls setup_from_god
    pub fn get_from_god_unit(mode: i32, god_unit: &GodUnit, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult {
        unsafe { result_get_from_god(mode, god_unit, conditions, None) }
    }
    /// Generates a new result from an AssetTable Preset 
    /// 
    /// Used for Epharim in Twin Strike Engage Attack
    pub fn get_from_preset<'a>(name: impl Into<&'a Il2CppString>) -> &'static mut AssetTableResult {
        unsafe { asset_table_result_get_preset_name(name.into(), None) }
    }

    pub fn commit_asset_table(&self, data: &AssetTable) { unsafe { asset_table_commit_result(self, data, None); } }

    pub fn setup_for_person(&self, mode: i32, person: Option<&PersonData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult {
        unsafe { result_setup_for_person(self, mode, person, conditions, None) }
    }

    /// Clears the current result and generates the result from PersonData
    /// 
    /// Used in Dragonstone transformation
    /// 
    /// Returns itself
    pub fn setup_for_person_job_item(&self, mode: i32, person: Option<&PersonData>,  job: Option<&JobData>, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult {
        unsafe { asset_table_result_setup_person(self, mode, person, job, equipped, conditions, None) }
    }
    /// Clears the current result and generates the result from GodData
    /// 
    /// Used for Emblem UnitInfo / Emblem Kizuna / Emblem Hub
    /// 
    /// Returns itself
    pub fn setup_for_god(&self, mode: i32, god: Option<&GodData>, is_darkness: bool,  conditions: &Array<&Il2CppString>) ->  &'static mut AssetTableResult {
        unsafe { asset_table_result_god_setup(self, mode, god, is_darkness, conditions, None)}
    }
    
    /// Clears the current result and generate the result from Unit and equipped items
    /// Used in Combat/Hub/UnitInfo 
    /// Return itself
    pub fn setup_for_unit(&self, mode: i32, unit: Option<&Unit>, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>) -> &'static mut AssetTableResult {
        unsafe { result_setup_from_unit(self, mode, unit, equipped, conditions, None) }
    }
    pub fn clear(&self) { unsafe { result_clear(self, None) } }
    pub fn commit(&self, mode: i32, person: Option<&PersonData>, job: Option<&JobData>, equipped: Option<&ItemData>) { unsafe { result_commit(self, mode, person, job, equipped, None); }}
    pub fn commit_accessory(&self, accessory: &AssetTableAccessory) { unsafe { result_commit_accessory(self, accessory, None); }}
    pub fn commit_mode(&self, mode: i32) { unsafe { result_commit_mode(self, mode, None);}}
    pub fn replace(&self, mode: i32) { unsafe { result_replace(self, mode, None);}}
}


#[unity::class("AssetTable", "Accessory")]
pub struct AssetTableAccessory {
    pub locator: Option<&'static Il2CppString>,
    pub model: Option<&'static Il2CppString>, 
}
impl AssetTableAccessory {
    pub fn to_string(&self) -> &'static Il2CppString { unsafe { accessory_to_string(self, None)}}
}
#[unity::class("AssetTable", "ConditionIndexes")]
pub struct AssetTableConditionIndexes {
    pub list: &'static mut List<SimpleList<i32>>,
}

#[unity::class("AssetTable", "AccessoryList")]
pub struct AssetTableAccessoryList {
    pub list: ListFields<AssetTableAccessory>,
}

impl AssetTableAccessoryList {
    pub fn try_add(&self, accessory: &AssetTableAccessory) { unsafe { accessory_list_try_add(self, accessory, None)}}
    pub fn clear(&self) {
        if let Some(class) = AssetTable::class().get_nested_types().iter().find(|x| x.get_name() == "AccessoryList") {
            let method = class.get_virtual_method("Clear").unwrap();
            let clear = unsafe { std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo)>(method.method_ptr) };
            clear(self, method.method_info);
        }
    }
}
// non reference list
#[repr(C)]
#[unity::class("System.Collections.Generic", "List`1")]
pub struct SimpleList<T: 'static> {
    pub items: &'static mut Il2CppArray<T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T: 'static> Deref for SimpleListFields<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.items.m_items.as_ptr(), self.size as usize) }
    }
}

impl<T: 'static> DerefMut for SimpleListFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.items.m_items.as_mut_ptr(), self.size as usize) }

    }
}

impl<T> SimpleList<T> {
    pub fn resize(&mut self, length: usize) {
        if self.items.len() != length {
            let new_array = unity::il2cpp::object::Il2CppArray::new_specific(self.items.get_class(), length as _).unwrap();
            new_array[..self.items.len()].swap_with_slice(self.items);
            self.items = new_array;
        }
    }

    pub fn add(&mut self, element: T) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Add")))
            .unwrap();

        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, T, &MethodInfo)>(
                method.method_ptr,
            )
        };

        add(self, element, method);
    }
    pub fn insert(&mut self, index: i32, element: T) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Insert")))
            .unwrap();

        let add = unsafe { std::mem::transmute::<_, extern "C" fn(&mut Self, i32, T, &MethodInfo)>(method.method_ptr)  };

        add(self, index, element, method);
    }
    pub fn len(&self) -> usize { self.size as _ }

    pub fn capacity(&self) -> usize { self.items.len() as _ }

    pub fn clear(&mut self) {
        let method = self.get_class().get_virtual_method("Clear").unwrap();

        let clear = unsafe { std::mem::transmute::<_, extern "C" fn(&mut Self, &MethodInfo)>(method.method_info.method_ptr) };
        clear(self, method.method_info);
    }
    pub fn remove(&mut self, item: T) -> bool {
        let method = self.get_class().get_virtual_method("Remove").unwrap();
        let remove = unsafe { std::mem::transmute::<_, extern "C" fn(&mut Self, T, &MethodInfo) -> bool >(method.method_info.method_ptr) };
        remove(self, item, method.method_info)
    }
}

#[unity::from_offset("App","AssetTable", "set_Conditions")]
fn asset_table_set_conditions(this: &AssetTable, value: &Array<&Il2CppString>, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", "set_Conditions")]
fn asset_table_mut_set_conditions(this: &mut AssetTable, value: &Array<&Il2CppString>, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", "get_Conditions")]
fn asset_table_get_conditions(this: &AssetTable, method_info: OptionalMethod) -> &'static mut Array<&'static Il2CppString>;

#[unity::from_offset("App","AssetTable", ".ctor")]
fn asset_table_ctor(this: &AssetTable, method_info: OptionalMethod);

#[unity::from_offset("App","AssetTable", ".cctor")]
fn asset_table_cctor( method_info: OptionalMethod);

#[skyline::from_offset(0x01bafdd0)]
fn condition_add_by_key(condition: &AssetTableConditionFlags, key: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb0200)]
fn condition_add_unit(condition: &AssetTableConditionFlags, unit: &Unit, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb7980)]
fn result_get_from_god(mode: i32, god_unit: &GodUnit, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb5be0)]
fn result_get_from_pid(mode: i32, pid: &Il2CppString, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb4180)]
fn result_setup_for_person(this: &AssetTableResult, mode: i32, person: Option<&PersonData>, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb2430)]
fn result_setup_from_unit(this: &AssetTableResult, mode: i32, unit: Option<&Unit>, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb4290)]
fn asset_table_result_setup_person(this: &AssetTableResult, mode: i32, person: Option<&PersonData>, job: Option<&JobData>, equipped: Option<&ItemData>, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb2d80)]
fn asset_table_result_god_setup(this: &AssetTableResult, mode: i32, god_data: Option<&GodData>, is_darkness: bool, conditions: &Array<&Il2CppString>, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb44d0)]
fn result_commit_mode(this: &AssetTableResult, mode: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb7ca0)]
fn asset_table_result_get_preset_name(preset_name: &Il2CppString, method_info: OptionalMethod) -> &'static mut AssetTableResult;

#[skyline::from_offset(0x01bb2ee0)]
fn asset_table_commit_result(this: &AssetTableResult, data: &AssetTable, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb2a80)]
fn result_commit(this: &AssetTableResult, mode: i32, person: Option<&PersonData>, job:  Option<&JobData>, equipped: Option<&ItemData>, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb48b0)]
fn result_commit_accessory(this: &AssetTableResult, accessory: &AssetTableAccessory, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb2750)]
fn result_clear(this: &AssetTableResult, method_info: OptionalMethod);

#[skyline::from_offset(0x01bb3be0)]
fn result_replace(this: &AssetTableResult, mode: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01baf640)]
fn accessory_list_try_add(this: &AssetTableAccessoryList, accessory: &AssetTableAccessory, method_info: OptionalMethod);

#[skyline::from_offset(0x01baf5a0)]
fn accessory_to_string(accessory: &AssetTableAccessory, method_info: OptionalMethod) -> &'static Il2CppString;