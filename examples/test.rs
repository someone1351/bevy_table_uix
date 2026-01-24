
#![allow(unused_mut)]
#![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

use std::collections::HashSet;
use bevy_table_ui::*;

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState, InputSystems}, prelude::* };
use bevy_table_uix::*;


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
                        title: "uix test".into(),
                        resolution: (1280, 720).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),

            //
            // assets::CustomAssetsPlugin,
            UiLayoutPlugin,
            UiInteractPlugin,
            UiDisplayPlugin,
            UixPlugin,

        ))

        .add_systems(Startup, (
            setup_fps,
            setup_camera,
            setup_ui,
        ))
        .add_systems(Update, (
            update_ui_roots,
            show_fps.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(250))),
        ))
        .add_systems(PreUpdate,(
            run_input,
            global_input,
        ).after(InputSystems))
        ;

    app.run();
}


fn global_input(
    mut exit: MessageWriter<AppExit>,
    mut key_events: MessageReader<KeyboardInput>,
) {

    for ev in key_events.read() {
        match ev.state {
            ButtonState::Released => {
                match ev.key_code {
                    KeyCode::F4 => {
                        exit.write(AppExit::Success);
                        println!("exit!");
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

}


fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera3d::default());
    commands.spawn(CameraUi::default());
}

#[derive(Component)]
struct FpsText;

fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

    // commands.spawn((
    //     Text::default(),
    //     TextLayout::new_with_justify(JustifyText::Center),
    //     Node {align_self:AlignSelf::Start,justify_self:JustifySelf::End,..Default::default()},
    // )).with_child((
    //     TextSpan::new(""),
    //     TextColor::from(bevy::color::palettes::css::WHITE),
    //     TextFont {font:font.clone(),font_size: 15.0,..default()},
    //     FpsText
    // ));
}

fn show_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut marker_query: Query< &mut TextSpan,With<FpsText>>,
    mut ui_user_message_writer: MessageWriter<UixUserMessage>,
    root_query: Query<Entity, With<UiRoot>,>,

) {
    let diag=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
    let fps = diag.and_then(|x|x.value()).map(|x|x.floor()).unwrap_or_default();
    let avg = diag.and_then(|x|x.average()).unwrap_or_default();

    for root_entity in root_query {
        ui_user_message_writer.write(UixUserMessage { entity: root_entity, event: "fps".into(), params: vec![fps.into(),avg.into()] });
    }
}



pub fn update_ui_roots(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,>,
    mut key_events: MessageReader<bevy::input::keyboard::KeyboardInput>,
) {
    if let Ok(window)=windows.single() {
        for mut ui_root in root_query.iter_mut() {
            ui_root.width=window.width();
            ui_root.height=window.height();
            // ui_root.scaling=window.resolution.base_scale_factor();
        }
    }
}

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let root=commands.spawn((
        UixFromAsset::new(asset_server.load("test.ui_conf")),
        UiColor{back:Color::srgba(0.1,0.1,0.1,0.8),cell:Color::srgba(0.01,0.3,0.6,0.8),..Default::default()},
        UiSize::max(),
    )).id();


    println!("root is {root}");
}

pub fn run_input(
    mut windows: Query<&mut Window>,
    mut prev_cursor : Local<Option<Vec2>>,
    mut ui_interact_input_event_writer: MessageWriter<UiInteractInputMessage>,
    ui_root_query : Query<Entity,With<UiRoot>>,

    mut key_events: MessageReader<KeyboardInput>,
    mut mouse_button_events : MessageReader<MouseButtonInput>,
    mut key_lasts : Local<HashSet<KeyCode>>,
){

    let Ok(window) = windows.single_mut() else {return;};
    let mouse_cursor = window.cursor_position();//.unwrap_or(Vec2::ZERO);

    //

    let device=0;
    let group=0;

    //
    for ev in key_events.read() {
        match ev.state {
            ButtonState::Pressed if !key_lasts.contains(&ev.key_code) => {
                key_lasts.insert(ev.key_code);

                for root_entity in ui_root_query.iter() {
                    match ev.key_code {
                        KeyCode::KeyW|KeyCode::ArrowUp => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusUp { root_entity, group, device });
                        }
                        KeyCode::KeyS|KeyCode::ArrowDown => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusDown { root_entity, group, device });
                        }
                        KeyCode::KeyA|KeyCode::ArrowLeft => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusLeft { root_entity, group, device });
                        }
                        KeyCode::KeyD|KeyCode::ArrowRight => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusRight { root_entity, group, device });
                        }
                        KeyCode::Tab|KeyCode::KeyE => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusNext { root_entity, group, device });
                        }
                        KeyCode::KeyQ => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPrev { root_entity, group, device });
                        }
                        KeyCode::Space|KeyCode::Enter => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressBegin{root_entity, group, device, button: 0 });
                        }
                        KeyCode::Escape => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressCancel{root_entity, device, button: 0 });
                        }
                        _ => {}
                    }
                }
            }
            ButtonState::Released => {
                key_lasts.remove(&ev.key_code);

                for root_entity in ui_root_query.iter() {
                    match ev.key_code {
                        KeyCode::Space|KeyCode::Enter => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::FocusPressEnd{root_entity, device, button: 0 });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    //
    for ev in mouse_button_events.read() {
        match ev.state {
            ButtonState::Pressed => {
                for root_entity in ui_root_query.iter() {
                    match ev.button {
                        MouseButton::Left => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 0 });

                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 1 });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 2 });
                        }
                        MouseButton::Right => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 2 });

                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 0 });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 1 });
                        }
                        MouseButton::Middle => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressBegin{root_entity, device, button: 1 });

                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 0 });
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressCancel{root_entity, device, button: 2 });
                        }
                        MouseButton::Forward => {
                        }
                        MouseButton::Back => {
                        }
                        _ => {}
                    }
                }
            }
            ButtonState::Released => {
                for root_entity in ui_root_query.iter() {
                    match ev.button {
                        MouseButton::Left => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 0 });
                        }
                        MouseButton::Right => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 2 });
                        }
                        MouseButton::Middle => {
                            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorPressEnd {root_entity, device, button: 1 });
                        }
                        _ => {}
                    }
                }
            }
        }

    }

    for root_entity in ui_root_query.iter() {
        if *prev_cursor!=mouse_cursor {
            let player=0;
            ui_interact_input_event_writer.write(UiInteractInputMessage::CursorMoveTo{root_entity,device: player,cursor:mouse_cursor});
        }
    }

    *prev_cursor=mouse_cursor;

}
