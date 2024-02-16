pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use super::{JobData, WeaponMask, PersonData, item::ItemData};

// Structs, methods required for PersonData, JobData, SkillArray

#[unity::class("App", "Capability")]
pub struct Capability {
    pub m_Data: &'static mut Array<u8>,
}

#[unity::class("App", "CapabilitySbyte")]
pub struct CapabilitySbyte {
    pub m_Data: &'static mut Array<i8>,
}
impl CapabilitySbyte {
    pub fn is_zero(&self) -> bool { unsafe { CapabilitySbyte_is_zero(self, None)} }
    pub fn add(&self, index: i32, value: i8) { unsafe { CapabilitySbyte_add(self, index, value, None); }}
}

impl Capability {
    pub fn is_zero(&self) -> bool { unsafe { Capability_is_zero(self, None)} }
    pub fn add(&self, index: i32, value: u8) { unsafe { Capability_add(self, index, value, None); }}
}

#[unity::class("App", "SkillArray")]
pub struct SkillArray {}

impl SkillArray {
    pub fn clear(&self){ unsafe { skillarray_clear(self, None); } }
}

#[unity::class("App", "PersonDataFlag")]
pub struct PersonDataFlag { 
    pub value: i32,
}
#[unity::class("App", "JobDataFlag")]
pub struct JobDataFlag { 
    pub value: i32,
}

impl PersonData {
    // Getters
    pub fn get_aptitude(&self) -> &'static WeaponMask { unsafe {person_get_apt(self, None)}}
    pub fn get_sub_aptitude(&self) -> &'static WeaponMask { unsafe { person_get_sub_apt(self, None)}}
    pub fn get_ascii_name(&self) -> &Il2CppString { unsafe {person_get_ascii_name(self, None) } }
    pub fn get_asset_force(&self) -> i32 { unsafe { person_get_asset_force(self, None) }  }
    pub fn get_attrs(&self) -> i32 { unsafe { person_get_attrs(self, None)} }
    pub fn get_combat_bgm(&self) -> &Il2CppString { unsafe { person_get_combat_bgm(self, None)}}
    pub fn get_common_skills(&self) -> &SkillArray { unsafe { person_get_CommonSkill(self, None) }  }
    pub fn get_flag(&self) -> &PersonDataFlag { unsafe { person_get_flag(self, None) }}
    pub fn get_gender(&self) -> i32 { unsafe { person_get_gender(self, None)}  }
    pub fn get_grow(&self) -> &mut Capability { unsafe { person_get_grow(self, None) } }
    pub fn get_help(&self) -> &Il2CppString { unsafe {person_get_help(self, None) }}
    pub fn get_internal_level(&self) -> i8 { unsafe { person_get_InternalLevel(self, None)} }
    pub fn get_job(&self) -> Option<&JobData> { unsafe { person_get_job(self, None) } }
    pub fn get_jid(&self) -> Option<&Il2CppString> { unsafe { person_get_jid(self, None) }}
    pub fn get_level(&self) -> u8 { unsafe { person_get_level(self, None) } }
    pub fn get_limit(&self) -> &mut CapabilitySbyte {  unsafe { person_get_limit(self, None) } }
    pub fn get_name(&self) -> Option<&Il2CppString> {  unsafe { person_get_name(self, None) } }
    pub fn get_summon_rank(&self) -> i32 { unsafe { person_get_summon_rank(self, None)}}
    pub fn get_unit_icon_id(&self) -> &Il2CppString { unsafe { get_UnitIconID(self, None )}}

    // Setters
    pub fn set_aid(&self, aid: &Il2CppString) { unsafe { }}
    pub fn set_sub_aptitude(&self, mask: &WeaponMask) { unsafe { person_set_sub_apt(self, mask, None)}}
    pub fn set_ascii_name(&self, name: &Il2CppString) { unsafe { person_set_ascii_name(self, name, None); }}
    pub fn set_attrs(&self, attr: i32) { unsafe { person_set_attrs(self, attr, None); }}
    pub fn set_gender(&self, gender: i32) { unsafe { person_set_gender(self, gender, None); }}
    pub fn set_grow(&self, value: &Capability) { unsafe { person_set_grow(self, value, None); }}
    pub fn set_flag(&self, value: &PersonDataFlag) { unsafe { person_set_flag(self, value, None); }}
    pub fn set_help(&self, help: &Il2CppString) { unsafe { person_set_help(self, help, None); }}
    pub fn set_internal_level(&self, value: i8) { unsafe { person_set_InternalLevel(self, value, None); }}
    pub fn set_jid(&self, jid: &Il2CppString) { unsafe { person_set_jid(self, jid, None); }}
    pub fn set_level(&self, level: u8) { unsafe { person_set_level(self, level, None); }}
    pub fn set_limit(&self, limits: &CapabilitySbyte) { unsafe { person_set_limit(self, limits, None); }}
    pub fn set_name(&self, name: &Il2CppString) { unsafe { person_set_name(self, name, None); }}
    pub fn set_unit_icon_id(&self, icon_id: &Il2CppString) { unsafe { person_set_UnitIconID(self, icon_id, None ); }}
    
}

impl JobData {
    pub fn is_low(&self) -> bool { unsafe { job_is_low(self, None)} }
    pub fn is_high(&self) -> bool { unsafe { job_is_high(self, None) }}
    pub fn has_high(&self) -> bool { unsafe { job_has_high_job(self, None) }}

    pub fn get_base(&self) -> &mut Capability { unsafe {job_get_base(self, None) } }
    pub fn get_diff_grow(&self) -> &mut CapabilitySbyte {unsafe {job_get_DiffGrow(self, None)}}
    pub fn get_diff_grow_l(&self) -> &mut CapabilitySbyte { unsafe { job_get_DiffGrowL(self, None) }}
    pub fn get_diff_grow_h(&self) -> &mut CapabilitySbyte { unsafe { job_get_DiffGrowH(self, None) }}
    pub fn get_diff_grow_n(&self) -> &mut CapabilitySbyte { unsafe { job_get_DiffGrowN(self, None) }}
    pub fn get_flag(&self) -> &JobDataFlag { unsafe { job_Get_Flag(self, None)}}
    pub fn get_high_jobs(&self) -> &List<JobData> { unsafe { job_GetHighJobs(self, None) }}
    pub fn get_high_job_1(&self) -> Option<&Il2CppString> { unsafe { job_get_high_job1(self, None)}}
    pub fn get_high_job_2(&self) -> Option<&Il2CppString> { unsafe { job_get_high_job2(self, None)}}
    pub fn get_job_style(&self) -> Option<&Il2CppString> { unsafe { get_JobStyle(self, None)}}
    pub fn get_internal_level(&self) -> i8 { unsafe { get_job_internal_level(self, None)}}
    pub fn get_limit(&self) -> &mut Capability { unsafe { job_get_limit(self, None) } }
    pub fn get_low_jobs(&self) -> &List<JobData> { unsafe {job_GetLowJobs(self, None)} }
    pub fn get_max_level(&self) -> u8 { unsafe { job_max_level(self, None)}}
    pub fn get_unique_items(&self) -> &Array<Il2CppString> { unsafe { job_get_uniqueItems(self, None)}}

    pub fn set_diff_grow(&self, grow :&CapabilitySbyte) {unsafe {job_set_DiffGrow(self, grow, None)}}
    pub fn set_diff_grow_l(&self,grow :&CapabilitySbyte) { unsafe { job_set_DiffGrowL(self, grow, None) }}
    pub fn set_diff_grow_h(&self,grow :&CapabilitySbyte) { unsafe { job_set_DiffGrowH(self, grow, None) }}
    pub fn set_diff_grow_n(&self,grow :&CapabilitySbyte) { unsafe { job_set_DiffGrowN(self, grow, None) }}

    pub fn set_limit(&self, limits: &Capability) { unsafe { job_set_limit(self, limits, None); }}
    pub fn set_max_level(&self, value: u8) { unsafe { job_set_maxLevel(self, value, None); }}
}
#[skyline::from_offset(0x24820c0)]
pub fn add_skill_array(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod) -> bool;



// PersonData 
#[unity::from_offset("App", "PersonData", "get_Name")] //#[skyline::from_offset(0x1f25d40)]
fn person_get_name(this: &PersonData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App", "PersonData", "get_UnitIconID")] //#[skyline::from_offset(0x1f25d20)]
fn get_UnitIconID(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_Gender")] //#[skyline::from_offset(0x1f25da0)]
fn person_get_gender(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_Grow")] //#[skyline::from_offset(0x1f26020)]
fn person_get_grow(this: &PersonData, method_info: OptionalMethod) -> &mut Capability;

#[skyline::from_offset(0x1f26140)]
fn person_get_combat_bgm(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_CommonSids")] //#[skyline::from_offset(0x1f26040)]
fn get_CommonSids(this: &PersonData, method_info: OptionalMethod) -> Option<&mut Array<&Il2CppString>>;

#[skyline::from_offset(0x1f2a6f0)]
fn person_get_CommonSkill(this: &PersonData, method_info: OptionalMethod) -> &SkillArray;

#[skyline::from_offset(0x1f26000)]
fn person_get_limit(this: &PersonData, method_info: OptionalMethod) -> & mut CapabilitySbyte;

#[skyline::from_offset(0x1f2a790)]
fn get_facedata(this: &PersonData, method_info: OptionalMethod) -> &PersonData;

#[skyline::from_offset(0x1f26160)]
fn person_get_ascii_name(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25f40)]
fn person_get_flag(this: &PersonData, method_info: OptionalMethod) -> &mut PersonDataFlag;

#[skyline::from_offset(0x1f261a0)]
fn person_get_attrs(this: &PersonData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1f29e30)]
fn person_get_job(this: &PersonData, method_info: OptionalMethod) -> Option<&JobData>;

#[skyline::from_offset(0x1f25c60)]
fn person_get_jid(this: &PersonData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[skyline::from_offset(0x1f25dc0)]
fn person_get_level(this: &PersonData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x1f25de0)]
fn person_get_InternalLevel(this: &PersonData, method_info: OptionalMethod) -> i8;

#[skyline::from_offset(0x1f25df0)]
fn person_set_InternalLevel(this: &PersonData, value: i8, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25dd0)]
fn person_set_level(this: &PersonData, value: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25db0)]
fn person_set_gender(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25c50)]
fn person_set_name(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26050)]
pub fn set_commonSids(this: &PersonData, value: &mut Array<&Il2CppString>, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25d30)]
fn person_set_UnitIconID(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26030)]
fn person_set_grow(this: &PersonData, value: &Capability, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26010)]
fn person_set_limit(this: &PersonData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25cc0)]
fn person_get_help(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25cd0)]
fn person_set_help(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a700)]
fn set_CommonSkill(this: &PersonData, value : &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a7a0)]
pub fn set_facedata(this: &PersonData, value : &PersonData, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26170)]
fn person_set_ascii_name(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25f50)]
fn person_set_flag(this: &PersonData, value: &PersonDataFlag, method_info: OptionalMethod);

#[skyline::from_offset(0x1f261b0)]
fn person_set_attrs(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25c70)]
fn person_set_jid(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25e60)]
fn person_get_asset_force(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_SubAptitude")]
fn person_get_sub_apt(this: &PersonData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "PersonData", "get_Aptitude")]
fn person_get_apt(this: &PersonData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "PersonData", "set_SubAptitude")]
fn person_set_sub_apt(this: &PersonData, value: &WeaponMask, method_info: OptionalMethod);

#[unity::from_offset("App", "PersonData", "get_SummonRank")]
fn person_get_summon_rank(this: &PersonData, method_info: OptionalMethod) -> i32;

// JobData 
#[skyline::from_offset(0x2055d20)]
fn job_is_low(this: &JobData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "JobData", "IsHigh")]
fn job_is_high(this: &JobData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2055d70)]
fn job_has_high_job(this: &JobData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "JobData", "get_Limit")]
fn job_get_limit(this: &JobData, method_info: OptionalMethod) -> & mut Capability;

#[unity::from_offset("App", "JobData", "get_Base")]
fn job_get_base(this: &JobData, method_info: OptionalMethod) -> & mut Capability;

#[unity::from_offset("App", "JobData", "get_DiffGrowLunatic")]
fn job_get_DiffGrowL(this: &JobData, method_info: OptionalMethod) -> &mut CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrowHard")]
fn job_get_DiffGrowH(this: &JobData, method_info: OptionalMethod) -> &mut CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrowNormal")]
fn job_get_DiffGrowN(this: &JobData, method_info: OptionalMethod) -> &mut CapabilitySbyte;

#[unity::from_offset("App", "JobData", "get_DiffGrow")]
fn job_get_DiffGrow(this: &JobData, method_info: OptionalMethod) -> &mut CapabilitySbyte;

#[skyline::from_offset(0x2054980)]
fn job_get_high_job1(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[skyline::from_offset(0x2054a20)]
fn job_get_high_job2(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[skyline::from_offset(0x2055fe0)]
fn job_GetLowJobs(this: &JobData, method_info: OptionalMethod) -> &List<JobData>;

#[unity::from_offset("App", "JobData", "GetHighJobs")]
fn job_GetHighJobs(this: &JobData, method_info: OptionalMethod) -> &List<JobData>;

#[skyline::from_offset(0x2053e80)]
fn job_max_level(this: &JobData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x02053e90)]
fn job_set_maxLevel(this: &JobData, value: u8, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_Limit")]
fn job_set_limit(this: &JobData, value :&Capability, method_info: OptionalMethod);

#[skyline::from_offset(0x2053ea0)]
fn get_job_internal_level(this: &JobData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App", "JobData", "set_DiffGrowLunatic")]
fn job_set_DiffGrowL(this: &JobData,value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrowHard")]
fn job_set_DiffGrowH(this: &JobData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrowNormal")]
fn job_set_DiffGrowN(this: &JobData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "set_DiffGrow")]
fn job_set_DiffGrow(this: &JobData, value: &CapabilitySbyte,  method_info: OptionalMethod);

#[unity::from_offset("App", "JobData", "get_Flag")]
pub fn job_Get_Flag(this: &JobData, method_info: OptionalMethod) -> &'static mut JobDataFlag;

#[skyline::from_offset(0x02053e20)]
pub fn get_JobStyle(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App", "JobData", "get_UniqueItems")]
fn job_get_uniqueItems(this: &JobData, method_info: OptionalMethod) -> &Array<Il2CppString>;

// Capabilities 
#[skyline::from_offset(0x25bcda0)]
fn Capability_is_zero(this: &Capability, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x025be030)]
fn CapabilitySbyte_is_zero(this: &CapabilitySbyte, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x25bcd00)]
fn Capability_add(this: &Capability, i: i32, v: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x25bdf90)]
fn CapabilitySbyte_add(this: &CapabilitySbyte, i: i32, v: i8,  method_info: OptionalMethod);

//Skill Array
#[unity::from_offset("App","SkillArray", "Clear")]
pub fn skillarray_clear(this: &SkillArray, method_info: OptionalMethod);