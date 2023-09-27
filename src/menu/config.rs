use unity::{prelude::*, system::List};

use super::BasicMenuResult;
use crate::proc::ProcInstFields;

#[unity::class("", "ConfigBasicMenuItem")]
pub struct ConfigBasicMenuItem {
    // Inlined BasicMenuItem here because C ABI dumb
    pub menu: &'static mut ProcInstFields,
    menu_item_content: *const u8,
    name: &'static Il2CppString,
    index: i32,
    full_index: i32,
    attribute: i32,
    cursor_color: unity::engine::Color,
    active_text_color: unity::engine::Color,
    inactive_text_color: unity::engine::Color,
    config_method: i32,
    pub title_text: &'static Il2CppString,
    pub command_text: &'static Il2CppString,
    pub help_text: &'static Il2CppString,
    pub is_arrow: bool,
    pub is_command_icon: bool,
    pub gauge_ratio: f32,
}

impl ConfigBasicMenuItem {
    fn new() -> &'static mut ConfigBasicMenuItem {
        let item = il2cpp::instantiate_class(ConfigBasicMenuItem::class().clone()).unwrap();
        unsafe { configbasicmenuitem_ctor(item, None) };
        item
    }

    pub fn new_switch<Methods: ConfigBasicMenuItemSwitchMethods>(title: impl AsRef<str>) -> &'static mut ConfigBasicMenuItem {
        let item = Self::new();

        item.config_method = 0;

        item.get_class_mut()
            .get_virtual_method_mut("CustomCall")
            .map(|method| method.method_ptr = Methods::custom_call as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetCommandText")
            .map(|method| method.method_ptr = Methods::set_command_text as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetHelpText")
            .map(|method| method.method_ptr = Methods::set_help_text as _);

        item.title_text = title.into();

        Methods::set_command_text(item, None);
        Methods::set_help_text(item, None);

        item
    }

    pub fn new_gauge<Methods: ConfigBasicMenuItemGaugeMethods>(title: impl AsRef<str>) -> &'static mut ConfigBasicMenuItem {
        let item = Self::new();

        item.config_method = 1;

        item.get_class_mut()
            .get_virtual_method_mut("CustomCall")
            .map(|method| method.method_ptr = Methods::custom_call as _);

        item.get_class_mut()
            .get_virtual_method_mut("SetHelpText")
            .map(|method| method.method_ptr = Methods::set_help_text as _);

        item.title_text = title.into();

        Methods::set_help_text(item, None);

        item
    }

    pub fn change_key_value_b(value: bool) -> bool {
        if unsafe { configbasicmenuitem_change_key_value_int(value as i32, 0, 1, 1, None) } == 1 {
            true
        } else {
            false
        }
    }

    pub fn change_key_value_i(value: i32, min: i32, max: i32, step: i32) -> i32 {
        unsafe { configbasicmenuitem_change_key_value_int(value, min, max, step, None) }
    }

    pub fn change_key_value_f(value: f32, min: f32, max: f32, step: f32) -> f32 {
        unsafe { configbasicmenuitem_change_key_value_float(value, min, max, step, None) }
    }
}

pub trait ConfigBasicMenuItemSwitchMethods {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
}

pub trait ConfigBasicMenuItemGaugeMethods {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod) -> BasicMenuResult;
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);
}

#[skyline::from_offset(0x25379a0)]
pub fn configbasicmenuitem_ctor(this: &ConfigBasicMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x2537920)]
fn configbasicmenuitem_change_key_value_int(value: i32, min: i32, max: i32, step: i32, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x2537970)]
fn configbasicmenuitem_change_key_value_float(value: f32, min: f32, max: f32, step: f32, method_info: OptionalMethod) -> f32;

#[unity::from_offset("", "ConfigBasicMenuItem", "UpdateText")]
pub fn configbasicmenuitem_update_text(this: &mut ConfigBasicMenuItem, method_info: OptionalMethod);

#[skyline::from_offset(0x3de9550)]
pub fn generic_list_add<T>(list: &mut List<T>, instance: &T, method_id: *mut u64);
