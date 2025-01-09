pub use unity::prelude::*;
use super::*;

#[unity::class("App", "TerrainData")]
pub struct TerrainData {
   pub parent: StructBaseFields,
   pub tid: &'static Il2CppString,
   pub name: &'static Il2CppString,
   pub cost_name: &'static Il2CppString,
   pub cost_type: i32,
   pub layer: i32,
   pub prohibition: i32,
   pub command: i32,
   pub sight: u8,
   pub destroyer: i32,
   pub hp: [i32; 3],
   pub defense: i8,
   pub avoid: i8,
   pub player_defense: i8,
   pub enemy_defense: i8,
}
impl Gamedata for TerrainData  {}
