
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, ecs::component::Component, platform::collections::{HashMap, HashSet}, prelude::{ ChildOf, Children, Entity, Resource, World}};
use bevy_table_ui::*;
use script_lang::*;

use crate::UixSelf;

use super::{event::UixUserEvent, UixEnv, UixEventListener, UixName};

/*
TODO
* add root node to core

* dont return parent for root
* return nil on attribs for root,
* don't allow setting attribs for root, give error

*/


pub fn uival_to_script_value(uival:UiVal) -> Value {
    match uival {
        UiVal::None => {
            Value::Nil
        }
        UiVal::Px(p) => {
            Value::int(p as i32)
        }
        UiVal::Scale(s) => {
            Value::float(s)
        }
    }
}

pub fn script_value_to_uival(v:Value) -> Result<UiVal,MachineError> {
    if v.is_int() {
        return Ok(UiVal::Px(v.as_float() as f32));
    }

    if v.is_float() {
        return Ok(UiVal::Scale(v.as_float() as f32));
    }

    if v.is_nil() {
        return Ok(UiVal::None);
    }

    if v.is_string() {
        let v=v.as_string();
        let v= v.trim();

        if v.ends_with("%") {
            if let Ok(v)=v[0..v.len()-1].parse::<f32>() {
                return Ok(UiVal::Scale(v*0.01));
            }
        }
    }

    Err(script_lang::MachineError::method("Expected float, int, percent or nil"))
}


pub fn col_to_script_value(col:Color) -> Value {
    Value::custom_unmanaged_mut(script_lang::Vec4(col.to_srgba().to_f32_array().map(|x|x.into())))
}
pub fn script_value_to_col(val:Value) -> Result<Color,MachineError> {
    let v=val.as_custom();

    if v.is_type::<script_lang::Vec4>() {
        let v=v.data_clone::<Vec4>()?.0.map(|x|x.clamp(0.0,1.0) as f32);
        Ok(Color::srgba(v[0],v[1],v[2],v[3]))
    } else if v.is_type::<script_lang::IVec4>() {
        let v=v.data_clone::<IVec4>()?.0.map(|x|x.clamp(0,255) as u8);
        Ok(Color::srgba_u8(v[0],v[1],v[2],v[3]))

    } else {
        Err(MachineError::method("Expected vec4 or ivec4"))
    }
}

pub fn script_value_to_bool(val:Value) -> Result<bool,MachineError> {
    val.get_bool().ok_or_else(||MachineError::method("expected bool"))
}
pub fn script_value_to_float(val:Value) -> Result<f32,MachineError> {
    if let Some(x)=val.get_float() {
        Ok(x as f32)
    } else if let Some(x)=val.get_int() {
        Ok(x as f32)
    } else {
        Err(MachineError::method("expected float"))
    }
}
pub fn script_value_to_int(val:Value) -> Result<i32,MachineError> {
    val.get_int().map(|x|x as i32).ok_or_else(||MachineError::method("expected int"))
}
pub fn script_value_to_uint(val:Value) -> Result<u32,MachineError> {
    val.get_int().map(|x|x as i32).ok_or_else(||MachineError::method("expected int"))
        .and_then(|x|if x<0{Err(MachineError::method("expected uint"))}else{Ok(x as u32)})
}

pub fn script_value_to_string(val:Value) -> Result<String,MachineError> {
    val.get_string().ok_or_else(||MachineError::method("expected string")).map(|x|x.to_string())
}

fn entity_get_field(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,&World)->Value) {
    lib_scope.field_named(field,move|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core();
        Ok(f(entity,world))
    }).custom_ref::<Entity>().end();
}

fn entity_get_field_mut(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,&mut World)->Value) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let world=context.core_mut();
        Ok(f(entity,world))
    }).custom_ref::<Entity>().end();
}

fn entity_set_field_mut(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,Value,&mut World,)->Result<(),MachineError>) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let to_val=context.param(1);
        let world=context.core_mut();
        f(entity,to_val,world)?;
        Ok(Value::Void)
    }).custom_ref::<Entity>().any().end();
}

fn entity_set_field_mut2<T:'static>(field:&str,lib_scope:&mut LibScope<World>,
    fs:fn(Entity,T,&mut World,),
    fv:fn(Value,)->Result<T,MachineError>,
) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let to_val=context.param(1);
        let world=context.core_mut();
        let v=fv(to_val)?;
        fs(entity,v,world);
        Ok(Value::Void)
    }).custom_ref::<Entity>().any().end();
}

pub fn register(lib_scope:&mut LibScope<World>) {
    entity_get_field("row_width_scale",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().row_width_scale)
    });
    entity_get_field("col_height_scale",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().col_height_scale)
    });

    // entity_set_field_mut2("row_width_scale",lib_scope,|entity,val,world|{
    //     world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=val;
    // },|val|script_value_to_float(val));

    entity_set_field_mut("row_width_scale",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=script_value_to_float(val)?; Ok(())
    });
    entity_set_field_mut("col_height_scale",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().col_height_scale=script_value_to_float(val)?; Ok(())
    });

    entity_get_field("padding_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.left)
    });
    entity_get_field("padding_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.right)
    });
    entity_get_field("padding_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.top)
    });
    entity_get_field("padding_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.bottom)
    });

    entity_set_field_mut("padding_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("padding_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.bottom=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("margin_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.left)
    });
    entity_get_field("margin_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.right)
    });
    entity_get_field("margin_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.top)
    });
    entity_get_field("margin_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.bottom)
    });

    entity_set_field_mut("margin_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("margin_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.bottom=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("border_left",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.left)
    });
    entity_get_field("border_right",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.right)
    });
    entity_get_field("border_top",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.top)
    });
    entity_get_field("border_bottom",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.bottom)
    });

    entity_set_field_mut("border_left",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.left=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_right",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.right=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_top",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.top=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("border_bottom",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.bottom=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("hgap",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap)
    });
    entity_get_field("vgap",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().vgap)
    });

    entity_set_field_mut("hgap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().hgap=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vgap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().vgap=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("hexpand",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().hexpand)
    });
    entity_get_field("vexpand",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().vexpand)
    });

    entity_set_field_mut("hexpand",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().hexpand=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vexpand",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().vexpand=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("hfill",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().hfill)
    });
    entity_get_field("vfill",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().vfill)
    });

    entity_set_field_mut("hfill",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().hfill=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vfill",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().vfill=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("hscroll",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().hscroll)
    });
    entity_get_field("vscroll",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().vscroll)
    });

    entity_set_field_mut("hscroll",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().hscroll=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("vscroll",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().vscroll=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("float",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiFloat>().cloned().unwrap_or_default().float)
    });

    entity_set_field_mut("float",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFloat>().or_default().get_mut().float=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("disable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiDisable>().cloned().unwrap_or_default().disable)
    });

    entity_set_field_mut("disable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiDisable>().or_default().get_mut().disable=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("hide",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiHide>().cloned().unwrap_or_default().hide)
    });
    entity_set_field_mut("hide",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiHide>().or_default().get_mut().hide=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("lock",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiLock>().cloned().unwrap_or_default().lock)
    });
    entity_set_field_mut("lock",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiLock>().or_default().get_mut().lock=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("span",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiSpan>().cloned().unwrap_or_default().span)
    });
    entity_set_field_mut("span",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSpan>().or_default().get_mut().span=script_value_to_uint(val)?; Ok(())
    });

    entity_get_field("halign",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().halign)
    });
    entity_get_field("valign",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().valign)
    });

    entity_set_field_mut("halign",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().halign=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("valign",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().valign=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("width",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().width)
    });
    entity_get_field("height",lib_scope,|entity,world|{
        uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().height)
    });

    entity_set_field_mut("width",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().width=script_value_to_uival(val)?; Ok(())
    });
    entity_set_field_mut("height",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().height=script_value_to_uival(val)?; Ok(())
    });

    entity_get_field("hoverable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiHoverable>().cloned().unwrap_or_default().enable)
    });

    entity_set_field_mut("hoverable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiHoverable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("pressable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("press_always",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().always)
    });
    entity_get_field("press_physical",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().physical)
    });

    entity_set_field_mut("pressable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("press_always",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().always=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("press_physical",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().physical=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("draggable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiDraggable>().cloned().unwrap_or_default().enable)
    });

    entity_set_field_mut("draggable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiDraggable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("selectable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("selected",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().selected)
    });
    entity_get_field("select_group",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().group)
    });

    entity_set_field_mut("selectable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("selected",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().selected=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("select_group",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().group=script_value_to_string(val)?; Ok(())
    });

    entity_get_field("focusable",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().enable)
    });
    entity_get_field("focused",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().focused)
    });
    entity_get_field("focus_group",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().group)
    });
    entity_get_field("focus_tab_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().tab_exit)
    });
    entity_get_field("focus_hdir_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_exit)
    });
    entity_get_field("focus_vdir_exit",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_exit)
    });
    entity_get_field("focus_hdir_wrap",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_wrap)
    });
    entity_get_field("focus_vdir_wrap",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_wrap)
    });
    entity_get_field("focus_hdir_press",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_press)
    });
    entity_get_field("focus_vdir_press",lib_scope,|entity,world|{
        Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_press)
    });

    entity_set_field_mut("focusable",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().enable=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focused",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().focused=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_group",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().group=script_value_to_int(val)?; Ok(())
    });
    entity_set_field_mut("focus_tab_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().tab_exit=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_hdir_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_exit=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_exit",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_exit=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_hdir_wrap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_wrap=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_wrap",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_wrap=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_hdir_press",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_press=script_value_to_bool(val)?; Ok(())
    });
    entity_set_field_mut("focus_vdir_press",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_press=script_value_to_bool(val)?; Ok(())
    });

    entity_get_field("color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut("color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().back_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });

    entity_get_field("padding_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.padding_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_get_field("border_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.border_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_get_field("margin_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.margin_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_get_field("cell_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.cell_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });


    entity_set_field_mut("padding_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().padding_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("border_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().border_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("margin_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().margin_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("cell_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().cell_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });

    entity_get_field("text_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.text_color.get(&None)).cloned().unwrap_or(Color::NONE))
    });
    entity_set_field_mut("text_color",lib_scope,|entity,val,world|{
        *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().text_color.entry(None).or_default()=script_value_to_col(val)?; Ok(())
    });


    entity_get_field("image_color",lib_scope,|entity,world|{
        col_to_script_value(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().color)
    });
    entity_get_field("image_width",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().width_scale)
    });
    entity_get_field("image_height",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().height_scale)
    });

    entity_set_field_mut("image",lib_scope,|entity,val,world|{
        let handle=world.resource::<AssetServer>().load(PathBuf::from(script_value_to_string(val)?));
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().handle=handle;
        Ok(())
    });
    entity_set_field_mut("image_color",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().color=script_value_to_col(val)?; Ok(())
    });
    entity_set_field_mut("image_width",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().width_scale=script_value_to_float(val)?; Ok(())
    });
    entity_set_field_mut("image_height",lib_scope,|entity,val,world|{
        world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().height_scale=script_value_to_float(val)?; Ok(())
    });

    entity_get_field("text",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().value)
    });
    entity_get_field("font_size",lib_scope,|entity,world|{
        Value::float(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().font_size)
    });
    entity_get_field("text_hlen",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().hlen)
    });
    entity_get_field("text_vlen",lib_scope,|entity,world|{
        Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().vlen)
    });
    entity_get_field("text_halign",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().halign.to_string())
    });
    entity_get_field("text_valign",lib_scope,|entity,world|{
        Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().valign.to_string())
    });

    entity_set_field_mut("text",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.value=script_value_to_string(val)?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("font",lib_scope,|entity,val,world|{
        let handle=world.resource::<AssetServer>().load(PathBuf::from(script_value_to_string(val)?));
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.font=handle;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("font_size",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.font_size=script_value_to_float(val)?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_hlen",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.hlen=script_value_to_uint(val)?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_vlen",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.vlen=script_value_to_uint(val)?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_halign",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.halign=val.get_parse().ok_or_else(||MachineError::method("expected halign"))?;
        c.update=true;
        Ok(())
    });
    entity_set_field_mut("text_valign",lib_scope,|entity,val,world|{
        let mut e=world.entity_mut(entity);
        let mut c=e.entry::<UiText>().or_default();
        let mut c=c.get_mut();
        c.valign=val.get_parse().ok_or_else(||MachineError::method("expected valign"))?;
        c.update=true;
        Ok(())
    });

    entity_get_field_mut("parent",lib_scope,|entity,world|{
        world.entity(entity).get::<ChildOf>()
            .map(|parent|parent.parent())
            .map(|parent_entity|self_entity_from_world(world,parent_entity))
            .unwrap_or(Value::Nil)
    });

    entity_set_field_mut("parent",lib_scope,|entity,val,world|{

        //let parent=val.as_custom().data_copy()?;
        // let mut e=world.entity_mut(entity);
        // e.set_parent(parent);

        //do nothing
        //  could check if parent == entity.parent and return err if not
        //  or allow changing parent ..

        Ok(())
    });

    entity_get_field("scaling",lib_scope,|entity,world|{
        world.entity(entity).get::<UiRoot>().map(|c|Value::float(c.scaling.min(0.0))).unwrap_or_default()
    });


    // lib_scope.field(true,node_get_field)
    //     .custom_ref::<Entity>().any().end();

    // lib_scope.field(true,node_set_field)
    //     .custom_ref::<Entity>().any().any().end();


    //env(entity,str,int?)
    lib_scope.method("env",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let name=context.param(1).get_string().unwrap();
        let ind=context.param(1).as_int();

        let world=context.core();

        if let Some(c)=world.entity(entity).get::<UixEnv>() {
            if let Some(v)=c.env.get(&name) { //.and_then(|v|v.get(field)).cloned()
                if let Some(ind)=calc_ind(ind,v.len()) {
                    // return Ok(Value::custom_unmanaged(v[ind]));
                    return Ok(v[ind].clone());
                }
            }
        }


        Ok(Value::Nil)
    }).custom_ref::<Entity>().str().optional().int().end();

    //get_field(env,str)
    lib_scope.field(true,|context|{
        let env:Env= context.param(0).as_custom().data_clone()?;
        // let entity=env.entity.as_custom().data_copy::<Entity>()?;
        let name=context.param(1).get_string().unwrap();

        Ok(Value::custom_unmanaged(EnvEntry{ entity: env.entity, name }))
    }).custom_ref::<Env>().str().end();

    //get_field(env_entry,int)
    lib_scope.field(true,|context|{
        let env_entry:EnvEntry= context.param(0).as_custom().data_clone()?;
        let ind=context.param(1).as_int();

        let world=context.core();
        let entity:Entity=env_entry.entity.as_custom().data_copy()?;
        // if let Some(v)=world.entity(entity).get::<UixEnv>().and_then(|c|c.env.get(&env_entry.name)) {
        //     if let Some(ind)=calc_ind(ind,v.len()) {
        //         return Ok(v[ind].clone());
        //     }
        // }
        if let Some(c)=world.entity(entity).get::<UixEnv>() {
            println!("1");
            if let Some(v)=c.env.get(&env_entry.name) {
            println!("2");
                if let Some(ind)=calc_ind(ind,v.len()) {
            println!("3");
                    // return Ok(Value::custom_unmanaged(v[ind]));
                    return Ok(v[ind].clone());
                }
            }
        }

            println!("4");
        Ok(Value::Nil)
    }).custom_ref::<EnvEntry>().int().end();

    //
    lib_scope.field(true,|context|{
        //do nothing
        Ok(Value::Void)
    }).custom_ref::<EnvEntry>().int().any().end();

    //add_child(entity)
    lib_scope.method("add_child",|mut context|{
        let parent_entity:Entity = context.param(0).as_custom().data_copy()?;
        let names = HashSet::<StringT>::from_iter((1..context.params_num()).map(|i|context.param(i).get_string().unwrap()));
        let world=context.core_mut();
        let child_entity=world.spawn(( UiLayoutComputed::default(), )).id();

        {
            if !names.is_empty() {
                let mut e=world.entity_mut(child_entity);
                e.insert((UixName{ names:names.clone() },));
            }

            let parent_entity_val=self_entity_from_world(world, parent_entity);
            let mut e=world.entity_mut(parent_entity);

            e.add_child(child_entity);

            let mut env=e.entry::<UixEnv>().or_default();

            for n in names.iter() {
                env.get_mut().env.entry(n.clone()).or_default().push(parent_entity_val.clone());
            }
        }

        let child_entity_val=self_entity_from_world(world, child_entity);
        Ok(child_entity_val)
        // Ok(Value::custom_unmanaged(child_entity))
    }).custom_ref::<Entity>().optional().str().variadic_end();

    //child(entity,int)
    lib_scope.method("child",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let child_ind=context.param(1).as_int();
        let world=context.core_mut();

        if child_ind>=0 {
            if let Some(children)=world.entity(entity).get::<Children>() {
                if let Some(&child_entity)=children.get(child_ind as usize) {
                    let child_entity_val=self_entity_from_world(world, child_entity);
                    return Ok(child_entity_val);
                    // return Ok(Value::custom_unmanaged(child_entity));
                }
            }
        }

        Ok(Value::Nil)
    }).custom_ref::<Entity>().int().end();

    //children_num(entity)
    lib_scope.method("children_num",|context|{
        let world=context.core();
        let entity:Entity = context.param(0).as_custom().data_copy()?;

        let n=world.entity(entity).get::<Children>().map(|children|children.len()).unwrap_or_default();

        Ok(Value::int(n))
    }).custom_ref::<Entity>().end();

    //remove(entity)
    lib_scope.method("remove",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        // let entity_val=context.param(0);
        let world=context.core_mut();

        //
        if let Some(parent_entity)=world.entity(entity).get::<ChildOf>().map(|c|c.parent()) {
            if let Some(names)=world.get::<UixName>(entity).map(|c|c.names.clone()) {
                if let Some(mut c)=world.get_mut::<UixEnv>(entity) {
                    for n in names {
                        let mut b=false;

                        if let Some(v)=c.env.get_mut(&n) {
                            if let Some(p)=v.iter().position(|x|{
                                x.as_custom().data_copy::<Entity>().map(|x|x==entity).unwrap_or_default()
                            }) {
                                v.remove(p);
                                b=v.is_empty();
                            }
                        }

                        if b {
                            c.env.remove(&n);
                        }
                    }
                }
            }
        }

        //
        let mut commands=world.commands();
        let mut e=commands.entity(entity);

        // e.remove_parent();
        e.remove::<ChildOf>();
        // e.despawn_recursive();
        e.remove::<Children>();

        Ok(Value::Void)
    }).custom_ref::<Entity>().end();

    //parent(entity)
    lib_scope.method("parent",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;

        let world=context.core_mut();
        let Some(parent_entity)=world.entity(entity).get::<ChildOf>().map(|parent|parent.parent()) else { return Ok(Value::Nil); };
        // let parent_entity=parent.parent();
        let parent_entity_val=self_entity_from_world(world,parent_entity);
        // Ok(Value::custom_unmanaged(parent.parent()))
        Ok(parent_entity_val)
    }).custom_ref::<Entity>().end();

    //string(entity)
    lib_scope.method("string",|mut context|{
        let world=context.core_mut();
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        Ok(Value::string(format!("{entity}")))
    }).custom_ref::<Entity>().end();

    //send(entity,str,any ...)
    lib_scope.method("send",|mut context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let event = context.param(1).get_string().unwrap();
        let params=(2..context.params_num()).map(|i|context.param(i).clone_root()).collect();
        let world=context.core_mut();
        world.send_event(UixUserEvent{ entity, event, params });
        Ok(Value::Void)
    }).custom_ref::<Entity>().str().optional().any().variadic_end();

    //call(stuff,ind,entity)
    lib_scope.method("call",|mut context|{
        let stuff=context.param(0).as_custom();
        let top_entity:Entity = context.param(2).as_custom().data_copy()?;
        let stub_ind=context.param(1).as_int().abs() as usize;

        let world=context.core_mut();

        stuff.with_data_ref(|stuff:&Stuff|{
            let mut element_entity_map = HashMap::<usize,Entity>::from_iter([(0,top_entity)]);
            let Some(node_range)=stuff.all_stubs.get(&stub_ind).cloned() else {return Ok(Value::Nil);};

            for node_ind in node_range {
                let (element_ind,parent_element_ind,attribs_range,names_range) = stuff.all_nodes.get(node_ind).unwrap().clone();
                let names=stuff.all_names.get(names_range).unwrap();

                let mut e=world.spawn((UiLayoutComputed::default(),));

                //
                let &parent_entity=element_entity_map.get(&parent_element_ind).unwrap();
                // e.set_parent(parent_entity);
                e.insert(ChildOf(parent_entity));

                //
                let entity=e.id();
                element_entity_map.insert(element_ind, entity);

                //
                if !names.is_empty() {
                    e.insert((UixName{ names:HashSet::from_iter(names.iter().map(|x|x.clone())) },));
                }

                //
                for attrib_ind in attribs_range {
                    let attrib=stuff.all_attribs.get(attrib_ind).unwrap().0.clone();
                    attrib(entity,world);
                }

                //
                let parent_entity_val=self_entity_from_world(world, parent_entity);

                let mut pe=world.entity_mut(parent_entity);
                let mut env=pe.entry::<UixEnv>().or_default();

                for n in names.iter() {
                    env.get_mut().env.entry(n.clone()).or_default().push(parent_entity_val.clone());
                }
            }

            let element_entity_map2: HashMap<usize, Value>=element_entity_map.iter().map(|(&k,&v)|{
                let vv=self_entity_from_world(world, v);
                (k,vv)
            }).collect();

            Ok(Value::custom_unmanaged(StuffResult(element_entity_map2)))
            // Ok(Value::custom_unmanaged(StuffResult(element_entity_map)))
        })
    }).custom_ref::<Stuff>().int().custom_ref::<Entity>().end();

    //
    lib_scope.field(true,|mut context|{
        let data=context.param(0).as_custom();
        let ind=context.param(1).as_int().abs() as usize;

        let world=context.core_mut();

        data.with_data_ref(|data:&StuffResult|{
            let Some(entity)=data.0.get(&ind).cloned() else {
                return Ok(Value::Nil);
            };

            // let entity_val=self_entity_from_world(world, entity);
            // Ok(entity_val)
            Ok(entity)
            // Ok(data.0.get(&ind).map(|&x|{
            //     Value::custom_unmanaged(x)
            // }).unwrap_or(Value::Nil))
        })
    }).custom_ref::<StuffResult>().int().end();

    //
    lib_scope.method("add_event_listener",|mut context|{

        let entity:Entity = context.param(0).as_custom().data_copy()?;
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

#[derive(Resource,)]
pub struct UixLibScope(pub Arc<LibScope<World>>);

impl Default for UixLibScope {
    fn default() -> Self {
        let mut lib_scope=LibScope::new_full();

        register(&mut lib_scope);

        Self(Arc::new(lib_scope))
    }
}

#[derive(Resource,Default)]
pub struct UixGcScope(pub Arc<Mutex<GcScope>>);


#[derive(Clone)]
pub struct AttribFunc(pub Arc<dyn Fn(Entity,&mut World)+Send+Sync>);
impl std::fmt::Debug for AttribFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ElementAttribFunc").finish()
    }
}
 //
 pub struct Stuff {
    // pub root_entity:Entity,
    pub all_stubs: HashMap<usize, Range<usize>> , //[root/stub_element_ind]=(nodes_start,nodes_end)
    pub all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>,//(element_ind,parent_element_ind,(attribs_start,attribs_end),names_range)
    pub all_attribs:Vec<AttribFunc>, //[]=func
    pub all_names : Vec<script_lang::StringT>,
}

pub struct StuffResult(HashMap<usize,Value>);

#[derive(Clone)]
struct Env {
    entity:script_lang::Value,
}
#[derive(Clone)]
struct EnvEntry {
    entity:script_lang::Value,
    name:script_lang::StringT,
}
pub fn calc_ind(ind : IntT , len:usize) -> Option<usize> {
    let len : IntT = len.try_into().unwrap_or_default();

    if ind >= len || (ind < 0 && ind.abs() > len) {
        None
    } else {
        let ind = if ind<0 {len+ind} else {ind};
        Some(ind.try_into().unwrap_or_default())
    }
}

    // pub fn new(entity:Entity) -> Self {
    //     Self { entity: script_lang::Value::custom_rc(entity) }
    // }
pub fn self_entity_from_world(world : &mut World,entity:Entity) -> script_lang::Value {
    world.entity_mut(entity).entry::<UixSelf>().or_insert_with(||UixSelf::new(entity)).get().entity.clone_leaf()
}