use bevy::prelude::IntoSystemConfigs;

// use crate::table_ui;
use bevy_table_ui as table_ui;

use super::systems;

#[derive(Default)]
pub struct MenuPlugin;

impl bevy::app::Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
        //     .init_resource::<InputConfig>()
            .add_systems(bevy::app::Startup, ( 
                systems::setup_ui, 
            ))
            .add_systems(bevy::app::PostUpdate,
                ( 
                    systems::run_input, 
                ) //.chain()
                // .after(input_map::InputMapSystem)
                .before(table_ui::UiInteractSystem)
            )
            .add_systems(bevy::app::Update,
                ( 
                    systems::run_ui, 
                ) //.chain()
            )
        ;
    }
}

