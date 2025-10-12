use unity::engine::Sprite;
use unity::prelude::{Il2CppClassData, Il2CppString, OptionalMethod};
use unity::system::Dictionary;
use crate::gamedata::PersonData;
use crate::resourcemanager::ResourceHandle;

#[unity::class("App", "SpriteAtlasManager")]
pub struct SpriteAtlasManager {
    pub handle: &'static ResourceHandle,
    sprite_atlas: *const u8,
    pub cache_table: &'static Dictionary<'static, &'static Il2CppString, &'static Sprite>,
}


#[unity::class("App", "FaceThumbnail")]
pub struct FaceThumbnail {}

impl FaceThumbnail {
    pub fn get_static_fields() -> &'static FaceThumbnailStaticFields {
        FaceThumbnail::class().get_static_fields::<FaceThumbnailStaticFields>()
    }
    pub fn get_path_person(person: &PersonData) -> Option<&'static Il2CppString> {
        unsafe { facethumb_nail_get_person(person, None) }
    }
    pub fn exists(name: &Il2CppString) -> bool {
        Self::get_static_fields().face_thumb.cache_table.entries.iter().
            any(|x| x.key.is_some_and(|key| key == name))
    }
}

pub struct FaceThumbnailStaticFields {
    pub path: &'static Il2CppString,
    pub face_thumb: &'static SpriteAtlasManager,
}

#[skyline::from_offset(0x02d521d0)]
pub fn facethumb_nail_get_person(person: &PersonData, method_info: OptionalMethod) -> Option<&'static Il2CppString>;
