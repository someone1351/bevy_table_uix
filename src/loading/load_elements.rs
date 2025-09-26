
use std::collections::{HashMap, HashSet};
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

use bevy::color::Color;
use bevy::ecs::prelude::*;
use bevy::asset::prelude::*;
use conf_lang::RecordContainer;
use bevy_table_ui as table_ui;
use table_ui::*;
use super::super::script_vals::*;

use super::super::assets::*;

// use super::loading_vals::*;

use super::vals::*;

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
        element_type:ElementType::Node{names:HashSet::new(),ignore_applies:HashSet::new(),},
        children:Vec::new(),applies:Vec::new(),
        apply_after:0,calcd_from_element_ind:None,
        calcd_node_params:BTreeSet::new(),
        calcd_env_params: BTreeSet::new(),
        calcd_created_from:0,
        has_script:false,
        //has_apply_decl_script:false,
        has_self_script:false,
        has_env_script:false,
        // has_template_use_script:false,
        // calcd_original:None,
        env: HashMap::new(),
        parent:None,
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
                    calcd_env_params: BTreeSet::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    env: HashMap::new(),
                    parent:Some(cur_element_ind),
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
                    calcd_env_params: BTreeSet::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    env: HashMap::new(),
                    parent:Some(cur_element_ind),
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
                        calcd_env_params: BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        //has_apply_decl_script:false,
                        has_self_script:false,
                        has_env_script:false,
                        // has_template_use_script:false,
                        // calcd_original:None,
                        env: HashMap::new(),
                        parent:Some(cur_element_ind),
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
                    calcd_env_params: BTreeSet::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    env: HashMap::new(),
                    parent:Some(cur_element_ind),
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
                    calcd_env_params: BTreeSet::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    env: HashMap::new(),
                    parent:Some(cur_element_ind),
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
                    calcd_env_params: BTreeSet::new(),
                    calcd_created_from:cur_element_ind,
                    has_script:false,
                    //has_apply_decl_script:false,
                    has_self_script:false,
                    has_env_script:false,
                    // has_template_use_script:false,
                    // calcd_original:None,
                    env: HashMap::new(),
                    parent:Some(cur_element_ind),
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
                        calcd_env_params: BTreeSet::new(),
                        calcd_created_from:cur_element_ind,
                        has_script:false,
                        //has_apply_decl_script:false,
                        has_self_script:false,
                        has_env_script:false,
                        // has_template_use_script:false,
                        // calcd_original:None,
                        env: HashMap::new(),
                        parent:Some(cur_element_ind),
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
