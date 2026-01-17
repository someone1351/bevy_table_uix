
use bevy::prelude::*;
use bevy_table_ui::{UiAffectPlugin, UiInteractSystem};


use super::{
    assets::*, messages::UixUserMessage, resources::*,  systems::*
};


#[derive(Default)]
pub struct UixPlugin;

impl bevy::app::Plugin for UixPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins((
                UiAffectPlugin,
            ))
            .init_asset::<UiAsset>()
            .init_asset_loader::<UiAssetLoader>()

            .init_resource::<UiModifiedAssets>()
            .init_resource::<UixLibScope>()
            .init_resource::<UixGcScope>()
            .add_message::<UixUserMessage>()
            // .add_systems(Startup, (
            //     init_asset ,
            // ))
            .add_systems(Update,(
                on_asset_modified_event,
                (
                    on_asset_modified_reinit,
                    on_asset_load,
                ).chain().run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),

                // on_affects,
                on_event_listeners,
            ).chain()
            .after(UiInteractSystem))
            // .add_systems(Update,(
            //     on_event_listeners,
            //  ).chain().after(InputSystems))
        ;
    }
}

