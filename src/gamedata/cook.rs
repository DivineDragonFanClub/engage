use unity::prelude::*;
use crate::gamedata::{*, person::CapabilitySbyte};

// CookData, TasteData and TasteConditionData
#[unity::class("App", "TasteDataFlag")]
pub struct TasteDataFlag {
    pub value: i32
}

#[unity::class("App", "TasteData")]
pub struct TasteData {
    pub parent: StructBaseFields, 
    pub tid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub grade: i32,
    pub augment: i8,
    pub other_enhance: i8,
    pub enhanced: &'static CapabilitySbyte,
    pub flag: &'static TasteDataFlag,
    pub cid: &'static Il2CppString,
}

#[unity::class("App", "CookData")]
pub struct CookData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub taste1: &'static Il2CppString,
    pub taste2: &'static Il2CppString,
    pub taste3: &'static Il2CppString,
}

#[unity::class("App", "TasteConditionData")]
pub struct TasteConditionData {}

impl Gamedata for CookData { }
impl Gamedata for TasteData { }
impl Gamedata for TasteConditionData {}

impl TasteData {
    pub fn get_name(&self) -> &'static Il2CppString { unsafe { Taste_GetName(self, None) } }
    pub fn get_grade(&self) -> &'static Il2CppString { unsafe { Taste_GetGrade(self, None)}}
}

impl TasteConditionData {
    pub fn get_name(&self) -> Option<&Il2CppString> { unsafe { TasteCondition_GetName(self, None)}}
}

#[unity::from_offset("App","TasteConditionData", "GetName")]
pub fn TasteCondition_GetName(this: &TasteConditionData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App","TasteData", "GetName")]
pub fn Taste_GetName(this: &TasteData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "TasteData", "GetGradeString")]
pub fn Taste_GetGrade(this: &TasteData, method_info: OptionalMethod) -> &'static Il2CppString;
