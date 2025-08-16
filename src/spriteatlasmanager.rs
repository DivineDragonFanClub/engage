use unity::engine::Sprite;
use unity::prelude::Il2CppString;
use unity::system::Dictionary;

#[unity::class("App", "SpriteAtlasManager")]
pub struct SpriteAlasManager {
    handle: *const u8,
    sprite_atlas: *const u8,
    pub cache_table: &'static Dictionary<'static, &'static Il2CppString, &'static Sprite>,
}