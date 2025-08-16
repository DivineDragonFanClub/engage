use unity::{prelude::*, engine::{ui::{IsImage, ImageFields}, Material, Sprite}};

#[unity::class("App", "UnitIcon")]
pub struct UnitIcon {
    parent: [u8;0xF8],
    index_atlas: &'static (),
    pallete_atlas: &'static (),
    pub pallete_sprite: Option<&'static Sprite>,
    pub icon_name: Option<&'static Il2CppString>,
    pallete_name: &'static Il2CppString,
    brightness: f32,
    material_instance: &'static Material,
}

impl UnitIcon {
    pub fn set_material(&mut self, material: &'static Material) {
        unsafe { uniticon_set_material(&self, material, None) }
    }

    pub fn get_material(&self) -> &'static mut Material {
        unsafe { uniticon_get_material(&self, None) }
    }
}

impl IsImage for UnitIcon { }

#[unity::from_offset("App", "UnitIcon", "get_material")]
fn uniticon_get_material(this: &UnitIcon, method_info: OptionalMethod) -> &'static mut Material;

#[unity::from_offset("App", "UnitIcon", "set_material")]
fn uniticon_set_material(this: &UnitIcon, material: &'static Material, method_info: OptionalMethod);
