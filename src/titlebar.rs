//! Wrappers to open and close a TitleBar when entering a menu.

use unity::prelude::*;

#[repr(i32)]
#[derive(PartialEq, Clone, Copy)]
pub enum TitleBarFooterType{
    Gold = 1,
    PieceOfBond = 2,
    Refine = 4,
    RefineGod = 8,
    Proof = 16,
    RelayTicket = 32,
    None = 0,
    GoldAndBond = 3,
    GoldAndRefine = 5,
    GoldAndBondAndRefine = 7,
    GoldAndBondAndRelayTicket = 35,
}

#[repr(C)]
#[unity::class("App", "TitleBar")]
pub struct TitleBar { }

impl TitleBar {
    fn get_instance() -> &'static TitleBar {
        TitleBar::class().get_static_fields::<&TitleBar>()
    }

    pub fn open_header(title: impl AsRef<str>, help: impl AsRef<str>, key_help_id: impl AsRef<str>) -> bool {
        unsafe {
            titlebar_open_header(
                Self::get_instance(),
                title.as_ref().into(),
                help.as_ref().into(),
                key_help_id.as_ref().into(),
                None,
            )
        }
    }

    pub fn close_header() -> bool {
        unsafe { titlebar_close_header(Self::get_instance(), None) }
    }

    pub fn hide_footer() {
        unsafe { titlebar_hide_footer(Self::get_instance(), None); }
    }

    pub fn show_footer() {
        unsafe { titlebar_show_footer(Self::get_instance(), None); }
    }
    pub fn open_footer(ty: TitleBarFooterType) {
        unsafe { titlebar_open_footer(Self::get_instance(), ty, None); }
    }
}

#[unity::from_offset("App", "TitleBar", "OpenHeader")]
fn titlebar_open_header(
    this: &'static TitleBar,
    title: &'static Il2CppString,
    help: &'static Il2CppString,
    key_help_id: &'static Il2CppString,
    method_info: OptionalMethod,
) -> bool;

#[unity::from_offset("App", "TitleBar", "CloseHeader")]
fn titlebar_close_header(this: &'static TitleBar, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "TitleBar", "HideFooter")]
fn titlebar_hide_footer(this: &'static TitleBar, method_info: OptionalMethod);

#[unity::from_offset("App", "TitleBar", "ShowFooter")]
fn titlebar_show_footer(this: &'static TitleBar, method_info: OptionalMethod);

#[unity::from_offset("App", "TitleBar", "OpenFooter")]
fn titlebar_open_footer(this: &'static TitleBar, ty: TitleBarFooterType, method_info: OptionalMethod);