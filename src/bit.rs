use unity::il2cpp::object::Array;
use unity::prelude::*;

#[unity::class("App", "BitStruct")]
pub struct BitStruct {
    pub bits: &'static Array<u8>,
}

// BitStructFields is the only part of the whole class that gets used in functions
impl BitStructFields {
    pub fn get(&self, index: i32) -> bool {
        unsafe { bitstruct_get(&self, index, None) }
    }
    pub fn set(&self, index: i32, value: bool) {
        unsafe { bitstruct_set(&self, index, value, None); }
    }
    pub fn set_by_toggle(&self, index: i32) {
        unsafe { bitstruct_set_toggle(&self, index, None); }
    }
}

#[skyline::from_offset(0x02988ee0)]
fn bitstruct_get(this: &BitStructFields, index: i32, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2988f30)]
fn bitstruct_set(this: &BitStructFields, index: i32, enable: bool, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02988fb0)]
fn bitstruct_set_toggle(this: &BitStructFields, index: i32, method_info: OptionalMethod) -> bool;