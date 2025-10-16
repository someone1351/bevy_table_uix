/*
Problems
* on asset modified, error reporting twice

*/
use std::collections::HashSet;

use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
use bevy_table_ui as table_ui;
// use script_lang::IntT;
use script_lang::Value;
use table_ui::*;
use crate::resources::UixGcScope;
use crate::resources::UixLibScope;
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


pub fn on_event_listeners<'a>(
    // ui_assets: Res<Assets<UiAsset>>,
    event_listeners_query: Query<(Entity,&UixEventListener)>,
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // lib_scope:Res<UixLibScope>,
    // lib_scope:Res<script_lang::LibScope<&mut World>>,
    // world: &mut World,


    // mut input_event_reader: MessageReader<UiInteractInputEvent>,
    mut interact_event_reader: MessageReader<UiInteractEvent>,
    mut user_event_reader: MessageReader<UixUserMessage>,
    time: Res<bevy::time::Time>,
    // mut gc_sope:GcS
) {
    //todo update
    // for (entity,event_listener) in event_listeners_query.iter() {
    // }

    // for ev in input_event_reader.read() {
    //     // ev.
    // }

    //
    let time_elapsed=time.delta_secs();

    //
    let mut bla = Vec::new();

    //update events
    for (_entity,event_listener) in event_listeners_query.iter() {
        if let Some((_,listeners))=event_listener.event_listeners.get_key_value("update") {
            let params= vec![Value::float(time_elapsed)];
            bla.push((
                // ev.entity,k.clone(),
                params,listeners.clone(),
            ));
        }
    }

    //interact events
    for ev in interact_event_reader.read() {
        if let Ok((_,event_listener))=event_listeners_query.get(ev.entity) {
            // event_listener.event_listeners.contains_key(ev.event_type.name())
            if let Some((_k,listeners))=event_listener.event_listeners.get_key_value(ev.event_type.name()) {
                let params= match ev.event_type {
                    UiInteractMessageType::HoverBegin { device } => vec![Value::int(device)],
                    UiInteractMessageType::HoverEnd { device } => vec![Value::int(device)],
                    UiInteractMessageType::PressBegin => vec![],
                    UiInteractMessageType::PressEnd => vec![],
                    UiInteractMessageType::Click => vec![],
                    // UiInteractMessageType::DragX { px, scale } => vec![Value::int((px+0.5) as IntT),Value::float(scale)],
                    // UiInteractMessageType::DragY { px, scale } => vec![Value::int((px+0.5) as IntT),Value::float(scale)],

                    UiInteractMessageType::DragX { px,  } => vec![Value::float(px),], //Value::float(scale)
                    UiInteractMessageType::DragY { px,  } => vec![Value::float(px),],
                    UiInteractMessageType::SelectBegin => vec![],
                    UiInteractMessageType::SelectEnd => vec![],
                    UiInteractMessageType::FocusBegin { group } => vec![Value::int(group)],
                    UiInteractMessageType::FocusEnd { group } => vec![Value::int(group)],
                };
                bla.push((
                    // ev.entity,k.clone(),
                    params,listeners.clone(),
                ));
            }
        }
    }

    //user events
    for ev in user_event_reader.read() {
        if let Ok((_,event_listener))=event_listeners_query.get(ev.entity) {
            // event_listener.event_listeners.contains_key(ev.event_type.name())
            if let Some((_k,listeners))=event_listener.event_listeners.get_key_value(ev.event.as_str()) {
                let params= ev.params.clone();
                bla.push((
                    // ev.entity,k.clone(),
                    params,listeners.clone(),
                ));
            }
        }
    }


    //
    commands.queue(move|world:&mut World|{
        let lib_scope = world.resource::<UixLibScope>().0.clone();
        let gc_scope = world.resource::<UixGcScope>().0.clone();
        let mut gc_scope=gc_scope.try_lock().unwrap();

        for (params,listeners) in bla {
            for listener in listeners {
                let mut var_scope = script_lang::VarScope::new();
                let mut machine = script_lang::Machine::new(&mut gc_scope, &lib_scope, &mut var_scope,  world);
                // machine.set_debug_print(true);
                // machine.set_debug(true);
                //println!("a");
                if let Err(e)=machine.call_value(listener,&params) {
                    e.eprint(None);
                }
            }
        }

        gc_scope.mark_and_sweep().unwrap();
    });

    // for (entity,event_listener) in event_listeners_query.iter() {

    // }
}
