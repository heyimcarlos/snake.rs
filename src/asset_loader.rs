use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "snake-graphics.png")]
    pub sprite_sheet: Handle<Image>,
    #[asset(texture_atlas(
        tile_size_x = 136.,
        tile_size_y = 136.,
        columns = 5,
        rows = 4,
        padding_x = 0.,
        padding_y = 0.
    ))]
    pub sprite_sheet_layout: Handle<TextureAtlasLayout>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<ImageAssets>,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let sprite_sheet_image = asset_server.load("snake-graphics.png");
    // let texture_atlas = TextureAtlas::from_grid(sprite_sheet_image, Vec2::new(32.0, 32.0), 3, 1); // Adjust as needed
    // let sprite_sheet_handle = texture_atlases.add(texture_atlas);

    // *scene_assets = SceneAssets {
    //     sprite_sheet: sprite_sheet_handle,
    // }
}
