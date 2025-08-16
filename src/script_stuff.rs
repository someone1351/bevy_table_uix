
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, platform::collections::{HashMap, HashSet}, prelude::{ ChildOf, Children, Entity, Resource, World}};
use bevy_table_ui::*;
use script_lang::*;

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

fn node_get_field(context:FuncContext<World>) -> Result<Value,MachineError> {
    let entity:Entity = context.param(0).as_custom().data_copy()?;
    let field=context.param(1).as_string();

    // let world=context.core();
    let world=context.core();

    Ok(match field.as_str() {
        "row_width_scale" => Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().row_width_scale),
        "col_height_scale" => Value::float(world.entity(entity).get::<UiCongruent>().cloned().unwrap_or_default().col_height_scale),

        "padding_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.left),
        "padding_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.right),
        "padding_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.top),
        "padding_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().padding.bottom),
        "border_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.left),
        "border_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.right),
        "border_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.top),
        "border_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().border.bottom),
        "margin_left" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.left),
        "margin_right" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.right),
        "margin_top" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.top),
        "margin_bottom" => uival_to_script_value(world.entity(entity).get::<UiEdge>().cloned().unwrap_or_default().margin.bottom),

        "hgap" => uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().hgap),
        "vgap" => uival_to_script_value(world.entity(entity).get::<UiGap>().cloned().unwrap_or_default().vgap),

        "hexpand" => uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().hexpand),
        "vexpand" => uival_to_script_value(world.entity(entity).get::<UiExpand>().cloned().unwrap_or_default().vexpand),

        "hfill" => uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().hfill),
        "vfill" => uival_to_script_value(world.entity(entity).get::<UiFill>().cloned().unwrap_or_default().vfill),

        "hscroll" => uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().hscroll),
        "vscroll" => uival_to_script_value(world.entity(entity).get::<UiScroll>().cloned().unwrap_or_default().vscroll),

        "float" => Value::float(world.entity(entity).get::<UiFloat>().cloned().unwrap_or_default().float),

        "disable" => Value::bool(world.entity(entity).get::<UiDisable>().cloned().unwrap_or_default().disable),

        "hide" => Value::bool(world.entity(entity).get::<UiHide>().cloned().unwrap_or_default().hide),

        "lock" => Value::bool(world.entity(entity).get::<UiLock>().cloned().unwrap_or_default().lock),

        "span" => Value::int(world.entity(entity).get::<UiSpan>().cloned().unwrap_or_default().span),

        "halign" => uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().halign),
        "valign" => uival_to_script_value(world.entity(entity).get::<UiAlign>().cloned().unwrap_or_default().valign),

        "width" => uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().width),
        "height" => uival_to_script_value(world.entity(entity).get::<UiSize>().cloned().unwrap_or_default().height),

        "hoverable" => Value::bool(world.entity(entity).get::<UiHoverable>().cloned().unwrap_or_default().enable),

        "pressable" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().enable),
        "press_always" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().always),
        "press_physical" => Value::bool(world.entity(entity).get::<UiPressable>().cloned().unwrap_or_default().physical),

        "draggable" => Value::bool(world.entity(entity).get::<UiDraggable>().cloned().unwrap_or_default().enable),

        "selectable" => Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().enable),
        "selected" => Value::bool(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().selected),
        "select_group" => Value::string(world.entity(entity).get::<UiSelectable>().cloned().unwrap_or_default().group),

        "focusable" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().enable),
        "focused" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().focused),
        "focus_group" => Value::int(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().group),
        "focus_tab_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().tab_exit),
        "focus_hdir_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_exit),
        "focus_vdir_exit" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_exit),
        "focus_hdir_wrap" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_wrap),
        "focus_vdir_wrap" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_wrap),
        "focus_hdir_press" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().hdir_press),
        "focus_vdir_press" => Value::bool(world.entity(entity).get::<UiFocusable>().cloned().unwrap_or_default().vdir_press),

        "color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE)),
        "padding_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.padding_color.get(&None)).cloned().unwrap_or(Color::NONE)),
        "border_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.back_color.get(&None)).cloned().unwrap_or(Color::NONE)),
        "margin_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.margin_color.get(&None)).cloned().unwrap_or(Color::NONE)),
        "cell_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.cell_color.get(&None)).cloned().unwrap_or(Color::NONE)),
        "text_color" => col_to_script_value(world.entity(entity).get::<UiAffect>().and_then(|x|x.text_color.get(&None)).cloned().unwrap_or(Color::NONE)),

        "image_color" => col_to_script_value(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().color),
        "image_width" => Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().width_scale),
        "image_height" => Value::float(world.entity(entity).get::<UiImage>().cloned().unwrap_or_default().height_scale),

        "text" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().value),
        "font_size" => Value::float(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().font_size),
        "text_hlen" => Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().hlen),
        "text_vlen" => Value::int(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().vlen),
        "text_halign" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().halign.to_string()),
        "text_valign" => Value::string(world.entity(entity).get::<UiText>().cloned().unwrap_or_default().valign.to_string()),

        "parent" => world.entity(entity).get::<ChildOf>().map(|parent|Value::custom_unmanaged(parent.parent())).unwrap_or(Value::Nil),
        // "children"   => world.entity(entity).get::<Children>().map(|children|children.iter()),

        "env" => {
            let name=context.param(1).get_string().unwrap();
            Value::custom_unmanaged(EnvEntry{ entity, name })
        },
        _ => Value::Nil,
    })
}

fn node_set_field(mut context:FuncContext<World>) -> Result<Value,MachineError> {
    let entity:Entity = context.param(0).as_custom().data_copy()?;
    let field=context.param(1).as_string();
    let val=context.param(2);

    // let (world,asset_server)=context.get_core_mut();
    let world=context.core_mut();
    let asset_server=world.resource::<AssetServer>();

    match field.as_str() {
        "row_width_scale" => {
            world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().row_width_scale=script_value_to_float(val)?;
        }
        "col_height_scale" => {
            world.entity_mut(entity).entry::<UiCongruent>().or_default().get_mut().col_height_scale=script_value_to_float(val)?;
        }

        "padding_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.left=script_value_to_uival(val)?;
        }
        "padding_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.right=script_value_to_uival(val)?;
        }
        "padding_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.top=script_value_to_uival(val)?;
        }
        "padding_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().padding.bottom=script_value_to_uival(val)?;
        }
        "border_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.left=script_value_to_uival(val)?;
        }
        "border_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.right=script_value_to_uival(val)?;
        }
        "border_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.top=script_value_to_uival(val)?;
        }
        "border_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().border.bottom=script_value_to_uival(val)?;
        }
        "margin_left" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.left=script_value_to_uival(val)?;
        }
        "margin_right" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.right=script_value_to_uival(val)?;
        }
        "margin_top" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.top=script_value_to_uival(val)?;
        }
        "margin_bottom" => {
            world.entity_mut(entity).entry::<UiEdge>().or_default().get_mut().margin.bottom=script_value_to_uival(val)?;
        }

        "hgap" => {
            world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().hgap=script_value_to_uival(val)?;
        }
        "vgap" => {
            world.entity_mut(entity).entry::<UiGap>().or_default().get_mut().vgap=script_value_to_uival(val)?;
        }

        "hexpand" => {
            world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().hexpand=script_value_to_uival(val)?;
        }
        "vexpand" => {
            world.entity_mut(entity).entry::<UiExpand>().or_default().get_mut().vexpand=script_value_to_uival(val)?;
        }

        "hfill" => {
            world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().hfill=script_value_to_uival(val)?;
        }
        "vfill" => {
            world.entity_mut(entity).entry::<UiFill>().or_default().get_mut().vfill=script_value_to_uival(val)?;
        }

        "hscroll" => {
            world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().hscroll=script_value_to_uival(val)?;
        }
        "vscroll" => {
            world.entity_mut(entity).entry::<UiScroll>().or_default().get_mut().vscroll=script_value_to_uival(val)?;
        }

        "float" => {
            world.entity_mut(entity).entry::<UiFloat>().or_default().get_mut().float=script_value_to_bool(val)?;
        }

        "disable" => {
            world.entity_mut(entity).entry::<UiDisable>().or_default().get_mut().disable=script_value_to_bool(val)?;
        }

        "hide" => {
            world.entity_mut(entity).entry::<UiHide>().or_default().get_mut().hide=script_value_to_bool(val)?;
        }

        "lock" => {
            world.entity_mut(entity).entry::<UiLock>().or_default().get_mut().lock=script_value_to_bool(val)?;
        }

        "span" => {
            world.entity_mut(entity).entry::<UiSpan>().or_default().get_mut().span=script_value_to_uint(val)?;
        }

        "halign" => {
            world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().halign=script_value_to_uival(val)?;
        },
        "valign" => {
            world.entity_mut(entity).entry::<UiAlign>().or_default().get_mut().valign=script_value_to_uival(val)?;
        },

        "width" => {
            world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().width=script_value_to_uival(val)?;
        },
        "height" => {
            world.entity_mut(entity).entry::<UiSize>().or_default().get_mut().height=script_value_to_uival(val)?;
        },

        "hoverable" => {
            world.entity_mut(entity).entry::<UiHoverable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }

        "pressable" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "press_always" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().always=script_value_to_bool(val)?;
        }
        "press_physical" => {
            world.entity_mut(entity).entry::<UiPressable>().or_default().get_mut().physical=script_value_to_bool(val)?;
        }

        "draggable" => {
            world.entity_mut(entity).entry::<UiDraggable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }

        "selectable" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "selected" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().selected=script_value_to_bool(val)?;
        }
        "select_group" => {
            world.entity_mut(entity).entry::<UiSelectable>().or_default().get_mut().group=script_value_to_string(val)?;
        }

        "focusable" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().enable=script_value_to_bool(val)?;
        }
        "focused" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().focused=script_value_to_bool(val)?;
        }
        "focus_group" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().group=script_value_to_int(val)?;
        }
        "focus_tab_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().tab_exit=script_value_to_bool(val)?;
        }
        "focus_hdir_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_exit=script_value_to_bool(val)?;
        }
        "focus_vdir_exit" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_exit=script_value_to_bool(val)?;
        }
        "focus_hdir_wrap" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_wrap=script_value_to_bool(val)?;
        }
        "focus_vdir_wrap" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_wrap=script_value_to_bool(val)?;
        }
        "focus_hdir_press" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().hdir_press=script_value_to_bool(val)?;
        }
        "focus_vdir_press" => {
            world.entity_mut(entity).entry::<UiFocusable>().or_default().get_mut().vdir_press=script_value_to_bool(val)?;
        }

        "color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().back_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "padding_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().padding_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "border_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().border_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "margin_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().margin_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "cell_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().cell_color.entry(None).or_default()=script_value_to_col(val)?;
        }
        "text_color" => {
            *world.entity_mut(entity).entry::<UiAffect>().or_default().get_mut().text_color.entry(None).or_default()=script_value_to_col(val)?;
        }

        "image" => {
            let handle=asset_server.load(PathBuf::from(script_value_to_string(val)?));
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiImage>().or_default();
            c.get_mut().handle=handle;
            e.entry::<UiInnerSize>().or_default();
        }
        "image_color" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().color=script_value_to_col(val)?;
        }
        "image_width" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().width_scale=script_value_to_float(val)?;
        },
        "image_height" => {
            world.entity_mut(entity).entry::<UiImage>().or_default().get_mut().height_scale=script_value_to_float(val)?;
        },

        "text" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().value=script_value_to_string(val)?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "font" => {
            let handle=asset_server.load(PathBuf::from(script_value_to_string(val)?));

            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().font=handle;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "font_size" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().font_size=script_value_to_float(val)?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_hlen" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().hlen=script_value_to_uint(val)?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_vlen" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().vlen=script_value_to_uint(val)?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_halign" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().halign=val.get_parse().ok_or_else(||MachineError::method("expected halign"))?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "text_valign" => {
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<UiText>().or_default();
            c.get_mut().valign=val.get_parse().ok_or_else(||MachineError::method("expected valign"))?;
            c.get_mut().update=true;
            e.entry::<UiTextComputed>().or_default();
            e.entry::<UiInnerSize>().or_default();
        }
        "parent" => {
            //let parent=val.as_custom().data_copy()?;
            // let mut e=world.entity_mut(entity);
            // e.set_parent(parent);

            //do nothing
        }
        "env" => {
            //do nothing when tried to be set
        }
        _ => {
            return Err(MachineError::method("invalid field"));
        }
    }

    Ok(Value::Void)
}

pub fn register(lib_scope:&mut LibScope<World>) {

    lib_scope.method("get_field",node_get_field)
        .custom_ref::<Entity>().any().end();

    lib_scope.method("set_field",node_set_field)
        .custom_ref::<Entity>().any().any().end();


    //env(entity,str,int?)
    lib_scope.method("env",|context|{
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let name=context.param(1).get_string().unwrap();
        let ind=context.param(1).as_int();

        let world=context.core();

        if let Some(c)=world.entity(entity).get::<UixEnv>() {
            if let Some(v)=c.env.get(&name) { //.and_then(|v|v.get(field)).cloned()
                if let Some(ind)=calc_ind(ind,v.len()) {
                    return Ok(Value::custom_unmanaged(v[ind]));
                }
            }
        }


        Ok(Value::Nil)
    }).custom_ref::<Entity>().str().optional().int().end();

    //get_field(env_entry,int)
    lib_scope.method("get_field",|context|{
        let env_entry:EnvEntry= context.param(0).as_custom().data_clone()?;
        let ind=context.param(1).as_int();

        let world=context.core();

        if let Some(c)=world.entity(env_entry.entity).get::<UixEnv>() {
            if let Some(v)=c.env.get(&env_entry.name) {
                if let Some(ind)=calc_ind(ind,v.len()) {
                    return Ok(Value::custom_unmanaged(v[ind]));
                }
            }
        }

        Ok(Value::Nil)
    }).custom_ref::<EnvEntry>().int().end();

    //
    lib_scope.method("set_field",|context|{
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

            let mut e=world.entity_mut(parent_entity);

            e.add_child(child_entity);

            let mut env=e.entry::<UixEnv>().or_default();

            for n in names.iter() {
                env.get_mut().env.entry(n.clone()).or_default().push(parent_entity);
            }
        }

        Ok(Value::custom_unmanaged(child_entity))
    }).custom_ref::<Entity>().optional().str().variadic_end();

    //child(entity,int)
    lib_scope.method("child",|context|{
        let world=context.core();
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let child_ind=context.param(1).as_int();

        if child_ind>=0 {
            if let Some(children)=world.entity(entity).get::<Children>() {
                if let Some(&child)=children.get(child_ind as usize) {
                    return Ok(Value::custom_unmanaged(child));
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
        let world=context.core_mut();

        //
        if let Some(parent_entity)=world.entity(entity).get::<ChildOf>().map(|c|c.parent()) {
            if let Some(names)=world.get::<UixName>(entity).map(|c|c.names.clone()) {
                if let Some(mut c)=world.get_mut::<UixEnv>(entity) {
                    for n in names {
                        let mut b=false;

                        if let Some(v)=c.env.get_mut(&n) {
                            if let Some(p)=v.iter().position(|&x|x==entity) {
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
    lib_scope.method("parent",|context|{
        let world=context.core();
        let entity:Entity = context.param(0).as_custom().data_copy()?;
        let Some(parent)=world.entity(entity).get::<ChildOf>() else { return Ok(Value::Nil); };
        Ok(Value::custom_unmanaged(parent.parent()))
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
                let mut pe=world.entity_mut(parent_entity);
                let mut env=pe.entry::<UixEnv>().or_default();

                for n in names.iter() {
                    env.get_mut().env.entry(n.clone()).or_default().push(parent_entity);
                }
            }

            Ok(Value::custom_unmanaged(StuffResult(element_entity_map)))
        })
    }).custom_ref::<Stuff>().int().custom_ref::<Entity>().end();

    //
    lib_scope.method("get_field",|mut context|{
        let data=context.param(0).as_custom();
        let ind=context.param(1).as_int().abs() as usize;

        data.with_data_ref(|data:&StuffResult|{
            Ok(data.0.get(&ind).map(|&x|Value::custom_unmanaged(x)).unwrap_or(Value::Nil))
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

pub struct StuffResult(HashMap<usize,Entity>);

#[derive(Clone)]
struct EnvEntry {
    entity:Entity,
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
