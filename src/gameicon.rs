use unity::prelude::*;
use unity::engine::Sprite;
use unity::system::Dictionary;
use crate::spriteatlasmanager::SpriteAtlasManager;

#[unity::class("App", "GameIcon")]
#[static_fields(GameIconStaticFields)]
pub struct GameIcon { }

impl GameIcon {
    pub fn try_get_system<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_trygetsystem(icon_name.into(), None) }
    }
    pub fn try_get_unit_icon_index<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_tryget_unit_icon(icon_name.into(), None) }
    }
}

pub struct GameIconStaticFields {
    pub skill: &'static SpriteAtlasManager,
    pub item: &'static SpriteAtlasManager,

    pub efficacy: &'static SpriteAtlasManager,
    pub efficacy_outline: &'static SpriteAtlasManager,

    pub item_kinds: &'static SpriteAtlasManager,
    pub item_kinds_outline: &'static SpriteAtlasManager,

    pub god_symbol: &'static SpriteAtlasManager,
    pub god_ring: &'static SpriteAtlasManager,

    pub system: &'static SpriteAtlasManager,
    pub unit_icon_index: &'static SpriteAtlasManager,
    pub unit_icon_pallete: &'static SpriteAtlasManager,
}



#[unity::from_offset("App", "GameIcon", "TyrGetUnitIconIndex")]
extern "C" fn gameicon_tryget_unit_icon(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;


#[unity::from_offset("App", "GameIcon", "TryGetSystem")]
extern "C" fn gameicon_trygetsystem(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;