
// use super::resources;
// use super::messages;
// use super::components;
use super::systems::*;
// use super::super::layout;

use bevy::app::*;
// use bevy::app::Update;
use bevy::ecs::prelude::*;
use bevy_table_ui::UiInteractSystem;
// use bevy::input::InputSystems;


#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiAffectSystem;

#[derive(Default)]
pub struct UixAffectPlugin;

impl bevy::app::Plugin for UixAffectPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app


            .add_systems(Update, (
                (

                    on_affects,
                ).chain().in_set(UiAffectSystem)

                    .after(UiInteractSystem)

                    ,
            ))


            // .add_systems(PostUpdate, (
            //     update_text_image,
            // )

            //     .after(table_ui::ui_init_computeds)
            //     .before(table_ui::ui_calc_computeds),
            // )
            ;

    }
    // fn finish(&self, app: &mut bevy::prelude::App) {
    //     // render::setup(app);
    // }
}


        // let render_app = match app.get_sub_app_mut(RenderApp) {
        //     Ok(render_app) => render_app,
        //     Err(_) => return,
        // };

        // render_app
        //     .init_resource::<RenderResourceNeedingDevice>();

            // .add_systems(PostUpdate, (
            //     remover_system,
            // ).chain()
            //     .after(table_ui::ui_calc_computeds)
            // )
  // .add_message::<loader::UiAssetEvent>()
            // .add_message::<input_map::InputMapEvent<UiInputMapping>>()

            // .register_type::<UiText>()
            // .register_type::<UiImage>()
            // .register_type::<UiId>()

            // .register_type::<interact::UiFocusable>()

            // .init_resource::<UiCustomOutput>()


                // (
                //     ui_asset_modified,
                //     ui_asset_load,
                // ).chain().before(table_ui::ui_init_computeds),