
#![allow(unused_mut)]
#![allow(dead_code)]
// #![allow(unused_variables)]
#![allow(unused_imports)]

use std::{collections::{HashMap, HashSet}, ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, ecs::{component::Component, entity, world::EntityRef},  prelude::{ ChildOf, Children, Entity, Resource, World}};
use bevy_table_ui::*;
use script_lang::*;

// use crate::UixSelf;

use crate::affect::{UixAffect, UixAffectAttrib};

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
    //copy ui_val
    lib_scope.method("copy", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(val))
    }).custom_ref::<UiVal>();

    //not ui_val
    lib_scope.method("not", |context|{
        Ok(Value::bool(if let UiVal::None=context.param(0).as_custom().data_clone()?{true}else{false}))
    }).custom_ref::<UiVal>();

    //get ui_val.px
    lib_scope.field_named("p", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(if let UiVal::Px(val)=val{Value::float(val)} else {Value::Nil})
    }).custom_ref::<UiVal>().end();

    //set ui_val.px
    lib_scope.field_named("p", |context|{
        let this=context.param(0).as_custom();
        let to=context.param(1).as_float();
        this.with_data_mut_ext(|data:&mut UiVal|{ *data=UiVal::Px(to as f32); Ok(Value::Void) })
    }).custom_mut_ref::<UiVal>().float().end();

    //get ui_val.scale
    lib_scope.field_named("s", |context|{
        let val:UiVal=context.param(0).as_custom().data_clone()?;
        Ok(if let UiVal::Scale(val)=val{Value::float(val)} else {Value::Nil})
    }).custom_ref::<UiVal>().end();

    //set ui_val.scale
    lib_scope.field_named("s", |context|{
        let this=context.param(0).as_custom();
        let to=context.param(1).as_float();
        this.with_data_mut_ext(|data:&mut UiVal|{ *data=UiVal::Scale(to as f32); Ok(Value::Void) })
    }).custom_mut_ref::<UiVal>().float().end();

    //
    #[derive(Clone)]
    struct NodeComputed(Value);

    //get entity.computed
    lib_scope.field_named("computed", |context|{
        let node=context.param(0);
        Ok(Value::custom_unmanaged(NodeComputed(node)))
    }).custom_ref::<Entity>().end();

    //
    #[derive(Clone)]
    enum NodeEdge { Padding(Value), Border(Value), Margin(Value), Cell(Value),}
    impl NodeEdge {
        pub fn node(&self) -> Value {
            let (NodeEdge::Padding(node)|NodeEdge::Border(node)|NodeEdge::Margin(node)|NodeEdge::Cell(node))=self;
            node.clone()
        }
    }

    //get computed.padding
    lib_scope.field_named("padding", |context|{
        let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
        let node=node_computed.0;
        Ok(Value::custom_unmanaged(NodeEdge::Padding(node)))
    }).custom_ref::<NodeComputed>().end();

    //get computed.border
    lib_scope.field_named("border", |context|{
        let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
        let node=node_computed.0;
        Ok(Value::custom_unmanaged(NodeEdge::Border(node)))
    }).custom_ref::<NodeComputed>().end();

    //get computed.margin
    lib_scope.field_named("margin", |context|{
        let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
        let node=node_computed.0;
        Ok(Value::custom_unmanaged(NodeEdge::Margin(node)))
    }).custom_ref::<NodeComputed>().end();

    //get computed.cell
    lib_scope.field_named("cell", |context|{
        let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
        let node=node_computed.0;
        Ok(Value::custom_unmanaged(NodeEdge::Cell(node)))
    }).custom_ref::<NodeComputed>().end();

    //get node_edge.left_size
    lib_scope.field_named("left_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            match node_edge {
                NodeEdge::Padding(_) => c.padding_size.left,
                NodeEdge::Border(_) => c.border_size.left,
                NodeEdge::Margin(_) => c.margin_size.left,
                NodeEdge::Cell(_) => c.cell_size.left,
            }
        }).unwrap_or_default();
        Ok(Value::float(v))
    }).custom_ref::<NodeEdge>().end();

    //get node_edge.right_size
    lib_scope.field_named("right_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            match node_edge {
                NodeEdge::Padding(_) => c.padding_size.right,
                NodeEdge::Border(_) => c.border_size.right,
                NodeEdge::Margin(_) => c.margin_size.right,
                NodeEdge::Cell(_) => c.cell_size.right,
            }
        }).unwrap_or_default();
        Ok(Value::float(v))
    }).custom_ref::<NodeEdge>().end();

    //get node_edge.top_size
    lib_scope.field_named("top_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            match node_edge {
                NodeEdge::Padding(_) => c.padding_size.top,
                NodeEdge::Border(_) => c.border_size.top,
                NodeEdge::Margin(_) => c.margin_size.top,
                NodeEdge::Cell(_) => c.cell_size.top,
            }
        }).unwrap_or_default();
        Ok(Value::float(v))
    }).custom_ref::<NodeEdge>().end();

    //get node_edge.bottom_size
    lib_scope.field_named("bottom_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            match node_edge {
                NodeEdge::Padding(_) => c.padding_size.bottom,
                NodeEdge::Border(_) => c.border_size.bottom,
                NodeEdge::Margin(_) => c.margin_size.bottom,
                NodeEdge::Cell(_) => c.cell_size.bottom,
            }
        }).unwrap_or_default();
        Ok(Value::float(v))
    }).custom_ref::<NodeEdge>().end();

    //get node_edge.sum_size
    lib_scope.field_named("sum_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            match node_edge {
                NodeEdge::Padding(_) => [c.padding_size.left+c.padding_size.right,c.padding_size.top+c.padding_size.bottom],
                NodeEdge::Border(_) => [c.border_size.left+c.border_size.right, c.border_size.top+c.border_size.bottom],
                NodeEdge::Margin(_) => [c.margin_size.left+c.margin_size.right, c.margin_size.top+c.margin_size.bottom],
                NodeEdge::Cell(_) => [c.cell_size.left+c.cell_size.right, c.cell_size.top+c.cell_size.bottom],
            }
        }).unwrap_or_default().map(|x|x as FloatT);
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<NodeEdge>().end();

    //get node_edge.rect_size
    lib_scope.field_named("rect_size", |context|{
        let node_edge:NodeEdge=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_edge.node().as_custom().data_clone()?;
        let c=get_component2::<UiLayoutComputed>(context.core(),entity).filter(|&c|c.enabled);
        let v=c.map(|c|{
            let r=match node_edge {
                NodeEdge::Padding(_) => c.padding_rect(),
                NodeEdge::Border(_) => c.border_rect(),
                NodeEdge::Margin(_) => c.margin_rect(),
                NodeEdge::Cell(_) => c.cell_rect(),
            };
            [r.width(),r.height()]
        }).unwrap_or_default().map(|x|x as FloatT);
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<NodeEdge>().end();

    //get computed.inner_rect_size
    lib_scope.field_named("inner_rect_size", |context|{
        let node_computed:NodeComputed=context.param(0).as_custom().data_clone()?;
        let entity:Entity=node_computed.0.as_custom().data_clone()?;

        let v=get_component2::<UiLayoutComputed>(context.core(),entity)
            .filter(|&c|c.enabled)
            .map(|c|[c.size.x,c.size.y])
            .unwrap_or_default().map(|x|x as FloatT);

        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<NodeComputed>().end();

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
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.padding.left).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.padding_left
    lib_scope.field_named("padding_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.padding_right
    lib_scope.field_named("padding_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.padding.right).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.padding_right
    lib_scope.field_named("padding_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.padding_top
    lib_scope.field_named("padding_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.padding.top).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.padding_top
    lib_scope.field_named("padding_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.padding_bottom
    lib_scope.field_named("padding_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.padding.bottom).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.padding_bottom
    lib_scope.field_named("padding_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.padding.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.margin_left
    lib_scope.field_named("margin_left", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.margin.left).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.margin_left
    lib_scope.field_named("margin_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.margin_right
    lib_scope.field_named("margin_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.margin.right).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.margin_right
    lib_scope.field_named("margin_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.margin_top
    lib_scope.field_named("margin_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.margin.top).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.margin_top
    lib_scope.field_named("margin_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.margin_bottom
    lib_scope.field_named("margin_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.margin.bottom).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.margin_bottom
    lib_scope.field_named("margin_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.margin.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();


    //get entity.border_left
    lib_scope.field_named("border_left", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.border.left).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.border_left
    lib_scope.field_named("border_left", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.left=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.border_right
    lib_scope.field_named("border_right", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.border.right).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.border_right
    lib_scope.field_named("border_right", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.right=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.border_top
    lib_scope.field_named("border_top", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.border.top).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.border_top
    lib_scope.field_named("border_top", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.top=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.border_bottom
    lib_scope.field_named("border_bottom", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiEdge>(context.core(),entity);
        let v=c.map(|c|c.border.bottom).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.border_bottom
    lib_scope.field_named("border_bottom", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiEdge>(context.core_mut(),entity,|c|c.border.bottom=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //
    //get entity.hgap
    lib_scope.field_named("hgap", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiGap>(context.core(),entity);
        let v=c.map(|c|c.hgap).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.hgap
    lib_scope.field_named("hgap", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiGap>(context.core_mut(),entity,|c|c.hgap=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.vgap
    lib_scope.field_named("vgap", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiGap>(context.core(),entity);
        let v=c.map(|c|c.vgap).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.vgap
    lib_scope.field_named("vgap", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiGap>(context.core_mut(),entity,|c|c.vgap=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.hexpand
    lib_scope.field_named("hexpand", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiExpand>(context.core(),entity);
        let v=c.map(|c|c.hexpand).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.hexpand
    lib_scope.field_named("hexpand", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiExpand>(context.core_mut(),entity,|c|c.hexpand=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.vexpand
    lib_scope.field_named("vexpand", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiExpand>(context.core(),entity);
        let v=c.map(|c|c.vexpand).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.vexpand
    lib_scope.field_named("vexpand", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiExpand>(context.core_mut(),entity,|c|c.vexpand=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.hfill
    lib_scope.field_named("hfill", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiFill>(context.core(),entity);
        let v=c.map(|c|c.hfill).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.hfill
    lib_scope.field_named("hfill", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiFill>(context.core_mut(),entity,|c|c.hfill=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.vfill
    lib_scope.field_named("vfill", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiFill>(context.core(),entity);
        let v=c.map(|c|c.vfill).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.vfill
    lib_scope.field_named("vfill", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiFill>(context.core_mut(),entity,|c|c.vfill=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.hscroll
    lib_scope.field_named("hscroll", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiScroll>(context.core(),entity);
        let v=c.map(|c|c.hscroll).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.hscroll
    lib_scope.field_named("hscroll", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiScroll>(context.core_mut(),entity,|c|c.hscroll=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.vscroll
    lib_scope.field_named("vscroll", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiScroll>(context.core(),entity);
        let v=c.map(|c|c.vscroll).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.vscroll
    lib_scope.field_named("vscroll", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiScroll>(context.core_mut(),entity,|c|c.vscroll=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

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
    //get entity.halign
    lib_scope.field_named("halign", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiAlign>(context.core(),entity);
        let v=c.map(|c|c.halign).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.halign
    lib_scope.field_named("halign", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiAlign>(context.core_mut(),entity,|c|c.halign=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.valign
    lib_scope.field_named("valign", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiAlign>(context.core(),entity);
        let v=c.map(|c|c.valign).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.valign
    lib_scope.field_named("valign", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiAlign>(context.core_mut(),entity,|c|c.valign=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.width
    lib_scope.field_named("width", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiSize>(context.core(),entity);
        let v=c.map(|c|c.width).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.width
    lib_scope.field_named("width", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiSize>(context.core_mut(),entity,|c|c.width=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //get entity.height
    lib_scope.field_named("height", |context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let c=get_component2::<UiSize>(context.core(),entity);
        let v=c.map(|c|c.height).unwrap_or_default();
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Entity>().end();

    //set entity.height
    lib_scope.field_named("height", |mut context|{
        let entity:Entity=context.param(0).as_custom().data_clone()?;
        let to=script_to_ui_val(context.param(1))?;
        set_component::<UiSize>(context.core_mut(),entity,|c|c.height=to);
        Ok(Value::Void)
    }).custom_ref::<Entity>().custom_ref::<UiVal>().or_nil().end();

    //
    entity_get_field3::<UiCursorable>("hoverable",lib_scope,|c|{
        Value::bool(c.hoverable)
    });
    entity_set_field_mut3::<UiCursorable>("hoverable",lib_scope,|c,v|{
        c.hoverable=script_value_to_bool(v)?; Ok(())
    });

    //
    // entity_get_field3::<UiPressable>("pressable",lib_scope,|c|{
    //     Value::bool(c.enable)
    // });
    // entity_set_field_mut3::<UiPressable>("pressable",lib_scope,|c,v|{
    //     c.enable=script_value_to_bool(v)?; Ok(())
    // });
    lib_scope.field_named("pressable",|context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core();
        let e=world.entity(entity);
        let p1=e.get::<UiCursorable>().cloned().unwrap_or_default().pressable;
        let p2=e.get::<UiFocusable>().cloned().unwrap_or_default().pressable;
        Ok((p1 && p2).into())
    }).custom_ref::<Entity>().end();

    lib_scope.field_named("pressable",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let to_val=context.param(1).as_bool();
        let world=context.core_mut();
        let mut e=world.entity_mut(entity);
        e.entry::<UiCursorable>().or_default().get_mut().pressable=to_val;
        e.entry::<UiFocusable>().or_default().get_mut().pressable=to_val;
        Ok(Value::Void)
    }).custom_ref::<Entity>().bool().end();
    // entity_get_field3::<UiPressable>("press_always",lib_scope,|c|{
    //     Value::bool(c.always)
    // });
    // entity_set_field_mut3::<UiPressable>("press_always",lib_scope,|c,v|{
    //     c.always=script_value_to_bool(v)?; Ok(())
    // });

    // entity_get_field3::<UiPressable>("press_physical",lib_scope,|c|{
    //     Value::bool(c.physical)
    // });
    // entity_set_field_mut3::<UiPressable>("press_physical",lib_scope,|c,v|{
    //     c.physical=script_value_to_bool(v)?; Ok(())
    // });

    //
    entity_get_field3::<UiCursorable>("draggable",lib_scope,|c|{
        Value::bool(c.draggable)
    });
    entity_set_field_mut3::<UiCursorable>("draggable",lib_scope,|c,v|{
        c.draggable=script_value_to_bool(v)?; Ok(())
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

    // entity_get_field3::<UiFocusable>("focused",lib_scope,|c|{
    //     Value::bool(c.focused)
    // });
    // entity_set_field_mut3::<UiFocusable>("focused",lib_scope,|c,v|{
    //     c.focused=script_value_to_bool(v)?; Ok(())
    // });

    entity_get_field3::<UiFocusable>("focus_group",lib_scope,|c|{
        Value::int(c.group)
    });
    entity_set_field_mut3::<UiFocusable>("focus_group",lib_scope,|c,v|{
        c.group=script_value_to_int(v)?; Ok(())
    });

    // entity_get_field3::<UiFocusable>("focus_tab_exit",lib_scope,|c|{
    //     Value::bool(c.tab_exit)
    // });
    // entity_set_field_mut3::<UiFocusable>("focus_tab_exit",lib_scope,|c,v|{
    //     c.tab_exit=script_value_to_bool(v)?; Ok(())
    // });

    entity_get_field3::<UiFocusable>("focus_hexit",lib_scope,|c|{
        Value::bool(c.hexit)
    });
    entity_set_field_mut3::<UiFocusable>("focus_hexit",lib_scope,|c,v|{
        c.hexit=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_vexit",lib_scope,|c|{
        Value::bool(c.vexit)
    });
    entity_set_field_mut3::<UiFocusable>("focus_vexit",lib_scope,|c,v|{
        c.vexit=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_hwrap",lib_scope,|c|{
        Value::bool(c.hwrap)
    });
    entity_set_field_mut3::<UiFocusable>("focus_hwrap",lib_scope,|c,v|{
        c.hwrap=script_value_to_bool(v)?; Ok(())
    });

    entity_get_field3::<UiFocusable>("focus_vwrap",lib_scope,|c|{
        Value::bool(c.vwrap)
    });
    entity_set_field_mut3::<UiFocusable>("focus_vwrap",lib_scope,|c,v|{
        c.vwrap=script_value_to_bool(v)?; Ok(())
    });

    // entity_get_field3::<UiFocusable>("focus_hdir_press",lib_scope,|c|{
    //     Value::bool(c.hdir_press)
    // });
    // entity_set_field_mut3::<UiFocusable>("focus_hdir_press",lib_scope,|c,v|{
    //     c.hdir_press=script_value_to_bool(v)?; Ok(())
    // });

    // entity_get_field3::<UiFocusable>("focus_vdir_press",lib_scope,|c|{
    //     Value::bool(c.vdir_press)
    // });
    // entity_set_field_mut3::<UiFocusable>("focus_vdir_press",lib_scope,|c,v|{
    //     c.vdir_press=script_value_to_bool(v)?; Ok(())
    // });

    //
    entity_get_field3::<UiColor>("color",lib_scope,|c|{
        col_to_script_value(c.back)
    });
    entity_set_field_mut3::<UiColor>("color",lib_scope,|c,v|{
        c.back=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiColor>("padding_color",lib_scope,|c|{
        col_to_script_value(c.padding)
    });
    entity_set_field_mut3::<UiColor>("padding_color",lib_scope,|c,v|{
        c.padding=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiColor>("margin_color",lib_scope,|c|{
        col_to_script_value(c.margin)
    });
    entity_set_field_mut3::<UiColor>("margin_color",lib_scope,|c,v|{
        c.margin=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiColor>("border_color",lib_scope,|c|{
        col_to_script_value(c.border)
    });
    entity_set_field_mut3::<UiColor>("border_color",lib_scope,|c,v|{
        c.border=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiColor>("cell_color",lib_scope,|c|{
        col_to_script_value(c.cell)
    });
    entity_set_field_mut3::<UiColor>("cell_color",lib_scope,|c,v|{
        c.cell=script_value_to_col(v)?; Ok(())
    });

    entity_get_field3::<UiText>("text_color",lib_scope,|c|{
        col_to_script_value(c.color)
    });
    entity_set_field_mut3::<UiText>("text_color",lib_scope,|c,v|{
        c.color=script_value_to_col(v)?; Ok(())
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

        stuff.with_data_ref_ext(|stuff:&Stuff|{
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
                if let Some(attrib_funcs)=stuff.all_state_attribs.get(&stuff_node.element_ind) {
                    let mut out_affect_attribs=Vec::new();

                    for (default_func,state_funcs) in attrib_funcs {
                        //
                        let mut state_funcs=state_funcs.iter().map(|(state,(func,priority))|(state,func,priority)).collect::<Vec<_>>();
                        state_funcs.sort_by(|x,y|x.2.cmp(&y.2));

                        //
                        let mut out_funcs=vec![default_func.clone()];
                        let mut out_states=HashMap::new(); //states:HashMap<UixAffectState,usize>

                        //
                        for (i,&(&state,func,_priority)) in state_funcs.iter().enumerate() {
                            out_funcs.push(func.clone());
                            out_states.insert(state, i+1);
                        }

                        out_affect_attribs.push(UixAffectAttrib{ funcs: out_funcs, states: out_states });

                    }

                    world.entity_mut(entity).insert(UixAffect(out_affect_attribs));
                }

                //
                for attrib_ind in stuff_node.attribs.clone() {
                    let attrib=stuff.all_attribs.get(attrib_ind).unwrap().clone();
                    attrib(entity,world);
                }

                //
                // let parent_entity_val=self_entity_from_world(world, parent_entity);

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

    //stuff_result.
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

        // let world=context.core_mut();

        data.with_data_ref_ext(|data:&StuffResult|{
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
        // let world=context.core_mut();
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        Ok(Value::string(format!("{entity}")))
    }).custom_ref::<Entity>().end();

    //
    #[derive(Clone)]
    struct NodeData(Value);

    //get node.data
    lib_scope.field_named("data", |mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core_mut();
        let node=self_entity_from_world(world, entity);
        Ok(Value::custom_unmanaged(NodeData(node)))
    }).custom_ref::<Entity>().end();

    //get node_data.field
    lib_scope.field(|context|{
        let node_data:NodeData=context.param(0).as_custom().data_clone()?;
        let entity:Entity = node_data.0.as_custom().data_clone()?;
        let field = context.param(1).get_string().unwrap();
        let world=context.core();

        let v=world.entity(entity).get::<UixData>()
            .and_then(|data|data.data.get(&field))
            .map(|v|v.clone_leaf())
            .unwrap_or(Value::Nil);

        Ok(v)
    }).custom_ref::<NodeData>().str().end();

    //set node_data.field
    lib_scope.field(|mut context|{
        let node_data:NodeData=context.param(0).as_custom().data_clone()?;
        let entity:Entity = node_data.0.as_custom().data_clone()?;
        let field = context.param(1).get_string().unwrap();
        let to=context.param(2);

        let world=context.core_mut();

        let mut entity_mut=world.entity_mut(entity);
        let mut data=entity_mut.entry::<UixData>().or_default();
        let mut data=data.get_mut();
        data.data.insert(field, to.clone_root());

        Ok(Value::Void)
    }).custom_ref::<NodeData>().str().any().end();
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
            .push((listener.clone_root(),false));

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
        env.as_custom().with_data_ref_ext(|data:&StuffResultEnv|Ok(Value::int(data.by_ind.len())))
    }).custom_ref::<StuffResultEnv>().end();

    //get stuff_result_env.int
    lib_scope.field(|mut context|{
        let env=context.param(0);

        env.as_custom().with_data_ref_ext(|data:&StuffResultEnv|{
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

        named_env.env.as_custom().with_data_ref_ext(|data:&StuffResultEnv|{
            Ok(Value::int(data.by_name.get(&named_env.name).map(|x|x.len()).unwrap_or(0)))
        })
    }).custom_ref::<StuffResultNamedEnv>().end();


    //get stuff_result_named_env.int
    lib_scope.field(|mut context|{
        let named_env:StuffResultNamedEnv=context.param(0).as_custom().data_clone()?;

        named_env.env.as_custom().with_data_ref_ext(|data:&StuffResultEnv|{
            Ok(data.by_name.get(&named_env.name).and_then(|v|{
                context.param(1).as_index(v.len()).map(|index|v.get(index).unwrap().clone())
            }).unwrap_or(Value::Nil))
        })
    }).custom_ref::<StuffResultNamedEnv>().int().end();
}

