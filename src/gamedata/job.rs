pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use super::{JobData, person::{Capability, CapabilitySbyte}};
use std::ops::Deref;
// Structs, methods required for PersonData, JobData, SkillArray

#[unity::class("App", "JobDataFlag")]
pub struct JobDataFlag { 
    pub value: i32,
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
    pub fn get_flag(&self) -> &'static mut JobDataFlag { unsafe { job_Get_Flag(self, None)}}
    pub fn get_high_jobs(&self) -> &List<JobData> { unsafe { job_GetHighJobs(self, None) }}
    pub fn get_high_job_1(&self) -> Option<&Il2CppString> { unsafe { job_get_high_job1(self, None)}}
    pub fn get_high_job_2(&self) -> Option<&Il2CppString> { unsafe { job_get_high_job2(self, None)}}
    pub fn get_job_style(&self) -> Option<&Il2CppString> { unsafe { get_JobStyle(self, None)}}
    pub fn get_internal_level(&self) -> i8 { unsafe { get_job_internal_level(self, None)}}
    pub fn get_limit(&self) -> &mut Capability { unsafe { job_get_limit(self, None) } }
    pub fn get_low_jobs(&self) -> &List<JobData> { unsafe {job_GetLowJobs(self, None)} }
    pub fn get_max_level(&self) -> u8 { unsafe { job_max_level(self, None)}}
    pub fn get_unique_items(&self) -> &Array<Il2CppString> { unsafe { job_get_uniqueItems(self, None)}}

    pub fn set_diff_grow(&self, grow: &CapabilitySbyte) {  unsafe {job_set_DiffGrow(self, grow, None)}}
    pub fn set_diff_grow_l(&self,grow: &CapabilitySbyte) { unsafe { job_set_DiffGrowL(self, grow, None) }}
    pub fn set_diff_grow_h(&self,grow: &CapabilitySbyte) { unsafe { job_set_DiffGrowH(self, grow, None) }}
    pub fn set_diff_grow_n(&self,grow: &CapabilitySbyte) { unsafe { job_set_DiffGrowN(self, grow, None) }}

    pub fn set_limit(&self, limits: &Capability) { unsafe { job_set_limit(self, limits, None); }}
    pub fn set_max_level(&self, value: u8) { unsafe { job_set_maxLevel(self, value, None); }}
}
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
fn job_Get_Flag(this: &JobData, method_info: OptionalMethod) -> &'static mut JobDataFlag;

#[skyline::from_offset(0x02053e20)]
fn get_JobStyle(this: &JobData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App", "JobData", "get_UniqueItems")]
fn job_get_uniqueItems(this: &JobData, method_info: OptionalMethod) -> &Array<Il2CppString>;
