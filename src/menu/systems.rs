

use bevy::color::Color;
// use bevy::hierarchy::BuildChildren;
use bevy::{asset::AssetServer, ecs::prelude::*, math::Vec2, window::Window};

use bevy_chair_input_map as input_map;
// use crate::table_uix::UixFromAsset;
use bevy_table_ui as table_ui;
use bevy_table_uix::UixFromAsset;
use table_ui::*;
use super::super::input;
use super::components::*;


pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let root=commands.spawn((
        MenuUiRoot,
        UiLayoutComputed::default(),
        UixFromAsset::new(asset_server.load("test.ui_conf")),

        UiColor{back:Color::srgba(0.01,0.3,0.1,0.8),..Default::default()},
        UiSize{width:UiVal::Px(500.0),height:UiVal::Px(500.0)},
    )).id();

    // let root=commands.spawn((
    //     // UiFromAsset::new(asset_server.load("menus.conf")),
    //     UiLayoutComputed::default(),
    //     // UiColor{back:Color::rgba(0.01,0.02,0.03,0.4),..Default::default()},
    //     MenuUiRoot,
    //     UiColor{
    //         back:Color::srgba(0.01,0.3,0.1,0.8),
    //         border:Color::srgba(0.3,0.1,0.1,0.8),
    //         ..Default::default()

    //     },
    //     // UiSize{width:UiVal::Px(500.0),height:UiVal::Px(500.0)},
    //     UiEdge{border:UiRectVal::new_scalar(UiVal::Px(30.0)),..Default::default()},
    //     UiPressable{enable:true,always:true,physical:true,..Default::default()},
    //     // UiScroll{
    //     //     hscroll:UiVal::Px(0.0),
    //     //     vscroll:UiVal::Scale(0.5),
    //     //     // vscroll:UiVal::Px(300.0)
    //     // },
    //     // UiSpan{ span: 2 },
    // )).id();

    // let scroll_bar=commands.spawn((
    //     UiLayoutComputed::default(),
    //     // UiSize{width:UiVal::Px(50.0),height:UiVal::Px(50.0)},
    //     // UiSize{width:UiVal::None,height:UiVal::Scale(1.0)},
    //     UiSize{width:UiVal::Px(50.0),height:UiVal::None},
    //     UiFill{ hfill: UiVal::None, vfill: UiVal::Scale(1.0) },
    //     UiColor{back:Color::srgba(0.1,0.8,0.2,1.0),..Default::default()},
    // )).id();

    // let scroll_bar_button=commands.spawn((
    //     MenuScrollBar,
    //     UiLayoutComputed::default(),
    //     // UiSize{width:UiVal::Px(50.0),height:UiVal::Px(50.0)},
    //     // UiSize{width:UiVal::None,height:UiVal::Scale(1.0)},
    //     UiSize{width:UiVal::Px(50.0),height:UiVal::Px(50.0)},
    //     // UiFill{ hfill: UiVal::None, vfill: UiVal::Scale(1.0) },
    //     UiColor{back:Color::srgba(0.8,0.4,0.2,1.0),..Default::default()},
    //     UiAlign{ halign: UiVal::None, valign: UiVal::Px(0.0) },
    //     UiFill{ hfill: UiVal::None, vfill: UiVal::None },

    //     UiDraggable {enable:true},
    // )).id();


    // let page=commands.spawn((
    //     // UiSize{width:UiVal::Px(300.0),height:UiVal::Px(300.0)},
    //     // UiSize{width:UiVal::Px(300.0),height:UiVal::None},
    //     UiSize{width:UiVal::Px(500.0),height:UiVal::Px(500.0)},
    //     // UiSize{width:UiVal::Scale(1.0),height:UiVal::Scale(1.0)},

    //     UiDraggable {enable:true},
    //     UiLayoutComputed::default(),
    //     // UiSize{width:UiVal::Scale(0.5),height:UiVal::Scale(0.5)},
    //     // UiSize{width:UiVal::Scale(1.0),height:UiVal::Scale(1.0)},
    //     UiColor{back:Color::srgba(0.01,0.02,0.9,1.0),..Default::default()},
    //     // UiAffectColor{
    //     //     back:HashMap::from([
    //     //         (None,Color::linear_rgb(0.01,0.02,0.5)),
    //     //         (Some(UiAffectState::Press),Color::linear_rgb(1.0,0.2,0.3)),
    //     //         (Some(UiAffectState::Hover),Color::linear_rgb(2.0,0.8,0.3)),
    //     //     ]),
    //     //     // updated:true,
    //     //     ..Default::default()
    //     // },
    //     UiAffectComputed::default(),
    //     UiTextComputed::default(),
    //     UiInnerSize::default(),
    //     UiText{
    //         value:"X".to_string(),
    //         font_size:40.0,
    //         // color:Color::rgba(1.0,0.0,0.0,1.0),
    //         color:Color::WHITE,
    //         font:asset_server.load("fonts/FiraMono-Medium.ttf"),
    //         // halign:UiTextHAlign::Left,
    //         // halign:UiTextHAlign::Center,
    //         halign:UiTextHAlign::Right,
    //         // valign:UiTextVAlign::Top,
    //         valign:UiTextVAlign::Bottom,
    //         // hlen:0,vlen:0,
    //         update:true,..Default::default()
    //     },
    //     // UiImage{
    //     //     handle:asset_server.load("bevy_logo_dark_big.png"),
    //     //     // width_scale : 0.1,
    //     //     // height_scale : 0.1,
    //     //     // color:Color::WHITE,
    //     //     color:Color::linear_rgba(1.0, 0.2,0.3, 0.5),
    //     //     // keep_aspect_ratio:false,
    //     //     ..Default::default()
    //     // },
    //     UiPressable{enable:true,always:true,physical:true,..Default::default()},
    //     UiHoverable{enable:true},
    //     UiGap{vgap:UiVal::Px(10.0),..Default::default()},
    //     UiSpan{span:1},
    //     // UiAlign{valign:UiVal::Scale(0.0),..Default::default()},
    //     // UiScroll{hscroll:UiVal::Px(0.0),vscroll:UiVal::Px(0.0)},
    //     // UiScroll{
    //     //     hscroll:UiVal::Px(330.0),
    //     //     // vscroll:UiVal::Scale(0.5),
    //     //     vscroll:UiVal::Px(300.0)
    //     // },
    //     // UiScrollable{ henable: true, venable: true },
    //     UiScroll{
    //         hscroll:UiVal::Px(0.0),
    //         // vscroll:UiVal::Scale(0.5),
    //         vscroll:UiVal::Px(0.0)
    //     },

    // )).id();

    // for i in 0 .. 33 {
    //     let thing=commands.spawn((
    //         // UiSize{width:UiVal::Px(300.0),height:UiVal::Px(30.0)},
    //         // UiSize{width:UiVal::Px(50.0),height:UiVal::Px(20.0)},
    //         UiLayoutComputed::default(),
    //         UiColor{back:Color::srgba(0.7,0.02,0.2,0.5),..Default::default()},

    //         UiTextComputed::default(),
    //         UiInnerSize::default(),
    //         UiText{
    //             value:format!("text {i}"),
    //             font_size:20.0,
    //             // color:Color::WHITE,
    //             color:Color::srgba(0.9,0.9,0.2,0.5),
    //             font:asset_server.load("fonts/FiraMono-Medium.ttf"),
    //             update:true,..Default::default()
    //         },
    //     )).id();

    //     commands.entity(page).add_child(thing);

    // }

    // commands.entity(root).add_child(page);
    // commands.entity(root).add_child(scroll_bar);
    // commands.entity(scroll_bar).add_child(scroll_bar_button);

    println!("root is {root}");
}


pub fn run_ui(
    // ui_root_query : Query<Entity,(With<MenuUiRoot>,)>,

    mut ui_interact_event_reader: EventReader<UiInteractEvent>,
    mut ui_scroll_query : Query<(&mut UiScroll,&UiLayoutComputed)>,
    mut ui_scroll_bar_query : Query<&mut UiAlign,(With<MenuScrollBar>,With<UiDraggable>)>,


) {
    for ev in ui_interact_event_reader.read() {
        println!("{ev:?}");

        match ev.event_type {
            UiInteractEventType::DragY{px,scale:_scale} => {
                if let Ok((mut scroll, layout_computed))=ui_scroll_query.get_mut(ev.entity) {
                    if let UiVal::Px(y)=&mut scroll.vscroll {
                        *y-=px;
                        // *y=y.clamp(-layout_computed.scroll_size.y, 0.0);
                        *y=y.clamp(0.0,layout_computed.scroll_size.y);
                        println!("scroll {} {px}",*y);
                    }

                } else if let Ok(mut align)=ui_scroll_bar_query.get_mut(ev.entity) {
                    if let UiVal::Px(y)=&mut align.valign {
                        *y+=px;
                    }
                }

            }
            _ => {}
        }
    }

}


pub fn run_input(
    mut windows: Query<&mut Window>,
    mut prev_cursor : Local<Option<Vec2>>,

    // mut ui_input_event_reader: EventReader<UiInteractInputEvent>,
    mut ui_interact_input_event_writer: EventWriter<UiInteractInputEvent>,
    mut input_map_event_reader: EventReader<input_map::InputMapEvent<input::Mapping>>,
    mut _input_map : ResMut<input_map::InputMap<input::Mapping>>,


    ui_root_query : Query<Entity,(With<MenuUiRoot>,)>,
){

    let Ok(window) = windows.get_single_mut() else {return;};
    let mouse_cursor = window.cursor_position();//.unwrap_or(Vec2::ZERO);

    let input_map_events=input_map_event_reader.read().cloned().collect::<Vec<_>>();

    for root_entity in ui_root_query.iter() {
        if *prev_cursor!=mouse_cursor {
            let player=0;
            ui_interact_input_event_writer.send(UiInteractInputEvent::CursorMoveTo{root_entity,device: player,cursor:mouse_cursor});
        }

        for ev in input_map_events.iter() {
            match *ev {
                input_map::InputMapEvent::JustPressed { mapping:input::Mapping::Ui(input::UiMapping::CursorOk), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::CursorPressBegin{root_entity, device: player});
                }
                input_map::InputMapEvent::JustReleased{ mapping:input::Mapping::Ui(input::UiMapping::CursorOk), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::CursorPressEnd{root_entity, device: player});
                }

                input_map::InputMapEvent::JustPressed { mapping:input::Mapping::Ui(input::UiMapping::CursorCancel), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::CursorPressCancel{root_entity, device: player});
                }

                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusUp), dir:1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusUp { root_entity, group: player });
                }
                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusUp), dir:-1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusDown { root_entity, group: player });
                }

                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusRight), dir:1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusRight { root_entity, group: player });
                }
                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusRight), dir:-1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusLeft { root_entity, group: player });
                }

                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusNext(0)), dir:1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusNext { root_entity, group: player });
                }
                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusNext(0)), dir:-1, player }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusPrev { root_entity, group: player });
                }

                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusOk), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusPressBegin{root_entity, group:player, device: player});
                }
                input_map::InputMapEvent::JustReleased{ mapping:input::Mapping::Ui(input::UiMapping::FocusOk), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusPressEnd{root_entity, device: player});
                }

                input_map::InputMapEvent::JustPressed{ mapping:input::Mapping::Ui(input::UiMapping::FocusCancel), player, .. }=>{
                    ui_interact_input_event_writer.send(UiInteractInputEvent::FocusPressCancel{root_entity, device: player});
                }
                // HScroll,
                // VScroll,


                // Test,

                // FocusInit{root_entity:Entity, group:i32},

                // FocusEnter{root_entity:Entity, group:i32},
                // FocusExit{root_entity:Entity, group:i32},

                _ => {}
            }
        }
    }

    *prev_cursor=mouse_cursor;
    // for ev in ui_interact_event_reader.read() {
    //     match &ev.event_type {
    //         UiInteractEventType::HoverBegin => {
    //             println!("hover begin {}",ev.entity);
    //         }
    //         UiInteractEventType::HoverEnd => {
    //             println!("hover end {}",ev.entity);
    //         }
    //         x => {

    //             println!("hmm {x:?}",)
    //         }
    //     }
    // }
}
