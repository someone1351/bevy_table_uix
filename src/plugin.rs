
use bevy::prelude::*;

use super::{
    assets::*, event::UixUserEvent, resources::*, script_stuff::{UixGcScope, UixLibScope}, systems::*
};


#[derive(Default)]
pub struct UixPlugin;

impl bevy::app::Plugin for UixPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .init_asset::<UiAsset>()
            .init_asset_loader::<UiAssetLoader>()

            .init_resource::<UiModifiedAssets>()
            .init_resource::<UixLibScope>()
            .init_resource::<UixGcScope>()
            .add_event::<UixUserEvent>()
            .add_systems(Startup, (
                init_asset ,
            ))
            .add_systems(FixedUpdate,(
                on_asset_modified_event,
                (
                    on_asset_modified_reinit,
                    on_asset_load,
                ).chain().run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),

                on_event_listeners,
            )
                .chain()
            )
        ;
    }
}

