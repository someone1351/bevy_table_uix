/*
* allow stubs' nodes to be added to anyone eg stub_a some_node
* add env to store named nodes
    eg
    env.abc.0
    env.0 ??
*/
// use bevy::{asset::{AssetEvent, AssetServer, Assets, Handle}, prelude::{Entity, EventReader, Local, Query, Res, ResMut}};

use core::panic;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ops::Range;
// use std::f32::consts::E;
// use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::color::Color;
// use bevy::prelude::BuildChildren;
use bevy::{ecs::prelude::*, prelude::DespawnRecursiveExt};
use bevy::asset::prelude::*;
use conf_lang::RecordContainer;
use bevy_table_ui as table_ui;
use table_ui::*;
use super::script_stuff::{AttribFunc, Stuff, UixGcScope};

use super::event::UixUserEvent;
use super::resources::UiModifiedAssets;
use super::script_stuff::UixLibScope;
use super::{
    assets::*,
    components::*,
    // resources::*,
};

// pub fn init_asset(
//     asset_server: Res<AssetServer>,
//     mut test:Local<Handle<UiAsset>>,
// ) {
//     // *test=asset_server.load("test.ui_conf");
//     // let x: Handle<UiAsset> =asset_server.load("test.ui_conf");
// }

pub fn on_asset_modified_event(
    mut asset_events: EventReader<AssetEvent<UiAsset>>,
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



fn make_attrib_func<T:Component+Default>(func : impl Fn(&mut T)+ Send+Sync+'static) -> Arc<dyn Fn(Entity,&mut World)+Send+Sync > {
    Arc::new(move |entity:Entity,world: &mut World| {
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<T>().or_default();
        func(&mut c);
    })
}

// pub fn init_lib_scope(
//     mut lib_scope : ResMut<script_lang::LibScope<(&mut World,&mut AssetServer)>>,

// ) {

// }

// pub fn on_asset_load2(
//     // ui_assets: Res<Assets<UiAsset>>,
//     // mut from_asset_query: Query<(Entity,&mut UixFromAsset)>,
//     // mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     // mut lib_scope:ResMut<UixLibScope>,
//     world: &mut World,
// ) {
// }

pub fn on_asset_load<'a>(
    ui_assets: Res<Assets<UiAsset>>,
    mut from_asset_query: Query<(Entity,&mut UixFromAsset)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // lib_scope:Res<UixLibScope>,
    // lib_scope:Res<script_lang::LibScope<&mut World>>,
    // world: &mut World,
) {
    //
    for (top_entity, mut from_asset) in from_asset_query.iter_mut() {
        if !from_asset.loaded || from_asset.init {
            continue;
        }

        //
        from_asset.init=true;

        commands.entity(top_entity)
            .despawn_descendants()
            .remove::<UixEnv>()
            ;

        //

        //
        let asset=ui_assets.get(from_asset.handle.id()).unwrap();

        //



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

        #[derive(Debug,Clone,Default)]
        struct ElementAttribCalc2 {
            in_template : Option<usize>, //template_use_id
            in_apply : Option<usize>, //apply_decl_id
            used:bool,
            ok:bool,
        }

        #[derive(Debug,Clone)]
        enum ElementType<'a> {
            // Root,
            Node {
                names : HashSet<&'a str>,
                // calcd_attribs : Vec<AttribFunc>,
                // calcd_from_node_element_ind : Option<usize>, //node_element_ind
                ignore_applies : HashSet<usize>, //apply_decl_id
            },
            Attrib {
                name : &'a str,
                on_state: Option<UiAffectState>,
                in_template: Option<usize>,
                func : AttribFunc,
                in_node : bool,
                calcd:ElementAttribCalc2,
            },
            Script {
                // srcs : Vec<(&'a str,conf_lang::Loc)>
                record : RecordContainer<'a>,
            },
            Apply {
                name : &'a str, //text_ind
                // apply_decl_id : usize,
                // shadows:bool,
                owner_apply_decl_id : Option<usize>, //element_ind
                // ignores : HashSet<usize>, //apply_decl_id
                // is_root:bool,
            },
            ApplyUse {
                apply_decl_element_ind:usize,
                // from_apply_use_element_ind:Option<usize>,
                // from_path:Vec<usize>,
            },
            TemplateDecl {
                name : &'a str, //text_ind
                // template_decl_id:usize,
            },
            TemplateUse {
                template_decl_element_ind:usize,
                // template_use_id:usize,
            },
            Stub {
                name : &'a str,
            },
            // From {
            //     element_ind:usize,
            // }
            // CalcApplyUse {
            //     apply_element_ind:usize,
            // }
            //CalcNode? what to do with its applies?
        }

        #[derive(Debug,Clone)]
        struct Element<'a> {
            element_type:ElementType<'a>,
            // parent:Option<usize>,
            children : Vec<usize>,
            applies : Vec<usize>, //element_ind
            apply_after : usize, //parent_apply_ind
            // depth:usize,

            // calcd_attribs : Vec<AttribFunc>,
            // calcd_attribs:HashMap<(&'a str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool,AttribFunc)>, //[(name,state)]=(in_template,in_apply.in_node,func)

            // calcd_from_node_element_ind : Option<usize>, //node_element_ind
            calcd_from_element_ind : Option<usize>, //element_ind

            // calcd_in_template : Option<usize>, /template_use_id
            // calcd_in_apply : Option<usize>, //apply/_decl_id

            calcd_node_params:BTreeSet<usize>, //element_ind

            // from_path:Vec<usize>,//element_ind

            calcd_created_from : usize,
        }

        //
        let mut elements = vec![Element{
            element_type: ElementType::Node {
                names:HashSet::new(),
                // calcd_attribs:HashMap::new(),
                // calcd_from_node_element_ind: None,
                ignore_applies:HashSet::new(),
            },
            // parent:None,
            children: Vec::new(),
            applies:Vec::new(),
            apply_after:0,
            // depth:0,
            calcd_from_element_ind : None,
            // calcd_attribs:HashMap::new(),
            // calcd_in_template: None,
            // calcd_in_apply: None,
            calcd_node_params:BTreeSet::new(),
            // from_path:Vec::new(),
            calcd_created_from:0,
        }];

        //elements
        {
            // let mut cur_element_ind = 0;
            // let mut text_inds: HashMap<&str, usize> = HashMap::<&str,usize>::new();
            let mut template_envs: Vec<HashMap<&str,(usize,RecordContainer)>> = vec![HashMap::new()]; //[template_env_ind][template_name]=element_ind
            let mut element_script_records = HashMap::<usize,RecordContainer>::new(); //[src_element_ind]=script_record
            // let mut apply_decl_count:usize=0;
            // let mut template_use_count=0;
            // let mut node_count=0;
            // let mut node_count=0;

            let mut last_element_stk: Vec<usize>=vec![0];

            //walk entire tree
            if let Err(e)=asset.conf.root().walk_ext(|mut walk|{
                let cur_element_ind=last_element_stk.last().cloned().unwrap();
                let cur_template_env_ind = template_envs.len()-1;

                let apply_after = elements.get(cur_element_ind).unwrap().applies.len();

                //
                match walk.record().tag().unwrap() {
                    "include" => {
                        let include_path=walk.record().value(0).get_str().unwrap();
                        let include_handle=asset_server.load(PathBuf::from(include_path));
                        let include_asset=ui_assets.get(include_handle.id()).unwrap();

                        walk.extend(include_asset.conf.root().children());
                    }
                    "template" if walk.record().node_label() == Some("template_decl") && walk.is_enter() => { //
                        walk.do_exit();

                        walk.set_named_note("in_node", false);


                        //
                        let template_name = walk.record().value(0).get_str().unwrap();
                        let template_decl_id=template_envs.last().unwrap().len();
                        let new_element_ind=elements.len();
                        elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                        //
                        walk.set_named_note("in_template", template_decl_id);

                        //
                        template_envs.push(template_envs.last().unwrap().clone());
                        template_envs.last_mut().unwrap().insert(template_name,(new_element_ind,walk.record()));


                        //
                        elements.push(Element {
                            element_type: ElementType::TemplateDecl {
                                name: template_name,
                                // template_decl_id,
                            },
                            // parent: Some(cur_element_ind),
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            // depth:elements.get(cur_element_ind).unwrap().depth+1,
                            calcd_from_element_ind: None,
                            // calcd_attribs:HashMap::new(),
                            // calcd_in_template: None,
                            // calcd_in_apply: None,
                            calcd_node_params:BTreeSet::new(),
                            // from_path:Vec::new(),
                            calcd_created_from:cur_element_ind,
                        });

                        //
                        // cur_element_ind=new_element_ind;
                        last_element_stk.push(new_element_ind);
                    }
                    "template" if walk.record().node_label() == Some("template_decl") && walk.is_exit() => {
                        // cur_element_ind=elements.get(cur_element_ind).unwrap().parent.unwrap();
                        last_element_stk.pop().unwrap();
                    }

                    "template" if walk.record().node_label() == Some("template_use") => {
                        walk.set_named_note("in_node", false);

                        let template_name = walk.record().value(0).get_str().unwrap();

                        let Some((template_decl_element_ind, _template_decl_record)) = template_envs.get(cur_template_env_ind).unwrap().get(&template_name).cloned() else {
                            return Err(walk.error("template not found"));
                        };


                        //
                        let new_element_ind=elements.len();
                        elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);


                        //
                        elements.push(Element {
                            element_type: ElementType::TemplateUse {
                                template_decl_element_ind,
                                // template_use_id: template_use_count,
                            } ,
                            // parent: Some(cur_element_ind),
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            // depth:elements.get(cur_element_ind).unwrap().depth+1,
                            calcd_from_element_ind: None,
                            // calcd_attribs:HashMap::new(),
                            // calcd_in_template: None,
                            // calcd_in_apply: None,
                            calcd_node_params:BTreeSet::new(),
                            // from_path:Vec::new(),
                            calcd_created_from:cur_element_ind, //template_decl_element_ind, //,
                        });

                        //
                        // template_use_count+=1;
                    }

                    "template" if walk.record().node_label() == Some("template_use") && walk.is_exit() => {
                    }
                    "apply" if walk.is_enter() => {
                        walk.do_exit();


                        //for an apply inside a node
                        //  as if searched for, will ret true
                        walk.set_named_note("in_node", false);

                        let mut prev_owner_apply_decl_id: Option<usize>=walk.find_named_note("owner_apply_decl_id").cloned();
                        {
                            let new_element_ind=elements.len();
                            walk.set_named_note("owner_apply_decl_id", new_element_ind);

                        }

                        //
                        for (i,v) in walk.record().values().enumerate() {
                            let name=v.str();
                            let apply_after=if i==0 {apply_after}else{1};

                            let new_element_ind=elements.len();
                            elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                            //
                            elements.get_mut(cur_element_ind).unwrap().applies.push(new_element_ind); //push apply onto parent's applies

                            //
                            // let new_apply_decl_id=apply_decl_count;
                            elements.push(Element {
                                element_type: ElementType::Apply {
                                    name,
                                    // apply_decl_id:new_apply_decl_id,
                                    // shadows: true,
                                    owner_apply_decl_id: prev_owner_apply_decl_id,
                                    // ignores:HashSet::new(),
                                    // is_root:i==0 && walk.depth()==0,
                                },
                                // parent:Some(cur_element_ind),
                                apply_after,
                                children: Vec::new(),
                                applies: Vec::new(),
                                // depth:elements.get(cur_element_ind).unwrap().depth+1+i,
                                calcd_from_element_ind: None,
                                // calcd_attribs:HashMap::new(),
                                // calcd_in_template: None,
                                // calcd_in_apply: None,
                                calcd_node_params:BTreeSet::new(),
                                // from_path:Vec::new(),
                                calcd_created_from:cur_element_ind,
                            });

                            //
                            // cur_element_ind=new_element_ind;
                            last_element_stk.push(new_element_ind);
                            prev_owner_apply_decl_id=Some(new_element_ind);

                            //

                            walk.push_named_note("in_apply", new_element_ind);
                            // apply_decl_count+=1;
                        }
                    }

                    "apply" if walk.is_exit() => {                    //
                        for _ in 0..walk.record().values_num() {
                            // cur_element_ind=elements.get(cur_element_ind).unwrap().parent.unwrap();
                            last_element_stk.pop().unwrap();
                        }

                    }


                    "node" if walk.is_enter() => {
                        walk.do_exit();

                        walk.set_named_note("in_node", true);

                        let names: HashSet<&str>=HashSet::from_iter(walk.record().values().map(|x|x.str()));

                        let new_element_ind=elements.len();
                        elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                        elements.push(Element {
                            element_type: ElementType::Node {
                                names, //calcd_attribs:HashMap::new(), calcd_from_node_element_ind: None
                                ignore_applies:HashSet::new(),
                            },
                            // parent: Some(cur_element_ind),
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            // depth:elements.get(cur_element_ind).unwrap().depth+1,
                            calcd_from_element_ind: None,
                            // calcd_attribs:HashMap::new(),
                            // calcd_in_template: None,
                            // calcd_in_apply: None,
                            calcd_node_params:BTreeSet::new(),
                            // from_path:Vec::new(),
                            calcd_created_from:cur_element_ind,
                        });

                        // cur_element_ind=new_element_ind;
                        last_element_stk.push(new_element_ind);
                    }
                    "node" if walk.is_exit() => {
                        // cur_element_ind=elements.get(cur_element_ind).unwrap().parent.unwrap();
                        last_element_stk.pop().unwrap();
                    }
                    "script" => {
                        //
                        let new_element_ind=elements.len();
                        elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                        //
                        element_script_records.insert(new_element_ind, walk.record());

                        //
                        elements.push(Element {
                            element_type: ElementType::Script {
                                // srcs : walk.record().text_values().map(|v|(v.str(),v.start_loc())).collect(),
                                record : walk.record(),
                            },
                            // parent: Some(cur_element_ind),
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            // depth:elements.get(cur_element_ind).unwrap().depth+1,
                            calcd_from_element_ind: None,
                            // calcd_attribs:HashMap::new(),
                            // calcd_in_template: None,
                            // calcd_in_apply: None,
                            calcd_node_params:BTreeSet::new(),
                            // from_path:Vec::new(),
                            calcd_created_from:cur_element_ind,
                        });
                    }

                    "on" => {
                        walk.do_exit();
                        let affect_state=walk.record().value(0).get_parsed::<UiAffectState>().unwrap();

                        walk.set_named_note("on",affect_state);
                    }

                    "stub" if walk.is_enter() => {
                        walk.do_exit();

                        // walk.set_named_note("in_node", false); // not necessary? since attribs can't be used inside stub

                        let name = walk.record().value(0).get_str().unwrap();

                        let new_element_ind=elements.len();
                        elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                        elements.push(Element {
                            element_type: ElementType::Stub { name, },
                            // parent: Some(cur_element_ind),
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            // depth:elements.get(cur_element_ind).unwrap().depth+1,
                            calcd_from_element_ind: None,
                            // calcd_attribs:HashMap::new(),
                            // calcd_in_template: None,
                            // calcd_in_apply: None,
                            calcd_node_params:BTreeSet::new(),
                            // from_path:Vec::new(),
                            calcd_created_from:cur_element_ind,
                        });

                        // cur_element_ind=new_element_ind;
                        last_element_stk.push(new_element_ind);
                    }
                    "stub" if walk.is_exit() => {
                        // cur_element_ind=elements.get(cur_element_ind).unwrap().parent.unwrap();
                        last_element_stk.pop().unwrap();
                    }
                    x => {
                        let on_state: Option<UiAffectState>=walk.find_named_note("on").cloned();
                        let in_node: bool=walk.find_named_note("in_node").cloned().unwrap_or_default();
                        // let in_apply: Option<usize>=walk.get_named_note("in_apply").cloned();
                        let in_template: Option<usize>=walk.find_named_note("in_template").cloned();

                        let mut attrib_funcs = Vec::new();

                        match x {
                            "size" => {
                                let w = walk.record().value(0).get_parsed::<UiVal>().unwrap();
                                let h = walk.record().value(1).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push(("width",make_attrib_func::<UiSize>(move|c|{
                                    c.width=w;
                                })));

                                attrib_funcs.push(("height",make_attrib_func::<UiSize>(move|c|{
                                    c.height=h;
                                })));
                            }
                            "width" => {
                                let width = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiSize>(move|c|{
                                    c.width=width;
                                })));
                            }
                            "height" => {
                                let height = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiSize>(move|c|{
                                    c.height=height;
                                })));
                            }
                            tag @ ("color"|"border_color"|"text_color"|"padding_color"|"margin_color"|"cell_color") => {
                                let color=match walk.record().node_label().unwrap() {
                                    "colorf" => {
                                        let a:[f32;4]=walk.record().get_parsed_array(1.0).unwrap();
                                        Color::srgba(a[0],a[1],a[2],a[3])
                                    }
                                    "colori" => {
                                        let a:[u8;4]=walk.record().get_parsed_array(255).unwrap();
                                        Color::srgba_u8(a[0],a[1],a[2],a[3])
                                    }
                                    _ => {panic!("");}
                                };

                                match tag {
                                    "color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.back_color.insert(on_state, color);
                                        })));
                                    }
                                    "border_color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.border_color.insert(on_state, color);
                                        })));
                                    }
                                    "text_color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.text_color.insert(on_state, color);
                                        })));
                                    }
                                    "padding_color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.padding_color.insert(on_state, color);
                                        })));
                                    }
                                    "margin_color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.margin_color.insert(on_state, color);
                                        })));
                                    }
                                    "cell_color" => {
                                        attrib_funcs.push((x,make_attrib_func::<UiAffect>(move|c|{
                                            c.cell_color.insert(on_state, color);
                                        })));
                                    }
                                    _=>{panic!("");}
                                }
                            }
                            "hoverable" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiHoverable>(move|c|{
                                    c.enable=v;
                                })));
                            }
                            "pressable" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiPressable>(move|c|{
                                    c.enable=v;
                                })));
                            }
                            "draggable" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiDraggable>(move|c|{
                                    c.enable=v;
                                })));
                            }
                            "selectable" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{
                                    c.enable=v;
                                })));
                            }
                            "focusable" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.enable=v;
                                })));
                            }

                            "press_always"=> {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiPressable>(move|c|{
                                    c.always=v;
                                })));
                            }
                            "press_physical" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiPressable>(move|c|{
                                    c.physical=v;
                                })));
                            }

                            "focused" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.focused=v;
                                })));
                            }
                            "focus_group" => {
                                let v: i32 = walk.record().value(0).get_parsed().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.group=v;
                                })));
                            }
                            "focus_tab_exit" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.tab_exit=v;
                                })));
                            }
                            "focus_hdir_exit" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.hdir_exit=v;
                                })));
                            }
                            "focus_vdir_exit" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.vdir_exit=v;
                                })));
                            }
                            "focus_hdir_wrap" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.hdir_wrap=v;
                                })));
                            }
                            "focus_vdir_wrap" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.vdir_wrap=v;
                                })));
                            }
                            "focus_hdir_press" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.hdir_press=v;
                                })));
                            }
                            "focus_vdir_press" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{
                                    c.vdir_press=v;
                                })));
                            }

                            "selected" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{
                                    c.selected=v;
                                })));
                            }
                            "select_group" => {
                                let v = walk.record().value(0).get_str().unwrap().to_string();

                                attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{
                                    c.group=v.clone(); //can't move from func's capture to c.group
                                })));
                            }

                            "border" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
                                let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v*-1.0);

                                attrib_funcs.push(("border_left",make_attrib_func::<UiEdge>(move|c|{
                                    c.border.left=v;
                                })));
                                attrib_funcs.push(("border_right",make_attrib_func::<UiEdge>(move|c|{
                                    c.border.right=v;
                                })));
                                attrib_funcs.push(("border_top",make_attrib_func::<UiEdge>(move|c|{
                                    c.border.top=v2;
                                })));
                                attrib_funcs.push(("border_bottom",make_attrib_func::<UiEdge>(move|c|{
                                    c.border.bottom=v2;
                                })));
                            }
                            "border_left" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.border.left=v;
                                })));
                            }
                            "border_right" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.border.right=v;
                                })));
                            }
                            "border_top" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.border.top=v;
                                })));
                            }
                            "border_bottom" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.border.bottom=v;
                                })));
                            }

                            "padding" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
                                let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v*-1.0);

                                attrib_funcs.push(("padding_left",make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.left=v;
                                })));
                                attrib_funcs.push(("padding_right",make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.right=v;
                                })));
                                attrib_funcs.push(("padding_top",make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.top=v2;
                                })));
                                attrib_funcs.push(("padding_bottom",make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.bottom=v2;
                                })));
                            }
                            "padding_left" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.left=v;
                                })));
                            }
                            "padding_right" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.right=v;
                                })));
                            }
                            "padding_top" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.top=v;
                                })));
                            }
                            "padding_bottom" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.padding.bottom=v;
                                })));
                            }

                            "margin" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
                                let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v*-1.0);

                                attrib_funcs.push(("margin_left",make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.left=v;
                                })));
                                attrib_funcs.push(("margin_right",make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.right=v;
                                })));
                                attrib_funcs.push(("margin_top",make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.top=v2;
                                })));
                                attrib_funcs.push(("margin_bottom",make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.bottom=v2;
                                })));
                            }
                            "margin_left" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.left=v;
                                })));
                            }
                            "margin_right" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.right=v;
                                })));
                            }
                            "margin_top" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.top=v;
                                })));
                            }
                            "margin_bottom" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                                    c.margin.bottom=v;
                                })));
                            }

                            "font" => {
                                let v = walk.record().value(0).get_str().unwrap();
                                let handle=asset_server.load(PathBuf::from(v)).clone();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.font=handle.clone(); //can't move
                                    c.update=true;
                                })));

                                attrib_funcs.push(("inner_size",make_attrib_func::<UiInnerSize>(move|_|{})));
                                attrib_funcs.push(("text_computed",make_attrib_func::<UiTextComputed>(move|_|{})));

                                // commands.add(move |world: &mut World| {
                                //     let mut e=world.entity_mut(parent_entity);
                                //     e.entry::<UiTextComputed>().or_default();
                                //     e.entry::<UiInnerSize>().or_default();
                                // });
                            }
                            "text" => {
                                let v = walk.record().value(0).get_str().unwrap().to_string();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.value=v.clone(); //can't move from func's capture
                                    c.update=true;
                                })));
                            }
                            "text_halign" => {
                                let v: UiTextHAlign = walk.record().value(0).get_parsed().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.halign=v;
                                    c.update=true;
                                })));
                            }
                            "text_valign" => {
                                let v: UiTextVAlign = walk.record().value(0).get_parsed().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.valign=v;
                                    c.update=true;
                                })));
                            }
                            "text_size" => {
                                let v = walk.record().value(0).get_parsed::<f32>().unwrap().abs();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.font_size=v;
                                    c.update=true;
                                })));
                            }
                            "text_hlen" => {
                                let v: u32 = walk.record().value(0).get_parsed().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.hlen=v;
                                    c.update=true;
                                })));
                            }
                            "text_vlen" => {
                                let v: u32 = walk.record().value(0).get_parsed().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{
                                    c.vlen=v;
                                    c.update=true;
                                })));
                            }

                            "image" => {
                                let v = walk.record().value(0).get_str().unwrap();
                                let handle=asset_server.load(PathBuf::from(v));

                                attrib_funcs.push((x,make_attrib_func::<UiImage>(move|c|{
                                    c.handle=handle.clone(); //can't move from func's capture
                                })));

                                attrib_funcs.push(("inner_size",make_attrib_func::<UiInnerSize>(move|_|{})));
                                // commands.add(move |world: &mut World| {
                                //     let mut e=world.entity_mut(parent_entity);
                                //     e.entry::<UiInnerSize>().or_default();
                                // });
                            }

                            "disabled" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiDisable>(move|c|{
                                    c.disable=v;
                                })));
                            }
                            "hidden" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiHide>(move|c|{
                                    c.hide=v;
                                })));
                            }
                            "floating" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiFloat>(move|c|{
                                    c.float=v;
                                })));
                            }
                            "locked" => {
                                let v : bool = walk.record().get_value(0).map(|x|x.get_parsed().unwrap()).unwrap_or(true);

                                attrib_funcs.push((x,make_attrib_func::<UiLock>(move|c|{
                                    c.lock=v;
                                })));
                            }

                            "gap" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
                                let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v*-1.0);

                                attrib_funcs.push(("hgap",make_attrib_func::<UiGap>(move|c|{
                                    c.hgap=v;
                                })));
                                attrib_funcs.push(("vgap",make_attrib_func::<UiGap>(move|c|{
                                    c.vgap=v2;
                                })));
                            }
                            "hgap" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiGap>(move|c|{
                                    c.hgap=v;
                                })));
                            }
                            "vgap" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiGap>(move|c|{
                                    c.vgap=v;
                                })));
                            }

                            "hexpand" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiExpand>(move|c|{
                                    c.hexpand=v;
                                })));
                            }
                            "vexpand" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiExpand>(move|c|{
                                    c.vexpand=v;
                                })));
                            }

                            "hfill" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiFill>(move|c|{
                                    c.hfill=v;
                                })));
                            }
                            "vfill" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiFill>(move|c|{
                                    c.vfill=v;
                                })));
                            }

                            "row_width_scale" => {
                                let v = walk.record().value(0).get_parsed::<f32>().unwrap().max(0.0);

                                attrib_funcs.push((x,make_attrib_func::<UiCongruent>(move|c|{
                                    c.row_width_scale=v;
                                })));
                            }
                            "col_height_scale" => {
                                let v = walk.record().value(0).get_parsed::<f32>().unwrap().max(0.0);

                                attrib_funcs.push((x,make_attrib_func::<UiCongruent>(move|c|{
                                    c.col_height_scale=v;
                                })));
                            }

                            "halign" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiAlign>(move|c|{
                                    c.halign=v;
                                })));
                            }
                            "valign" => {
                                let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiAlign>(move|c|{
                                    c.valign=v;
                                })));
                            }

                            "span" => {
                                let v = walk.record().value(0).get_parsed::<u32>().unwrap();

                                attrib_funcs.push((x,make_attrib_func::<UiSpan>(move|c|{
                                    c.span=v;
                                })));
                            }

                            // "hscroll" => {}
                            // "vscroll" => {}
                            x => {
                                panic!("{x:?}",)
                            }
                        }

                        //
                        for (attrib_name,func) in attrib_funcs {
                            let new_element_ind=elements.len();
                            elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);
                            elements.push(Element {
                                element_type: ElementType::Attrib {
                                    name:attrib_name,
                                    on_state,
                                    in_template,
                                    func:AttribFunc(func),
                                    in_node,
                                    calcd:Default::default(),
                                },
                                // parent: Some(cur_element_ind),
                                children: Vec::new(),
                                applies: Vec::new(),
                                apply_after,
                                // depth:elements.get(cur_element_ind).unwrap().depth+1,
                                calcd_from_element_ind: None,
                                // calcd_attribs:HashMap::new(),
                                // calcd_in_template: None,
                                // calcd_in_apply: None,
                                calcd_node_params:BTreeSet::new(),
                                // from_path:Vec::new(),
                                calcd_created_from:cur_element_ind,
                            });
                        }
                    }
                }

                Ok(())
            }) {
                eprintln!("{}",e.msg(None));
                continue;
            }
        }

        //calc apply ignores for node
        {
            struct Work{element_ind:usize,depth:usize,}
            let mut work_stk=vec![Work{ element_ind: 0, depth:0 }];

            let mut ancestor_stk: Vec<usize> = Vec::new(); //element_ind

            while let Some(cur_work)=work_stk.pop() {
                let cur_element=elements.get(cur_work.element_ind).unwrap();
                work_stk.extend(cur_element.children.iter().rev().map(|&c|Work { element_ind: c,depth:cur_work.depth+1 }));
                ancestor_stk.truncate(cur_work.depth);

                /*
                apply x
                    apply a #ignored
                    apply y
                        apply a #ignored
                        apply z
                            apply b #used
                            node a b
                apply b
                node x
                    node y
                        node z
                -------------
                apply x
                    node
                        apply y
                            apply z
                                node a
                        apply a #used
                        node y
                            node z
                node x
                -------------
                apply x
                    apply q
                        apply a #used? yes
                        apply y
                            apply a
                            apply z
                                node a
                        node y
                            node z
                node x
                    node q


                */

                /*
                apply x
                    node y
                apply y #no?
                node x
                    apply y #yes
                */
                //

                // if let ElementType::Node { .. } = &cur_element.element_type {

                //     let mut calcd_ignores = HashSet::new();

                //     if ancestor_stk.len()>0 {
                //         for ancestor_ind in (0.. ancestor_stk.len()-1).rev() { //ancestor_stk.len()-1 == grandparent
                //             let ancestor_element_ind=ancestor_stk.get(ancestor_ind).cloned().unwrap();
                //             let ancestor_element=elements.get(ancestor_element_ind).unwrap();

                //             if let //ElementType::Root|
                //                 ElementType::Node{..}|ElementType::TemplateDecl{..}
                //                 |ElementType::Stub { .. } //needed? probably since similar to a node, no ? yes stub can contain nodes which can contain applies
                //                 // |ElementType::ApplyUse { .. }
                //                 // |ElementType::ApplyUse { .. }|ElementType::TemplateUse { .. } //these needed? don't think so, since when element tree created, these aren't filled, and working off applies which aren't evaluated
                //                 =&ancestor_element.element_type { //why? and what about apply_use? or stub, or template_use?
                //                 break;
                //             }
                //             println!("=== {:?} {:?}",ancestor_element.element_type,ancestor_element.applies);
                //             // break;

                //             // calcd_ignores.extend(ancestor_element.applies.iter().map(|&apply_element_ind|{
                //             //     // let apply_element=elements.get(apply_element_ind).unwrap();
                //             //     // let ElementType::Apply { .. }=apply_element.element_type else {panic!("");};
                //             //     // apply_decl_id
                //             //     apply_element_ind
                //             // }));
                //             calcd_ignores.extend(ancestor_element.applies.iter());
                //         }
                //     }

                //     //


                //     let cur_element=elements.get_mut(cur_work.element_ind).unwrap();
                //     let ElementType::Node { ignore_applies, .. } = &mut cur_element.element_type else {panic!("");};
                //     *ignore_applies=calcd_ignores;
                // }



                // let cur_element=elements.get(cur_work.element_ind).unwrap();
                // println!("= {} : {:?} : {:?}",cur_work.element_ind,ancestor_stk,cur_element.element_type);
                //
                ancestor_stk.push(cur_work.element_ind);

            }
        }

        //
        // applies inside a template should only affect the template's descendants and not siblings
        // attrib set by apply/template are always overwritten

        //
        if false {

            println!("\n");

            {
                let mut stk=vec![(0,0)];

                while let Some((cur_element_ind,depth))=stk.pop() {
                    let cur_element=elements.get(cur_element_ind).unwrap();
                    stk.extend(cur_element.children.iter().rev().map(|&x|(x,depth+1)));
                    let depth="    ".repeat(depth);
                    println!("{depth}{:?}, applies={:?}, after={}",
                        &cur_element.element_type,
                        &cur_element.applies,
                        // cur_element.applies.iter().map(|&i|{
                        //     // let ElementType::Apply { .. }=&elements.get(i).unwrap().element_type else{panic!("")};
                        //     // *apply_decl_id
                        //     i
                        // }).collect::<Vec<_>>(),
                        cur_element.apply_after,
                    );
                }
            }
            println!("\n");

        }

        // struct ElementAttribCalc {
        //     in_template : Option<usize>, //template_use_id
        //     in_apply : Option<usize>, //apply_decl_id
        //     used:bool,
        //     ok:bool,
        // }

        // let mut element_attrib_calcs: HashMap<usize,ElementAttribCalc> = HashMap::new(); //[attrib_element_ind]

        // struct CallElement {
        //     element_ind:usize,
        //     children:Vec<CallElement>,

        // }

        // let mut call_element_trees: HashMap<Option<usize>, Vec<CallElement>> = HashMap::new(); // [stub]=
        //
        {
            struct Thing { //
                // entity : Entity,
                // names : HashSet<&'a str>,
                applies : Vec<(usize, //apply_element_ind
                    // Option<usize>
                    // Vec<usize>,
                    usize, //from_element_ind
                )>, //apply_element_ind, from_apply_use_element_ind
                // attribs:HashMap<(&'a str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool)>, //[(name,state)]=(in_template,in_apply.in_node,)
                apply_after:usize,
                element_ind:usize,

                // attribs:HashMap<(&'a str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool,AttribFunc)>, //[(name,state)]=(in_template,in_apply.in_node,func)
            }

            #[derive(Clone)]
            struct Work {
                element_ind:usize,
                // exit:bool,
                from_applies:HashSet<usize>, //apply_element_ind
                in_template:Option<usize>, //template_use_id
                in_apply:Option<usize>, //apply_decl_id

                // from_element2_ind:Option<usize>,

                // inside_decl:bool, //inside template or apply decl
                new_from_parent:Option<usize>, //element_ind
                thing_apply_after_offset:usize, //Option<usize>,

                node_depth:usize,
                thing_depth:usize,

                // last_apply_use:Option<usize>, //element_ind
                // from_path:Vec<usize>,  //element_ind
                created_from:usize,

            }

            let mut work_stk=vec![Work{
                element_ind:0,
                // exit:false,
                in_template:None,
                in_apply:None,
                from_applies:HashSet::new(),
                // from_element2_ind:None,
                // inside_decl:false,
                new_from_parent:None,
                thing_apply_after_offset:0,

                node_depth:0,
                thing_depth:0,

                // last_apply_use:None,
                // from_path:Vec::new(),
                created_from:0,
            }];

            // struct Callll {
            //     ret:usize, //apply_use_element_ind
            //     call:usize, //apply_element_ind
            //     from:Vec<usize>, //top_node,
            // }

            let mut things: Vec<Thing>=vec![];
            let mut node_stk_attribs: Vec<HashMap<(&str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool,AttribFunc,usize)>> = Vec::new(); //[node_depth][(name,state)]=(in_template,in_apply.in_node,func,element_ind)
            // let mut call_element_key_stk;
            while let Some(cur_work)=work_stk.pop() {

                // if let ElementType::Apply{..}=&elements.get(cur_work.element_ind).unwrap().element_type {
                //     println!("is an apply {}",cur_work.element_ind);
                // }

                // if let ElementType::Apply{..} //stop
                //     |ElementType::TemplateDecl{..} //not necessary?
                //     =&elements.get(cur_work.element_ind).unwrap().element_type {
                //     continue;
                // }


                //
                let mut the_new_element_ind:Option<usize> = None;

                //replace element added via apply/template with new copy
                // if enter
                {
                    // Node {.. },Script {..},TemplateUse {..}, Stub {..},
                    // Attrib {..}, Apply {..}, ApplyUse {..}, TemplateDecl {..},
                    //
                    //
                    // cur_work.inside_apply_after
                    if let Some(new_from_parent)=cur_work.new_from_parent {
                        let cur_element=elements.get(cur_work.element_ind).unwrap();

                        //if ElementType::Script{..}, set parent.script=true?

                        if let ElementType::Node {..}|ElementType::TemplateUse{..}|ElementType::Stub{..}
                            // |ElementType::Script{..}
                            //not needed, can just pass the same one around, actually is needed, need unique element_ind for attrib for debug purposes
                            //also attrib needs to be added to its new parent, but don't need a new copy for that
                            |ElementType::Attrib{..}
                            =&cur_element.element_type {

                            //
                            let mut new_element=Element {
                                children: Vec::new(), ..cur_element.clone()
                            };
                            // new_element.depth=elements.get(new_from_parent).unwrap().depth+1;
                            new_element.calcd_from_element_ind=Some(cur_work.element_ind);
                            // new_element.apply_after+=cur_work.inside_apply_after;
                            // new_element.parent=; //no

                            //
                            let new_element_ind=elements.len();
                            elements.get_mut(new_from_parent).unwrap().children.push(new_element_ind);
                            elements.push(new_element);

                            the_new_element_ind=Some(new_element_ind);

                            // cur_work.element_ind=new_element_ind;
                        // } else if let ElementType::Attrib {..}=&cur_element.element_type { //not needed
                            // elements.get_mut(new_from_parent).unwrap().children.push(cur_work.element_ind);
                        }
                    }
                }

                //
                node_stk_attribs.truncate(cur_work.node_depth);
                things.truncate(cur_work.thing_depth);

                //
                let (node_depth, thing_depth,)= {
                    let cur_element=elements.get(cur_work.element_ind).unwrap();

                    let cur_is_node=if let ElementType::Node{..}=&cur_element.element_type {true}else{false};
                    let cur_is_thing=if let ElementType::Node{..}|ElementType::TemplateUse{..}|ElementType::Stub{..} =&cur_element.element_type {true}else{false};

                    (
                        cur_work.node_depth+if cur_is_node{1}else{0},
                        cur_work.thing_depth+if cur_is_thing{1}else{0},
                    )
                };

                //node attribs
                {
                    let cur_element=elements.get(cur_work.element_ind).unwrap();

                    if let ElementType::Node { .. } = &cur_element.element_type {
                        node_stk_attribs.push(HashMap::new());
                    }
                }

                //attrib
                // if enter
                {//
                    let cur_element=elements.get_mut(cur_work.element_ind).unwrap(); //can user either ind

                    if let ElementType::Attrib { name,in_node, on_state, func, calcd,.. } = &mut cur_element.element_type {
                        let in_node=*in_node;
                        let on_state=*on_state;

                        let node_attribs=node_stk_attribs.last_mut().unwrap();

                        let prev=node_attribs.get(&(name,on_state));
                        let prev_element_ind=prev.map(|x|x.4);
                        let (prev_in_template,prev_in_apply,prev_in_node, )=prev.map(|x|(x.0,x.1,x.2,)).unwrap_or_default();

                        let in_apply=cur_work.in_apply;
                        let in_template=cur_work.in_template;


                        let ok = in_node || !prev_in_node || (in_apply!=prev_in_apply || prev_in_template!=in_template);

                        let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);


                        calcd.in_template=in_template;
                        calcd.in_apply=in_apply;
                        calcd.ok=ok;
                        calcd.used=ok;
                        // element_attrib_calcs.insert(cur_element_ind, ElementAttribCalc { in_template, in_apply, used: ok, ok, });

                        if ok {
                            node_attribs.insert((name,on_state), (in_template,in_apply,in_node,func.clone(),cur_element_ind));

                            // if let Some(prev_element_attrib_calc)=element_attrib_calcs.get_mut(&prev_element_ind) {
                            //     prev_element_attrib_calc.used=false;
                            // }
                            if let Some(prev_element_ind)=prev_element_ind{
                                let ElementType::Attrib { calcd,.. } = &mut elements.get_mut(prev_element_ind).unwrap().element_type else {
                                    panic!("");
                                };
                                calcd.used=false;

                            }

                        }
                    }
                }

                //
                // if let ElementType::Apply{..}
                //     |ElementType::TemplateDecl{..} //not necessary
                //     =&elements.get(cur_work.element_ind).unwrap().element_type {

                // } else
                if let Some(cur_element_ind)=the_new_element_ind {

                    // let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
                    let cur_element=elements.get_mut(cur_element_ind).unwrap();
                    cur_element.calcd_created_from=cur_work.created_from;
                }


                //
                let mut new_applies: Vec<(usize,
                    // Option<usize>
                    // Vec<usize>,
                    usize,
                )> = Vec::new();

                //get template applies
                // if enter
                {
                    let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
                    let cur_element=elements.get(cur_work.element_ind).unwrap();
                    // let mut from_path = cur_work.from_path.clone();
                    // from_path.push(cur_work.element_ind);

                    if let ElementType::TemplateUse { template_decl_element_ind, .. } = &cur_element.element_type {
                        let template_decl_element=elements.get(*template_decl_element_ind).unwrap();
                        // new_applies.extend(template_decl_element.applies.iter().map(|&apply_element_ind|apply_element_ind));
                        new_applies.extend(template_decl_element.applies.iter().map(|&apply_element_ind|(
                            apply_element_ind,
                            // None
                            // cur_work.last_apply_use,
                            // cur_work.from_path.clone(),
                            cur_element_ind,
                        )));
                        //
                    }
                }

                //applies...

                // if enter
                {
                    let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);

                    //should use cur_element_ind?
                    //  for cur_element.apply_after? since it's modified? that apply_after is only modified in thing,
                    //    which is because the applies are all added to it,
                    //    so shouldn't apply to here
                    let cur_element=elements.get(cur_work.element_ind).unwrap();

                    // let cur_element_depth=cur_element.depth;
                    //should go depth first, instead of breadth for get new applies, but how would handle apply_after index?
                    //    would need to increment it by the amount added when before after_apply,
                    //    in thing have a count of the number added, which can be added to element.apply_after

                    if let ElementType::Node { names, ignore_applies, .. } = cur_element.element_type.clone() {

                        //
                        let last_stub_thing_ind=things.iter().enumerate().rev().find_map(|(i,t)|{
                            if let ElementType::Stub{..}=&elements.get(t.element_ind).unwrap().element_type {
                                Some(i)
                            } else {
                                None
                            }
                        });

                        //
                        let mut before_applies: Vec<(usize,
                            // Option<usize>
                            // Vec<usize>,
                            usize, //from
                        )> = Vec::new();
                        let mut after_applies: Vec<(usize,
                            // Option<usize>
                            // Vec<usize>,
                            usize, //from
                        )> = Vec::new();

                        //get before applies
                        for (thing_ind,thing) in things.iter().enumerate() {
                            let apply_after=things.get(thing_ind+1).map(|x|x.apply_after).unwrap_or(cur_element.apply_after);

                            for (thing_apply_ind,&(apply_element_ind,from)) in thing.applies.iter().enumerate() {
                                // let apply_element_ind=*apply_element_ind;
                                let apply_element=elements.get(apply_element_ind).unwrap();
                                let ElementType::Apply { name:apply_name, .. }=&apply_element.element_type else {panic!("")};

                                if !cur_work.from_applies.contains(&apply_element_ind) //prevent recursion //apply_decl_id
                                    && thing_apply_ind<apply_after //is before
                                    && names.contains(apply_name)
                                {
                                    before_applies.push((apply_element_ind,from)); //need to add owner of the apply?
                                }
                            }
                        }

                        //get after applies
                        for (thing_ind,thing) in things.iter().enumerate().rev() {
                            //don't allow nodes in stubs to use applies that come after the stub
                            if let Some(last_stub)=last_stub_thing_ind {
                                // println!("thing {thing_ind}, last_stub {last_stub}");
                                if thing_ind<=last_stub {
                                    continue;
                                }
                            }

                            //
                            let apply_after=things.get(thing_ind+1).map(|x|x.apply_after).unwrap_or(cur_element.apply_after);

                            for (thing_apply_ind,&(apply_element_ind,from)) in thing.applies.iter().enumerate() {
                                // let apply_element_ind=*apply_element_ind;
                                let apply_element=elements.get(apply_element_ind).unwrap();
                                let ElementType::Apply { name:apply_name, .. }=&apply_element.element_type else {panic!("")};

                                if !cur_work.from_applies.contains(&apply_element_ind) //prevent recursion //apply_decl_id
                                    && thing_apply_ind>=apply_after //is after //what does thing_ind have to do with apply_after? oh thing_apply_ind
                                    && names.contains(apply_name)
                                {
                                    after_applies.push((apply_element_ind,from));
                                }
                            }
                        }


                        //apply the applies on cur
                        //  should add ApplyUse, with children added to its element, filter_map_children, removing Apply
                        // let mut the_apply_after=cur_element.applies.len();
                        let mut the_apply_after_count=0;


                        //
                        let mut apply_use_element_inds: Vec<Option<usize>>=Vec::new();

                        //add apply_use elements
                        {

                            let parent_applies_len=cur_element.applies.len();

                            for &(apply_element_ind,from) in before_applies.iter().chain(after_applies.iter()) {
                                // let apply_element_ind=*apply_element_ind;




                                //
                                // let apply_element=elements.get(apply_element_ind).unwrap();
                                // let ElementType::Apply{..}=apply_element.element_type else {panic!("");};

                                if ignore_applies.contains(&apply_element_ind) { //apply_decl_id
                                    apply_use_element_inds.push(None);
                                    continue;
                                }

                                let apply_use_element_ind=elements.len();
                                apply_use_element_inds.push(Some(apply_use_element_ind));



                                elements.get_mut(cur_element_ind).unwrap().children.push(apply_use_element_ind);  //the cur_work.element_ind was wrong,

                                // println!("------- apply use {}")
                                elements.push(Element {
                                    // element_type: ElementType::ApplyUse { apply_decl_element_ind: apply_element_ind },
                                    element_type:ElementType::ApplyUse { apply_decl_element_ind: apply_element_ind,
                                        // from_path: from_path.clone(),
                                    } ,
                                    // parent: Some(cur_work.element_ind),
                                    // parent:None, //don't need it
                                    children: Vec::new(),
                                    applies: Vec::new(),
                                    // apply_after: cur_element.applies.len(), //correct? no
                                    apply_after:parent_applies_len, //new_applies.len(), //wrong? since this refers to parent.applies and not thing.applies, so should be parent.applies.len()
                                    // depth: cur_element_depth+1,
                                    calcd_from_element_ind: None,
                                    // calcd_from_element_ind: Some(cur_work.element_ind),
                                    // calcd_attribs:HashMap::new(),
                                    // calcd_in_template: None,
                                    // calcd_in_apply: None,
                                    calcd_node_params:BTreeSet::new(),
                                    // from_path:Vec::new(),
                                    calcd_created_from:from,
                                });

                            }

                        }
                        //
                        // for (i,&(apply_element_ind,_from)) in (after_applies.iter().rev().chain(before_applies.iter().rev()) ).enumerate()
                        for (i,&(apply_element_ind,_)) in (after_applies.iter().rev().chain(before_applies.iter().rev()) ).enumerate()
                        {

                            //apply_use
                            // let apply_element_ind=*apply_element_ind;
                            // let apply_use_element_ind=apply_use_element_inds[apply_use_element_inds.len()-i-1];

                            let Some(apply_use_element_ind)=apply_use_element_inds[apply_use_element_inds.len()-i-1] else {
                                continue;
                            };

                            // let mut from_path=from_path.clone();
                            // from_path.push(apply_use_element_ind);

                            //
                            let apply_element=elements.get(apply_element_ind).unwrap();
                            let ElementType::Apply{owner_apply_decl_id:parent_owner_apply_decl_id,..}=apply_element.element_type else {panic!("");};

                            let the_apply_after=the_apply_after_count;
                            the_apply_after_count+=apply_element.applies.len();

                            // if ignore_applies.contains(&apply_decl_id) {
                            //     continue;
                            // }

                            let mut from_applies = cur_work.from_applies.clone();
                            from_applies.insert(apply_element_ind); //apply_decl_id

                            //what happens with apply children
                            // applies are added to thing with new_applies


                            //
                            work_stk.extend(apply_element.children.iter().rev().map(|&child_element_ind|{
                                let child_element=elements.get(child_element_ind).unwrap();
                                // if let ElementType::Apply{..}=&child_element.element_type {
                                //     println!("is an apply {child_element_ind}");
                                // }
                                let in_apply=if let ElementType::Attrib{..}|ElementType::TemplateUse{..}=child_element.element_type.clone(){
                                    parent_owner_apply_decl_id //use cur apply's parent
                                }else{ //use cur apply
                                    // Some(apply_decl_id)
                                    Some(apply_element_ind)
                                };

                                let in_template=if let ElementType::Attrib{in_template,..}=child_element.element_type.clone(){
                                    in_template
                                } else {
                                    cur_work.in_template //why this?
                                };

                                Work{
                                    element_ind: child_element_ind,
                                    // exit : false,
                                    in_template,
                                    in_apply,
                                    from_applies:from_applies.clone(),
                                    // from_element2_ind:Some(new_element2_ind),

                                    // inside_decl:true,
                                    new_from_parent:Some(apply_use_element_ind),
                                    // inside_decl: todo!(),
                                    thing_apply_after_offset: the_apply_after,
                                    node_depth,
                                    thing_depth,
                                    // last_apply_use:Some(apply_use_element_ind),
                                    // from_path:from_path.clone(),
                                    created_from:apply_use_element_ind,
                                }
                            }));

                        }


                        //get applies for new thing
                        //  add cur_element.applies then before+after applies
                        //  could do before_applies, cur_applies, after_applies

                        // // for element_ind in [cur_work.element_ind].into_iter().chain(before_applies.into_iter()).chain(after_applies.into_iter())
                        // // for element_ind in before_applies.into_iter().chain([cur_work.element_ind].into_iter()).chain(after_applies.into_iter())
                        // for &element_ind in before_applies.iter().chain([cur_work.element_ind].iter()).chain(after_applies.iter())
                        // for (i,(element_ind,_from)) in
                        //     before_applies.iter().enumerate().map(|(i,&x)|(Some(i),x))
                        //     .chain([(None,(cur_work.element_ind,cur_work.created_from))])
                        //     .chain(after_applies.iter().enumerate().map(|(i,&x)|(Some(i+before_applies.len()),x)))
                        for (i,element_ind) in
                            before_applies.iter().enumerate().map(|(i,&x)|(Some(i),x.0))
                            .chain([(None,cur_work.element_ind)])
                            .chain(after_applies.iter().enumerate().map(|(i,&x)|(Some(i+before_applies.len()),x.0)))
                        {
                            let element=elements.get(element_ind).unwrap();

                            // let apply_use_element_ind=*apply_use_element_inds.get(i).unwrap();

                            // if let ElementType::Node{..}|ElementType::Apply {..}|ElementType::TemplateUse{..} = &element.element_type { //should always be apply or node?

                                let from=if let Some(i)=i {
                                    // from_path.push(apply_use_element_inds[i]);
                                    let Some(apply_use_element_ind)=apply_use_element_inds[i] else {
                                        continue;
                                    };
                                    apply_use_element_ind
                                    // 88
                                } else {
                                    // cur_work.last_apply_use
                                    // println!("aaa {} {}",cur_element_ind,cur_work.created_from);
                                    // cur_work.created_from
                                    cur_element_ind
                                    // 99
                                };

                                // let u=if let Some(i)=i {
                                //     Some(apply_use_element_inds[i])
                                // } else {
                                //     cur_work.last_apply_use
                                // };
                                new_applies.extend(element.applies.iter().map(|&apply_element_ind|(apply_element_ind,from)));
                                //(apply_element_ind,Some(apply_use_element_ind))
                            // }
                        }

                    }
                }


                //push children (to work)
                // if enter
                {
                    let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
                    let cur_element=elements.get(cur_work.element_ind).unwrap();
                    // println!("hmm {}",cur_work.element_ind);
                    match &cur_element.element_type {
                        // ElementType::Root => {
                        //     work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                        //         element_ind: child_element_ind,
                        //         exit,
                        //         from_applies:HashSet::new(),
                        //         in_apply:None,
                        //         in_template:None,
                        //     }));
                        // }
                        ElementType::Node { .. } => {

                            // //
                            // let new_element2_ind=elements2.len();
                            // elements2.push(Element2 {
                            //     element2_type: Element2Type::Node { attribs: HashMap::new() } ,
                            //     parent: cur_work.parent_element2_ind,
                            //     children: Vec::new(),
                            // });

                            // let mut from_path = cur_work.from_path.clone();
                            // from_path.push(
                            //     // cur_work.element_ind
                            //     99
                            // );

                            //
                            work_stk.extend(cur_element.children.iter().rev().filter_map(|&child_element_ind|{
                                let child_element=elements.get(child_element_ind).unwrap();
                                if let ElementType::ApplyUse { .. } =&child_element.element_type {
                                    None
                                } else {
                                    let w=Work{
                                        element_ind: child_element_ind,
                                        // exit:false,
                                        from_applies:cur_work.from_applies.clone(),
                                        in_apply:cur_work.in_apply,
                                        in_template:cur_work.in_template,
                                        new_from_parent: the_new_element_ind, //so new element is created for child
                                        thing_apply_after_offset: 0,
                                        // from_element2_ind: cur_element2_ind,
                                        // inside_parent:cur_work.inside_parent,
                                        node_depth,
                                        thing_depth,
                                        // last_apply_use:cur_work.last_apply_use,
                                        // from_path:from_path.clone(),
                                        created_from:cur_element_ind,
                                    };
                                    Some(w)
                                }
                            }));
                        }
                        ElementType::Stub { .. } => {
                            // //
                            // let new_element2_ind=elements2.len();
                            // elements2.push(Element2 {
                            //     element2_type: Element2Type::Stub {  },
                            //     parent: cur_work.parent_element2_ind,
                            //     children: Vec::new(),
                            // });

                            // let mut from_path = cur_work.from_path.clone();
                            // from_path.push(cur_work.element_ind);

                            //
                            work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                                element_ind: child_element_ind,
                                // exit:false,
                                from_applies:cur_work.from_applies.clone(),
                                in_apply:cur_work.in_apply,
                                in_template:cur_work.in_template,
                                // from_element2_ind: cur_element2_ind,
                                new_from_parent: the_new_element_ind, //so new element is created for child
                                thing_apply_after_offset: 0,
                                node_depth,
                                thing_depth,
                                // last_apply_use:cur_work.last_apply_use,
                                // from_path:from_path.clone(),
                                created_from:cur_element_ind,
                            }));
                        }
                        ElementType::TemplateUse { template_decl_element_ind,   .. } => {
                            // //
                            // let new_element2_ind=elements2.len();
                            // elements2.push(Element2 {
                            //     element2_type: Element2Type::TemplateUse {  },
                            //     parent: cur_work.parent_element2_ind,
                            //     children: Vec::new(),
                            // });

                            //
                            // let mut from_path = cur_work.from_path.clone();
                            // from_path.push(cur_work.element_ind);

                            work_stk.extend(elements.get(*template_decl_element_ind).unwrap().children.iter().rev().map(|&child_element_ind|{
                                let child_element=elements.get(child_element_ind).unwrap();
                                let in_template=if let ElementType::Attrib{..}=&child_element.element_type {
                                    cur_work.in_template
                                }else{
                                    // Some(*template_use_id)
                                    Some(cur_element_ind)
                                };

                                Work{
                                    element_ind: child_element_ind,
                                    // exit:false,
                                    from_applies:cur_work.from_applies.clone(),
                                    in_apply:cur_work.in_apply,
                                    in_template,
                                    // from_element2_ind: cur_element2_ind,
                                    // new_from_parent: if the_new_element_ind.is_some(){the_new_element_ind}else{Some(cur_work.element_ind)}, //so new element is created for child
                                    new_from_parent: Some(the_new_element_ind.unwrap_or(cur_work.element_ind)), //so new element is created for child
                                    thing_apply_after_offset: 0,
                                    node_depth,
                                    thing_depth,
                                    // last_apply_use:cur_work.last_apply_use,
                                    // from_path:from_path.clone(),
                                    created_from:cur_element_ind,
                                }
                            }));
                        }
                        // ElementType::ApplyUse { apply_decl_element_ind } => {

                        // }
                        _=> {}
                    }
                }

                //push thing
                // if enter
                {
                    let cur_element=elements.get(cur_work.element_ind).unwrap();

                    let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
                    match &cur_element.element_type {
                        ElementType::Node {  ..  } => {
                            things.push(Thing {
                                applies: new_applies,
                                apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                                element_ind:cur_element_ind,
                            });
                        }
                        ElementType::Stub { .. } => {
                            things.push(Thing {
                                applies: Vec::new(), //new_applies, //doesn't use new applies? should be empty anyway
                                apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                                element_ind:cur_element_ind,
                            });
                        }
                        ElementType::TemplateUse { .. } => {
                            things.push(Thing {
                                applies: new_applies,
                                apply_after: cur_element.apply_after+cur_work.thing_apply_after_offset, //for elements added by apply
                                element_ind:cur_element_ind,
                            });
                        }
                        _=> {}
                    }
                }


            }
        }

        //calc attribs
        // {

        //     #[derive(Clone)]
        //     struct Work {
        //         element_ind:usize,
        //         depth:usize,
        //         exit:bool,
        //     }

        //     let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false, }];
        //     let mut node_stk=Vec::new();

        //     while let Some(cur_work)=work_stk.pop() {
        //         let cur_element=elements.get(cur_work.element_ind).unwrap();


        //         match &cur_element.element_type {
        //             ElementType::Node{..} if cur_work.exit => {
        //                 node_stk.pop().unwrap();
        //             }
        //             ElementType::Node{..} => {
        //                 node_stk.push(cur_work.element_ind);
        //                 work_stk.push(Work{exit:true, ..cur_work.clone()})
        //             }
        //             ElementType::TemplateUse{..} => {
        //             }
        //             ElementType::Stub{..} => {
        //             }
        //             ElementType::ApplyUse{..} => {
        //             }
        //             ElementType::Attrib { ..  } => {
        //             }
        //             _ => {
        //                 continue;
        //             }
        //         }

        //         if cur_work.exit {
        //             continue;
        //         }

        //         work_stk.extend(cur_element.children.iter().rev().map(|&child|Work { element_ind: child, depth: cur_work.depth+1, exit:false, }));

        //         if let ElementType::Attrib { name,in_node, on_state, func, .. } = cur_element.element_type.clone() {
        //             let node_element=elements.get(*node_stk.last().unwrap()).unwrap();
        //             // node_element.calcd_attribs.push(value);


        //             let prev=node_element.calcd_attribs.get(&(name,on_state));
        //             let (prev_in_template,prev_in_apply,prev_in_node)=prev.map(|x|(x.0,x.1,x.2)).unwrap_or_default();

        //             let in_apply=cur_element.calcd_in_apply;
        //             let in_template=cur_element.calcd_in_template;


        //             let ok = in_node || !prev_in_node || (in_apply!=prev_in_apply || prev_in_template!=in_template);

        //             if ok {
        //                 let node_element=elements.get_mut(*node_stk.last().unwrap()).unwrap();
        //                 node_element.calcd_attribs.insert((name,on_state), (in_template,in_apply,in_node,func.clone()));
        //             }
        //         }
        //     }

        // }


        //calc nodes params
        {

            #[derive(Clone)]
            struct Work {
                element_ind:usize,
                exit:bool,
                parent:Option<usize>,
                in_decl:bool,
            }


            let mut work_stk=vec![Work { element_ind:0, exit:false, parent:None,in_decl:false, }];

            while let Some(cur_work)=work_stk.pop() {
                let cur_element=elements.get(cur_work.element_ind).unwrap();

                if !cur_work.exit {

                    // if let
                    //     //ElementType::TemplateUse{..}|
                    //     ElementType::ApplyUse{..}=&cur_element.element_type {
                    // } else
                    {
                        work_stk.push(Work{exit:true, ..cur_work.clone()});

                        let in_decl=if let ElementType::TemplateDecl{..}|ElementType::Apply{..}=&cur_element.element_type {true} else {cur_work.in_decl};

                        work_stk.extend(cur_element.children.iter().rev().map(|&element_ind|Work {
                            element_ind,
                            exit:false,
                            parent:Some(cur_work.element_ind),
                            in_decl,
                        }));
                    }
                }

                let Some(parent_element_ind)=cur_work.parent else {
                    continue;
                };

                if !cur_work.exit {
                    //works by traversing expanded nodes
                    //  getting nodes and storing them as params in their parent
                    //  and then extending ancestor params with those
                    //it works fine for unexpanded apply decls
                    //  since they don't need to store params from an apply to its ancestors
                    //but doesn't work for templates in templates,
                    //  as it does need to store params from its ancestors
                    //  and it is unexpanded
                    //  can just take params from template use's decl, and extend cur template use's params
                    //    problem is need to differentiate when inside a template decl or not


                    match &cur_element.element_type {
                        ElementType::Node { .. }  => {

                            let parent_element=elements.get_mut(parent_element_ind).unwrap();
                            parent_element.calcd_node_params.insert(cur_work.element_ind);
                        }
                        ElementType::TemplateUse{ template_decl_element_ind, .. } if cur_work.in_decl  => {
                            let decl_element=elements.get(*template_decl_element_ind).unwrap();
                            let params=decl_element.calcd_node_params.clone();

                            let cur_element=elements.get_mut(cur_work.element_ind).unwrap();
                            cur_element.calcd_node_params.extend(params);
                        }
                        _ =>  {}
                    }
                    // if let ElementType::Node { .. }  = &cur_element.element_type {
                    //     let parent_element=elements.get_mut(parent_element_ind).unwrap();
                    //     parent_element.calcd_node_params.insert(cur_work.element_ind);
                    // }
                } else {
                    match &cur_element.element_type {
                        ElementType::Stub { .. }|ElementType::Apply { .. }
                        |ElementType::ApplyUse{..}
                        |ElementType::TemplateDecl { .. }
                        // |ElementType::TemplateUse{ .. }
                        => {
                        }
                        // ElementType::TemplateUse{ template_decl_element_ind, .. }  => {
                        //     let decl_element=elements.get(*template_decl_element_ind).unwrap();
                        //     let params=decl_element.calcd_node_params.clone();

                        //     let parent_element=elements.get_mut(parent_element_ind).unwrap();
                        //     parent_element.calcd_node_params.extend(params);
                        // }
                        _ => {
                            let cur_nodes_params=cur_element.calcd_node_params.clone();
                            let parent_element=elements.get_mut(parent_element_ind).unwrap();
                            parent_element.calcd_node_params.extend(cur_nodes_params);
                        }
                    }
                }
                // match &cur_element.element_type {
                //     ElementType::Node { .. } => {
                //         let parent_element=elements.get_mut(parent_element_ind).unwrap();
                //         parent_element.calcd_node_params.insert(cur_work.element_ind);
                //     }
                //     ElementType::TemplateUse { .. } => {
                //     }
                //     ElementType::Apply { .. } => {
                //     }
                //     ElementType::Attrib { .. } => {
                //     }
                //     ElementType::Script { .. } => {
                //     }
                //     ElementType::TemplateDecl { .. } => {
                //     }
                //     ElementType::Stub { .. } => {
                //     }
                //     ElementType::ApplyUse { .. } => {
                //     }
                // }
            }
        }

        //debug
        if false {
            println!("=====");

            struct Work {
                element_ind:usize,
                depth:usize,
            }

            let mut work_stk=vec![Work{ element_ind: 0, depth: 0 }];

            while let Some(cur_work)=work_stk.pop() {
                let cur_element_ind=cur_work.element_ind;
                let cur_element=elements.get(cur_element_ind).unwrap();
                work_stk.extend(cur_element.children.iter().rev().map(|&child|Work { element_ind: child, depth: cur_work.depth+1 }));

                let indent="    ".repeat(cur_work.depth);
                let from_path=&cur_element.calcd_created_from;
                let params = &cur_element.calcd_node_params;

                match &cur_element.element_type {
                    ElementType::Node { names,ignore_applies,.. } => {
                        println!("{indent}node {names:?}, e={cur_element_ind}, ignore_applies={ignore_applies:?}, from={from_path:?}, params={params:?}", );
                    }
                    ElementType::TemplateUse { template_decl_element_ind, .. } => {
                        let ElementType::TemplateDecl { name, .. }=elements.get(*template_decl_element_ind).unwrap().element_type else {panic!("");};

                        println!("{indent}template use, e={cur_element_ind} : {name:?}, e2={template_decl_element_ind}, from={from_path:?}, params={params:?}",);
                    }
                    ElementType::Apply { name,.. } => {
                        println!("{indent}apply, e={cur_element_ind} : {name:?}, from={from_path:?}, params={params:?}",);
                    }
                    ElementType::Attrib { name,in_node,calcd, ..  } => {

                        // let s=if let Some(calc)=element_attrib_calcs.get(&cur_work.element_ind) {
                        //     let in_template=calc.in_template;
                        //     let in_apply=calc.in_apply;
                        //     let ok=calc.ok;
                        //     let used=calc.used;
                        //     format!("in_template_use={in_template:?}, in_apply_decl={in_apply:?}, ok={ok}, used={used}")
                        // } else {String::new()};


                        println!("{indent}attrib {name:?}, e={cur_element_ind}, in_node={in_node}, calcd={calcd:?}, from={from_path:?}, params={params:?}", );
                    }
                    ElementType::Script { .. } => {
                        println!("{indent}script, e={cur_element_ind}, from={from_path:?}, params={params:?}");
                    }
                    ElementType::TemplateDecl { name, .. } => {
                        // let name=texts[*name];
                        println!("{indent}template decl, e={cur_element_ind} : {name:?}, from={from_path:?}, params={params:?},",);
                    }
                    ElementType::Stub { name } => {
                        println!("{indent}stub {name:?}, e={cur_element_ind}, from={from_path:?}, params={params:?} , ");
                    }
                    ElementType::ApplyUse { apply_decl_element_ind,   } => {
                        // let ElementType::Apply { apply_decl_id, .. }=elements.get(*apply_decl_element_ind).unwrap().element_type else {panic!("");};
                        println!("{indent}apply use, e={cur_element_ind} : e2={apply_decl_element_ind:?}, from={from_path:?}, params={params:?}",);
                    }
                }
                // println!("{indent}={:?}",cur_element.calcd_node_params);
            }
        }

        //get attribs
        {

        }


        let mut all_stubs: HashMap<usize, Range<usize>> = HashMap::new(); //[root/stub_element_ind]=(nodes_start,nodes_end)
        let mut all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>=Vec::new(); //(element_ind,parent_ind,attribs_start,attribs_end)
        let mut all_attribs: Vec<AttribFunc>=Vec::new(); //[]=func
        let mut all_names: Vec<script_lang::StringT>=Vec::new();
        let mut all_names_map = HashSet::<script_lang::StringT>::new();
        //
        {

            #[derive(Clone)]
            struct Work {
                element_ind:usize,
                parent:Option<usize>,
                stub:Option<usize>,
            }

            let mut work_stk=vec![Work{ element_ind: 0, parent:None,stub:None,}];

            let mut creates:BTreeMap<usize,BTreeMap<usize,usize>>= BTreeMap::new(); //[root/stub][node]=parent
            let mut attribs:HashMap<usize,Vec<AttribFunc>> = HashMap::new(); //[element_ind]=attribs
            let mut element_ind_inds: HashMap<usize,usize>=HashMap::new(); //[element_ind]=ind;

            while let Some(cur_work)=work_stk.pop() {
                let cur_element=elements.get(cur_work.element_ind).unwrap();

                match &cur_element.element_type {
                    ElementType::Node{..}=> {
                        if let Some(parent)=cur_work.parent {
                            let stub=cur_work.stub.unwrap_or(0);
                            creates.entry(stub).or_default().insert(cur_work.element_ind,parent);
                        }

                        //
                        let ind=element_ind_inds.len();
                        element_ind_inds.insert(cur_work.element_ind,ind);
                    }
                    ElementType::Attrib{func, calcd,..}=> {
                        if calcd.used {
                            attribs.entry(cur_work.parent.unwrap()).or_default().push(func.clone());
                        }
                        // cur_element.
                    }
                    _=>{}
                }

                if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                    let stub=if let ElementType::Stub{..}=&cur_element.element_type {
                        Some(cur_work.element_ind)
                    } else {
                        cur_work.stub
                    };

                    let parent=if let ElementType::Node{..}=&cur_element.element_type {
                        if Some(cur_work.element_ind)==stub { //if parent is stub, then parent = 0
                            Some(0)
                        } else {
                            Some(cur_work.element_ind)
                        }
                    } else {
                        cur_work.parent
                    };

                    work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                        element_ind: child, parent, stub,
                    }));
                }
            }

            //
            //stubs[stub_element_ind]=(nodes_start,nodes_end)
            //nodes[ind]=(node_element_ind,parent_ind,attribs_start,attribs_end)
            //attribs[ind]=attrib_func
            // let mut element_ind_to_ind: HashMap<usize, usize>= HashMap::new(); //[element_ind]=ind
            // element_ind_to_ind.insert(0,0);

            // all_nodes.resize(element_ind_inds.len(), Default::default());

            for (&stub_element_ind,node_parents) in creates.iter() {
                let nodes_start=all_nodes.len();
                println!("{node_parents:?}");
                //
                for (&node_element_ind,&parent_element_ind) in node_parents.iter() {
                    //
                    let funcs=attribs.get(&node_element_ind);
                    let attribs_start=all_attribs.len();

                    if let Some(funcs)=funcs {
                        all_attribs.extend(funcs.iter().map(|x|x.clone()));
                    }

                    let attribs_end=all_attribs.len();

                    let node_element=elements.get(node_element_ind).unwrap();
                    let ElementType::Node { names, ..}=&node_element.element_type else {panic!("");};
                    let names_start=all_names.len();

                    for &n in names.iter() {
                        let mut x=script_lang::StringT::new(n);

                        if let Some(y)=all_names_map.get(&script_lang::StringT::new(n)).cloned() {
                            x=y;
                        } else {
                            all_names_map.insert(x.clone());
                        }

                        all_names.push(x);
                    }

                    let names_end=all_names.len();

                    //
                    // println!("pe is {parent_element_ind}, {:?}",element_ind_to_ind);
                    // let parent_ind=*element_ind_to_ind.get(&parent_element_ind).unwrap();
                    // let ind=element_ind_to_ind.len();
                    all_nodes.push((node_element_ind,parent_element_ind,attribs_start..attribs_end,names_start..names_end));
                    // element_ind_to_ind.insert(node_element_ind, ind);
                }

                //
                let nodes_end=all_nodes.len();
                all_stubs.insert(stub_element_ind, nodes_start..nodes_end);

            }
        }
        println!("all_stubs : {all_stubs:?}");
        println!("all_nodes : {all_nodes:?}");
        // println!("all_stubs : {all_stubs:?}");
        // let mut all_stubs: HashMap<usize, (usize,usize)> = HashMap::new(); //[root/stub_element_ind]=(nodes_start,nodes_end)
        // let mut all_nodes: Vec<(usize,usize,usize,usize)>=Vec::new(); //(element_ind,parent_ind,attribs_start,attribs_end)
        // let mut all_attribs=Vec::new(); //[]=func


        //script
        {

            #[derive(Clone)]
            struct Work {
                element_ind:usize,
                depth:usize,
                exit:bool,
                parent:Option<usize>,
                in_use:bool,
                inside:Option<usize>,
            }

            let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_use:false,inside:None}];
            struct ApplyCall {
                inside_element_ind:Option<usize>,
                parent_element_ind:usize,
                apply_use_element_ind:usize,
            }
            let mut apply_calls_stk: Vec<Vec<ApplyCall>> = Vec::new();//vec![Vec::new()]; //[[element_ind]] //(Option<usize>,usize,usize)
            // let mut node_ret_stk: Vec<Vec<(usize,usize)>> = Vec::new(); //()

            // let mut src_decls = String::new();
            let mut src = String::new();
            src+="var root;\n";
            src+="var _stubs;\n";

            while let Some(cur_work)=work_stk.pop() {

                // println!("cur element={}, depth={}, dir={}, call_stk={:?}",cur_work.element_ind,cur_work.depth,if cur_work.exit{"exit"}else{"enter"},apply_calls_stk);


                //
                let cur_element=elements.get(cur_work.element_ind).unwrap();

                if !cur_work.exit {
                    if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::TemplateDecl{..}|ElementType::Apply{..}
                        |ElementType::ApplyUse{..}|ElementType::TemplateUse{..}
                        =&cur_element.element_type
                    {
                        let in_use = if let ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                            // Some(cur_work.element_ind)
                            true
                        } else {
                            // cur_work.in_apply_use
                            cur_work.in_use
                        };

                        let inside = if let ElementType::Node{..}
                            |ElementType::ApplyUse{..}
                            |ElementType::TemplateUse{..}
                        =&cur_element.element_type {
                            Some(cur_work.element_ind)
                        } else {
                            // cur_work.in_apply_use
                            cur_work.inside
                        };
                        // calls_stk.push(Vec::new());
                        work_stk.push(Work{exit:true, ..cur_work.clone()});
                        work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                            element_ind: child, depth: cur_work.depth+1, exit:false,
                            parent:Some(cur_work.element_ind),
                            in_use,
                            inside,
                        }));
                    }
                }


                //
                if cur_work.exit && !cur_work.in_use {
                    if match &cur_element.element_type {
                        ElementType::Node{..} if cur_work.depth==0 => true,
                        ElementType::Stub{..} => true,
                        _ => false,
                    } {

                        let indent="    ".repeat(cur_work.depth);

                        // //
                        // src+=&format!("{indent}var _ns {{_c{}}};\n",cur_work.element_ind);


                        //
                        let mut v=apply_calls_stk.pop().unwrap();
                        v.sort_by(|x,y|{
                            let ElementType::ApplyUse {
                                //apply_decl_element_ind:a,
                                ..
                            }=&elements.get(x.apply_use_element_ind).unwrap().element_type else {panic!("");};

                            let ElementType::ApplyUse {
                                //apply_decl_element_ind:b,
                                ..
                            }=&elements.get(y.apply_use_element_ind).unwrap().element_type else {panic!("");};
                            // let r=a.cmp(b);
                            let r=x.apply_use_element_ind.cmp(&y.apply_use_element_ind);
                            if let std::cmp::Ordering::Equal=r{
                                x.inside_element_ind.cmp(&y.inside_element_ind)
                            } else {
                                r
                            }
                        });

                        for x in v.iter() {
                            let apply_use_element=elements.get(x.apply_use_element_ind).unwrap();
                            let ElementType::ApplyUse { apply_decl_element_ind,  }=&apply_use_element.element_type else {panic!("");};
                            // let apply_decl_element=elements.get(*apply_decl_element_ind).unwrap();
                            // let ElementType::Apply {is_root, .. }=&apply_decl_element.element_type else {panic!("");};
                            let params=[x.parent_element_ind].iter().chain(apply_use_element.calcd_node_params.iter())
                                .map(|&x|format!("_ns.{x}")).collect::<Vec<_>>().join(" ");

                            let mut pres=Vec::new();

                            let mut cur_from=apply_use_element.calcd_created_from;

                            while cur_from!=0 {
                                let from_element=elements.get(cur_from).unwrap();
                                match &from_element.element_type {
                                    ElementType::ApplyUse { .. } => {
                                        pres.push(format!("r{cur_from}."));
                                        break;
                                    }
                                    ElementType::TemplateUse { .. } => {
                                        pres.push(format!("t{cur_from}."));
                                    }

                                    ElementType::Node { .. } if from_element.calcd_created_from==0 => {
                                        pres.push(format!("r{cur_from}."));
                                    }
                                    _ => {}
                                }

                                // pres.push(cur_from);
                                cur_from=from_element.calcd_created_from;
                            }
                            pres.reverse();
                            let pre=pres.join("");

                            // let pre=if let Some(inside_element_ind)=x.inside_element_ind {


                            //     // let r=if let Some(from_apply_use_element_ind2)=from_apply_use_element_ind.clone() {
                            //     //     from_apply_use_element_ind2
                            //     // } else {
                            //     //     inside_element_ind //wrong name?, sometimes is node ret
                            //     // };

                            //     // format!("r{r}.({inside_element_ind},{from_apply_use_element_ind:?}).",)
                            //     String::new()
                            // } else{
                            //     String::new()
                            // };

                            let apply_use_element_ind=x.apply_use_element_ind;
                            src+=&format!("{indent}var _r{apply_use_element_ind} {{call _{pre}a{apply_decl_element_ind} {params}}};\n");
                            //{in_apply_use_element_ind:?} {parent_element_ind}
                        }
                    }
                }

                if let ElementType::ApplyUse{..} = &cur_element.element_type {
                    // let indent="    ".repeat(cur_work.depth-1);
                    // src+=&format!("{indent}_apply_use{} ns\n",cur_work.element_ind);
                    // calls_stk.get_mut(cur_work.depth-1).unwrap().push(cur_work.element_ind);
                    // calls_stk.last_mut().unwrap().push((*apply_decl_element_ind));
                    if !cur_work.exit {
                        //(cur_work.inside,cur_work.parent.unwrap(),cur_work.element_ind)
                        apply_calls_stk.last_mut().unwrap().push(ApplyCall {
                            inside_element_ind:cur_work.inside,
                            parent_element_ind:cur_work.parent.unwrap(),
                            apply_use_element_ind:cur_work.element_ind,
                        });
                    }
                } else if !cur_work.in_use {
                    if cur_work.exit && match &cur_element.element_type {
                        ElementType::Node{..} if cur_work.parent.is_some() => true,
                        ElementType::Apply{..} => true,
                        _=>false,
                    } {


                        let indent="    ".repeat(cur_work.depth);
                        let mut params=Vec::new();

                        // let mut tmp_stk=cur_element.children.iter().filter_map(|&child_element_ind|{
                        //     let child_element=elements.get(child_element_ind).unwrap();

                        //     if let ElementType::Node{..}|ElementType::TemplateUse{..}=&child_element.element_type {
                        //         Some(child_element.applies.iter().map(move|&a|(child_element_ind,a)))
                        //     } else {
                        //         None
                        //     }
                        // }).flatten().collect::<Vec<_>>();
                        // // if template use, store its dict eg t123:r123 instead of flattening it
                        // while let Some((element_ind,   ))=tmp_stk.pop() {

                        // }

                        for &child_element_ind in cur_element.children.iter() {
                            // let child_element=elements.get(child_element_ind).unwrap();
                            let mut tmp_stk=vec![child_element_ind];

                            while let Some(tmp_element_ind)=tmp_stk.pop() {
                                let tmp_element=elements.get(tmp_element_ind).unwrap();
                                match &tmp_element.element_type {
                                    ElementType::Node{..}=>{
                                        tmp_stk.extend(tmp_element.children.iter());

                                        for &apply_element_ind in tmp_element.applies.iter() {
                                            params.push(format!(":a{apply_element_ind} _r{child_element_ind}.a{apply_element_ind}"));
                                        }
                                    }
                                    ElementType::TemplateUse{..}=>{
                                        params.push(format!(":t{tmp_element_ind} _r{child_element_ind}")); //.t{tmp_element_ind}

                                    }
                                    _=>{}
                                }
                            }
                        }

                        //
                        // for &child_element_ind in cur_element.children.iter() {
                        //     let child_element=elements.get(child_element_ind).unwrap();

                        //     if let ElementType::Node{..}=&child_element.element_type {
                        //         for &apply_element_ind in child_element.applies.iter() {
                        //             params.push(format!(":a{apply_element_ind} _r{child_element_ind}.a{apply_element_ind}"));
                        //         }
                        //     }
                        // }

                        //

                        params.extend(cur_element.applies.iter().map(|&x|format!(":a{x} _a{x}")));
                        let applies_ret=params.join(" ");

                        let s=if params.is_empty(){""}else{" "};
                        src+=&format!("{indent}return {{dict{s}{applies_ret}}};\n");

                        // match &cur_element.element_type {
                        //     ElementType::Node{..} if cur_work.parent.is_some() => {
                        //         let indent="    ".repeat(cur_work.depth);

                        //         let mut params=Vec::new();
                        //         for &child_element_ind in cur_element.children.iter() {
                        //             let child_element=elements.get(child_element_ind).unwrap();

                        //             if let ElementType::Node{..}=&child_element.element_type {
                        //                 for &apply_element_ind in child_element.applies.iter() {
                        //                     params.push(format!(":a{apply_element_ind} _r{child_element_ind}.a{apply_element_ind}"));
                        //                 }
                        //             }
                        //         }

                        //         params.extend(cur_element.applies.iter().map(|&x|format!(":a{x} _a{x}")));
                        //         let applies_ret=params.join(" ");

                        //         let s=if params.is_empty(){""}else{" "};
                        //         src+=&format!("{indent}return {{dict{s}{applies_ret}}};\n");

                        //     }
                        //     ElementType::Apply{..} => {

                        //     }
                        //     _ => {}
                        // }
                    }


                    match &cur_element.element_type {
                        ElementType::Node{..} if cur_work.depth==0 && !cur_work.exit => { //enter
                            apply_calls_stk.push(Vec::new());
                            // node_ret_stk.push(Vec::new());
                        }
                        ElementType::Node{..} if cur_work.depth==0 => { //exit
                        }
                        ElementType::Node{..} if !cur_work.exit => { //enter
                            // calls_stk.get_mut(cur_work.depth-1).unwrap().push(cur_work.element_ind);

                            let indent="    ".repeat(cur_work.depth-1);
                            let params=cur_element.calcd_node_params.iter().map(|&x|format!("_p{x}")).collect::<Vec<_>>().join(" ");
                            let s=if cur_element.calcd_node_params.is_empty(){""}else{" "};

                            src+=&format!("{indent}fn _n{} {{self{s}{params}}} {{\n",cur_work.element_ind);
                            // src+=&format!("{indent}    var r {{dict}};\n",);
                        }
                        ElementType::Node{..} => { //exit
                            let indent="    ".repeat(cur_work.depth-1);
                            // src+=&format!("{indent}    return {{array}};\n");
                            // src+=&format!("{indent}    return r;\n");

                            src+=&format!("{indent}}}\n");

                            // src+=&format!("{indent}var _rn{} {{call _node{}}};\n", cur_work.element_ind, cur_work.element_ind);

                            {
                                let parent_element=elements.get(cur_work.parent.unwrap()).unwrap();
                                let params=[cur_work.element_ind].iter().chain(cur_element.calcd_node_params.iter()).map(|&x|{
                                    if cur_work.depth==1 {
                                        format!("_ns.{x}")
                                    } else if let ElementType::Stub { .. }=&parent_element.element_type {
                                        format!("_ns.{x}")
                                    } else {
                                        format!("_p{x}")
                                    }

                                }).collect::<Vec<_>>().join(" ");
                                src+=&format!("{indent}var _r{} {{call _n{} {params}}};\n", cur_work.element_ind, cur_work.element_ind,);
                            }
                        }
                        ElementType::Apply{..} if !cur_work.exit => { //enter
                            let indent="    ".repeat(cur_work.depth-1);
                            let params=cur_element.calcd_node_params.iter().map(|&x|format!("_p{x}")).collect::<Vec<_>>().join(" ");
                            let s=if cur_element.calcd_node_params.is_empty(){""}else{" "};
                            // let r= if cur_work.parent==Some(0){"_"}else{"_r."};
                            src+=&format!("{indent}fn _a{} {{self{s}{params}}} {{\n",cur_work.element_ind);
                            // src+=&format!("{indent}    var r {{dict}};\n",);
                        }
                        ElementType::Apply{..} => { //exit
                            let indent="    ".repeat(cur_work.depth-1);

                            src+=&format!("{indent}}}\n");
                            // src+=&format!("{indent}set _a{} _a;\n",cur_work.element_ind);
                        }
                        ElementType::TemplateDecl{..} if !cur_work.exit => { //enter
                            let indent="    ".repeat(cur_work.depth-1);
                            let params=cur_element.calcd_node_params.iter().map(|&x|format!("_p{x}")).collect::<Vec<_>>().join(" ");
                            let s=if cur_element.calcd_node_params.is_empty(){""}else{" "};

                            src+=&format!("{indent}fn _t{} {{self{s}{params}}} {{\n",cur_work.element_ind);
                        }
                        ElementType::TemplateDecl{..} => { //exit
                            let indent="    ".repeat(cur_work.depth-1);
                            let applies_ret=cur_element.applies.iter().map(|&x|format!(":a{x} _a{x}")).collect::<Vec<_>>().join(" ");
                            let s=if cur_element.applies.is_empty(){""}else{" "};
                            src+=&format!("{indent}    return {{dict{s}{applies_ret}}};\n");
                            src+=&format!("{indent}}}\n");
                        }
                        ElementType::TemplateUse{template_decl_element_ind,..} if !cur_work.exit => { //enter
                            // calls_stk.get_mut(cur_work.depth-1).unwrap().push(cur_work.element_ind);
                            let indent="    ".repeat(cur_work.depth-1);
                            // let element=elements.get(*template_decl_element_ind).unwrap();
                            let params=cur_element.calcd_node_params.iter().map(|&x|format!("_p{x}")).collect::<Vec<_>>().join(" ");
                            let s=if cur_element.calcd_node_params.is_empty(){""}else{" "};

                            src+=&format!("{indent}var _r{} {{call _t{template_decl_element_ind} self{s}{params}}};\n",cur_work.element_ind);
                            continue;
                        }
                        ElementType::TemplateUse{..} => { //exit
                        }
                        ElementType::Stub{name,..} if !cur_work.exit => { //enter
                            apply_calls_stk.push(Vec::new());

                            let indent="    ".repeat(cur_work.depth-1);
                            src+=&format!("{indent}fn stub_{name} {{parent}} {{ #{}\n",cur_work.element_ind);
                        }
                        ElementType::Stub{..} => { //exit
                            let indent="    ".repeat(cur_work.depth-1);

                            src+=&format!("{indent}}}\n");
                        }
                        ElementType::ApplyUse{..} if !cur_work.exit => { //enter
                            // let indent="    ".repeat(cur_work.depth-1);
                            // src+=&format!("{indent}_apply_use{} ns\n",cur_work.element_ind);
                            // calls_stk.get_mut(cur_work.depth-1).unwrap().push(cur_work.element_ind);
                            // calls_stk.last_mut().unwrap().push((*apply_decl_element_ind));
                            // calls_stk.last_mut().unwrap().push((cur_work.parent.unwrap(),cur_work.element_ind));
                            continue;
                        }
                        ElementType::ApplyUse{..} => { //exit
                        }
                        ElementType::Script { record  } if !cur_work.exit => { //enter
                            let indent="    ".repeat(cur_work.depth-1);
                            //src+=&format!("{indent}_script\n");
                            // if let Some(loc)=record.text_values().next().map(|x|x.start_loc())
                            if record.has_text()
                            {
                                if let Some(p)=record.path() {
                                    src+=&format!("{indent}#pragma_source {p:?}\n");
                                }

                                let loc=record.text_values().next().unwrap().start_loc();
                                src+=&format!("{indent}#pragma_loc {} {} {}\n",loc.pos,loc.row,loc.col);
                            }
                            for t in record.text_values() {
                                src+=&format!("{indent}{}\n",t.str());
                                // loc.
                            }

                            if record.has_text() {
                                src+=&format!("{indent}#pragma_clear_loc\n");

                                if record.path().is_some() {
                                    src+=&format!("{indent}#pragma_clear_source\n");
                                }
                            }

                            continue;
                        }
                        ElementType::Script { .. } => { //exit
                        }
                        ElementType::Attrib {..} => {continue;}
                        // _ => { continue; }
                    }

                    //

                    //
                    if !cur_work.exit //&& !cur_work.in_use
                    {
                        if match &cur_element.element_type {
                            ElementType::Node{..} if cur_work.depth==0 => true,
                            ElementType::Stub{..} => true,
                            _ => false,
                        } {

                            let indent="    ".repeat(cur_work.depth);

                            let p=if let ElementType::Stub{..}=&cur_element.element_type{"parent"}else{"root"};
                            //
                            src+=&format!("{indent}var _ns {{call _stubs {} {p}}};\n",cur_work.element_ind);
                        }
                    }
                }




            }

            // println!("===\n\n{src}\n====");

            let compiler=script_lang::langs::cexpr_compiler::Compiler::new();

            let build = compiler.compile(src.as_str(), 0, None, true, );
            // script_lang::cexpr_compiler::


            if let Err(e)=&build {
                eprintln!("{}",e.msg());
                continue;
            }


            let stuff = script_lang::Value::custom_unmanaged(Stuff{  all_stubs, all_nodes, all_attribs, all_names });

            let compiler=script_lang::cexpr_compiler::Compiler::new();
            let compile_result=compiler.compile(&src, 0,from_asset.handle.path().map(|x|x.path()),  true);

            if let Err(e)=compile_result {
                eprintln!("{}",e.msg());
                continue;
            }
            let build=compile_result.unwrap();


            commands.queue(move|world:&mut World|{
                let lib_scope = world.resource::<UixLibScope>().0.clone();
                let gc_scope = world.resource::<UixGcScope>().0.clone();
                let mut gc_scope=gc_scope.try_lock().unwrap();

                let mut var_scope = script_lang::VarScope::new();
                var_scope.decl("_stubs",Some(stuff)).unwrap();
                var_scope.decl("root",Some(script_lang::Value::custom_unmanaged(top_entity))).unwrap();

                {
                    let mut machine = script_lang::Machine::new(&mut gc_scope, &lib_scope, &mut var_scope,  world);
                    // machine.set_debug_print(true);
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

        //


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


    // mut input_event_reader: EventReader<UiInteractInputEvent>,
    mut interact_event_reader: EventReader<UiInteractEvent>,
    mut user_event_reader: EventReader<UixUserEvent>,
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
            let params= vec![script_lang::Value::float(time_elapsed)];
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
                    UiInteractEventType::HoverBegin { device } => vec![script_lang::Value::int(device)],
                    UiInteractEventType::HoverEnd { device } => vec![script_lang::Value::int(device)],
                    UiInteractEventType::PressBegin => vec![],
                    UiInteractEventType::PressEnd => vec![],
                    UiInteractEventType::Click => vec![],
                    UiInteractEventType::DragX { px, scale } => vec![script_lang::Value::float(px),script_lang::Value::float(scale)],
                    UiInteractEventType::DragY { px, scale } => vec![script_lang::Value::float(px),script_lang::Value::float(scale)],
                    UiInteractEventType::SelectBegin => vec![],
                    UiInteractEventType::SelectEnd => vec![],
                    UiInteractEventType::FocusBegin { group } => vec![script_lang::Value::int(group)],
                    UiInteractEventType::FocusEnd { group } => vec![script_lang::Value::int(group)],
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
                println!("a");
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