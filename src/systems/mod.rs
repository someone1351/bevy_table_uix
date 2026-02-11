
/*
Problems
* on asset modified, error reporting twice

*/
use std::collections::HashSet;

use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
// use bevy_table_ui::layout::messages::UiLayoutComputedChanged;
use bevy_table_ui as table_ui;
use script_lang::StringT;
// use script_lang::IntT;
use script_lang::Value;
use table_ui::*;
use crate::resources::UixGcScope;
use crate::resources::UixLibScope;
// use crate::script_vals::AttribFuncType;
// use crate::script_vals::UiAffectState;
// use crate::script_stuff::self_entity_from_world;


use super::messages::*;
use super::resources::*;
use super::script_utils::*;

use super::assets::*;
use super::components::*;
// use super::resources::*;
// use super::loading_utils::*;

use super::loading::*;

pub fn on_asset_modified_event(
    mut asset_events: MessageReader<AssetEvent<UiAsset>>,
    mut modified_assets : ResMut<UiModifiedAssets>,
) {
    // let mut modified_handles = HashSet::<AssetId<UiAsset>>::new();

    for ev in asset_events.read() {
        println!("asset  {ev:?}", );
        if let AssetEvent::LoadedWithDependencies{id} = ev {
            modified_assets.modified_handles.insert(id.clone());
        }
    }

}
pub fn on_asset_modified_reinit(
    ui_assets: ResMut<Assets<UiAsset>>,
    mut from_asset_query: Query<(Entity,&mut UixFromAsset)>,
    mut modified_assets : ResMut<UiModifiedAssets>,
) {

    //
    let mut modified_from_assets = HashSet::<Entity>::new();

    //
    for (entity, from_asset) in from_asset_query.iter() {
        if !modified_assets.modified_handles.contains(&from_asset.handle.id())
            && from_asset.all_dep_ids.intersection(&modified_assets.modified_handles).count()==0
        {
            continue;
        }

        // println!("hmm3 {entity} : {:?} {:?}",
        //     modified_assets.modified_handles.contains(&from_asset.handle.id()),
        //     from_asset.all_dep_ids.intersection(&modified_assets.modified_handles).count(),
        // );

        modified_from_assets.insert(entity);
    }

    //
    for entity in modified_from_assets {
        let (_,mut from_asset)=from_asset_query.get_mut(entity).unwrap();

        //
        from_asset.init=false;
        from_asset.loaded=true;
        from_asset.all_dep_ids.clear();

        //
        let mut stk=vec![from_asset.handle.id()];

        while let Some(id)=stk.pop() {
            let asset=ui_assets.get(id).unwrap();
            stk.extend(asset.dependencies.iter().map(|y|y.id()));
            from_asset.all_dep_ids.extend(asset.dependencies.iter().map(|y|y.id()));
        }
    }

    //
    modified_assets.modified_handles.clear();

}


pub fn on_asset_load<'a>(
    ui_assets: Res<Assets<UiAsset>>,
    asset_server: Res<AssetServer>,
    mut from_asset_query: Query<(Entity,&mut UixFromAsset)>,
    mut commands: Commands,
    mut event_listeners_query: Query<&mut UixEventListener,>,
) {
    //
    for (top_entity, mut from_asset) in from_asset_query.iter_mut() {
        if !from_asset.loaded || from_asset.init {
            continue;
        }

        //
        from_asset.init=true;

        commands.entity(top_entity)
            // .despawn_descendants()
            .despawn_related::<Children>()

            // .remove::<UixEnv>()
            ;

        if let Ok(mut x)=event_listeners_query.get_mut(top_entity) {
            x.event_listeners.clear();
        }

        //
        let asset=ui_assets.get(from_asset.handle.id()).unwrap();

        //
        //replace all id's with element_ind? eg template_decl_id, template_use_id, apply_decl_id, apply_use_id

        //could allow recursive applies outside a stub, that affect it's descendents,
        //  so a node added by a stub can create another stub
        //  but what if a stub is initd on a nodes creation, and its descendent does the same thing, and so on, it would be recursive

        //in an apply, a stub and an apply that affects a node in that stub, the apply doesn't work

        //for applying an apply, could just add an apply_use, and handle its children being added with node/stub/template_use/etc
        //  when adding apply_use's other sibling, need to skip adding apply_use with them, so can add extra info related to applies ...
        //  don't add a thing for the apply_use?
        //  would need to set apply_after for the apply_use parent thing?

        //
        let Some(mut elements)=load_elements(&ui_assets, &asset_server, asset) else {
            continue;
        };

        //calc apply ignores for node
        calc_node_apply_ignores(&mut elements);

        //
        debug_print_elements1(&elements);


        //
        expand_elements(&mut elements);

        //calc nodes params

        calc_node_params2(&mut elements);

        calc_env_params2(&mut elements);

        mark_used(&mut elements,false);
        mark_has_script(&mut elements,);
        mark_has_script_rest(&mut elements,);



        let only_script=true;

        calc_script_returns(&mut elements,only_script);
        calc_script_apply_calls(&mut elements,only_script);
        calc_script_apply_calls_has_rets(&mut elements,);
        //debug
        debug_print_elements2(&elements);

        //get attribs
        //{ }

        // println!("all_stubs : {all_stubs:?}");

        //script
        {
            let syntax_tree=gen_script_syntax_tree(&elements,only_script);
            // optimise_script_syntax_tree(&elements,&mut syntax_tree);

            debug_print_script_syntax_tree(&syntax_tree);
            let src=gen_script_src(&syntax_tree);
            let stuff=gen_stubs(&elements);

            // println!("all_stubs : {:?}",stuff.all_stubs);
            // println!("all_nodes : {:?}",stuff.all_nodes);

            let stuff = Value::custom_unmanaged(stuff);
            println!("===\n\n{src}\n====");

            // {

            //     std::fs::write("src_debug.txt", &src).expect("Should be able to write to `/foo/tmp`");
            //     let src2=gen_script_src(&syntax_tree);
            //     std::fs::write("src_debug2.txt", &src2).expect("Should be able to write to `/foo/tmp`");
            // }

            // let compiler=script_lang::langs::cexpr_compiler::Compiler::new();
            // let build = compiler.compile(src.as_str(), 0, None, true, );
            // // script_lang::cexpr_compiler::

            // if let Err(e)=&build {
            //     eprintln!("{}",e.msg());
            //     continue;
            // }

            let compiler=script_lang::cexpr_compiler::Compiler::new();
            let compile_result=compiler.compile(&src, 0,from_asset.handle.path().map(|x|x.path()),  true);

            if let Err(e)=compile_result {
                eprintln!("{}",e.msg());
                continue;
            }

            let build=compile_result.unwrap();

            commands.queue(move|world:&mut World|{
                // let top_entity_val=world.entity_mut(top_entity).entry::<UixSelf>().or_insert_with(||UixSelf::new(top_entity)).get_mut().entity.clone();
                // let top_entity_val=UixSelf::new_from_world(world,top_entity);
                let top_entity_val=self_entity_from_world(world,top_entity);

                let lib_scope = world.resource::<UixLibScope>().0.clone();
                let gc_scope = world.resource::<UixGcScope>().0.clone();
                let mut gc_scope=gc_scope.try_lock().unwrap();

                let mut var_scope = script_lang::VarScope::new();
                var_scope.decl("_stubs",Some(stuff)).unwrap();
                // var_scope.decl("root",Some(Value::custom_unmanaged(top_entity))).unwrap();
                var_scope.decl("root",Some(top_entity_val)).unwrap();

                {
                    let mut machine = script_lang::Machine::new(&mut gc_scope, &lib_scope, &mut var_scope,  world);
                    // build.print();
                    // machine.set_debug_print(true);
                    // machine.set_debug(true);
                    if let Err(e)=machine.run_build(&build) {
                        // eprint!("{}",)
                        e.eprint(None);
                    }
                }
                // gc_scope.test();
                gc_scope.mark_and_sweep().unwrap();

                // gc_scope.test();
            });
        }
    }
}


// pub fn on_event_listeners2<'a>(
//     aaa:Query<Ref<UiText>>,
// ) {

//     // for x in aaa.iter() {
//     //     if x.is_changed() {
//     //         println!("a '{}'",x.0);
//     //     }
//     // }
// }

pub fn on_event_listeners<'a>(
    // ui_assets: Res<Assets<UiAsset>>,
    event_listeners_query: Query<(Entity,&UixEventListener)>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // lib_scope:Res<UixLibScope>,
    // lib_scope:Res<script_lang::LibScope<&mut World>>,
    // world: &mut World,

    ui_layout_computed_query:Query<(Entity,&UiLayoutComputed)>,


    // mut input_event_reader: MessageReader<UiInteractInputEvent>,
    mut interact_event_reader: MessageReader<UiInteractEvent>,
    mut user_event_reader: MessageReader<UixUserMessage>,
    // mut computed_layout_changed_reader: MessageReader<UiLayoutComputedChanged>,

    time: Res<bevy::time::Time>,
    // mut gc_sope:GcS
    // aaa:Query<Ref<UiText>>,
) {

    // for x in aaa.iter() {
    //     if x.is_changed() {
    //         println!("'{}'",x.0);
    //     }
    // }
    //todo update
    // for (entity,event_listener) in event_listeners_query.iter() {
    // }

    // for ev in input_event_reader.read() {
    //     // ev.
    // }

    //
    let time_elapsed=time.delta_secs();

    //
    // let mut output_events: Vec<(Entity,StringT, Vec<Value>)> = Vec::new(); //(entity,event_name,params)
    let mut output_events: Vec<(Entity,StringT, Vec<(&'static str,Value)>)> = Vec::new(); //(entity,event_name,first_param,rest_params) ,Vec<Value>

    //update events
    for (entity,event_listener) in event_listeners_query.iter() {
        if let Some((key,_))=event_listener.event_listeners.get_key_value("update") {
            output_events.push((
                entity,
                key.clone(),
                vec![("delta_time",Value::float(time_elapsed))],
            ));
        }
    }

    //interact events
    for ev in interact_event_reader.read() {
        if let Ok((_,event_listener))=event_listeners_query.get(ev.entity) {
            let name=match ev.event_type {
                UiInteractMessageType::CursorHoverBegin{..} => "hover_begin",
                UiInteractMessageType::CursorHoverEnd{..} => "hover_end",
                UiInteractMessageType::CursorPressBegin{..} => "press_begin",
                UiInteractMessageType::CursorPressEnd{..} => "press_end",
                UiInteractMessageType::CursorClick{..} => "click",

                UiInteractMessageType::CursorScroll{..} => "scroll",
                UiInteractMessageType::CursorDragBegin{..} => "drag_begin",
                UiInteractMessageType::CursorDragEnd{..} => "drag_end",
                UiInteractMessageType::CursorDragX { .. } => "drag_x",
                UiInteractMessageType::CursorDragY { .. } => "drag_y",
                UiInteractMessageType::SelectBegin => "select_begin",
                UiInteractMessageType::SelectEnd => "select_end",
                UiInteractMessageType::FocusBegin{..} => "focus_begin",
                UiInteractMessageType::FocusEnd{..} => "focus_end",

                UiInteractMessageType::FocusPressBegin {.. } => "press_begin",
                UiInteractMessageType::FocusPressEnd {.. } => "press_end",
                UiInteractMessageType::FocusClick {.. } => "click",
            };

            if let Some((key,_))=event_listener.event_listeners.get_key_value(name) {
                let params:Vec<(&str,Value)>= match ev.event_type {
                    UiInteractMessageType::CursorHoverBegin { device, cursor } => vec![
                        ("device",device.into()),
                        ("cursor",cursor.to_array().into())
                    ],
                    UiInteractMessageType::CursorHoverEnd { device } => vec![
                        ("device",device.into()),
                    ],
                    UiInteractMessageType::CursorPressBegin{device,button,first, cursor, outer_offset, inner_offset } => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_cursor",true.into()),
                        ("first",first.into()),
                        ("cursor",cursor.to_array().into()),
                        ("outer_offset",outer_offset.to_array().into()),
                        ("inner_offset",inner_offset.to_array().into()),
                    ],
                    UiInteractMessageType::CursorPressEnd{device,button,last,} => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_cursor",true.into()),
                        ("last",last.into()),
                    ],
                    UiInteractMessageType::FocusPressBegin{device,button,} => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_focus",true.into()),
                    ],
                    UiInteractMessageType::FocusPressEnd{device,button,} => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_focus",true.into()),
                    ],
                    UiInteractMessageType::CursorClick{device,button,} => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_cursor",true.into()),
                    ],
                    UiInteractMessageType::FocusClick{device,button,} => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("is_focus",true.into()),
                    ],
                    UiInteractMessageType::CursorDragX { dist, delta, device, button } => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("dist",dist.into()),
                        ("delta",delta.into()),
                    ],
                    UiInteractMessageType::CursorDragY { dist, delta, device, button  } => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("dist",dist.into()),
                        ("delta",delta.into()),
                    ],
                    UiInteractMessageType::SelectBegin => vec![],
                    UiInteractMessageType::SelectEnd => vec![],
                    UiInteractMessageType::FocusBegin { group, device } => vec![
                        ("device",device.into()),
                        ("group",group.into()),
                    ],
                    UiInteractMessageType::FocusEnd { group, device } => vec![
                        ("device",device.into()),
                        ("group",group.into()),
                    ],
                    UiInteractMessageType::CursorDragBegin { device, button, outer_offset, inner_offset, cursor } => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                        ("outer_offset",outer_offset.to_array().into()),
                        ("inner_offset",inner_offset.to_array().into()),
                        ("cursor",cursor.to_array().into())
                    ],
                    UiInteractMessageType::CursorDragEnd { device, button } => vec![
                        ("device",device.into()),
                        ("button",button.into()),
                    ],
                    UiInteractMessageType::CursorScroll { scroll, device, axis } => vec![
                        ("device",device.into()),
                        ("scroll",scroll.into()),
                        ("axis",axis.into()),
                    ],
                };

                output_events.push((
                    ev.entity,
                    key.clone(),
                    params,
                ));
            }
        }
    }

    //user events
    for ev in user_event_reader.read() {
        if let Ok((_,event_listener))=event_listeners_query.get(ev.entity) {
            // event_listener.event_listeners.contains_key(ev.event_type.name())
            if let Some((key,_))=event_listener.event_listeners.get_key_value(ev.event.as_str()) {
                output_events.push((
                    ev.entity,
                    key.clone(),
                    ev.params.clone(),
                ));
            }
        }
    }

    //ui computed layout changed
    for (entity,ui_layout_computed) in ui_layout_computed_query.iter() {
        if ui_layout_computed.changed {
            if let Ok((_,event_listener))=event_listeners_query.get(entity) {
                if let Some((key,_))=event_listener.event_listeners.get_key_value("layout_changed") {
                    output_events.push((
                        entity,
                        key.clone(),
                        vec![],
                    ));
                }
            }
        }
    }

    // //ui computed layout changed
    // for ev in computed_layout_changed_reader.read() {

    // }

    //
    commands.queue(move|world:&mut World|{
        let lib_scope = world.resource::<UixLibScope>().0.clone();
        let gc_scope = world.resource::<UixGcScope>().0.clone();
        let mut gc_scope=gc_scope.try_lock().unwrap();

        for (entity,key,mut first_params, ) in output_events {
            first_params.push(("entity",self_entity_from_world(world,entity)));

            let first_params:script_lang::Dict=first_params.into();
            let all_params = vec![Value::custom_managed(first_params, &mut gc_scope)];
            // all_params.extend(rest_params);

            let e=world.entity(entity);
            let Some(c)=e.get::<UixEventListener>() else {continue;};
            let Some(listeners)=c.event_listeners.get(&key) else {continue;};
            let listeners=listeners.clone();

            let mut found_errs=HashSet::<usize>::new();

            // let listeners = ;
            for (i,(listener,has_err)) in listeners.iter().enumerate() {
                if *has_err {continue;}

                let mut var_scope = script_lang::VarScope::new();
                let mut machine = script_lang::Machine::new(&mut gc_scope, &lib_scope, &mut var_scope,  world);
                // machine.set_debug_print(true);
                // machine.set_debug(true);
                //println!("a");
                if let Err(e)=machine.call_value(listener.clone(),&all_params) {
                    e.eprint(None);
                    found_errs.insert(i);
                }
            }

            if !found_errs.is_empty() {
                let mut e=world.entity_mut(entity);
                let Some(mut c)=e.get_mut::<UixEventListener>() else {continue;};
                let Some(listeners)=c.event_listeners.get_mut(&key) else {continue;};

                for i in found_errs {
                    if let Some((_,has_err))=listeners.get_mut(i) {
                        *has_err=true;
                    }
                }
            }
        }

        gc_scope.mark_and_sweep().unwrap();
    });

    // for (entity,event_listener) in event_listeners_query.iter() {

    // }
}
