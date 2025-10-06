use unity::prelude::{Il2CppString, OptionalMethod};
use crate::proc::{Bindable, ProcInstFields};

#[unity::class("App", "SoftwareKeyboard")]
pub struct SoftwareKeyboard {
    pub proc: ProcInstFields,
    arg: [u8; 0xb8],
    pub max_length: i32,
    pub header_text: Option<&'static Il2CppString>,
    pub sub_text: Option<&'static Il2CppString>,
    pub initial_text: Option<&'static Il2CppString>,
    pub preset: i32,
    call_back: *const (),
}

impl Bindable for SoftwareKeyboard {}

impl SoftwareKeyboard {
    pub fn get_result() -> Option<&'static Il2CppString> { unsafe { keyboard_get_result(None) } }
}

#[unity::from_offset("App", "SoftwareKeyboard", "GetResult")]
fn keyboard_get_result(method_info: OptionalMethod) -> Option<&'static Il2CppString>;