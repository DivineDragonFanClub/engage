//! Utilities to display popup dialogues. Tied to the [`ProcInst`](crate::proc::ProcInst) system.

use unity::prelude::*;

use crate::proc::Bindable;

pub struct GameMessage;

impl GameMessage {
    pub fn create_key_wait<'a>(parent: &impl Bindable, message: impl Into<&'a Il2CppString>) {
        unsafe {
            gamemessage_create_key_wait(parent, message.into(), None);
        }
    }
    pub fn create_key_wait_mess<'a>(parent: &impl Bindable, mess_label: impl Into<&'a Il2CppString>) {
        let message = crate::mess::Mess::get(mess_label);
        unsafe {
            gamemessage_create_key_wait(parent, message, None);
        }
    }
    pub fn create_gold_gain(parent: &impl Bindable, amount: i32, label: Option<&Il2CppString>) {
        unsafe { create_gold_gain_bind(parent, amount, label, None) }
    }
}

#[skyline::from_offset(0x2270d00)]
fn gamemessage_create_key_wait<P: Bindable>(proc: &P, mess: &Il2CppString, method_info: OptionalMethod) -> *const u8;

#[unity::from_offset("App","GameMessage", "CreateGoldGain")]
fn create_gold_gain_bind<P: Bindable>(parent: &P, gold: i32, message: Option<&Il2CppString>, method_info: OptionalMethod);