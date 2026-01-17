
use std::collections::{HashMap, HashSet};
// use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::color::Color;
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
use bevy::text::{Justify, TextColor, TextFont, TextLayout};
use conf_lang::RecordContainer;
use bevy_table_ui as table_ui;
use table_ui::*;

use super::super::script_vals::*;

use super::super::assets::*;

// use super::loading_vals::*;

use super::vals::*;

// fn get_attrib_default_func<C:Default,R>(f:Fn()->R) -> R {

// }

fn make_attrib_func<T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(func : impl Fn(&mut T)+Send+Sync+'static) -> AttribFuncType {
    Arc::new(move |entity:Entity,world: &mut World| {
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<T>().or_default();
        let mut c=c.get_mut();
        func(&mut c);
    })
}

fn make_attrib_func2<X:Component+Default,T:Component<Mutability = bevy::ecs::component::Mutable>+Default,>(func : impl Fn(&mut T)+Send+Sync+'static) -> AttribFuncType {
    Arc::new(move |entity:Entity,world: &mut World| {
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<T>().or_default();
        let mut c=c.get_mut();
        func(&mut c);

        e.entry::<X>().or_default();

    })
}

//only used for on_state attribs, that don't have a default attrib_setter, bit of a messy way of doing things though
//  could instead provide default attribs for all attribs for all nodes? wasteful?
//  could just have a hashmap of default attribs ...
fn make_attrib_default_func_alt<T,R>(func : impl Fn(&mut T)->&mut R+Send+Sync+'static) -> AttribFuncType
where
    T:Component<Mutability = bevy::ecs::component::Mutable>+Default,
    R:Default+Clone,

{
    make_attrib_func::<T>(move|c|{
        let d= func(&mut T::default()).clone();
        let v=func(c);
        *v=d;
    })
}
// trait AttribDefaultFunc<T:Component<Mutability = bevy::ecs::component::Mutable>+Default,R:Default+Clone>=Fn(&mut T)->&mut R+Send+Sync+'static

// ;
// fn make_attrib_default_func_alt<T,R,F>(func : F) -> AttribFuncType
// where
//     T:Component<Mutability = bevy::ecs::component::Mutable>+Default,
//     R:Default+Clone,
//     F: Fn(&mut T)->&mut R+Send+Sync+'static,
// {
//     make_attrib_func::<T>(move|c|{
//         let d= func(&mut T::default()).clone();
//         let v=func(c);
//         *v=d;
//     })
// }
// fn make_attrib_default_func_altx<T,R:Default+Clone>(func : impl Fn(&mut T)->&mut (impl Component)+Send+Sync+'static) -> AttribFuncType
// where
//     T:Component<Mutability = bevy::ecs::component::Mutable>+Default,

// {
//     make_attrib_default_func_alt::<T,_,_>(func)
// }
fn make_attrib_default_func_alt2<X,T,R>(func : impl Fn(&mut T)->&mut R+Send+Sync+'static) -> AttribFuncType
where
    X:Component+Default,
    T:Component<Mutability = bevy::ecs::component::Mutable>+Default,
    R:Default+Clone,
{
    make_attrib_func2::<X,T>(move|c|{
        let d= func(&mut T::default()).clone();
        let v=func(c);
        *v=d;
    })
}
fn make_attrib_default_func<T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(func : impl Fn(&mut T,T)+Send+Sync+'static) -> AttribFuncType {
    make_attrib_func::<T>(move|c|func(c,T::default()))
}
fn make_attrib_default_func2<X:Component+Default,T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(func : impl Fn(&mut T,T)+Send+Sync+'static) -> AttribFuncType {
    make_attrib_func2::<X,T>(move|c|func(c,T::default()))
}

pub fn load_elements<'a>(
    ui_assets: &'a Assets<UiAsset>,
    asset_server: &AssetServer,
    asset:&'a UiAsset,
    // get_asset:impl FnMut(&str) ->&UiAsset,
) -> Option<Vec<Element<'a>>>{
    let mut elements: Vec<Element> = vec![Element{
        element_type:ElementType::Node{names:HashSet::new(),ignore_applies:HashSet::new(),},
        children:Vec::new(),applies:Vec::new(),
        apply_after:0,calcd_from_element_ind:None,
        calcd_node_params:Vec::new(),
        calcd_env_params: Vec::new(),
        calcd_created_from:0,
        has_script:false,
        //has_apply_decl_script:false,
        has_self_script:false,
        has_env_script:false,
        has_apply_script:false,
        // has_template_use_script:false,
        // calcd_original:None,
        // env: HashMap::new(),
        parent:None,
        rets : Vec::new(),
        apply_calls:Vec::new(),
    }];

    //elements

    let mut template_envs: Vec<HashMap<&str,(usize,RecordContainer)>> = vec![HashMap::new()]; //[template_env_ind][template_name]=element_ind
    let mut element_script_records = HashMap::<usize,RecordContainer>::new(); //[src_element_ind]=script_record

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

                // let include_asset=get_asset(include_path);

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
                    element_type: ElementType::TemplateDecl { name: template_name, used: true },
                    children: Vec::new(),
                    applies: Vec::new(),
                    apply_after,
                    calcd_from_element_ind: None,
                    calcd_node_params:Vec::new(),
                    calcd_env_params: Vec::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    has_apply_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    // env: HashMap::new(),
                    parent:Some(cur_element_ind),
                    rets : Vec::new(),
                    apply_calls:Vec::new(),
                });

                //
                // cur_element_ind=new_element_ind;
                last_element_stk.push(new_element_ind);
            }
            "template" if walk.record().node_label() == Some("template_decl") && walk.is_exit() => {
                // cur_element_ind=elements.get(cur_element_ind).unwrap().parent.unwrap();
                last_element_stk.pop().unwrap();
            }

            "template" if walk.record().node_label() == Some("template_use") && walk.is_enter() => {

                walk.do_exit(); //for applies declared in template_use
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
                    element_type: ElementType::TemplateUse { template_decl_element_ind, } ,
                    children: Vec::new(),
                    applies: Vec::new(),
                    apply_after,
                    calcd_from_element_ind: None,
                    calcd_node_params:Vec::new(),
                    calcd_env_params: Vec::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    has_apply_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    // env: HashMap::new(),
                    parent:Some(cur_element_ind),
                    rets : Vec::new(),
                    apply_calls:Vec::new(),
                });

                //
                // template_use_count+=1;


                last_element_stk.push(new_element_ind); //for applies declared in template_use
            }

            "template" if walk.record().node_label() == Some("template_use") && walk.is_exit() => {
                last_element_stk.pop().unwrap(); //for applies declared in template_use
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
                        element_type: ElementType::Apply { name, owner_apply_decl_id: prev_owner_apply_decl_id, used: true },
                        apply_after,
                        children: Vec::new(),
                        applies: Vec::new(),
                        calcd_from_element_ind: None,
                        calcd_node_params:Vec::new(),
                        calcd_env_params: Vec::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        //has_apply_decl_script:false,
                        has_self_script:false,
                        has_env_script:false,
                        has_apply_script:false,
                        // has_template_use_script:false,
                        // calcd_original:None,
                        // env: HashMap::new(),
                        parent:Some(cur_element_ind),
                        rets : Vec::new(),
                        apply_calls:Vec::new(),
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
                    element_type: ElementType::Node { names,  ignore_applies:HashSet::new(), },
                    children: Vec::new(),
                    applies: Vec::new(),
                    apply_after,
                    calcd_from_element_ind: None,
                    calcd_node_params:Vec::new(),
                    calcd_env_params: Vec::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    has_apply_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    // env: HashMap::new(),
                    parent:Some(cur_element_ind),
                    rets : Vec::new(),
                    apply_calls:Vec::new(),
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
                    element_type: ElementType::Script { record : walk.record(), },
                    children: Vec::new(),
                    applies: Vec::new(),
                    apply_after,
                    calcd_from_element_ind: None,
                    calcd_node_params:Vec::new(),
                    calcd_env_params: Vec::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    has_apply_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    // env: HashMap::new(),
                    parent:Some(cur_element_ind),
                    rets : Vec::new(),
                    apply_calls:Vec::new(),
                });
            }

            "on" => {
                walk.do_exit();
                let affect_state=walk.record().value(0).get_parsed::<UiAffectState>().unwrap();
                let priority=walk.record().value(1).get_parsed::<i32>();

                walk.set_named_note("on",affect_state);
                walk.set_named_note("priority",priority);
            }

            "stub" if walk.is_enter() => {
                walk.do_exit();

                // walk.set_named_note("in_node", false); // not necessary? since attribs can't be used inside stub

                let name = walk.record().value(0).get_str().unwrap();

                let new_element_ind=elements.len();
                elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);

                elements.push(Element {
                    element_type: ElementType::Stub { name, },
                    children: Vec::new(),
                    applies: Vec::new(),
                    apply_after,
                    calcd_from_element_ind: None,
                    calcd_node_params:Vec::new(),
                    calcd_env_params: Vec::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    has_apply_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    // env: HashMap::new(),
                    parent:Some(cur_element_ind),
                    rets : Vec::new(),
                    apply_calls:Vec::new(),
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
                let on_priority:Option<i32>=walk.find_named_note("priority").cloned();

                let in_node: bool=walk.find_named_note("in_node").cloned().unwrap_or_default();
                // let in_apply: Option<usize>=walk.get_named_note("in_apply").cloned();
                let in_template: Option<usize>=walk.find_named_note("in_template").cloned();

                //
                let mut attrib_funcs: Vec<(&str, Arc<dyn Fn(Entity, &mut World) + Send + Sync>)> = Vec::new();

                //
                do_attribs(x,
                    // on_state,
                    asset_server,&walk,&mut attrib_funcs);

                //
                for (attrib_name,func) in attrib_funcs {
                    let new_element_ind=elements.len();
                    elements.get_mut(cur_element_ind).unwrap().children.push(new_element_ind);
                    elements.push(Element {
                        element_type: ElementType::Attrib {
                            name:attrib_name,
                            on_state,
                            on_priority,
                            in_template,
                            func:AttribFunc(func),
                            in_node,
                            calcd:Default::default(),
                        },
                        children: Vec::new(),
                        applies: Vec::new(),
                        apply_after,
                        calcd_from_element_ind: None,
                        calcd_node_params:Vec::new(),
                        calcd_env_params: Vec::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        //has_apply_decl_script:false,
                        has_self_script:false,
                        has_env_script:false,
                        has_apply_script:false,
                        // has_template_use_script:false,
                        // calcd_original:None,
                        // env: HashMap::new(),
                        parent:Some(cur_element_ind),
                        rets : Vec::new(),
                        apply_calls:Vec::new(),
                    });
                }
            }
        }

        Ok(())
    }) {
        eprintln!("{}",e.msg(None));
        return None;
    }

    Some(elements)
}

fn do_attribs<'a>(
    x:&'a str,
    // on_state:Option<UiAffectState>,
    asset_server: &AssetServer,
    walk : &conf_lang::Walk,
    attrib_funcs: &mut Vec<(&'a str, AttribFuncType)>, //
) {
    match x {
        "size" => {
            let w = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            let h = walk.record().value(1).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("width",make_attrib_func::<UiSize>(move|c|{c.width=w;})));
            attrib_funcs.push(("height",make_attrib_func::<UiSize>(move|c|{c.height=h;})));
        }
        "width" => {
            let width = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiSize>(move|c|{c.width=width;})));
        }
        "height" => {
            let height = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiSize>(move|c|{c.height=height;})));
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
                "colorh" => {
                    walk.record().value(0).get_parsed::<HexColor>().unwrap().0
                }
                "colorn" => {
                    walk.record().value(0).get_parsed::<NamedColor>().unwrap().0
                }
                _ => {panic!("");}
            };

            match tag {
                "color" => {
                    attrib_funcs.push((x,make_attrib_func::<UiColor>(move|c|{c.back=color;})));
                }
                "border_color" => {
                    attrib_funcs.push((x,make_attrib_func::<UiColor>(move|c|{c.border=color;})));
                }
                "padding_color" => {
                    attrib_funcs.push((x,make_attrib_func::<UiColor>(move|c|{c.padding=color;})));
                }
                "margin_color" => {
                    attrib_funcs.push((x,make_attrib_func::<UiColor>(move|c|{c.margin=color;})));
                }
                "cell_color" => {
                    attrib_funcs.push((x,make_attrib_func::<UiColor>(move|c|{c.cell=color;})));
                }
                "text_color" => {
                    // attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{c.color=color;})));
                    attrib_funcs.push((x,make_attrib_func2::<UiText,TextColor,>(move|c|{c.0=color;})));
                }
                _=>{panic!("");}
            }
        }
        "hoverable" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiCursorable>(move|c|{c.hoverable=v;})));
        }
        "pressable" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            // // let v : bool = walk.record().value(0).parsed_or(true);
            // // attrib_funcs.push((x,make_attrib_func::<(UiCursorable,UiFocusable)>(move|c|{
            // //     c.0.pressable=v;
            // //     c.1.pressable=v;
            // // })));
            // // attrib_funcs.push((x,make_attrib_func::<UiCursorable>(move|c|{c.pressable=v;})));
            // let f1=make_attrib_func::<UiCursorable>(move|c|{c.pressable=v;});
            // let f2=make_attrib_func::<UiFocusable>(move|c|{c.pressable=v;});

            // attrib_funcs.push((x,Arc::new(move|entity:Entity, world:&mut World|{
            //     f1(entity,world);
            //     f2(entity,world);
            // }) ));

            attrib_funcs.push((x,Arc::new(move|entity,world|{
                let mut e=world.entity_mut(entity);
                e.entry::<UiCursorable>().or_default().get_mut().pressable=v;
                e.entry::<UiFocusable>().or_default().get_mut().pressable=v;
            }) ));

        }
        "draggable" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiCursorable>(move|c|{c.draggable=v;})));
        }
        "selectable" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{c.enable=v;})));
        }
        "focusable" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{ c.enable=v; })));
        }

        // "press_always"=> {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiPressable>(move|c|{c.always=v;})));
        // }
        // "press_physical" => {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiPressable>(move|c|{c.physical=v;})));
        // }

        // "focused" => {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.focused=v;})));
        // }
        "focus_group" => {
            let v: i32 = walk.record().value(0).get_parsed().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.group=v;})));
        }
        // "focus_tab_exit" => {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.tab_exit=v;})));
        // }
        "focus_hexit" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.hexit=v;})));
        }
        "focus_vexit" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.vexit=v;})));
        }
        "focus_hwrap" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.hwrap=v;})));
        }
        "focus_vwrap" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.vwrap=v;})));
        }
        // "focus_hdir_press" => {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.hdir_press=v;})));
        // }
        // "focus_vdir_press" => {
        //     let v : bool = walk.record().value(0).parsed_or(true);
        //     attrib_funcs.push((x,make_attrib_func::<UiFocusable>(move|c|{c.vdir_press=v;})));
        // }

        "selected" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{c.selected=v;})));
        }
        "select_group" => {
            let v = walk.record().value(0).get_str().unwrap().to_string();
            attrib_funcs.push((x,make_attrib_func::<UiSelectable>(move|c|{c.group=v.clone();}))); //can't move from func's capture to c.group
        }

        "border" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

            attrib_funcs.push(("border_left",make_attrib_func::<UiEdge>(move|c|{c.border.left=v;})));
            attrib_funcs.push(("border_right",make_attrib_func::<UiEdge>(move|c|{c.border.right=v;})));
            attrib_funcs.push(("border_top",make_attrib_func::<UiEdge>(move|c|{c.border.top=v2;})));
            attrib_funcs.push(("border_bottom",make_attrib_func::<UiEdge>(move|c|{c.border.bottom=v2;})));
        }
        "hborder" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("border_left",make_attrib_func::<UiEdge>(move|c|{c.border.left=v;})));
            attrib_funcs.push(("border_right",make_attrib_func::<UiEdge>(move|c|{c.border.right=v;})));
        }
        "vborder" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("border_top",make_attrib_func::<UiEdge>(move|c|{c.border.top=v;})));
            attrib_funcs.push(("border_bottom",make_attrib_func::<UiEdge>(move|c|{c.border.bottom=v;})));
        }
        "border_left" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.border.left=v;})));
        }
        "border_right" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.border.right=v;})));
        }
        "border_top" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.border.top=v;})));
        }
        "border_bottom" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.border.bottom=v;})));
        }

        "padding" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

            attrib_funcs.push(("padding_left",make_attrib_func::<UiEdge>(move|c|{c.padding.left=v;})));
            attrib_funcs.push(("padding_right",make_attrib_func::<UiEdge>(move|c|{c.padding.right=v;})));
            attrib_funcs.push(("padding_top",make_attrib_func::<UiEdge>(move|c|{c.padding.top=v2;})));
            attrib_funcs.push(("padding_bottom",make_attrib_func::<UiEdge>(move|c|{c.padding.bottom=v2;})));
        }
        "hpadding" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("padding_left",make_attrib_func::<UiEdge>(move|c|{c.padding.left=v;})));
            attrib_funcs.push(("padding_right",make_attrib_func::<UiEdge>(move|c|{c.padding.right=v;})));
        }
        "vpadding" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("padding_top",make_attrib_func::<UiEdge>(move|c|{c.padding.top=v;})));
            attrib_funcs.push(("padding_bottom",make_attrib_func::<UiEdge>(move|c|{c.padding.bottom=v;})));
        }
        "padding_left" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.padding.left=v;})));
        }
        "padding_right" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.padding.right=v;})));
        }
        "padding_top" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.padding.top=v;})));
        }
        "padding_bottom" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.padding.bottom=v;})));
        }

        "margin" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

            attrib_funcs.push(("margin_left",make_attrib_func::<UiEdge>(move|c|{c.margin.left=v;})));
            attrib_funcs.push(("margin_right",make_attrib_func::<UiEdge>(move|c|{c.margin.right=v;})));
            attrib_funcs.push(("margin_top",make_attrib_func::<UiEdge>(move|c|{c.margin.top=v2;})));
            attrib_funcs.push(("margin_bottom",make_attrib_func::<UiEdge>(move|c|{c.margin.bottom=v2;})));
        }
        "hmargin" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("margin_left",make_attrib_func::<UiEdge>(move|c|{c.margin.left=v;})));
            attrib_funcs.push(("margin_right",make_attrib_func::<UiEdge>(move|c|{c.margin.right=v;})));
        }
        "vmargin" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push(("margin_top",make_attrib_func::<UiEdge>(move|c|{c.margin.top=v;})));
            attrib_funcs.push(("margin_bottom",make_attrib_func::<UiEdge>(move|c|{c.margin.bottom=v;})));
        }
        "margin_left" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.margin.left=v;})));
        }
        "margin_right" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.margin.right=v;})));
        }
        "margin_top" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.margin.top=v;})));
        }
        "margin_bottom" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{c.margin.bottom=v;})));
        }

        "font" => {
            let v = walk.record().value(0).get_str().unwrap();
            let handle=asset_server.load(PathBuf::from(v)).clone();

            // attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{c.font=handle.clone();c.update=true;})));
            attrib_funcs.push((x,make_attrib_func2::<UiText,TextFont>(move|c|c.font=handle.clone())));

            // attrib_funcs.push(("inner_size",make_attrib_func::<UiInnerSize>(move|_|{})));
            // attrib_funcs.push(("text_computed",make_attrib_func::<UiTextComputed>(move|_|{})));

            // commands.add(move |world: &mut World| {
            //     let mut e=world.entity_mut(parent_entity);
            //     e.entry::<UiTextComputed>().or_default();
            //     e.entry::<UiInnerSize>().or_default();
            // });
        }
        "text" => {
            let v = walk.record().value(0).get_str().unwrap().to_string(); //v can't move from func's capture

            let v=if v.contains("\\") {
                let cs=v.chars().collect::<Vec<_>>();
                let mut out=String::new();
                let mut i=0;

                while i<cs.len() {
                    if cs[i]=='\\' {
                        let x=match cs[i+1] {
                            '\r' if cs.get(i+2).cloned()==Some('\n') => (vec![' '],2),
                            '\r' => (vec![],1),
                            '\n' => (vec![' '],1),
                            'n' => (vec!['\n'],1),
                            'r' => (vec!['\r'],1),
                            's' => (vec![' '],1),
                            't' => (vec!['\t'],1),
                            '\\' => (vec!['\\'],1),
                            _ => (cs[i..=i+1].to_vec(),1),
                        };

                        out.extend(x.0);
                        i+=x.1+1;
                    } else {
                        out.push(cs[i]);
                        i+=1;
                    }
                }
                out
            } else {
                v
            };

            attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|c.0=v.clone())));
        }
        "text_halign" => {
            let v: Justify = walk.record().value(0).get_parsed().unwrap();
            attrib_funcs.push((x,make_attrib_func2::<UiText,TextLayout>(move|c|c.justify=v)));
        }
        "text_valign" => {
            let v: UiTextVAlign = walk.record().value(0).get_parsed().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiTextVAlign>(move|c|*c=v)));
        }
        "text_size" => {
            let v = walk.record().value(0).get_parsed::<f32>().unwrap().abs();
            attrib_funcs.push((x,make_attrib_func2::<UiText,TextFont>(move|c|{c.font_size=v;})));
        }
        // "text_hlen" => {
        //     let v: u32 = walk.record().value(0).get_parsed().unwrap();
        //     attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{c.hlen=v;c.update=true;})));
        // }
        // "text_vlen" => {
        //     let v: u32 = walk.record().value(0).get_parsed().unwrap();
        //     attrib_funcs.push((x,make_attrib_func::<UiText>(move|c|{c.vlen=v;c.update=true;})));
        // }

        "image" => {
            let v = walk.record().value(0).get_str().unwrap();
            let handle=asset_server.load(PathBuf::from(v)); //can't move from func's capture

            attrib_funcs.push((x,make_attrib_func::<UiImage>(move|c|{c.handle=handle.clone();})));

            // attrib_funcs.push(("inner_size",make_attrib_func::<UiInnerSize>(move|_|{})));
            // commands.add(move |world: &mut World| {
            //     let mut e=world.entity_mut(parent_entity);
            //     e.entry::<UiInnerSize>().or_default();
            // });
        }

        "disabled" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiDisable>(move|c|{c.disable=v;})));
        }
        "hidden" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiHide>(move|c|{c.hide=v;})));
        }
        "floating" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiFloat>(move|c|{c.float=v;})));
        }
        "locked" => {
            let v : bool = walk.record().value(0).parsed_or(true);
            attrib_funcs.push((x,make_attrib_func::<UiLock>(move|c|{c.lock=v;})));
        }

        "gap" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

            attrib_funcs.push(("hgap",make_attrib_func::<UiGap>(move|c|{c.hgap=v;})));
            attrib_funcs.push(("vgap",make_attrib_func::<UiGap>(move|c|{c.vgap=v2;})));
        }
        "hgap" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiGap>(move|c|{c.hgap=v;})));
        }
        "vgap" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiGap>(move|c|{c.vgap=v;})));
        }

        "hexpand" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiExpand>(move|c|{c.hexpand=v;})));
        }
        "vexpand" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiExpand>(move|c|{c.vexpand=v;})));
        }

        "hfill" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiFill>(move|c|{c.hfill=v;})));
        }
        "vfill" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiFill>(move|c|{c.vfill=v;})));
        }

        "row_width_scale" => {
            let v = walk.record().value(0).get_parsed::<f32>().unwrap().max(0.0);
            attrib_funcs.push((x,make_attrib_func::<UiCongruent>(move|c|{c.row_width_scale=v;})));
        }
        "col_height_scale" => {
            let v = walk.record().value(0).get_parsed::<f32>().unwrap().max(0.0);
            attrib_funcs.push((x,make_attrib_func::<UiCongruent>(move|c|{c.col_height_scale=v;})));
        }

        "halign" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiAlign>(move|c|{c.halign=v;})));
        }
        "valign" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiAlign>(move|c|{c.valign=v;})));
        }

        "span" => {
            let v = walk.record().value(0).get_parsed::<u32>().unwrap();
            attrib_funcs.push((x,make_attrib_func::<UiSpan>(move|c|{c.span=v;})));
        }

        // "hscroll" => {}
        // "vscroll" => {}
        x => {
            panic!("{x:?}",)
        }
    }
}

pub fn get_default_attribs<'a>() -> HashMap<&'a str,AttribFuncType> {
    let mut output = HashMap::new();

    output.insert("width", make_attrib_default_func::<UiSize>(|c,d|{c.width=d.width;}));
    output.insert("height", make_attrib_default_func::<UiSize>(|c,d|{c.height=d.height;}));
    output.insert("color", make_attrib_default_func::<UiColor>(|c,d|{c.back=d.back;}));
    output.insert("border_color", make_attrib_default_func::<UiColor>(|c,d|{c.border=d.border;}));
    output.insert("padding_color", make_attrib_default_func::<UiColor>(|c,d|{c.padding=d.padding;}));
    output.insert("margin_color", make_attrib_default_func::<UiColor>(|c,d|{c.margin=d.margin;}));
    output.insert("cell_color", make_attrib_default_func::<UiColor>(|c,d|{c.cell=d.cell;}));
    output.insert("text_color", make_attrib_default_func2::<UiText,TextColor>(|c,d|c.0=d.0));
    output.insert("hoverable", make_attrib_default_func::<UiCursorable>(|c,d|{c.hoverable=d.hoverable;}));
    // output.insert("pressable", make_attrib_default_func::<UiPressable>(|c,d|{c.enable=d.enable;}));
    output.insert("pressable", Arc::new(|entity,world|{
        let mut e=world.entity_mut(entity);
        e.entry::<UiCursorable>().or_default().get_mut().pressable=false;
        e.entry::<UiFocusable>().or_default().get_mut().pressable=false;
    }));
    output.insert("draggable", make_attrib_default_func::<UiCursorable>(|c,d|{c.draggable=d.draggable;}));
    output.insert("selectable", make_attrib_default_func::<UiSelectable>(|c,d|{c.enable=d.enable;}));
    output.insert("focusable", make_attrib_default_func::<UiFocusable>(|c,d|{c.enable=d.enable;}));
    // output.insert("press_always", make_attrib_default_func::<UiPressable>(|c,d|{c.always=d.always;}));
    // output.insert("press_physical", make_attrib_default_func::<UiPressable>(|c,d|{c.physical=d.physical;}));
    // output.insert("focused", make_attrib_default_func::<UiFocusable>(|c,d|{c.focused=d.focused;}));
    output.insert("focus_group", make_attrib_default_func::<UiFocusable>(|c,d|{c.group=d.group;}));
    // output.insert("focus_tab_exit", make_attrib_default_func::<UiFocusable>(|c,d|{c.tab_exit=d.tab_exit;}));
    output.insert("focus_hexit", make_attrib_default_func::<UiFocusable>(|c,d|{c.hexit=d.hexit;}));
    output.insert("focus_vexit", make_attrib_default_func::<UiFocusable>(|c,d|{c.vexit=d.vexit;}));
    output.insert("focus_hwrap", make_attrib_default_func::<UiFocusable>(|c,d|{c.hwrap=d.hwrap;}));
    output.insert("focus_vwrap", make_attrib_default_func::<UiFocusable>(|c,d|{c.vwrap=d.vwrap;}));
    // output.insert("focus_hdir_press", make_attrib_default_func::<UiFocusable>(|c,d|{c.hdir_press=d.hdir_press;}));
    // output.insert("focus_vdir_press", make_attrib_default_func::<UiFocusable>(|c,d|{c.vdir_press=d.vdir_press;}));
    output.insert("selected", make_attrib_default_func::<UiSelectable>(|c,d|{c.selected=d.selected;}));
    output.insert("select_group", make_attrib_default_func::<UiSelectable>(|c,d|{c.group=d.group;}));

    output.insert("padding_left", make_attrib_default_func::<UiEdge>(|c,d|{c.padding.left=d.padding.left;}));
    output.insert("padding_right", make_attrib_default_func::<UiEdge>(|c,d|{c.padding.right=d.padding.right;}));
    output.insert("padding_top", make_attrib_default_func::<UiEdge>(|c,d|{c.padding.top=d.padding.top;}));
    output.insert("padding_bottom", make_attrib_default_func::<UiEdge>(|c,d|{c.padding.bottom=d.padding.bottom;}));

    output.insert("border_left", make_attrib_default_func::<UiEdge>(|c,d|{c.border.left=d.border.left;}));
    output.insert("border_right", make_attrib_default_func::<UiEdge>(|c,d|{c.border.right=d.border.right;}));
    output.insert("border_top", make_attrib_default_func::<UiEdge>(|c,d|{c.border.top=d.border.top;}));
    output.insert("border_bottom", make_attrib_default_func::<UiEdge>(|c,d|{c.border.bottom=d.border.bottom;}));

    output.insert("margin_left", make_attrib_default_func::<UiEdge>(|c,d|{c.margin.left=d.margin.left;}));
    output.insert("margin_right", make_attrib_default_func::<UiEdge>(|c,d|{c.margin.right=d.margin.right;}));
    output.insert("margin_top", make_attrib_default_func::<UiEdge>(|c,d|{c.margin.top=d.margin.top;}));
    output.insert("margin_bottom", make_attrib_default_func::<UiEdge>(|c,d|{c.margin.bottom=d.margin.bottom;}));

    output.insert("font", make_attrib_default_func2::<UiText,TextFont>(|c,d|c.font=d.font));
    output.insert("text", make_attrib_default_func::<UiText>(|c,d|c.0=d.0));
    output.insert("text_halign", make_attrib_default_func::<TextLayout>(|c,d|c.justify=d.justify));
    output.insert("text_valign", make_attrib_default_func::<UiTextVAlign>(|c,d|*c=d));
    output.insert("text_size", make_attrib_default_func2::<UiText,TextFont>(|c,d|c.font_size=d.font_size));
    // output.insert("text_hlen", make_attrib_default_func::<UiText>(|c,d|{c.hlen=d.hlen;c.update=true;}));
    // output.insert("text_vlen", make_attrib_default_func::<UiText>(|c,d|{c.vlen=d.vlen;c.update=true;}));


    output.insert("image", make_attrib_default_func::<UiImage>(|c,d|{c.handle=d.handle;}));
    output.insert("disabled", make_attrib_default_func::<UiDisable>(|c,d|{c.disable=d.disable;}));
    output.insert("hidden", make_attrib_default_func::<UiHide>(|c,d|{c.hide=d.hide;}));


    output.insert("floating", make_attrib_default_func::<UiFloat>(|c,d|{c.float=d.float;}));
    output.insert("locked", make_attrib_default_func::<UiLock>(|c,d|{c.lock=d.lock;}));

    output.insert("hgap", make_attrib_default_func::<UiGap>(|c,d|{c.hgap=d.hgap;}));
    output.insert("vgap", make_attrib_default_func::<UiGap>(|c,d|{c.vgap=d.vgap;}));


    output.insert("hexpand", make_attrib_default_func::<UiExpand>(|c,d|{c.hexpand=d.hexpand;}));
    output.insert("vexpand", make_attrib_default_func::<UiExpand>(|c,d|{c.vexpand=d.vexpand;}));
    output.insert("hfill", make_attrib_default_func::<UiFill>(|c,d|{c.hfill=d.hfill;}));
    output.insert("vfill", make_attrib_default_func::<UiFill>(|c,d|{c.vfill=d.vfill;}));

    output.insert("row_width_scale", make_attrib_default_func::<UiCongruent>(|c,d|{c.row_width_scale=d.row_width_scale;}));
    output.insert("col_height_scale", make_attrib_default_func::<UiCongruent>(|c,d|{c.col_height_scale=d.col_height_scale;}));

    output.insert("halign", make_attrib_default_func::<UiAlign>(|c,d|{c.halign=d.halign;}));
    output.insert("valign", make_attrib_default_func::<UiAlign>(|c,d|{c.valign=d.valign;}));

    output.insert("span", make_attrib_default_func::<UiSpan>(|c,d|{c.span=d.span;}));
    output.insert("span", make_attrib_default_func_alt::<UiSpan,_>(|c|{&mut c.span}));
    output.insert("span", make_attrib_default_func_alt(|c: &mut UiSpan|{&mut c.span}));

    output
}