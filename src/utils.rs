/*
* allow stubs' nodes to be added to anyone eg stub_a some_node
* add env to store named nodes
    eg
    env.abc.0
    env.0 ??
*/

// #![allow(unused_mut)]
// #![allow(unused_variables)]
#![allow(dead_code)]
use std::collections::{BTreeMap, BTreeSet};
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #[allow(unused_parens)]
use std::fmt::Debug;
use core::panic;
use std::cmp::Ordering;
// use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Display;
// use std::fmt::format;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::color::Color;
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use conf_lang::RecordContainer;
use bevy_table_ui as table_ui;
// use ron::de;
use table_ui::*;
use super::script_stuff::{AttribFunc, Stuff};


use super::assets::*;


//
#[derive(Debug,Clone,Default)]
pub struct ElementAttribCalc {
    in_template : Option<usize>, //template_use_id
    in_apply : Option<usize>, //apply_decl_id
    used:bool,
    ok:bool,
}

#[derive(Debug,Clone)]
pub enum ElementType<'a> {
    // Root,
    Node {
        names : HashSet<&'a str>,
        ignore_applies : HashSet<usize>, //apply_decl_id
    },
    Attrib {
        name : &'a str,
        on_state: Option<UiAffectState>,
        in_template: Option<usize>,
        func : AttribFunc,
        in_node : bool,
        calcd:ElementAttribCalc,
    },
    Script {
        record : RecordContainer<'a>,
    },
    Apply {
        name : &'a str, //text_ind
        owner_apply_decl_id : Option<usize>, //element_ind
        used:bool,
    },
    ApplyUse {
        apply_decl_element_ind:usize,
    },
    TemplateDecl {
        name : &'a str, //text_ind
        used:bool,
    },
    TemplateUse {
        template_decl_element_ind:usize,
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
pub struct Element<'a> {
    element_type:ElementType<'a>,
    children : Vec<usize>,
    applies : Vec<usize>, //element_ind
    apply_after : usize, //parent_apply_ind
    calcd_from_element_ind : Option<usize>, //element_ind
    calcd_node_params:BTreeSet<usize>, //element_ind
    calcd_created_from : usize,
    calcd_original : Option<usize>,
    has_script:bool,
    has_apply_script:bool,
}

fn make_attrib_func<T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(func : impl Fn(&mut T)+Send+Sync+'static) -> Arc<dyn Fn(Entity,&mut World)+Send+Sync > {
    Arc::new(move |entity:Entity,world: &mut World| {
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<T>().or_default();
        let mut c=c.get_mut();
        func(&mut c);
    })
}


pub fn load_elements<'a>(
    ui_assets: &'a Assets<UiAsset>,
    asset_server: &AssetServer,
    asset:&'a UiAsset,
    // get_asset:impl FnMut(&str) ->&UiAsset,
) -> Option<Vec<Element<'a>>>{
    let mut elements: Vec<Element> = vec![Element{
        element_type: ElementType::Node {
            names:HashSet::new(),
            ignore_applies:HashSet::new(),
        },
        children: Vec::new(),
        applies:Vec::new(),
        apply_after:0,
        calcd_from_element_ind : None,
        calcd_node_params:BTreeSet::new(),
        calcd_created_from:0,
        has_script:false,
        has_apply_script:false,
        calcd_original:None,
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
                        calcd_node_params:BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        has_apply_script:false,
                        calcd_original:None,
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
                        element_type: ElementType::TemplateUse { template_decl_element_ind, } ,
                        children: Vec::new(),
                        applies: Vec::new(),
                        apply_after,
                        calcd_from_element_ind: None,
                        calcd_node_params:BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        has_apply_script:false,
                        calcd_original:None,
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
                            element_type: ElementType::Apply { name, owner_apply_decl_id: prev_owner_apply_decl_id, used: true },
                            apply_after,
                            children: Vec::new(),
                            applies: Vec::new(),
                            calcd_from_element_ind: None,
                            calcd_node_params:BTreeSet::new(),
                            calcd_created_from:cur_element_ind,
                            has_script:false,
                            has_apply_script:false,
                            calcd_original:None,
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
                        calcd_node_params:BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        has_apply_script:false,
                        calcd_original:None,
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
                        calcd_node_params:BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        has_apply_script:false,
                        calcd_original:None,
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
                        children: Vec::new(),
                        applies: Vec::new(),
                        apply_after,
                        calcd_from_element_ind: None,
                        calcd_node_params:BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        has_apply_script:false,
                        calcd_original:None,
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

                    //
                    let mut attrib_funcs: Vec<(&str, Arc<dyn Fn(Entity, &mut World) + Send + Sync>)> = Vec::new();

                    //
                    do_attribs(x,on_state,asset_server,&walk,&mut attrib_funcs);

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
                            children: Vec::new(),
                            applies: Vec::new(),
                            apply_after,
                            calcd_from_element_ind: None,
                            calcd_node_params:BTreeSet::new(),
                            calcd_created_from:cur_element_ind,
                            has_script:false,
                            has_apply_script:false,
                            calcd_original:None,
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
    on_state:Option<UiAffectState>,
    asset_server: &AssetServer,
    walk : &conf_lang::Walk,
    attrib_funcs: &mut Vec<(&'a str, Arc<dyn Fn(Entity, &mut World) + Send + Sync>)>,
) {
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
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

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
        "hborder" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.border.left=v;
                c.border.right=v;
            })));
        }
        "vborder" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.border.top=v;
                c.border.bottom=v;
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
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

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
        "hpadding" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.padding.left=v;
                c.padding.right=v;
            })));
        }
        "vpadding" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.padding.top=v;
                c.padding.bottom=v;
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
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

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
        "hmargin" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.margin.left=v;
                c.margin.right=v;
            })));
        }
        "vmargin" => {
            let v = walk.record().value(0).get_parsed::<UiVal>().unwrap();

            attrib_funcs.push((x,make_attrib_func::<UiEdge>(move|c|{
                c.margin.top=v;
                c.margin.bottom=v;
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
            // let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or_else(||if let UiVal::Scale(_)=v{v*-1.0}else{v});
            let v2 = walk.record().value(1).get_parsed::<UiVal>().unwrap_or(v);

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

}
pub fn calc_node_apply_ignores(elements:&mut Vec<Element>) { //not currently used?

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


    //
    // applies inside a template should only affect the template's descendants and not siblings
    // attrib set by apply/template are always overwritten
}

pub fn calc_applies(elements:&mut Vec<Element>) {

    struct Thing { //
        applies : Vec<(
            usize, //apply_element_ind
            usize, //from_element_ind
        )>, //apply_element_ind, from_apply_use_element_ind
        apply_after:usize,
        element_ind:usize,
    }

    #[derive(Clone)]
    struct Work {
        element_ind:usize,
        from_applies:HashSet<usize>, //apply_element_ind
        in_template:Option<usize>, //template_use_id
        in_apply:Option<usize>, //apply_decl_id
        new_from_parent:Option<usize>, //element_ind
        thing_apply_after_offset:usize, //Option<usize>,

        node_depth:usize,
        thing_depth:usize,

        created_from:usize,
    }

    let mut work_stk=vec![Work{
        element_ind:0,
        in_template:None,
        in_apply:None,
        from_applies:HashSet::new(),
        new_from_parent:None,
        thing_apply_after_offset:0,

        node_depth:0,
        thing_depth:0,

        created_from:0,
    }];

    let mut things: Vec<Thing>=vec![]; //stack of elements, (cur element and its ancestors?) + info
    let mut node_stk_attribs: Vec<HashMap<(&str,Option<UiAffectState>),(Option<usize>,Option<usize>,bool,AttribFunc,usize)>> = Vec::new(); //[node_depth][(name,state)]=(in_template,in_apply.in_node,func,element_ind)

    while let Some(cur_work)=work_stk.pop() {
        let mut the_new_element_ind:Option<usize> = None;

        //replace element added via apply/template with new copy

        // cur_work.inside_apply_after
        if let Some(new_from_parent)=cur_work.new_from_parent {
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            if let ElementType::Node {..}|ElementType::TemplateUse{..}|ElementType::Stub{..}
                //not needed, can just pass the same one around, actually is needed, need unique element_ind for attrib for debug purposes
                //  also attrib needs to be added to its new parent, but don't need a new copy for that
                //
                |ElementType::Attrib{..}=&cur_element.element_type
            {

                //
                let mut new_element=Element {
                    children: Vec::new(),
                    calcd_original:Some(cur_work.element_ind),
                    ..cur_element.clone()
                };

                new_element.calcd_from_element_ind=Some(cur_work.element_ind);

                //
                let new_element_ind=elements.len();
                elements.get_mut(new_from_parent).unwrap().children.push(new_element_ind);
                elements.push(new_element);

                the_new_element_ind=Some(new_element_ind);
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
        {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind); //for attribs added via applies
            let cur_element=elements.get_mut(cur_element_ind).unwrap();

            if let ElementType::Attrib { name,in_node, on_state, func, calcd,.. } = &mut cur_element.element_type {
                let in_node=*in_node;
                let on_state=*on_state;

                let node_attribs=node_stk_attribs.last_mut().unwrap();

                let prev=node_attribs.get(&(*name,on_state.clone()));
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

                if ok {
                    node_attribs.insert((name,on_state), (in_template,in_apply,in_node,func.clone(),cur_element_ind));

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
        if let Some(cur_element_ind)=the_new_element_ind {
            let cur_element=elements.get_mut(cur_element_ind).unwrap();
            cur_element.calcd_created_from=cur_work.created_from;
        }

        //
        let mut new_applies: Vec<(usize, usize, )> = Vec::new(); //apply_element_ind,from_element_ind
        //get template applies
        //applies...

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

            match cur_element.element_type.clone() {
                ElementType::TemplateUse { template_decl_element_ind, .. } => {
                    let template_decl_element=elements.get(template_decl_element_ind).unwrap();
                    new_applies.extend(template_decl_element.applies.iter().map(|&apply_element_ind|( apply_element_ind, cur_element_ind, )));
                }
                ElementType::Stub { .. } => {
                    new_applies.extend(cur_element.applies.iter().map(|&apply_element_ind|( apply_element_ind, cur_element_ind, )));
                }
                ElementType::Node { names, ignore_applies, .. } => {
                    let last_stub_thing_ind=things.iter().enumerate().rev().find_map(|(i,t)|{
                        if let ElementType::Stub{..}=&elements.get(t.element_ind).unwrap().element_type {
                            Some(i)
                        } else {
                            None
                        }
                    });

                    //
                    let mut before_applies: Vec<(usize, usize)> = Vec::new(); //(apply_element_ind,from_element_ind)
                    let mut after_applies: Vec<(usize, usize)> = Vec::new();

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
                        //  except if node's first stub ancestor is also parent of the apply
                        if let Some(last_stub_thing_ind)=last_stub_thing_ind { //ancestor or cur element stub,
                            if thing_ind<last_stub_thing_ind { //swapped <=
                                continue;
                            }
                        }

                        //
                        let apply_after=things.get(thing_ind+1) //why +1 ? a descendant?
                            .map(|x|x.apply_after).unwrap_or(cur_element.apply_after);

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
                            if ignore_applies.contains(&apply_element_ind) { //apply_decl_id
                                apply_use_element_inds.push(None);
                                continue;
                            }

                            let apply_use_element_ind=elements.len();
                            apply_use_element_inds.push(Some(apply_use_element_ind));

                            elements.get_mut(cur_element_ind).unwrap().children.push(apply_use_element_ind);  //the cur_work.element_ind was wrong,

                            elements.push(Element {
                                element_type:ElementType::ApplyUse { apply_decl_element_ind: apply_element_ind,} ,
                                children: Vec::new(),
                                applies: Vec::new(),
                                apply_after:parent_applies_len, //new_applies.len(), //wrong? since this refers to parent.applies and not thing.applies, so should be parent.applies.len()
                                calcd_from_element_ind: None,
                                calcd_node_params:BTreeSet::new(),
                                calcd_created_from:from,
                                has_script:false,
                                has_apply_script:false,
                                calcd_original:None,
                            });
                        }
                    }

                    //
                    for (i,&(apply_element_ind,_)) in (after_applies.iter().rev().chain(before_applies.iter().rev()) ).enumerate() {
                        //apply_use
                        let Some(apply_use_element_ind)=apply_use_element_inds[apply_use_element_inds.len()-i-1] else {
                            continue;
                        };

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
                                in_template,
                                in_apply,
                                from_applies:from_applies.clone(),
                                new_from_parent:Some(apply_use_element_ind),
                                thing_apply_after_offset: the_apply_after,
                                node_depth,
                                thing_depth,
                                created_from:apply_use_element_ind,
                            }
                        }));
                    }

                    //get applies for new thing
                    //  add cur_element.applies then before+after applies
                    //  could do before_applies, cur_applies, after_applies

                    for (i,element_ind) in
                        before_applies.iter().enumerate().map(|(i,&x)|(Some(i),x.0))
                        .chain([(None,cur_work.element_ind)])
                        .chain(after_applies.iter().enumerate().map(|(i,&x)|(Some(i+before_applies.len()),x.0)))
                    {
                        let element=elements.get(element_ind).unwrap();

                        let from=if let Some(i)=i {
                            let Some(apply_use_element_ind)=apply_use_element_inds[i] else {
                                continue;
                            };
                            apply_use_element_ind
                        } else {
                            cur_element_ind
                        };

                        new_applies.extend(element.applies.iter().map(|&apply_element_ind|(apply_element_ind,from)));
                    }
                }
                _ => {
                }
            }
        }

        //push children (to work)
        {
            let cur_element_ind=the_new_element_ind.unwrap_or(cur_work.element_ind);
            let cur_element=elements.get(cur_work.element_ind).unwrap();

            match &cur_element.element_type {
                ElementType::Node { .. } => {
                    work_stk.extend(cur_element.children.iter().rev().filter_map(|&child_element_ind|{
                        let child_element=elements.get(child_element_ind).unwrap();
                        if let ElementType::ApplyUse { .. } =&child_element.element_type {
                            None
                        } else {
                            let w=Work{
                                element_ind: child_element_ind,
                                from_applies:cur_work.from_applies.clone(),
                                in_apply:cur_work.in_apply,
                                in_template:cur_work.in_template,
                                new_from_parent: the_new_element_ind, //so new element is created for child
                                thing_apply_after_offset: 0,
                                node_depth,
                                thing_depth,
                                created_from:cur_element_ind,
                            };
                            Some(w)
                        }
                    }));
                }
                ElementType::Stub { .. } => {
                    work_stk.extend(cur_element.children.iter().rev().map(|&child_element_ind|Work{
                        element_ind: child_element_ind,
                        from_applies:cur_work.from_applies.clone(),
                        in_apply:cur_work.in_apply,
                        in_template:cur_work.in_template,
                        new_from_parent: the_new_element_ind, //so new element is created for child
                        thing_apply_after_offset: 0,
                        node_depth,
                        thing_depth,
                        created_from:cur_element_ind,
                    }));
                }
                ElementType::TemplateUse { template_decl_element_ind,   .. } => {
                    let template_decl_element=elements.get(*template_decl_element_ind).unwrap();

                    work_stk.extend(template_decl_element.children.iter().rev().map(|&child_element_ind|{
                        let child_element=elements.get(child_element_ind).unwrap();
                        let in_template=if let ElementType::Attrib{..}=&child_element.element_type {
                            cur_work.in_template
                        }else{
                            // Some(*template_use_id)
                            Some(cur_element_ind)
                        };

                        Work{
                            element_ind: child_element_ind,
                            from_applies:cur_work.from_applies.clone(),
                            in_apply:cur_work.in_apply,
                            in_template,
                            // new_from_parent: if the_new_element_ind.is_some(){the_new_element_ind}else{Some(cur_work.element_ind)}, //so new element is created for child
                            new_from_parent: Some(the_new_element_ind.unwrap_or(cur_work.element_ind)), //so new element is created for child
                            thing_apply_after_offset: 0,
                            node_depth,
                            thing_depth,
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
                        // applies: Vec::new(), //new_applies, //doesn't use new applies? should be empty anyway
                        applies: new_applies,
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


pub fn calc_node_params(elements:&mut Vec<Element>) {
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
            work_stk.push(Work{exit:true, ..cur_work.clone()});

            let in_decl=if let ElementType::TemplateDecl{..}|ElementType::Apply{..}=&cur_element.element_type {true} else {cur_work.in_decl};

            work_stk.extend(cur_element.children.iter().rev().map(|&element_ind|Work {
                element_ind,
                exit:false,
                parent:Some(cur_work.element_ind),
                in_decl,
            }));
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
        } else {
            match &cur_element.element_type {
                ElementType::Stub { .. }|ElementType::Apply { .. }
                |ElementType::ApplyUse{..}
                |ElementType::TemplateDecl { .. }
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
    }
}


pub fn gen_stubs(elements:&Vec<Element>) -> Stuff {
    let mut all_stubs: HashMap<usize, Range<usize>> = HashMap::new(); //[root/stub_element_ind]=(nodes_start,nodes_end)
    let mut all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>=Vec::new(); //(element_ind,parent_ind,attribs_start,attribs_end)
    let mut all_attribs: Vec<AttribFunc>=Vec::new(); //[]=func
    let mut all_names: Vec<script_lang::StringT>=Vec::new();
    let mut all_names_map = HashSet::<script_lang::StringT>::new();

    //
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

    for (&stub_element_ind,node_parents) in creates.iter() {
        let nodes_start=all_nodes.len();
        // println!("{node_parents:?}");
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
            all_nodes.push((node_element_ind,parent_element_ind,attribs_start..attribs_end,names_start..names_end));
        }

        //
        let nodes_end=all_nodes.len();
        all_stubs.insert(stub_element_ind, nodes_start..nodes_end);

    }

    Stuff{  all_stubs, all_nodes, all_attribs, all_names }
}

/*
TODO
* don't generate funcs for nodes/applies/templates that don't have any script tags

*/
pub fn mark_used(elements:&mut Vec<Element>) {
    for element in elements.iter_mut() {

        match &mut element.element_type {
            ElementType::Apply { used, .. } => {
                *used=false;
            }
            ElementType::TemplateDecl { used, .. } => {
                *used=false;
            }
            _ => {

            }
        }
    }

    //
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        match &cur_element.element_type {
            ElementType::Apply { .. }|ElementType::TemplateDecl { .. } => {},
            _ => {
                work_stk.extend(cur_element.children.iter());
            }
        }

        match &cur_element.element_type {
            ElementType::ApplyUse { apply_decl_element_ind } => {
                let apply_decl_element_ind=*apply_decl_element_ind;
                let apply_element=elements.get_mut(apply_decl_element_ind).unwrap();
                let ElementType::Apply { used, .. }=&mut apply_element.element_type else {panic!("");};
                *used=true;
            },
            ElementType::TemplateUse { template_decl_element_ind } => {
                let template_decl_element_ind=*template_decl_element_ind;
                let tempalate_decl_element=elements.get_mut(template_decl_element_ind).unwrap();
                let ElementType::TemplateDecl { used, .. }=&mut tempalate_decl_element.element_type else {panic!("");};
                *used=true;
            },
            _ => {
            }
        }
    }
}

pub fn mark_has_script(elements:&mut Vec<Element>) {
    let mut parents: HashMap<usize, usize>=HashMap::new(); //[element]=parent
    let mut work_stk=vec![0];

    while let Some(cur_element_ind)=work_stk.pop() {
        let cur_element=elements.get(cur_element_ind).unwrap();

        if cur_element.has_script {
            // break;
        }

        //
        match &cur_element.element_type {
            ElementType::Apply { used, .. }|ElementType::TemplateDecl { used, .. } if !(*used) => {
            }
            _=>{
                work_stk.extend(cur_element.children.iter().rev());
                parents.extend(cur_element.children.iter().map(|&child_element_ind|(child_element_ind, cur_element_ind)));
            }
        }

        // //
        // let (has_script,has_apply_script)=match cur_element.element_type {
        //     ElementType::Script { .. } => (true,false),
        //     ElementType::TemplateUse { template_decl_element_ind  } => {
        //         let element=elements.get(template_decl_element_ind).unwrap();
        //         let ElementType::TemplateDecl { name,used, .. }=&element.element_type else {panic!("");};
        //         println!("---- {name:?} used={used}, script={}",element.has_script);
        //         if !used {
        //             (false,false)
        //         } else {
        //             (element.has_script,element.has_apply_script)
        //         }
        //         // *used && element.has_script
        //     },
        //     // ElementType::Apply { name, owner_apply_decl_id, used }
        //     _ => (false,false),
        // };

        //
        let has_script=match cur_element.element_type {
            ElementType::Script { .. } => true,
            ElementType::TemplateUse { template_decl_element_ind  } => {
                let element=elements.get(template_decl_element_ind).unwrap();
                let ElementType::TemplateDecl { name,used, .. }=&element.element_type else {panic!("");};
                println!("---- {name:?} used={used}, script={}",element.has_script);

                *used && element.has_script
            },
            _ => false,
        };

        //
        let has_apply_script=match cur_element.element_type {
            ElementType::TemplateUse { template_decl_element_ind  } => {
                let element=elements.get(template_decl_element_ind).unwrap();
                let ElementType::TemplateDecl { .. }=&element.element_type else {panic!("");};

                element.has_apply_script
            },
            _ => false,
        };

        //has_apply_script means it returns an apply decl func or a template decl that has a descendent that is an apply decl func with script

        // cur_element

        //set ancestors to has_script
        if has_script {
            let mut has_apply_script=has_apply_script;
            let mut element_ind=Some(cur_element_ind);

            while let Some(element_ind2)=element_ind {
                let element=elements.get_mut(element_ind2).unwrap();


                // match &element.element_type {
                //     ElementType::Apply { used, .. }|ElementType::TemplateDecl { used, .. } if !(*used) => {
                //         break;
                //     }
                //     _ => {
                element.has_script=true;
                element.has_apply_script=element.has_apply_script||has_apply_script;
                element_ind=parents.get(&element_ind2).cloned();
                //     }
                // }

                //
                if let ElementType::Apply{..} = &element.element_type {
                    has_apply_script=true;
                }

                // match &element.element_type {
                //     ElementType::Apply{..} => {
                //         has_apply_script=true;
                //     }
                //     _ => {
                //     }
                // }
            }
        }
    }
}



#[derive(Debug)]
pub enum ScriptSyntaxTemplateUseOrApplyDecl {
    ApplyDecl(usize),
    TemplateUse(usize),
}



#[derive(Debug)]
pub enum ScriptSyntaxNodeOrApplyUse {
    Node(usize),
    ApplyUse(usize),
}
// pub enum ScriptSyntaxDecl {
//     Node,
//     Apply,
//     Template,
// }

#[derive(Debug)]
pub enum ScriptSyntaxNodeOrApplyOrTemplate {
    Node(usize),
    Apply(usize),
    Template(usize),
}


pub struct ScriptSyntaxNode(usize);

impl Debug for ScriptSyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Node").field(&self.0).finish()
    }
}
pub struct ScriptSyntaxTemplateUse(usize);

impl Debug for ScriptSyntaxTemplateUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TemplateUse").field(&self.0).finish()
    }
}
pub struct ScriptSyntaxTemplateDecl(usize);

impl Debug for ScriptSyntaxTemplateDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TemplateDecl").field(&self.0).finish()
    }
}
pub struct ScriptSyntaxApplyDecl(usize);

impl Debug for ScriptSyntaxApplyDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ApplyDecl").field(&self.0).finish()
    }
}
pub struct ScriptSyntaxApplyUse(usize);


impl Debug for ScriptSyntaxApplyUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ApplyUse").field(&self.0).finish()
    }
}
impl Display for ScriptSyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for ScriptSyntaxTemplateUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl Display for ScriptSyntaxTemplateDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Display for ScriptSyntaxApplyDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl Display for ScriptSyntaxApplyUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}
pub enum ScriptSyntax {
    Root {
        children:Vec<usize>,
    },
    // InitStub {
    //     name:String,
    //     children:Vec<usize>,
    // },
    // InitVar {name:String,},

    Insert {
        path:Option<PathBuf>,
        loc :conf_lang::Loc,
        insert : String,
    },



    Decl {
        // decl : ScriptSyntaxDecl,
        name : ScriptSyntaxNodeOrApplyOrTemplate, //element_ind
        params : Vec<ScriptSyntaxNode>, //node element_inds
        children:Vec<usize>, //syntax_inds
        returns : Vec<(
            Option<ScriptSyntaxNode>, //node_element_ind
            ScriptSyntaxTemplateUseOrApplyDecl, //template_use_element_ind or apply_decl_element_ind
        )>,
    },

    Stub {
        name : String,
        children:Vec<usize>, //syntax_inds
    },

    CallStub {
        is_root:bool,
        stub : usize,//element_ind

    },
    CallTemplate {
        ret : Option<ScriptSyntaxTemplateUse>, //template_use_element_ind
        func : ScriptSyntaxTemplateDecl, //template_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
    CallApply {
        ret : Option<ScriptSyntaxApplyUse>, //apply_use_element_ind
        func_froms : Option<(
            ScriptSyntaxNodeOrApplyUse, //node_element_ind or apply_use_element_ind
            Vec<ScriptSyntaxTemplateUse>, //template_use_element_inds
        )>,
        func_apply : ScriptSyntaxApplyDecl, //apply_decl_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
    CallNode {
        ret:bool,
        in_func:bool, //inside template_decl, apply_decl or node
        func : ScriptSyntaxNode, //node_element_ind
        params : Vec<ScriptSyntaxNode>, //node_element_inds
    },
}

impl ScriptSyntax {
    pub fn get_children(&self) -> Option<&Vec<usize>> {
        match self {
            ScriptSyntax::Root{children}|ScriptSyntax::Decl{children,..}|ScriptSyntax::Stub{children,..}=>Some(children),
            _ =>None,
        }
    }
    pub fn get_children_mut(&mut self) -> Option<&mut Vec<usize>> {
        match self {
            ScriptSyntax::Root{children}|ScriptSyntax::Decl{children,..}|ScriptSyntax::Stub{children,..}=>Some(children),
            _ =>None,
        }
    }
}

pub fn gen_script_syntax_tree(elements:&Vec<Element>) -> Vec<ScriptSyntax> {
    let mut syntax_tree: Vec<ScriptSyntax> = vec![ScriptSyntax::Root { children: Vec::new() }];
    let mut syntax_stk: Vec<usize> = vec![0];//Vec::new(); //syntax_ind

    #[derive(Clone)]
    struct Work {
        element_ind:usize,
        depth:usize,
        exit:bool,
        parent:Option<usize>,
        in_a_use:bool,
        inside:Option<usize>,
    }

    let mut work_stk=vec![Work{ element_ind: 0, depth: 0, exit:false,parent:None, in_a_use:false,inside:None}];

    struct ApplyCallStkItem {
        inside_element_ind:Option<usize>,
        parent_element_ind:usize,
        apply_use_element_ind:usize,
    }

    let mut apply_calls_stk: Vec<Vec<ApplyCallStkItem>> = Vec::new();

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        if !cur_element.has_script {
            // continue;
        }

        //push children on work_stk
        if !cur_work.exit { //enter
            if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::TemplateDecl{..}|ElementType::Apply{..}
                |ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type
            {
                let in_use = if let ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                    true
                } else {
                    cur_work.in_a_use
                };

                let inside = if let ElementType::Node{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
                    Some(cur_work.element_ind)
                } else {
                    cur_work.inside
                };

                work_stk.push(Work{exit:true, ..cur_work.clone()});
                work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                    element_ind: child, depth: cur_work.depth+1, exit:false,
                    parent:Some(cur_work.element_ind),
                    in_a_use: in_use,
                    inside,
                }));
            }
        }

        //call applies, at root node and stubs
        if cur_work.exit
            && !cur_work.in_a_use
            && match &cur_element.element_type {  //on exit, not in apply/template use
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            let mut apply_calls=apply_calls_stk.pop().unwrap();

            apply_calls.sort_by(|x,y|{
                match x.apply_use_element_ind.cmp(&y.apply_use_element_ind) {
                    Ordering::Equal=>x.inside_element_ind.cmp(&y.inside_element_ind),
                    r=>r,
                }
            });

            for apply_call in apply_calls.iter() {
                let apply_use_element=elements.get(apply_call.apply_use_element_ind).unwrap();
                // let ElementType::ApplyUse { apply_decl_element_ind,  }=&apply_use_element.element_type else {panic!("");};

                let apply_in = {
                    if let ElementType::Stub{..} = &cur_element.element_type {
                        let mut b=false;
                        let mut cur_from=apply_use_element.calcd_created_from;

                        while cur_from!=0 {
                            let from_element=elements.get(cur_from).unwrap();

                            if from_element.calcd_created_from==cur_work.element_ind {
                                b=true;
                                break;
                            }

                            cur_from=from_element.calcd_created_from;
                        }

                        b
                    } else {
                        true
                    }
                };

                //
                let mut from_ret = None;
                let mut from_template_decls = Vec::new();
                if apply_in
                {
                    let mut cur_from=apply_use_element.calcd_created_from;
                    let to=if let ElementType::Stub{..} = &cur_element.element_type { cur_work.element_ind } else {0};

                    //
                    while cur_from!=to {
                        let from_element=elements.get(cur_from).unwrap();

                        match &from_element.element_type {
                            ElementType::ApplyUse { .. } => {
                                from_ret= Some(ScriptSyntaxNodeOrApplyUse::ApplyUse(cur_from));
                                break;
                            }
                            ElementType::TemplateUse { .. } => {
                                from_template_decls.push(ScriptSyntaxTemplateUse(cur_from));
                            }
                            ElementType::Node { .. } if from_element.calcd_created_from==to => {
                                from_ret= Some(ScriptSyntaxNodeOrApplyUse::Node(cur_from));
                                break; //not necessary
                            }
                            _ => { //will skip over nodes,
                            }
                        }

                        cur_from=from_element.calcd_created_from;
                    }
                }

                //
                if from_ret.is_none() && !from_template_decls.is_empty() {
                    panic!("");
                }

                let apply_use_element_ind=apply_call.apply_use_element_ind;
                let ElementType::ApplyUse { apply_decl_element_ind }=&elements.get(apply_use_element_ind).unwrap().element_type else {panic!("");};
                let apply_decl_element=elements.get(*apply_decl_element_ind).unwrap();

                let ret=apply_decl_element.has_apply_script.then_some(ScriptSyntaxApplyUse(apply_use_element_ind));

                let new_syntax_ind=syntax_tree.len();
                syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                // let params=[apply_call.parent_element_ind].iter().chain().collect();
                let mut params=vec![ScriptSyntaxNode(apply_call.parent_element_ind)];

                // let param_has_scripts:Vec<bool>=apply_decl_element.calcd_node_params.iter().map(|&param_element_ind|{
                //     let param_element=elements.get(param_element_ind).unwrap();
                //     param_element.has_script
                // }).collect();

                // params.extend(apply_use_element.calcd_node_params.iter().enumerate().filter_map(|(param_ind,&param_element_ind)|{
                //     // let param_element=elements.get(param_element_ind).unwrap();
                //     param_has_scripts[param_ind].then_some(ScriptSyntaxNode(param_element_ind))
                // }));
                params.extend(apply_use_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                    let param_element=elements.get(param_element_ind).unwrap();
                    let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                    let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                    has_script.then_some(ScriptSyntaxNode(param_element_ind))
                }));

                syntax_tree.push(ScriptSyntax::CallApply {
                    ret,
                    // func_froms: froms.iter().rev().cloned().collect(),
                    func_froms:from_ret.map(|from_ret|(from_ret,from_template_decls)),
                    func_apply: ScriptSyntaxApplyDecl(*apply_decl_element_ind),
                    params,
                });
            }
        }

        //add apply call to stk
        if !cur_work.exit { //enter //&& cur_element.has_script
            if let ElementType::ApplyUse{ apply_decl_element_ind } = &cur_element.element_type {
                let apply_element=elements.get(*apply_decl_element_ind).unwrap();
                // let ElementType::Apply {used , ..}=&apply_element.element_type else {panic!("");};

                if !apply_element.has_script {
                    continue;
                }

                apply_calls_stk.last_mut().unwrap().push(ApplyCallStkItem {
                    inside_element_ind:cur_work.inside,
                    parent_element_ind:cur_work.parent.unwrap(),
                    apply_use_element_ind:cur_work.element_ind,
                });
            }
        }

        //handle node,apply,template_decl returns
        if !cur_work.in_a_use && cur_work.exit && cur_element.has_script && match &cur_element.element_type {
            ElementType::Node{..} if cur_work.parent.is_some() => true,
            ElementType::Apply{..} => true,
            ElementType::TemplateDecl{..}=>true, //added
            _=>false,
        } {
            let mut return_items: Vec<(Option<ScriptSyntaxNode>,ScriptSyntaxTemplateUseOrApplyDecl)> = Vec::new();

            //apply/template uses returned by cur element's descendents
            for &child_element_ind in cur_element.children.iter() {
                if !elements.get(child_element_ind).unwrap().has_script {
                    continue;
                }

                //
                let mut tmp_stk=vec![child_element_ind];

                while let Some(tmp_element_ind)=tmp_stk.pop() {
                    let tmp_element=elements.get(tmp_element_ind).unwrap();

                    // if tmp_element.has_apply_script {} //should wrap below in this?

                    match &tmp_element.element_type {
                        ElementType::Node{..}=>{
                            tmp_stk.extend(tmp_element.children.iter());

                            for &apply_element_ind in tmp_element.applies.iter() {
                                let apply_element=elements.get(apply_element_ind).unwrap();

                                if !apply_element.has_script {
                                    continue;
                                }

                                return_items.push((Some(ScriptSyntaxNode(child_element_ind)),ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
                            }
                        }
                        ElementType::TemplateUse{..}=>{
                            if tmp_element.has_apply_script
                            {

                                if tmp_element_ind==child_element_ind {
                                    return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(tmp_element_ind)));
                                } else {
                                    return_items.push((Some(ScriptSyntaxNode(child_element_ind)),ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(tmp_element_ind)));
                                }
                            }
                        }
                        _=>{}
                    }
                }
            }

            //apply uses returned from cur element
            for &apply_element_ind in cur_element.applies.iter() {
                let apply_element=elements.get(apply_element_ind).unwrap();

                if !apply_element.has_script {
                    continue;
                }

                return_items.push((None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(apply_element_ind)));
            }

            let last_func_syntax_ind=syntax_stk.last().cloned().unwrap();
            let ScriptSyntax::Decl { returns, .. }=syntax_tree.get_mut(last_func_syntax_ind).unwrap() else {panic!("");};
            *returns = return_items;
        }

        //
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type {  //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            apply_calls_stk.push(Vec::new());
        }

        if !cur_work.in_a_use {
            match &cur_element.element_type {
                ElementType::Stub{name,..} if !cur_work.exit => { //enter
                    //
                    // apply_calls_stk.push(Vec::new());

                    //stub func
                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    syntax_stk.push(new_syntax_ind);
                    syntax_tree.push(ScriptSyntax::Stub { name: name.to_string(), children: Vec::new() });

                    //call stub
                    // let new_syntax_ind=syntax_tree.len();
                    // syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    // syntax_tree.push(ScriptSyntax::CallStub { is_root:true, stub: cur_work.element_ind } );
                }
                ElementType::Stub{..} => { //exit
                    syntax_stk.pop().unwrap();
                }
                _ => {

                }
            }
        }
        //
        if !cur_work.in_a_use && cur_element.has_script {
            match &cur_element.element_type {
                ElementType::Node{..} if cur_work.depth==0 && !cur_work.exit => { //enter
                    //
                    // apply_calls_stk.push(Vec::new());

                    //call stub
                    // let new_syntax_ind=syntax_tree.len();
                    // syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    // syntax_tree.push(ScriptSyntax::CallStub { is_root:true, stub: cur_work.element_ind } );
                }
                ElementType::Node{..} if cur_work.depth==0 => { //exit
                }
                // ElementType::Stub{name,..} if !cur_work.exit => { //enter
                //     //
                //     // apply_calls_stk.push(Vec::new());

                //     //stub func
                //     let new_syntax_ind=syntax_tree.len();
                //     syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                //     syntax_stk.push(new_syntax_ind);
                //     syntax_tree.push(ScriptSyntax::Stub { name: name.to_string(), children: Vec::new() });

                //     //call stub
                //     // let new_syntax_ind=syntax_tree.len();
                //     // syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                //     // syntax_tree.push(ScriptSyntax::CallStub { is_root:true, stub: cur_work.element_ind } );
                // }
                // ElementType::Stub{..} => { //exit
                //     syntax_stk.pop().unwrap();
                // }

                ElementType::Node{..} if !cur_work.exit => { //enter
                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    syntax_stk.push(new_syntax_ind);

                    // let params= cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                    //     let param_element=elements.get(param_element_ind).unwrap();
                    //     param_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    // }).collect::<Vec<_>>();

                    // let params= cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                    //     let param_element=elements.get(param_element_ind).unwrap();
                    //     // param_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    //     Some(ScriptSyntaxNode(param_element_ind))
                    // }).collect::<Vec<_>>();

                    let params= cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                        let param_element=elements.get(param_element_ind).unwrap();
                        let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                        let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                        has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    }).collect::<Vec<_>>();

                    syntax_tree.push(ScriptSyntax::Decl {
                        name: ScriptSyntaxNodeOrApplyOrTemplate::Node(cur_work.element_ind),
                        params,
                        children: Vec::new(),
                        returns: Vec::new(),
                    });
                }
                ElementType::Node{..} => { //exit
                    let parent_element=elements.get(cur_work.parent.unwrap()).unwrap();

                    syntax_stk.pop().unwrap();
                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                    let in_func=if let ElementType::Stub{..}=&parent_element.element_type {false}else{cur_work.depth!=1};

                    // let params=[cur_work.element_ind].iter().chain().collect();
                    let mut params=vec![ScriptSyntaxNode(cur_work.element_ind)];

                    // params.extend(cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                    //     let param_element=elements.get(param_element_ind).unwrap();
                    //     param_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    // }));

                    params.extend(cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                        let param_element=elements.get(param_element_ind).unwrap();
                        let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                        let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                        has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    }));

                    syntax_tree.push(ScriptSyntax::CallNode {
                        ret:cur_element.has_apply_script,
                        in_func,
                        func: ScriptSyntaxNode(cur_work.element_ind),
                        params,
                    });
                }
                ElementType::Apply{..} if !cur_work.exit => { //enter
                    let params=cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                        let param_element=elements.get(param_element_ind).unwrap();
                        param_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    }).collect::<Vec<_>>();

                    //
                    let new_syntax_ind=syntax_tree.len();

                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                    syntax_stk.push(new_syntax_ind);
                    syntax_tree.push(ScriptSyntax::Decl {
                        // decl: ScriptSyntaxDecl::Apply,
                        name: ScriptSyntaxNodeOrApplyOrTemplate::Apply(cur_work.element_ind),
                        params,
                        children: Vec::new(),
                        returns: Vec::new(),
                    });

                }
                ElementType::Apply{..} => { //exit
                    syntax_stk.pop().unwrap();
                }
                ElementType::TemplateDecl{..} if !cur_work.exit => { //enter
                    let params=cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                        let param_element=elements.get(param_element_ind).unwrap();
                        param_element.has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    }).collect::<Vec<_>>();

                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                    syntax_stk.push(new_syntax_ind);

                    syntax_tree.push(ScriptSyntax::Decl {
                        name: ScriptSyntaxNodeOrApplyOrTemplate::Template(cur_work.element_ind),
                        params,
                        children: Vec::new(),
                        returns: Vec::new(),
                    });
                }
                ElementType::TemplateDecl{..} => { //exit
                    syntax_stk.pop().unwrap();
                }
                ElementType::TemplateUse{template_decl_element_ind,..} if !cur_work.exit => { //enter
                    let new_syntax_ind=syntax_tree.len();
                    syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

                    // let template_decl_element=elements.get(*template_decl_element_ind).unwrap();

                    // let param_has_scripts:Vec<bool>=template_decl_element.calcd_node_params.iter().map(|&param_element_ind|{
                    //     let param_element=elements.get(param_element_ind).unwrap();
                    //     param_element.has_script
                    // }).collect();

                    let ret=cur_element.has_apply_script.then_some(ScriptSyntaxTemplateUse(cur_work.element_ind));

                    // let params=cur_element.calcd_node_params.iter().enumerate().filter_map(|(param_ind,&param_element_ind)|{
                    //     // let param_element=elements.get(param_element_ind).unwrap();
                    //     param_has_scripts[param_ind].then_some(ScriptSyntaxNode(param_element_ind))
                    // }).collect();

                    let params=cur_element.calcd_node_params.iter().filter_map(|&param_element_ind|{
                        let param_element=elements.get(param_element_ind).unwrap();
                        let orig_element=param_element.calcd_original.map(|orig_element_ind|elements.get(orig_element_ind).unwrap());
                        let has_script=orig_element.map(|x|x.has_script).unwrap_or(param_element.has_script);
                        has_script.then_some(ScriptSyntaxNode(param_element_ind))
                    }).collect();

                    syntax_tree.push(ScriptSyntax::CallTemplate {
                        ret,
                        func: ScriptSyntaxTemplateDecl(*template_decl_element_ind),
                        params,
                    });

                    continue;
                }
                ElementType::TemplateUse{..} => { //exit
                }
                ElementType::ApplyUse{..} if !cur_work.exit => { //enter
                    continue;
                }
                ElementType::ApplyUse{..} => { //exit
                }
                ElementType::Script { record  } if !cur_work.exit => { //enter
                    for t in record.text_values() {
                        let new_syntax_ind=syntax_tree.len();
                        syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);
                        syntax_tree.push(ScriptSyntax::Insert { path: t.path().map(|x|x.to_path_buf()), loc: t.start_loc(), insert: t.str().to_string() } );
                    }

                    continue;
                }
                ElementType::Script { .. } => { //exit
                }
                ElementType::Attrib {..} => {
                    continue;
                }

                _ => {

                }
            }
        }

        //call stubs
        if !cur_work.in_a_use && !cur_work.exit && match &cur_element.element_type { //enter
            ElementType::Node{..} if cur_work.depth==0 => true,
            ElementType::Stub{..} => true,
            _ => false,
        } {
            let new_syntax_ind=syntax_tree.len();
            syntax_tree.get_mut(syntax_stk.last().cloned().unwrap()).unwrap().get_children_mut().unwrap().push(new_syntax_ind);

            let is_root=if let ElementType::Stub{..}=&cur_element.element_type{false}else{true};

            syntax_tree.push(ScriptSyntax::CallStub { is_root, stub: cur_work.element_ind } );
        }
    } //end while

    //
    syntax_tree
}

pub fn debug_print_script_syntax_tree(syntax_tree:&Vec<ScriptSyntax>) {
    let mut stk=vec![(0,0)]; //ind,depth

    while let Some((cur_ind,depth))=stk.pop() {
        let indent="    ".repeat(depth);
        let cur=syntax_tree.get(cur_ind).unwrap();
        match cur {
            ScriptSyntax::Root { .. } => {
                println!("{indent}root");
            }
            ScriptSyntax::Insert {..} => {
                println!("{indent}script");
            }
            ScriptSyntax::Stub { name, ..  } => {
                println!("{indent}stub {name:?}");
            }
            ScriptSyntax::CallStub { is_root, stub } => {
                println!("{indent}call_stub {stub}, is_root={is_root}");
            }
            ScriptSyntax::CallTemplate { ret, func, params } => {
                println!("{indent}call_template {func:?}({params:?}) => {ret:?}");
            }
            ScriptSyntax::CallApply { ret, func_froms, func_apply, params } => {
                println!("{indent}call_apply {func_froms:?}{func_apply:?}({params:?}) => {ret:?}");
            }
            ScriptSyntax::CallNode { in_func, func, params, ret  } => {
                println!("{indent}call_node {func:?}({params:?}), in_func={in_func}, ret={ret}");
            }
            ScriptSyntax::Decl { name, params, returns, ..  } => {
                println!("{indent}decl {name:?}({params:?}), => {returns:?}");
            }
        }

        if let Some(children)=cur.get_children() {
            stk.extend(children.iter().map(|&child_ind|(child_ind,depth+1)).rev());
        }

    }
}


pub fn gen_script_src(syntax_tree:&Vec<ScriptSyntax>) -> String {
    let mut stk=vec![(0,0,false)]; //ind,depth,exit
    let mut src=String::new();
    src+="var root;\n";
    src+="var _stubs;\n";
    while let Some((cur_ind,depth, exit))=stk.pop() {
        let indent="    ".repeat(if depth<=1{0}else{depth-1});
        let cur=syntax_tree.get(cur_ind).unwrap();
        match cur {
            ScriptSyntax::Root { .. } => {
            }
            ScriptSyntax::Insert { //path, loc,
                insert , ..} => {
                src+=&format!("{indent}{insert}\n");
            }
            ScriptSyntax::Stub { name, .. } if !exit => {
                src+=&format!("{indent}fn stub_{name} (parent) {{\n");
            }
            ScriptSyntax::Stub { .. } => {
                src+=&format!("{indent}}}\n");
            }
            ScriptSyntax::CallStub { is_root, stub } => {
                let parent = if *is_root{"root"}else{"parent"};
                src+=&format!("{indent}var _ns [call _stubs {stub} {parent}]\n");
            }
            ScriptSyntax::CallTemplate { ret, func, params } => {
                let mut params2=vec!["self".to_string()];
                params2.extend(params.iter().map(|x|format!("_p{x}")));
                let params2=params2.join(" ");
                let c=format!("call _t{func} {params2}");
                let x=if let Some(ret)=ret {
                    format!("var _rt{ret} [{c}]")
                } else {
                    c
                };
                src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::CallApply { ret, func_froms, func_apply, params } => {
                let mut params2=Vec::new();
                params2.extend(params.iter().map(|x|format!("_ns.{x}")));
                let params2=params2.join(" ");
                let mut func = Vec::new();

                if let Some((func_first,rest))=func_froms {
                    let first=match func_first {
                        ScriptSyntaxNodeOrApplyUse::Node(e) => format!("n{e}"),
                        ScriptSyntaxNodeOrApplyUse::ApplyUse(e) => format!("a{e}"),
                    };

                    func.push(format!("r{first}"));
                    func.extend(rest.iter().map(|t|format!("t{t}")));
                }

                func.push(format!("a{func_apply}"));
                let func=func.join(".");

                let c=format!("call _{func} {params2}");
                let x=if let Some(ret)=ret {
                    format!("var _ra{ret} [{c}]")
                } else {
                    c
                };
                src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::CallNode { in_func, func, params, ret  } => {

                let params=params.iter().map(|x|format!("_{}{x}",if *in_func{"p"}else{"ns."})).collect::<Vec<_>>().join(" ");

                let c=format!("call _n{func} {params}");
                let x=if *ret {
                    format!("var _rn{func} [{c}]")
                }else {
                    c
                };
                src+=&format!("{indent}{x}\n");
            }
            ScriptSyntax::Decl { name, params, .. }  if !exit => { //enter
                let mut params2=vec!["self".to_string()];
                params2.extend(params.iter().map(|x|format!("_p{x}")));
                let params2=params2.join(" ");
                let name = match name {
                    ScriptSyntaxNodeOrApplyOrTemplate::Node(x) => format!("_n{x}"),
                    ScriptSyntaxNodeOrApplyOrTemplate::Apply(x) => format!("_a{x}"),
                    ScriptSyntaxNodeOrApplyOrTemplate::Template(x) => format!("_t{x}"),
                };

                src+=&format!("{indent}fn {name} ({params2}) {{\n");
            }
            ScriptSyntax::Decl { returns, .. } => { //exit
                if !returns.is_empty()
                {
                    let returns=returns.iter().map(|(k,v)|match (k,v) {
                        (Some(k),ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(v)) => format!("\"a{v}\" _rn{k}.a{v}"),
                        (Some(k),ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(v)) => format!("\"t{v}\" _rn{k}.t{v}"),
                        (None,ScriptSyntaxTemplateUseOrApplyDecl::ApplyDecl(v)) => format!("\"a{v}\" _a{v}"),
                        (None,ScriptSyntaxTemplateUseOrApplyDecl::TemplateUse(v)) => format!("\"t{v}\" _rt{v}"),
                    }).collect::<Vec<_>>().join(" ");

                    src+=&format!("{indent}    return [dict {returns}]\n");
                }

                src+=&format!("{indent}}}\n");
            }
        }

        if !exit {
            match cur {
                ScriptSyntax::Decl{..}|ScriptSyntax::Stub{..} => {
                    stk.push((cur_ind,depth,true));
                }
                _ => {}
            }

            if let Some(children)=cur.get_children() {
                stk.extend(children.iter().map(|&child_ind|(child_ind,depth+1,false)).rev());
            }
        }
    }

    src
}

                        // if let Some(loc)=record.text_values().next().map(|x|x.start_loc())
                        // if record.has_text()
                        // {
                        //     if let Some(p)=record.path() {
                        //         src+=&format!("{indent}#pragma_source {p:?}\n");
                        //     }

                        //     let loc=record.text_values().next().unwrap().start_loc();
                        //     src+=&format!("{indent}#pragma_loc {} {} {}\n",loc.pos,loc.row,loc.col);
                        // }
                        // if record.has_text() {
                        //     src+=&format!("{indent}#pragma_clear_loc\n");

                        //     if record.path().is_some() {
                        //         src+=&format!("{indent}#pragma_clear_source\n");
                        //     }
                        // }

pub fn debug_print_elements2(elements:&Vec<Element>) {
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


pub fn debug_print_elements(elements:&Vec<Element>) {
    println!("=====");

    struct Work { element_ind:usize, depth:usize, }

    let mut work_stk=vec![Work{ element_ind: 0, depth: 0 }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element_ind=cur_work.element_ind;
        let cur_element=elements.get(cur_element_ind).unwrap();
        work_stk.extend(cur_element.children.iter().rev().map(|&child|Work { element_ind: child, depth: cur_work.depth+1 }));

        let indent="    ".repeat(cur_work.depth);
        let from_path=&cur_element.calcd_created_from;
        let params = &cur_element.calcd_node_params;
        let has_script=cur_element.has_script;
        let has_apply_script=cur_element.has_apply_script;
        let calcd_original=cur_element.calcd_original.map(|x|format!("{x}")).unwrap_or("_".to_string());

        match &cur_element.element_type {
            ElementType::Node { names,ignore_applies,.. } => {
                println!("{indent}node {names:?}, e={cur_element_ind}, ignaps={ignore_applies:?}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}, orig={calcd_original}", );
            }
            ElementType::TemplateUse { template_decl_element_ind, .. } => {
                let ElementType::TemplateDecl { name, .. }=elements.get(*template_decl_element_ind).unwrap().element_type else {panic!("");};

                println!("{indent}template use, e={cur_element_ind} : {name:?}, e2={template_decl_element_ind}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}, orig={calcd_original}",);
            }
            ElementType::Apply { name,used,.. } => {
                println!("{indent}apply, e={cur_element_ind} : {name:?}, from={from_path:?}, params={params:?}, used={used}, scr={has_script:?}, ascr={has_apply_script:?}",);
            }
            ElementType::Attrib { name,in_node,calcd, ..  } => {
                println!("{indent}attrib {name:?}, e={cur_element_ind}, in_node={in_node}, calcd={calcd:?}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}, orig={calcd_original}", );
            }
            ElementType::Script { .. } => {
                println!("{indent}script, e={cur_element_ind}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}");
            }
            ElementType::TemplateDecl { name, used, .. } => {
                // let name=texts[*name];
                println!("{indent}template decl, e={cur_element_ind} : {name:?}, from={from_path:?}, params={params:?}, used={used}, scr={has_script:?}, ascr={has_apply_script:?}",);
            }
            ElementType::Stub { name } => {
                println!("{indent}stub {name:?}, e={cur_element_ind}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}, orig={calcd_original}");
            }
            ElementType::ApplyUse { apply_decl_element_ind,   } => {
                // let ElementType::Apply { apply_decl_id, .. }=elements.get(*apply_decl_element_ind).unwrap().element_type else {panic!("");};
                println!("{indent}apply use, e={cur_element_ind} : e2={apply_decl_element_ind:?}, from={from_path:?}, params={params:?}, scr={has_script:?}, ascr={has_apply_script:?}",);
            }
        }
        // println!("{indent}={:?}",cur_element.calcd_node_params);
    }
}
