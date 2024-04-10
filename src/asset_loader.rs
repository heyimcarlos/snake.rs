use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::util::calc_sprite_index;

// @todo: just use this to map parts to the indexes
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SpritePart {
    HeadUp,
    HeadDown,
    HeadLeft,
    HeadRight,
    TailUp,
    TailDown,
    TailLeft,
    TailRight,
    BodyHorizontal,
    BodyVertical,
    BodyTopLeft,
    BodyTopRight,
    BodyBottomLeft,
    BodyBottomRight,
    Apple,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "snake-graphics.png")]
    pub sprite_sheet: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 5, rows = 4,))]
    pub sprite_sheet_layout: Handle<TextureAtlasLayout>,
}

// @todo: use built-in indices instead of manually mappng them
impl ImageAssets {
    pub fn get_sprite_index(&self, part: SpritePart) -> usize {
        match part {
            SpritePart::HeadUp => calc_sprite_index(0, 3, 5),
            SpritePart::HeadDown => calc_sprite_index(1, 4, 5),
            SpritePart::HeadLeft => calc_sprite_index(1, 3, 5),
            SpritePart::HeadRight => calc_sprite_index(0, 4, 5),
            SpritePart::TailUp => calc_sprite_index(2, 3, 5),
            SpritePart::TailDown => calc_sprite_index(3, 4, 5),
            SpritePart::TailLeft => calc_sprite_index(3, 3, 5),
            SpritePart::TailRight => calc_sprite_index(2, 4, 5),
            SpritePart::BodyHorizontal => calc_sprite_index(0, 1, 5),
            SpritePart::BodyVertical => calc_sprite_index(1, 2, 5),
            SpritePart::BodyTopLeft => calc_sprite_index(0, 2, 5),
            SpritePart::BodyTopRight => calc_sprite_index(0, 0, 5),
            SpritePart::BodyBottomLeft => calc_sprite_index(2, 2, 5),
            SpritePart::BodyBottomRight => calc_sprite_index(1, 0, 5),
            SpritePart::Apple => calc_sprite_index(3, 0, 5),
        }
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}
