//! Methods to manipulate and query the message system.

use unity::prelude::*;

#[repr(C)]
#[unity::class("App", "Mess")]
pub struct Mess { }

impl Mess {
    pub fn get(label: &Il2CppString) -> &'static Il2CppString {
        unsafe { mess_get(label, None) }
    }

    pub fn load(filename: &Il2CppString) -> bool {
        unsafe { mess_load(filename, None) }
    }

    pub fn get_language_directory_name() -> &'static Il2CppString {
        unsafe { mess_get_language_directory_name(None) }
    }
}

#[repr(C)]
pub struct MessStaticFields {
    __: [u8; 0xb0],
    pub mess_file_dictionary: &'static Il2CppObject<()>,
    pub sound_file_dictionary: &'static Il2CppObject<()>,
    pub event_file_dictionary: &'static Il2CppObject<()>,
    pub mess_data_dictionary: &'static Il2CppObject<()>,
    pub sound_data_dictionary: &'static Il2CppObject<()>,
    pub event_data_dictionary: &'static Il2CppObject<()>,
    pub path_dictionary: &'static Il2CppObject<()>,
}

#[repr(C)]
#[unity::class("App", "MsgFile")]
pub struct MsgFile {
    resource_file_ptr: *const u8,
    file_object_ptr: *const u8,
    pub reference_count: i32,
}

impl MsgFile {
    pub fn get_text_num() {

    }
}

#[unity::from_offset("App", "Mess", "GetLanguageDirectoryName")]
fn mess_get_language_directory_name(method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x25d54f0)]
pub fn mess_load_impl(filename: &Il2CppString, is_warning: bool, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Mess", "Get")]
fn mess_get(label: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x25d3e40)]
fn mess_load(filename: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2782e80)]
pub fn msgfile_ctor(this: &MsgFile, method_info: OptionalMethod);

#[skyline::from_offset(0x1e96440)]
pub fn msbt_load(this: &mut MsgFile, msbt: &Il2CppArray<u8>, method_info: OptionalMethod);

#[skyline::from_offset(0x1e97550)]
pub fn msbt_get_text_num(this: &MsgFile, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1e97470)]
pub fn msbt_get_label(this: &MsgFile, index: usize, method_info: OptionalMethod) -> &'static Il2CppString;

#[skyline::from_offset(0x1e97770)]
pub fn msbt_get_text(this: &MsgFile, index: usize, method_info: OptionalMethod) -> *const u8;

#[skyline::from_offset(0x3cc4c60)]
pub fn generic_dictionary_add(this: &(), path: &Il2CppString, instance: *const u8, method_id: *mut u64);
