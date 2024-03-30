use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub sprite_sheet: Handle<TextureAtlas>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet_image = asset_server.load("snake-graphics.png");
    let texture_atlas = TextureAtlas::from_grid(sprite_sheet_image, Vec2::new(32.0, 32.0), 3, 1); // Adjust as needed
    let sprite_sheet_handle = texture_atlases.add(texture_atlas);

    *scene_assets = SceneAssets {
        sprite_sheet: sprite_sheet_handle,
    }
}
