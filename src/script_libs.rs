
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{collections::{HashMap, HashSet}, ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, ecs::{component::Component, entity, world::EntityRef},  prelude::{ ChildOf, Children, Entity, Resource, World}};
use bevy_table_ui::*;
use script_lang::*;

// use crate::UixSelf;

use super::components::*;
use super::messages::*;
use super::script_vals::*;

use super::script_utils::*;

/*
TODO
* add root node to core

* dont return parent for root
* return nil on attribs for root,
* don't allow setting attribs for root, give error

*/



pub fn register_attribs(lib_scope:&mut LibScope<World>) {
    lib_scope.method("copy", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged(val))
    }).custom_ref::<UiVal>();

    lib_scope.field_named("px", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(if let UiVal::Px(val)=val{Value::float(val)} else {Value::Nil})
    }).custom_ref::<UiVal>().end();

    lib_scope.field_named("px", |context|{
        let this=context.param(0).as_custom();
        let to=context.param(1).as_float();
        this.with_data_mut(|data:&mut UiVal|{ *data=UiVal::Px(to as f32); Ok(Value::Void) })
    }).custom_ref::<UiVal>().float().end();

    lib_scope.field_named("scale", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(if let UiVal::Scale(val)=val{Value::float(val)} else {Value::Nil})
    }).custom_ref::<UiVal>().end();

    lib_scope.field_named("scale", |context|{
        let this=context.param(0).as_custom();
        let to=context.param(1).as_float();
        this.with_data_mut(|data:&mut UiVal|{ *data=UiVal::Scale(to as f32); Ok(Value::Void) })
    }).custom_ref::<UiVal>().float().end();

    //
    #[derive(Clone)]
    struct NodeComputed(Value);

    lib_scope.field_named("computed", |context|{
        let node=context.param(0);
        Ok(Value::custom_unmanaged(NodeComputed(node)))
    }).custom_ref::<Entity>().end();

    // lib_scope.field_named("inner_width", |context|{
    //     let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
    //     let entity:Entity=node_computed.0.as_custom().data_clone()?;
    //     let world=context.core();
    //     let Some(computed)=world.entity(entity).get::<UiLayoutComputed>() else {
    //         return Ok(Value::Nil);
    //     };

    //     Ok(Value::float(computed.size.x))

    // }).custom_ref::<NodeComputed>();

    // // entity_get_field3::<UiLayoutComputed>("computed_width",lib_scope,|c|{
    // //     Value::float(c.)
    // // });

    //
    entity_get_field3::<UiCongruent>("row_width_scale",lib_scope,|c|{
        Value::float(c.row_width_scale)
    });
    entity_set_field_mut3::<UiCongruent>("row_width_scale",lib_scope,|c,v|{
        c.row_width_scale=script_value_to_float(v)?; Ok(())
    });

    entity_get_field3::<UiCongruent>("col_height_scale",lib_scope,|c|{
        Value::float(c.col_height_scale)
    });
    entity_set_field_mut3::<UiCongruent>("col_height_scale",lib_scope,|c,v|{
        c.col_height_scale=script_value_to_float(v)?; Ok(())
    });

    //get entity.padding_left
    lib_scope.field_named("padding_left", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.padding.left).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.padding_left
    lib_scope.field_named("padding_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.padding_right
    lib_scope.field_named("padding_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.padding.right).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.padding_right
    lib_scope.field_named("padding_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.padding_top
    lib_scope.field_named("padding_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.padding.top).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.padding_top
    lib_scope.field_named("padding_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.padding_bottom
    lib_scope.field_named("padding_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.padding.bottom).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.padding_bottom
    lib_scope.field_named("padding_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.margin_left
    lib_scope.field_named("margin_left", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.margin.left).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.margin_left
    lib_scope.field_named("margin_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.margin_right
    lib_scope.field_named("margin_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.margin.right).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.margin_right
    lib_scope.field_named("margin_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.margin_top
    lib_scope.field_named("margin_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.margin.top).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.margin_top
    lib_scope.field_named("margin_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.margin_bottom
    lib_scope.field_named("margin_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.margin.bottom).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.margin_bottom
    lib_scope.field_named("margin_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.border_left
    lib_scope.field_named("border_left", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.border.left).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.border_left
    lib_scope.field_named("border_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.border_right
    lib_scope.field_named("border_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.border.right).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.border_right
    lib_scope.field_named("border_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.border_top
    lib_scope.field_named("border_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.border.top).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.border_top
    lib_scope.field_named("border_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.border_bottom
    lib_scope.field_named("border_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let edge=get_component2::<UiEdge>(context.core(),entity);
        let v=edge.map(|edge|edge.border.bottom).map(|v|Value::custom_unmanaged(v)).unwrap_or_default();
        Ok(v)
    }).custom_ref::<Entity>().end();

    //set entity.border_bottom
    lib_scope.field_named("border_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=context.param(1);
        let to=to.is_nil().then_some(Ok(UiVal::None)).unwrap_or_else(||to.as_custom().data_clone())?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //
    entity_get_field3::<UiGap>("hgap",lib_scope,|c|{
        uival_to_script_value(c.hgap)
    });
    entity_set_field_mut3::<UiGap>("hgap",lib_scope,|c,v|{
        c.hgap=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiGap>("vgap",lib_scope,|c|{
        uival_to_script_value(c.vgap)
    });
    entity_set_field_mut3::<UiGap>("vgap",lib_scope,|c,v|{
        c.vgap=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiExpand>("hexpand",lib_scope,|c|{
        uival_to_script_value(c.hexpand)
    });
    entity_set_field_mut3::<UiExpand>("hexpand",lib_scope,|c,v|{
        c.hexpand=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiExpand>("vexpand",lib_scope,|c|{
        uival_to_script_value(c.vexpand)
    });
    entity_set_field_mut3::<UiExpand>("vexpand",lib_scope,|c,v|{
        c.vexpand=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiFill>("hfill",lib_scope,|c|{
        uival_to_script_value(c.hfill)
    });
    entity_set_field_mut3::<UiFill>("hfill",lib_scope,|c,v|{
        c.hfill=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiFill>("vfill",lib_scope,|c|{
        uival_to_script_value(c.vfill)
    });
    entity_set_field_mut3::<UiFill>("vfill",lib_scope,|c,v|{
        c.vfill=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiScroll>("hscroll",lib_scope,|c|{
        uival_to_script_value(c.hscroll)
    });
    entity_set_field_mut3::<UiScroll>("hscroll",lib_scope,|c,v|{
        c.hscroll=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiScroll>("vscroll",lib_scope,|c|{
        uival_to_script_value(c.vscroll)
    });
    entity_set_field_mut3::<UiScroll>("vscroll",lib_scope,|c,v|{
        c.vscroll=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiFloat>("float",lib_scope,|c|{
        Value::bool(c.float)
    });
    entity_set_field_mut3::<UiFloat>("float",lib_scope,|c,v|{
        c.float=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiDisable>("disable",lib_scope,|c|{
        Value::bool(c.disable)
    });
    entity_set_field_mut3::<UiDisable>("disable",lib_scope,|c,v|{
        c.disable=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiDisable>("disable",lib_scope,|c|{
        Value::bool(c.disable)
    });
    entity_set_field_mut3::<UiDisable>("disable",lib_scope,|c,v|{
        c.disable=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiHide>("hide",lib_scope,|c|{
        Value::bool(c.hide)
    });
    entity_set_field_mut3::<UiHide>("hide",lib_scope,|c,v|{
        c.hide=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiLock>("lock",lib_scope,|c|{
        Value::bool(c.lock)
    });
    entity_set_field_mut3::<UiLock>("lock",lib_scope,|c,v|{
        c.lock=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiSpan>("span",lib_scope,|c|{
        Value::int(c.span)
    });
    entity_set_field_mut3::<UiSpan>("span",lib_scope,|c,v|{
        c.span=script_value_to_uint(v)?; Ok(())
    });

    //
    entity_get_field3::<UiAlign>("halign",lib_scope,|c|{
        uival_to_script_value(c.halign)
    });
    entity_set_field_mut3::<UiAlign>("halign",lib_scope,|c,v|{
        c.halign=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiAlign>("valign",lib_scope,|c|{
        uival_to_script_value(c.valign)
    });
    entity_set_field_mut3::<UiAlign>("valign",lib_scope,|c,v|{
        c.valign=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiSize>("width",lib_scope,|c|{
        uival_to_script_value(c.width)
    });
    entity_set_field_mut3::<UiSize>("width",lib_scope,|c,v|{
        c.width=script_value_to_uival(v)?; Ok(())
    });

    entity_get_field3::<UiSize>("height",lib_scope,|c|{
        // println!("hmm height {:?}",c.height);
        uival_to_script_value(c.height)
    });
    entity_set_field_mut3::<UiSize>("height",lib_scope,|c,v|{
        c.height=script_value_to_uival(v)?; Ok(())
    });

    //
    entity_get_field3::<UiHoverable>("hoverable",lib_scope,|c|{
        Value::bool(c.enable)
    });
    entity_set_field_mut3::<UiHoverable>("hoverable",lib_scope,|c,v|{
        c.enable=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiPressable>("pressable",lib_scope,|c|{
        Value::bool(c.enable)
    });
    entity_set_field_mut3::<UiPressable>("pressable",lib_scope,|c,v|{
        c.enable=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiPressable>("press_always",lib_scope,|c|{
        Value::bool(c.always)
    });
    entity_set_field_mut3::<UiPressable>("press_always",lib_scope,|c,v|{
        c.always=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiPressable>("press_physical",lib_scope,|c|{
        Value::bool(c.physical)
    });
    entity_set_field_mut3::<UiPressable>("press_physical",lib_scope,|c,v|{
        c.physical=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiDraggable>("draggable",lib_scope,|c|{
        Value::bool(c.enable)
    });
    entity_set_field_mut3::<UiDraggable>("draggable",lib_scope,|c,v|{
        c.enable=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiSelectable>("selectable",lib_scope,|c|{
        Value::bool(c.enable)
    });
    entity_set_field_mut3::<UiSelectable>("selectable",lib_scope,|c,v|{
        c.enable=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiSelectable>("selected",lib_scope,|c|{
        Value::bool(c.selected)
    });
    entity_set_field_mut3::<UiSelectable>("selected",lib_scope,|c,v|{
        c.selected=script_value_to_bool(v)?; Ok(())
    });


    entity_get_field3::<UiSelectable>("select_group",lib_scope,|c|{
        Value::string(&c.group)
    });
    entity_set_field_mut3::<UiSelectable>("select_group",lib_scope,|c,v|{
        c.group=script_value_to_string(v)?; Ok(())
    });

    //
    entity_get_field3::<UiFocusable>("focusable",lib_scope,|c|{
        Value::bool(c.enable)
    });
    entity_set_field_mut3::<UiFocusable>("focusable",lib_scope,|c,v|{
        c.enable=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focused",lib_scope,|c|{
        Value::bool(c.focused)
    });
    entity_set_field_mut3::<UiFocusable>("focused",lib_scope,|c,v|{
        c.focused=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_group",lib_scope,|c|{
        Value::int(c.group)
    });
    entity_set_field_mut3::<UiFocusable>("focus_group",lib_scope,|c,v|{
        c.group=script_value_to_int(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_tab_exit",lib_scope,|c|{
        Value::bool(c.tab_exit)
    });
    entity_set_field_mut3::<UiFocusable>("focus_tab_exit",lib_scope,|c,v|{
        c.tab_exit=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_hdir_exit",lib_scope,|c|{
        Value::bool(c.hdir_exit)
    });
    entity_set_field_mut3::<UiFocusable>("focus_hdir_exit",lib_scope,|c,v|{
        c.hdir_exit=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_vdir_exit",lib_scope,|c|{
        Value::bool(c.vdir_exit)
    });
    entity_set_field_mut3::<UiFocusable>("focus_vdir_exit",lib_scope,|c,v|{
        c.vdir_exit=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_hdir_wrap",lib_scope,|c|{
        Value::bool(c.hdir_wrap)
    });
    entity_set_field_mut3::<UiFocusable>("focus_hdir_wrap",lib_scope,|c,v|{
        c.hdir_wrap=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_vdir_wrap",lib_scope,|c|{
        Value::bool(c.vdir_wrap)
    });
    entity_set_field_mut3::<UiFocusable>("focus_vdir_wrap",lib_scope,|c,v|{
        c.vdir_wrap=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_hdir_press",lib_scope,|c|{
        Value::bool(c.hdir_press)
    });
    entity_set_field_mut3::<UiFocusable>("focus_hdir_press",lib_scope,|c,v|{
        c.hdir_press=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_vdir_press",lib_scope,|c|{
        Value::bool(c.vdir_press)
    });
    entity_set_field_mut3::<UiFocusable>("focus_vdir_press",lib_scope,|c,v|{
        c.vdir_press=script_value_to_bool(v)?; Ok(())
    });

    //
    entity_get_field3::<UiAffect>("color",lib_scope,|c|{
        col_to_script_value(c.back_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("color",lib_scope,|c,v|{
        *c.back_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiAffect>("padding_color",lib_scope,|c|{
        col_to_script_value(c.padding_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("padding_color",lib_scope,|c,v|{
        *c.padding_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiAffect>("margin_color",lib_scope,|c|{
        col_to_script_value(c.margin_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("margin_color",lib_scope,|c,v|{
        *c.margin_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiAffect>("border_color",lib_scope,|c|{
        col_to_script_value(c.border_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("border_color",lib_scope,|c,v|{
        *c.border_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiAffect>("cell_color",lib_scope,|c|{
        col_to_script_value(c.cell_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("cell_color",lib_scope,|c,v|{
        *c.cell_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiAffect>("text_color",lib_scope,|c|{
        col_to_script_value(c.text_color.get(&None).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut3::<UiAffect>("text_color",lib_scope,|c,v|{
        *c.text_color.entry(None).or_default()=script_value_to_col(v)?; Ok(())
    });

    //
    entity_get_field3::<UiImage>("image_color",lib_scope,|c|{
        col_to_script_value(c.color)
    });
    entity_set_field_mut3::<UiImage>("image_color",lib_scope,|c,v|{
        c.color=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiImage>("image_width",lib_scope,|c|{
        Value::float(c.width_scale)
    });
    entity_set_field_mut3::<UiImage>("image_width",lib_scope,|c,v|{
        c.width_scale=script_value_to_float(v)?; Ok(())
    });

    entity_get_field3::<UiImage>("image_height",lib_scope,|c|{
        Value::float(c.height_scale)
    });
    entity_set_field_mut3::<UiImage>("image_height",lib_scope,|c,v|{
        c.height_scale=script_value_to_float(v)?; Ok(())
    });

    //
    entity_get_field3::<UiText>("text",lib_scope,|c|{
        Value::string(&c.value)
    });
    entity_set_field_mut3::<UiText>("text",lib_scope,|c,v|{
        c.value=script_value_to_string(v)?; c.update=true; Ok(())
    });

    entity_get_field3::<UiText>("font_size",lib_scope,|c|{
        Value::float(c.font_size)
    });
    entity_set_field_mut3::<UiText>("font_size",lib_scope,|c,v|{
        c.font_size=script_value_to_float(v)?; c.update=true; Ok(())
    });

    entity_get_field3::<UiText>("text_hlen",lib_scope,|c|{
        Value::int(c.hlen)
    });
    entity_set_field_mut3::<UiText>("text_hlen",lib_scope,|c,v|{
        c.hlen=script_value_to_uint(v)?; c.update=true; Ok(())
    });

    entity_get_field3::<UiText>("text_vlen",lib_scope,|c|{
        Value::int(c.vlen)
    });
    entity_set_field_mut3::<UiText>("text_vlen",lib_scope,|c,v|{
        c.vlen=script_value_to_uint(v)?; c.update=true; Ok(())
    });

    entity_get_field3::<UiText>("text_halign",lib_scope,|c|{
        Value::string(c.halign.as_str())
    });
    entity_set_field_mut3::<UiText>("text_halign",lib_scope,|c,v|{
        let v=v.get_string().and_then(|v|v.as_str().parse().ok()).ok_or_else(||MachineError::method("expected halign"))?;
        c.halign=v; c.update=true; Ok(())
    });

    entity_get_field3::<UiText>("text_valign",lib_scope,|c|{
        Value::string(c.valign.as_str())
    });
    entity_set_field_mut3::<UiText>("text_valign",lib_scope,|c,v|{
        let v=v.get_string().and_then(|v|v.as_str().parse().ok()).ok_or_else(||MachineError::method("expected valign"))?;
        c.valign=v; c.update=true; Ok(())
    });

    //
    entity_set_field_mut("image",lib_scope,|entity,val,world|{
        let handle=world.resource::<AssetServer>().load(PathBuf::from(script_value_to_string(val)?));
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiImage>().or_default();
        let mut c=c.get_mut();

        c.handle=handle; Ok(())
    });

    entity_set_field_mut("font",lib_scope,|entity,val,world|{
        let handle=world.resource::<AssetServer>().load(PathBuf::from(script_value_to_string(val)?));
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();

        c.font=handle; c.update=true; Ok(())
    });

    //

    //scaling(root_entity)
    entity_get_field("scaling",lib_scope,|entity,world|{
        world.entity(entity).get::<UiRoot>().map(|c|Value::float(c.scaling.min(0.0))).unwrap_or_default()
    });
}


pub fn register_stuff(lib_scope:&mut LibScope<World>)
{

    //call(stuff,ind,entity)
    lib_scope.method("call",|mut context|{
        let stuff=context.param(0).as_custom();
        let top_entity:Entity = context.param(2).as_custom().data_clone()?;
        let stub_ind=context.param(1).as_int().abs() as usize;

        let world=context.core_mut();

        stuff.with_data_ref(|stuff:&Stuff|{
            let mut element_entity_map = HashMap::<usize,Entity>::from_iter([(0,top_entity)]);
            let Some(node_range)=stuff.all_stubs.get(&stub_ind).cloned() else {return Ok(Value::Nil);};

            for node_ind in node_range {
                let stuff_node = stuff.all_nodes.get(node_ind).unwrap();
                let names=stuff.all_names.get(stuff_node.names.clone()).unwrap();

                let mut e=world.spawn((UiLayoutComputed::default(),));

                //
                let &parent_entity=element_entity_map.get(&stuff_node.parent_element_ind).unwrap();
                // e.set_parent(parent_entity);
                e.insert(ChildOf(parent_entity));

                //
                let entity=e.id();
                element_entity_map.insert(stuff_node.element_ind, entity);

                //
                if !names.is_empty() {
                    e.insert((UixName{ names:HashSet::from_iter(names.iter().map(|x|x.clone())) },));
                }

                //
                for attrib_ind in stuff_node.attribs.clone() {
                    let attrib=stuff.all_attribs.get(attrib_ind).unwrap().0.clone();
                    attrib(entity,world);
                }

                //
                let parent_entity_val=self_entity_from_world(world, parent_entity);

                // let mut pe=world.entity_mut(parent_entity);
                // let mut env=pe.entry::<UixEnv>().or_default();

                // for n in names.iter() {
                //     env.get_mut().env.entry(n.clone()).or_default().push(parent_entity_val.clone());
                // }
            }

            //
            let element_entity_map2: HashMap<usize, Value>=element_entity_map.iter().map(|(&k,&v)|{
                let vv=self_entity_from_world(world, v);
                (k,vv)
            }).collect();

            //
            let mut envs=HashMap::new();
            //
            if let Some(stub_envs)=stuff.all_envs.get(&stub_ind) {
                for (&env_element_ind,stuff_env) in stub_envs {
                    let v=StuffResultEnv{
                        by_ind: stuff_env.by_ind.iter().map(|&element_ind|element_entity_map2.get(&element_ind).unwrap().clone()).collect(),
                        by_name: stuff_env.by_name.iter().map(|(name,named_env)|{
                            (name.clone(),named_env.iter().map(|&element_ind|element_entity_map2.get(&element_ind).unwrap().clone()).collect())
                        }).collect(),
                    };
                    let v=Value::custom_unmanaged(v);
                    envs.insert(env_element_ind, v);
                }
            }


            //
            Ok(Value::custom_unmanaged(StuffResult{ nodes: element_entity_map2, envs }))
            // Ok(Value::custom_unmanaged(StuffResult(element_entity_map)))
        })
    }).custom_ref::<Stuff>().int().custom_ref::<Entity>().end();

    //
    lib_scope.field(|mut context|{ //field_no_symbols
        let data=context.param(0).as_custom();

        //
        let ind =context.param(1).get_string().unwrap();

        let is_node = ind.starts_with('n');
        let is_env = ind.starts_with('e');

        if !is_node && !is_env {
            return Ok(Value::Nil);
        }

        let ind:Option<usize> = ind.get(1..).and_then(|ind|ind.parse().ok());

        let Some(ind)=ind else {
            return Ok(Value::Nil);
        };

        // let ind=context.param(1).as_int().abs() as usize;

        let world=context.core_mut();

        data.with_data_ref(|data:&StuffResult|{
            // let Some(entity)=data.nodes.get(&ind).cloned() else {
            //     return Ok(Value::Nil);
            // };
            // Ok(entity)
            if is_node {
                let Some(entity)=data.nodes.get(&ind).cloned() else {
                    return Ok(Value::Nil);
                };
                Ok(entity)
            } else { //env
                let Some(env)=data.envs.get(&ind).cloned() else {
                    return Ok(Value::Nil);
                };
                Ok(env)
            }

            // let entity_val=self_entity_from_world(world, entity);
            // Ok(entity_val)
            // Ok(data.0.get(&ind).map(|&x|{
            //     Value::custom_unmanaged(x)
            // }).unwrap_or(Value::Nil))
        })
    }).custom_ref::<StuffResult>()
        // .int()
        .str()
        .end();


}

pub fn register_misc(lib_scope:&mut LibScope<World>) {

    //string(entity)
    lib_scope.method("string",|mut context|{
        let world=context.core_mut();
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        Ok(Value::string(format!("{entity}")))
    }).custom_ref::<Entity>().end();
}

pub fn register_events(lib_scope:&mut LibScope<World>) {
    //send(entity,str,any ...)
    lib_scope.method("send",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let event = context.param(1).get_string().unwrap();
        let params=(2..context.params_num()).map(|i|context.param(i).clone_root()).collect();
        let world=context.core_mut();
        world.write_message(UixUserMessage{ entity, event, params });
        Ok(Value::Void)
    }).custom_ref::<Entity>().str().optional().any().variadic_end();

    //
    lib_scope.method("add_event_listener",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let event=context.param(1).get_string().unwrap();
        let listener=context.param(2);

        let world=context.core_mut();

        let mut e=world.entity_mut(entity);
        e.entry::<UixEventListener>().or_default().get_mut()
            .event_listeners.entry(event).or_default()
            .push(listener.clone_root());

        Ok(Value::Void)
    }).custom_ref::<Entity>().str().func().end();

}
pub fn register(lib_scope:&mut LibScope<World>) {
    //remove(entity)
    lib_scope.method("remove",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core_mut();

        //
        let mut e=world.entity_mut(entity);

        e.despawn();
        // e.remove_parent();
        // e.remove::<ChildOf>();
        // e.remove::<Children>();

        Ok(Value::Void)
    }).custom_ref::<Entity>().end();


    //

    //get node.parent
    entity_get_field_mut("parent",lib_scope,|entity,world|{
        if world.entity(entity).contains::<UiRoot>() {return Value::Nil;}

        let parent_entity=world.entity(entity).get::<ChildOf>().map(|parent|parent.parent());
        let parent_entity_val=parent_entity.map(|parent_entity|self_entity_from_world(world,parent_entity));
        parent_entity_val.unwrap_or(Value::Nil)
    });


    #[derive(Clone)]
    struct NodeChildren { entity:Value, }
    // #[derive(Clone)]
    // struct NodeNamedChildren { entity:Value, name:StringT, }
    #[derive(Clone)]
    struct NodeAncestors { entity:Value, }
    // #[derive(Clone)]
    // struct NodeNamedAncestors { entity:Value, name:StringT, }

    //get node.children
    lib_scope.field_named("ancestors",|mut context|{
        let entity_val=context.param(0);
        Ok(Value::custom_unmanaged(NodeAncestors{entity:entity_val.clone()}))
    }).custom_ref::<Entity>().end();

    //get node_ancestors.int
    lib_scope.field(|mut context|{
        let ancestors_val=context.param(0);
        let node_ancestors:NodeAncestors=ancestors_val.as_custom().data_clone()?;
        let entity:Entity=node_ancestors.entity.as_custom().data_clone()?;
        let world=context.core();
        let ancestors=get_ancestors(world,entity);

        let Some(index)=context.param(1).as_index(ancestors.len()) else {
            return Ok(Value::Nil);
        };

        let world=context.core_mut();
        let ancestor_entity=ancestors.get(index).cloned().unwrap();
        let ancestor_val=self_entity_from_world(world, ancestor_entity);

        return Ok(ancestor_val)

    }).custom_ref::<NodeAncestors>().int().end();

    //get node_ancestors.str
    lib_scope.field(|mut context|{
        let ancestors_val=context.param(0);
        let name=context.param(1).get_string().unwrap();
        let node_ancestors:NodeAncestors=ancestors_val.as_custom().data_clone()?;
        let entity:Entity=node_ancestors.entity.as_custom().data_clone()?;

        let world=context.core();
        let ancestors=get_ancestors(world,entity);

        let world=context.core_mut();
        let ancestors= ancestors.into_iter().filter_map(|ancestor_entity|{
            let is_name = world.entity(ancestor_entity).get::<UixName>().map(|c|c.names.contains(&name)).unwrap_or_default();
            is_name.then(||self_entity_from_world(world, ancestor_entity))
            // if is_name {
            //     Some(self_entity_from_world(world, ancestor_entity))
            // } else {
            //     None
            // }
        }).collect::<Vec<_>>();

        Ok(Value::custom_unmanaged(ancestors))

        // return Ok(Value::custom_unmanaged(NodeNamedAncestors{ entity: node_ancestors.entity.clone(), name }))
    }).custom_ref::<NodeAncestors>().str().end();

    //get node.children
    lib_scope.field_named("children",|mut context|{
        let entity_val=context.param(0);
        Ok(Value::custom_unmanaged(NodeChildren{entity:entity_val.clone()}))
    }).custom_ref::<Entity>().end();

    //get node_children.int
    lib_scope.field(|mut context|{
        let children_val=context.param(0);
        let node_children:NodeChildren=children_val.as_custom().data_clone()?;
        let entity:Entity=node_children.entity.as_custom().data_clone()?;

        let world=context.core();

        let Some(children)= world.entity(entity).get::<Children>() else {return Ok(Value::Nil);};
        let Some(index)=context.param(1).as_index(children.len()) else {return Ok(Value::Nil);};
        let child_entity=children.get(index).cloned().unwrap();

        let world=context.core_mut();
        let child_entity_val=self_entity_from_world(world, child_entity);

        Ok(child_entity_val)
    }).custom_ref::<NodeChildren>().int().end();

    //get node_children.str
    lib_scope.field(|mut context|{
        let children_val=context.param(0);
        let node_children:NodeChildren=children_val.as_custom().data_clone()?;
        let entity:Entity=node_children.entity.as_custom().data_clone()?;
        let name=context.param(1).get_string().unwrap();

        let world=context.core_mut();
        let children=world.entity(entity).get::<Children>().map(|children|children.iter().map(|x|*x).collect::<Vec<_>>()).unwrap_or_default();
        let children= children.into_iter().filter_map(|child_entity|{
            let is_name = world.entity(child_entity).get::<UixName>().map(|c|c.names.contains(&name)).unwrap_or_default();
            is_name.then(||self_entity_from_world(world, child_entity))
            // if is_name {
            //     let child_entity_val=self_entity_from_world(world, child_entity);
            //     Some(child_entity_val)
            // } else {
            //     None
            // }
        }).collect::<Vec<_>>();

        Ok(Value::custom_unmanaged(children))
    }).custom_ref::<NodeChildren>().str().end();

    //len(node_children)
    lib_scope.method("len",|context|{
        let node_children_val=context.param(0);
        let node_children:NodeChildren=node_children_val.as_custom().data_clone()?;
        let entity:Entity=node_children.entity.as_custom().data_clone()?;
        let world=context.core();
        let n=world.entity(entity).get::<Children>().map(|children|children.len()).unwrap_or(0);
        Ok(Value::int(n))
    }).custom_ref::<NodeChildren>().end();

    //clear(node_children,name?)

    //pop(node_children)

    //

    //len stuff_result_env
    lib_scope.method("len",|mut context|{
        let env=context.param(0);
        env.as_custom().with_data_ref(|data:&StuffResultEnv|Ok(Value::int(data.by_ind.len())))
    }).custom_ref::<StuffResultEnv>().end();

    //get stuff_result_env.int
    lib_scope.field(|mut context|{
        let env=context.param(0);

        env.as_custom().with_data_ref(|data:&StuffResultEnv|{
            Ok(context.param(1).as_index(data.by_ind.len()).map(|index|data.by_ind[index].clone()).unwrap_or(Value::Nil))
        })
    }).custom_ref::<StuffResultEnv>().int().end();

    //
    #[derive(Clone)]
    struct StuffResultNamedEnv {
        env:Value,
        name : StringT,
    }

    //get stuff_result_env.str
    lib_scope.field(|mut context|{
        let env=context.param(0);
        let name=context.param(1).get_string().unwrap();
        Ok(Value::custom_unmanaged(StuffResultNamedEnv{name,env}))
    }).custom_ref::<StuffResultEnv>().str().end();


    //len stuff_result_named_env
    lib_scope.method("len",|mut context|{
        let named_env:StuffResultNamedEnv=context.param(0).as_custom().data_clone()?;

        named_env.env.as_custom().with_data_ref(|data:&StuffResultEnv|{
            Ok(Value::int(data.by_name.get(&named_env.name).map(|x|x.len()).unwrap_or(0)))
        })
    }).custom_ref::<StuffResultNamedEnv>().end();


    //get stuff_result_named_env.int
    lib_scope.field(|mut context|{
        let named_env:StuffResultNamedEnv=context.param(0).as_custom().data_clone()?;

        named_env.env.as_custom().with_data_ref(|data:&StuffResultEnv|{
            Ok(data.by_name.get(&named_env.name).and_then(|v|{
                context.param(1).as_index(v.len()).map(|index|v.get(index).unwrap().clone())
            }).unwrap_or(Value::Nil))
        })
    }).custom_ref::<StuffResultNamedEnv>().int().end();
}

