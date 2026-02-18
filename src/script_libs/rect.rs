
use std::{collections::{HashMap, HashSet}, ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, ecs::{component::Component, entity, world::EntityRef}, math::{IVec2, IVec3, IVec4, Rect, Vec2, Vec3, Vec4}, prelude::{ ChildOf, Children, Entity, Resource, World}, text::{Justify, TextColor, TextFont, TextLayout}};
use bevy_table_ui::*;
use script_lang::*;

// use crate::UixSelf;


use super::super::components::*;
use super::super::messages::*;
use super::super::script_vals::*;

use super::super::script_utils::*;


pub fn register_rect(lib_scope:&mut LibScope<World>) {
    //copy(rect)
    lib_scope.method("copy",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(rect))
    }).custom_ref::<Rect>().custom_ref::<Vec2>().end();

    //get rect.contains
    lib_scope.method("contains",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        let point: Vec2=context.param(1).as_custom().data_clone()?;
        Ok(rect.contains(point).into())
    }).custom_ref::<Rect>().custom_ref::<Vec2>().end();

    //get rect.min
    lib_scope.method("min",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(rect.min))
    }).custom_ref::<Rect>().end();

    //get rect.max
    lib_scope.method("max",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(rect.max))
    }).custom_ref::<Rect>().end();

    //get rect.size
    lib_scope.method("size",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged(rect.size()))
    }).custom_ref::<Rect>().end();

    //get rect.sum
    lib_scope.method("sum",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged(Vec2::new(rect.min.x+rect.max.x,rect.min.y+rect.max.y)))
    }).custom_ref::<Rect>().end();

    //get rect.center
    lib_scope.method("center",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged(rect.center()))
    }).custom_ref::<Rect>().end();

    //get rect.left
    lib_scope.method("left",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(rect.min.x.into())
    }).custom_ref::<Rect>().end();

    //get rect.right
    lib_scope.method("right",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(rect.max.x.into())
    }).custom_ref::<Rect>().end();

    //get rect.top
    lib_scope.method("top",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(rect.min.y.into())
    }).custom_ref::<Rect>().end();

    //get rect.bottom
    lib_scope.method("bottom",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(rect.max.y.into())
    }).custom_ref::<Rect>().end();

    //get rect.string
    lib_scope.method("string",|context|{
        let rect: Rect=context.param(0).as_custom().data_clone()?;
        Ok(format!("Rect({}, {}, {}, {})", rect.min.x, rect.min.y, rect.max.x, rect.max.y).into())
    }).custom_ref::<Rect>().end();
}
