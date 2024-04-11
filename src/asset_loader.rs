use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SpritePart {
    BodyTopRight = 0,
    BodyHorizontal = 1,
    BodyTopLeft = 2,
    HeadUp = 3,
    HeadRight = 4,
    BodyBottomRight = 5,
    BodyVertical = 7,
    HeadLeft = 8,
    HeadDown = 9,
    BodyBottomLeft = 12,
    TailUp = 13,
    TailRight = 14,
    Apple = 15,
    TailLeft = 18,
    TailDown = 19,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "snake-graphics.png")]
    pub sprite_sheet: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64., tile_size_y = 64., columns = 5, rows = 4,))]
    pub sprite_sheet_layout: Handle<TextureAtlasLayout>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>();
    }
}
