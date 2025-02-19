
use bevy::prelude::*;

use super::{resources::InputConfig, systems::*,assets::*};


#[derive(Default)]
pub struct InputPlugin;

impl bevy::app::Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_asset::<InputAsset>()
            .init_asset_loader::<InputAssetLoader>()
            
            .init_resource::<InputConfig>()
            .add_systems(Startup, ( init, ))
            .add_systems(Update,( on_modified, load, ).chain())
        ;
    }
}

