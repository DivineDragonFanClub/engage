use unity::prelude::*;
use unity::system::Dictionary;

#[unity::class("App", "ResourceManager")]
pub struct ResourceManager {}

#[repr(C)]
pub struct ResourceManagerStaticFields {
    junk: [u8; 0x40],
    pub files: &'static Dictionary<'static, &'static Il2CppString, bool>,

}
impl ResourceManager {
    pub fn file_exist<'a>(path: impl Into<&'a Il2CppString>) -> bool {
        unsafe { resource_manager_file_exist(path.into(), None) }
    }
    pub fn is_loading() -> bool { unsafe { is_loading(None) } }
}



#[unity::class("App", "ResourceHandle")]
pub struct ResourceHandle {
    pub path: &'static Il2CppString,
    handle: [u8; 0x18],
}

#[unity::from_offset("App", "ResourceManager", "IsLoading")]
pub fn is_loading(method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x020169f0)]
fn resource_manager_file_exist(path: &Il2CppString, method_info: OptionalMethod) -> bool;