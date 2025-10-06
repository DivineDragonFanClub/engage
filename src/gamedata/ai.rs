use unity::prelude::*;
use unity::il2cpp::object::Array;
use super::{GamedataArray, PersonData, unit::Unit, StructDataArrayFields, WeaponMask};

#[unity::class("App", "AIData")]
pub struct AIData {
    pub parent: StructDataArrayFields,
    pub code: i8,
    pub mind: i8,
    pub active: i8,
    pub trans: i8,
    __: i32,
    pub str_value1: &'static Il2CppString,
    pub str_value2: &'static Il2CppString,
}
impl GamedataArray for AIData {}

#[unity::class("App", "UnitAI")]
pub struct UnitAI {
    pub flag: &'static mut WeaponMask,
    pub band: u8,
    pub active: u8,
    pub priority: u8,
    pub heal_rate_a: u8,
    pub heal_rate_b: u8,
    pub battle_rate_type: u8,
    pub prohibit_engage_attack: u8,
    pub prohibit_rod: u8,
    pub prohibit_overlap: u8,
    pub rerewarp_count: u8,
    pub rerewarp_count_max: u8,
    pub rerewarp_last_x: u8,
    pub rerewarp_last_z: u8,
    pub rerwarp_event: Option<&'static Il2CppString>,
    pub random_flag: &'static mut WeaponMask,
    pub move_limit: &'static MoveLimitRange,
    pub vs_type: u8,
    pub bullet_pattern: u8,
    pub sequence: &'static Array<&'static Il2CppString>,
    pub value: &'static Array<&'static AIValue>,
    pub unit: &'static Unit,
    pub vs_think: i32,
}

#[unity::class("App", "UnitAI.MoveLimitRange")]
pub struct MoveLimitRange {
    pub m_type: u8,
    pub x: i8,
    pub z: i8,
    pub w: i8,
    pub h: i8,
}

#[unity::class("App", "AIValue")]
pub struct AIValue {
    pub v8_1: i8,
    pub v8_2: i8,
    pub v16: i16,
}


impl UnitAI {
    pub fn setup_versus_ai(&self) { unsafe { set_up_versus_ai(self, None) }  }
    pub fn set_versus_ai_type(&self, value: i32) { unsafe { set_versus_type(self, value, None) } }
    pub fn set_active(&self, value: u8) { unsafe { unitai_set_active(self, value, None); }}
    pub fn set_sequence<'a>(&self, order: i32, name: impl Into<&'a Il2CppString>) {
        unsafe { unitai_set_sequence(self, order, name.into(), None) }
    }
    pub fn set_flag(&self, value: i32) {
        unsafe { unitai_set_flag(self, value, None); }
    }
    pub fn set_value(&self, order: i32, index: i32, value: i32) {
        unsafe { set_unit_ai_value(self, order, index, value, None) };
    }
    pub fn set_value_str<'a>(&self, order: i32, index: i32, str: impl Into<&'a Il2CppString>) {
        unsafe { unitai_set_value_str(self, order, index, str.into(), None) };
    }

    pub fn get_value(&self, order: i32, index: i32) -> &'static mut AIValue {
        unsafe { unitai_get_value(self, order, index, None) }
    }
}
impl AIValue {
    pub fn get_person(&self) -> Option<&'static PersonData> { unsafe { aivalue_get_person(self, None )} }
    pub fn set_str_value<'a>(&self, str: impl Into<&'a Il2CppString>) { unsafe { aivalue_set_str_value(self, str.into(), None) };  }
    pub fn set_value(&self, value: i16) { unsafe { aivalue_set_value(self, value, None) }; }
    pub fn get_value(&self) -> i32 { unsafe { ai_value_get_value(self, None) } }
    pub fn get_flag_value(&self) -> i32 { unsafe { aivalue_get_flag_value(self, None) } }
}

#[skyline::from_offset(0x01f5f8d0)]
fn set_unit_ai_value(this: &UnitAI, order: i32, index: i32, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01f60810)]
pub fn set_up_versus_ai(this: &UnitAI, method_info: OptionalMethod);

#[skyline::from_offset(0x01f5f4c0)]
pub fn set_versus_type(this: &UnitAI, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitAI", "set_Active")]
fn unitai_set_active(this: &UnitAI, value: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x01f5f660)]
fn unitai_set_sequence(this: &UnitAI, order: i32, name: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitAI", "SetFlag")]
fn unitai_set_flag(this: &UnitAI, flag_value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01f5fa70)]
fn unitai_set_value_str(this: &UnitAI, order: i32, index: i32, str: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitAI", "GetValue")]
fn unitai_get_value(this: &UnitAI,   order: i32, index: i32, method_info: OptionalMethod) -> &'static mut AIValue;

#[skyline::from_offset(0x027b2e80)]
fn aivalue_set_str_value(this: &AIValue, str: &Il2CppString, method_info: OptionalMethod); 

#[skyline::from_offset(0x027b2830)]
fn aivalue_set_value(this: &AIValue, str: i16, method_info: OptionalMethod); 

#[unity::from_offset("App", "AIValue", "GetPerson")]
fn aivalue_get_person(this: &AIValue, method_info: OptionalMethod) -> Option<&'static PersonData>;

#[skyline::from_offset(0x027b2820)]
pub fn ai_value_get_value(this: &AIValue, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "AIValue", "GetFlagValue")]
fn aivalue_get_flag_value(this: &AIValue, method_info: OptionalMethod) -> i32;