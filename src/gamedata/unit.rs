pub use unity::prelude::*;

use super::{ItemData, JobData, WeaponMask, PersonData};

#[repr(C)]
#[unity::class("App", "Unit")]
pub struct Unit {
    padding: [u8; 0x30],
    pub person: &'static mut Il2CppObject<PersonData>
}

pub trait UnitMethods {
    fn learn_job_skill(&self, job: &Il2CppObject<JobData>);
    fn class_change(&self, job: &Il2CppObject<JobData>);
    fn add_item(&self, item: &Il2CppObject<ItemData>);
    fn set_level(&self, level: i32);
    fn set_selected_weapon(&self, weapon_mask: &Il2CppObject<WeaponMask>);
    fn get_job(&self) -> &'static Il2CppObject<JobData>;
    fn is_engaging(&self) -> bool;
    fn is_engage_owner(&self) -> bool;
}

impl UnitMethods for Il2CppObject<Unit> {
    /// Learn the skill for a job regardless of the unit's level.
    fn learn_job_skill(&self, job: &Il2CppObject<JobData>) {
        unsafe {
            unit_learnjobskill(self, job, None);
        }
    }

    /// Performs a class change on the unit without playing the sequence
    fn class_change(&self, job: &Il2CppObject<JobData>) {
        unsafe {
            unit_classchange(self, job, 0 as _, None);
        }
    }

    /// Add item to the unit's inventory without a notification
    fn add_item(&self, item: &Il2CppObject<ItemData>) {
        unsafe {
            unit_itemadd(self, item, None);
        }
    }

    fn set_level(&self, level: i32) {
        unsafe {
            unit_set_level(self, level, None);
        }
    }

    fn set_selected_weapon(&self, weapon_mask: &Il2CppObject<WeaponMask>) {
        unsafe {
            unit_setselectedweapon(self, weapon_mask, None);
        }
    }

    fn get_job(&self) -> &'static Il2CppObject<JobData> {
        unsafe { unit_get_job(self, None) }
    }

    fn is_engaging(&self) -> bool {
        unsafe { unit_is_engaging(self, None) }
    }

    fn is_engage_owner(&self) -> bool {
        unsafe { unit_is_engage_owner(self, None) }
    }
}

#[skyline::from_offset(0x1a3f400)]
extern "C" fn unit_itemadd(this: &Il2CppObject<Unit>, item: &Il2CppObject<ItemData>, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "ClassChange")]
extern "C" fn unit_classchange(this: &Il2CppObject<Unit>, job: &Il2CppObject<JobData>, item: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "LearnJobSkill")]
extern "C" fn unit_learnjobskill(this: &Il2CppObject<Unit>, job: &Il2CppObject<JobData>, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "set_Level")]
extern "C" fn unit_set_level(this: &Il2CppObject<Unit>, level: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "get_Job")]
extern "C" fn unit_get_job(this: &Il2CppObject<Unit>, method_info: OptionalMethod) -> &'static Il2CppObject<JobData>;

#[unity::from_offset("App", "Unit", "SetSelectedWeapon")]
extern "C" fn unit_setselectedweapon(this: &Il2CppObject<Unit>, weapon_mask: &Il2CppObject<WeaponMask>, method_info: OptionalMethod);

// App.Unit$$IsEngaging	7101a265e0	bool App.Unit$$IsEngaging(App_Unit_o * __this, MethodInfo * method)	96
#[skyline::from_offset(0x1a265e0)]
extern "C" fn unit_is_engaging(this: &Il2CppObject<Unit>, method_info: OptionalMethod) -> bool;

// App.Unit$$IsEngageOwner	7101a197a0	bool App.Unit$$IsEngageOwner(App_Unit_o * __this, MethodInfo * method)	112
#[skyline::from_offset(0x1a197a0)]
extern "C" fn unit_is_engage_owner(this: &Il2CppObject<Unit>, method_info: OptionalMethod) -> bool;