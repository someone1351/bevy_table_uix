
#![allow(unused_mut)]
#![allow(unused_variables)]
// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]

use std::collections::HashSet;
use bevy_table_ui::{self as table_ui, CameraUi, UiColor, UiInteractInputEvent, UiLayoutComputed, UiRoot, UiSize, UiVal};

use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState, InputSystem}, prelude::* };
use bevy_table_uix::UixFromAsset;

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
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
            FrameTimeDiagnosticsPlugin::default(),

            //
            // assets::CustomAssetsPlugin,
            table_ui::UiLayoutPlugin,
            table_ui::UiInteractPlugin,
            table_ui::UiDisplayPlugin,
            table_ui::UiAffectPlugin,
            bevy_table_uix::UixPlugin,

        ))

        .add_systems(Startup, (
            setup_fps,
            setup_camera,
            setup_ui,
        ))
        .add_systems(Update, (
            update_ui_roots,
            show_fps.run_if(bevy::time::common_conditions::on_timer(std::time::Duration::from_millis(300))),
        ))
        .add_systems(PreUpdate,(
            run_input,
            global_input,
        ).after(InputSystem))
        ;

    app.run();
}


fn global_input(
    mut exit: EventWriter<AppExit>,
    mut key_events: EventReader<KeyboardInput>,
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
) {
    if let Ok(mut text)=marker_query.single_mut() {
        let v=diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS);
        let fps = v.and_then(|x|x.value()).map(|x|x.round()).unwrap_or_default();
        let avg = v.and_then(|x|x.average()).unwrap_or_default();
        text.0 =format!("{fps:.0} {avg:.0}");
    }
}



pub fn update_ui_roots(
    windows: Query<&Window>,
    mut root_query: Query<&mut UiRoot,>,
    mut key_events: EventReader<bevy::input::keyboard::KeyboardInput>,
) {

    let window_size=windows.single()
        .and_then(|window|Ok((window.width(),window.height())))
        .unwrap_or_default();

    for mut x in root_query.iter_mut() {
        x.width=window_size.0;
        x.height=window_size.1;
    }
}
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let root=commands.spawn((
        UiLayoutComputed::default(),
        UixFromAsset::new(asset_server.load("test.ui_conf")),

        UiColor{back:Color::srgba(0.01,0.3,0.1,0.8),..Default::default()},
        UiSize{width:UiVal::Px(500.0),height:UiVal::Px(500.0)},
    )).id();


    println!("root is {root}");
}

pub fn run_input(
    mut windows: Query<&mut Window>,
    mut prev_cursor : Local<Option<Vec2>>,
    mut ui_interact_input_event_writer: EventWriter<UiInteractInputEvent>,
    ui_root_query : Query<Entity,(With<UiLayoutComputed>,Without<ChildOf>)>,

    mut key_events: EventReader<KeyboardInput>,
    mut mouse_button_events : EventReader<MouseButtonInput>,
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
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusUp { root_entity, group });
                        }
                        KeyCode::KeyS|KeyCode::ArrowDown => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusDown { root_entity, group });
                        }
                        KeyCode::KeyA|KeyCode::ArrowLeft => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusRight { root_entity, group });
                        }
                        KeyCode::KeyD|KeyCode::ArrowRight => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusLeft { root_entity, group });
                        }
                        KeyCode::Tab|KeyCode::KeyE => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusNext { root_entity, group });
                        }
                        KeyCode::KeyQ => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusPrev { root_entity, group });
                        }
                        KeyCode::Space|KeyCode::Enter => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusPressBegin{root_entity, group, device});
                        }
                        KeyCode::Escape => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusPressCancel{root_entity, device});
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
                            ui_interact_input_event_writer.write(UiInteractInputEvent::FocusPressEnd{root_entity, device});
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
                            ui_interact_input_event_writer.write(UiInteractInputEvent::CursorPressBegin{root_entity, device});
                        }
                        MouseButton::Right => {
                            ui_interact_input_event_writer.write(UiInteractInputEvent::CursorPressCancel{root_entity, device});
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
                            ui_interact_input_event_writer.write(UiInteractInputEvent::CursorPressEnd {root_entity, device});
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
            ui_interact_input_event_writer.write(UiInteractInputEvent::CursorMoveTo{root_entity,device: player,cursor:mouse_cursor});
        }
    }

    *prev_cursor=mouse_cursor;

}
