
// #![allow(unused_mut)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// // #![allow(unused_assignments)]
// // #[allow(unused_parens)]

// use hello::input_map;

// use std::collections::HashMap;

use bevy_chair_input_map as input_map;
use bevy_table_ui as table_ui;

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, };
// use table_uix::UiAsset;
// use table_ui::*;
// use bevy::app::AppExit;
// use bevy::render::view::screenshot::ScreenshotManager;
// use bevy::time::common_conditions::on_timer;
// use bevy::window::PrimaryWindow;

mod input;
mod menu;


fn main() {
    println!("Hello, world!");

    //
    let mut app = App::new();

    app
        .insert_resource(Time::<Fixed>::from_hz(60.0))

        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override:Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Spadens".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),

            //
            // assets::CustomAssetsPlugin,
            input_map::InputMapPlugin::<input::Mapping>::default(),
            table_ui::UiLayoutPlugin,
            table_ui::UiInteractPlugin,
            table_ui::UiDisplayPlugin,
            table_ui::UiAffectPlugin,
            bevy_table_uix::UixPlugin,

            input::InputPlugin,
            menu::MenuPlugin,
        ))

        .add_systems(Startup, (
            setup_fps,
            setup_camera,
            setup_input,
            // setup_ui,
        ))
        .add_systems(Update, (
            show_fps.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),

            // run_input,
            // global_input,
            // // run_ui.before(table_ui::UiInteractSystemSet),
        ))
        .add_systems(PreUpdate,
        (
            run_input,
            global_input,
            // run_ui.before(table_ui::UiInteractSystemSet),
            ).after(input_map::InputMapSystem)
        )
        ;

    app.run();
}

fn setup_input(
    mut input_map: ResMut<input_map::InputMap<input::Mapping>>,
) {

    // #[derive(Clone,Debug,serde::Deserialize,Hash,PartialEq,Eq,Ord,PartialOrd)]

    // pub enum SomeGlobalMapping {
    //     Exit,
    //     Test(String),
    //     ScreenShot
    // }

    // let a:Box<dyn std::any::Any>=Box::new(SomeGlobalMapping::Exit);
    // let b:Box<dyn std::any::Any>=Box::new(String::from("value"));

    input_map.set_player_devices(0, [input_map::Device::Other,input_map::Device::Gamepad(0)]);
    // input_map.set_player_bind_mode_devices(0,[input_map::Device::Other,input_map::Device::Gamepad(0)]);
    // input_map.set_player_mapping_binds(0, [
    //     (input::Mapping::Global(input::GlobalMapping::Exit),vec![input_map::Binding::Key(KeyCode::F4)],1.0,0.0,0.0),
    // ]);
}

fn run_input(
    mut input_map_event: EventReader<input_map::InputMapEvent<input::Mapping>>,
    mut input_map: ResMut<input_map::InputMap<input::Mapping>>,
) {

    for ev in input_map_event.read() {
        match ev {
            input_map::InputMapEvent::JustPressed{..} | input_map::InputMapEvent::JustReleased{..}
            | input_map::InputMapEvent::BindPressed { ..}| input_map::InputMapEvent::BindReleased { ..}
            | input_map::InputMapEvent::Repeat{..}
            => {
                println!("{ev:?}");
            }
            _=>{}
        }
        match ev {
            input_map::InputMapEvent::BindReleased { ..} => {
                input_map.set_player_bind_mode_devices(0,[]);
            }
            _=>{}
        }
        // match ev {
        //     input_map::InputMapEvent::JustPressed(_player,m,d) => {

        //     }
        //     input_map::InputMapEvent::JustPressed(_player,m,d) => {

        //     }
        //     input_map::InputMapEvent::Repeat(_player,_m ,_d ,_t ) => {

        //     }
        //     input_map::InputMapEvent::ValueChanged(_player, m, d) => {

        //     }
        //     input_map::InputMapEvent::TempValueChanged(_player, m, d) => {

        //     }
        // };
    }
}



fn generate_screenshot_path<P>(dir : P, prefix : &str, ext : &str) -> Option<std::path::PathBuf>
where
    P: AsRef<std::path::Path>,
{
    let dir=dir.as_ref();
    let name_start=prefix.to_string();
    let name_end=".".to_string()+ext;

    //
    let mut last_num=0;

    //
    if !std::fs::create_dir_all(dir).is_ok() {
        eprintln!("Failed to create screenshot directory {dir:?}.");
        return None;
    }

    let Ok(existing) = std::fs::read_dir(dir) else {
        eprintln!("Failed to read screenshot directory {dir:?}.");
        return None;
    };

    for x in existing.into_iter() {
        let Ok(x)=x else {
            continue;
        };

        let Some(x)=x.file_name().to_str().map(|x|x.to_string()) else {
            continue;
        };

        if !x.starts_with(name_start.as_str()) || !x.ends_with(name_end.as_str()) {
            continue;
        }

        let Ok(x)=x[name_start.len() .. x.len()-name_end.len()].to_string().parse::<u32>() else {
            continue;
        };

        last_num=last_num.max(x);
    }

    //
    Some(dir.join(format!("{name_start}{:04}{name_end}", last_num+1)))
}

fn global_input(
    mut input_map_event: EventReader<input_map::InputMapEvent<input::Mapping>>,
    mut exit: EventWriter<AppExit>,
    // mut commands: Commands,
    // mut screenshot_manager: ResMut<bevy::render::view::screenshot::ScreenshotManager>,
    // mut windows: Query<&mut Window>,
    // main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut commands: Commands,
) {
    // let Ok(window_entity) = main_window.get_single() else {return;};

    for ev in input_map_event.read() {
        match ev {
            input_map::InputMapEvent::JustPressed{mapping:input::Mapping::Global(input::GlobalMapping::Exit),..} => {
                // exit.send(AppExit);
                exit.send(AppExit::Success);
                println!("exit!");
            },
            input_map::InputMapEvent::JustPressed{mapping:input::Mapping::Global(input::GlobalMapping::ScreenShot),..} => {
                if let Some(path) = generate_screenshot_path("./screenshots","screenshot_","png") {
                    // if screenshot_manager.save_screenshot_to_disk(window_entity, &path).is_err() {
                    //     eprintln!("Failed to take screenshot at {path:?}.");
                    // }
                    commands
                        .spawn(bevy::render::view::screenshot::Screenshot::primary_window())
                        .observe(bevy::render::view::screenshot::save_to_disk(path));
                }
            },
            _ => {}
        };
    }
}


fn setup_camera(mut commands: Commands) {
    // commands.spawn(( Camera2dBundle { camera: Camera { ..default() }, ..default() }, ));
    // commands.spawn(( Camera3dBundle { camera: Camera { ..default() }, ..default() }, ));
    commands.spawn(Camera3d::default());
}

#[derive(Component)]
struct FpsText;

fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn((
        Text::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {align_self:AlignSelf::Start,justify_self:JustifySelf::End,..Default::default()},
    )).with_child((
        TextSpan::new(""),
        TextColor::from(bevy::color::palettes::css::WHITE),
        TextFont {font:font.clone(),font_size: 15.0,..default()},
        FpsText
    ));
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut marker_query: Query< &mut TextSpan,With<FpsText>>,
) {
    if let Ok(mut text)=marker_query.get_single_mut() {
        let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
        let avg = v.and_then(|x|x.average()).unwrap_or_default();
        text.0 =format!("{fps:.0} {avg:.0}");
    }
}

