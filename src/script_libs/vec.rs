
use std::{collections::{HashMap, HashSet}, ops::Range, path::PathBuf, sync::{Arc, Mutex}};

use bevy::{asset::AssetServer, color::{Color, ColorToComponents}, ecs::{component::Component, entity, world::EntityRef}, math::{IVec2, IVec3, IVec4, Rect, Vec2, Vec3, Vec4}, prelude::{ ChildOf, Children, Entity, Resource, World}, text::{Justify, TextColor, TextFont, TextLayout}};
use bevy_table_ui::*;
use script_lang::*;

// use crate::UixSelf;


use super::super::components::*;
use super::super::messages::*;
use super::super::script_vals::*;

use super::super::script_utils::*;


fn get_field_inds(fields:&str,) -> Vec<(char, Option<usize>)> {
    fields.chars().map(|c|(c,match c {
        'r'|'x'=>Some(0),
        'g'|'y'=>Some(1),
        'b'|'z'=>Some(2),
        'a'|'w'=>Some(3),
        _=>None,
    })).collect()
}

fn get_field_util<T:Copy,const FROM_N: usize>(fields:&str,from:&[T;FROM_N]) -> Result<Vec<T>, String> {
    let field_inds = get_field_inds(fields);

    if field_inds.len()>4 {
        return Err("too many fields".to_string());
    }

    for &(c,field) in field_inds.iter() {
        if field.is_none() || field.unwrap()>FROM_N {
            return Err(format!("invalid field '{c}'"));
        }
    }

    Ok(field_inds.iter().map(|x|from[x.1.unwrap()]).collect())
}

fn get_field_util_float<const FROM_N: usize>(fields:&str,from:&[f32;FROM_N]) -> Result<Value, String> {
    let v=get_field_util::<f32,FROM_N>(fields,from)?;

    Ok(match v.len() {
        1 => Value::float(v[0]),
        2 => Value::custom_unmanaged_mut(Vec2::new(v[0],v[1])),
        3 => Value::custom_unmanaged_mut(Vec3::new(v[0],v[1],v[2])),
        _ => Value::custom_unmanaged_mut(Vec4::new(v[0],v[1],v[2],v[3])),
    })
}

fn get_field_util_int<const FROM_N: usize>(fields:&str,from:&[i32;FROM_N]) -> Result<Value, String> {
    let v=get_field_util::<i32,FROM_N>(fields,from)?;
    Ok(match v.len() {
        1 => Value::int(v[0]),
        2 => Value::custom_unmanaged_mut(IVec2::new(v[0],v[1])),
        3 => Value::custom_unmanaged_mut(IVec3::new(v[0],v[1],v[2])),
        _ => Value::custom_unmanaged_mut(IVec4::new(v[0],v[1],v[2],v[3])),
    })
}

fn set_field_util<T:Copy,const FROM_N: usize,const TO_N: usize>(fields:&str,from:&mut [T;FROM_N],to:&[T;TO_N]) -> Result<(), String> {

    let field_inds = get_field_inds(fields);

    if field_inds.len()>FROM_N {
        return Err("too many fields".to_string());
    }

    if HashSet::<char>::from_iter(field_inds.iter().map(|x|x.0)).len()!=field_inds.len() {
        return Err("duplicate fields".to_string());
    }

    for &(c,field) in field_inds.iter() {
        if field.is_none() || field.unwrap()>FROM_N {
            return Err(format!("invalid field '{c}'"));
        }
    }

    for (i,&(_,field)) in field_inds.iter().enumerate() {
        from[field.unwrap()]=to[i];
    }

    Ok(())
}
pub fn register_vec(lib_scope:&mut LibScope<World>) {

    //vec2(ivec2)
    lib_scope.method("vec2",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec2>()?;
        let a=Vec2::from_array(p0.to_array().map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().end();

    //vec3(ivec3)
    lib_scope.method("vec3",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec3>()?;
        let a=Vec3::from_array(p0.to_array().map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().end();

    //vec4(ivec4)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec4>()?;
        let a=Vec4::from_array(p0.to_array().map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec4>().end();

    //ivec2(vec2)
    lib_scope.method("ivec2",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let a=IVec2::from_array(p0.to_array().map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().end();

    //ivec3(vec3)
    lib_scope.method("ivec3",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec3>()?;
        let a=IVec3::from_array(p0.to_array().map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().end();

    //ivec4(vec4)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec4>()?;
        let a=IVec4::from_array(p0.to_array().map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec4>().end();

    //vec2(), vec2(f), vec2(f,f)
    lib_scope.method("vec2",|context|{
        let p0=context.param(0).as_float();
        let p1=context.get_param(1).map(|q|q.as_float()).unwrap_or(p0);
        let a=Vec2::from_array([p0,p1].map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    })
        .end()
        .float().end()
        .float().float().end();

    //vec3(), vec3(f), vec3(f,f,f)
    lib_scope.method("vec3",|context|{
        let p0=context.param(0).as_float();
        let p1=context.get_param(1).map(|q|q.as_float()).unwrap_or(p0);
        let p2=context.get_param(2).map(|q|q.as_float()).unwrap_or(p0);
        let a=Vec3::from_array([p0,p1,p2].map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    })
        .end()
        .float().end()
        .float().float().float().end();

    //vec4(), vec4(f), vec4(f,f,f,f)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_float();
        let p1=context.get_param(1).map(|q|q.as_float()).unwrap_or(p0);
        let p2=context.get_param(2).map(|q|q.as_float()).unwrap_or(p0);
        let p3=context.get_param(3).map(|q|q.as_float()).unwrap_or(p0);
        let a=Vec4::from_array([p0,p1,p2,p3].map(|q|q as f32));
        Ok(Value::custom_unmanaged(a))
    })
        .end()
        .float().end()
        .float().float().float().float().end();

    //vec3(v2,f)
    lib_scope.method("vec3",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let p1=context.param(1).as_float() as f32;
        let a=Vec3::from_array([p0[0],p0[1],p1]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().end();

    //vec3(f,v2)
    lib_scope.method("vec3",|context|{
        let p0=context.param(0).as_float() as f32;
        let p1=context.param(1).as_custom().data_clone::<Vec2>()?;
        let a=Vec3::from_array([p0,p1[0],p1[1]]);
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec2>().end();

    //vec4(v3,f)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec3>()?;
        let p1=context.param(1).as_float() as f32;
        let a=Vec4::from_array([p0[0],p0[1],p0[2],p1]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec3>().float().end();

    //vec4(v3,f)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_float() as f32;
        let p1=context.param(1).as_custom().data_clone::<Vec3>()?;
        let a=Vec4::from_array([p0,p1[0],p1[1],p1[2]]);
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec3>().end();

    //vec4(v2,v2)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let p1=context.param(1).as_custom().data_clone::<Vec2>()?;
        let a=Vec4::from_array([p0[0],p0[1],p1[0],p1[1]]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //vec4(v2,f,f)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<Vec2>()?;
        let p1=context.param(1).as_float() as f32;
        let p2=context.param(2).as_float() as f32;
        let a=Vec4::from_array([p0[0],p0[1],p1,p2]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<Vec2>().float().float().end();

    //vec4(f,f,v2)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_float() as f32 ;
        let p1=context.param(1).as_float() as f32;
        let p2=context.param(2).as_custom().data_clone::<Vec2>()?;
        let a=Vec4::from_array([p0,p1,p2[0],p2[1]]);
        Ok(Value::custom_unmanaged(a))
    }).float().float().custom_ref::<Vec2>().end();

    //vec4(f,v2,f)
    lib_scope.method("vec4",|context|{
        let p0=context.param(0).as_float() as f32;
        let p1=context.param(1).as_custom().data_clone::<Vec2>()?;
        let p2=context.param(2).as_float() as f32;
        let a=Vec4::from_array([p0,p1[0],p1[1],p2]);
        Ok(Value::custom_unmanaged(a))
    }).float().custom_ref::<Vec2>().float().end();

    //ivec2(), ivec2(i), ivec2(i,i)
    lib_scope.method("ivec2",|context|{
        let p0=context.param(0).as_int();
        let p1=context.get_param(1).map(|q|q.as_int()).unwrap_or(p0);
        let a=IVec2::from_array([p0,p1].map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).optional().int().int().end();

    //ivec3(), ivec3(i), ivec3(i,i,i)
    lib_scope.method("ivec3",|context|{
        let p0=context.param(0).as_int();
        let p1=context.get_param(1).map(|q|q.as_int()).unwrap_or(p0);
        let p2=context.get_param(2).map(|q|q.as_int()).unwrap_or(p0);
        let a=IVec3::from_array([p0,p1,p2].map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).end().int().end().int().int().int().end();

    //ivec4(), ivec4(i), ivec4(i,i,i,i)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_int();
        let p1=context.get_param(1).map(|q|q.as_int()).unwrap_or(p0);
        let p2=context.get_param(2).map(|q|q.as_int()).unwrap_or(p0);
        let p3=context.get_param(3).map(|q|q.as_int()).unwrap_or(p0);
        let a=IVec4::from_array([p0,p1,p2,p3].map(|q|q as i32));
        Ok(Value::custom_unmanaged(a))
    }).end().int().end().int().int().int().int().end();

    //ivec3(iv2,i)
    lib_scope.method("ivec3",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec2>()?;
        let p1=context.param(1).as_int() as i32;
        let a=IVec3::from_array([p0[0],p0[1],p1]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().end();

    //ivec3(i,iv2)
    lib_scope.method("ivec3",|context|{
        let p0=context.param(0).as_int() as i32;
        let p1=context.param(1).as_custom().data_clone::<IVec2>()?;
        let a=IVec3::from_array([p0,p1[0],p1[1]]);
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec2>().end();

    //ivec4(iv3,i)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec3>()?;
        let p1=context.param(1).as_int() as i32;
        let a=IVec4::from_array([p0[0],p0[1],p0[2],p1]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec3>().int().end();

    //ivec4(iv3,i)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_int() as i32;
        let p1=context.param(1).as_custom().data_clone::<IVec3>()?;
        let a=IVec4::from_array([p0,p1[0],p1[1],p1[2]]);
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec3>().end();

    //ivec4(iv2,iv2)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec2>()?;
        let p1=context.param(1).as_custom().data_clone::<IVec2>()?;
        let a=IVec4::from_array([p0[0],p0[1],p1[0],p1[1]]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //ivec4(iv2,i,i)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_custom().data_clone::<IVec2>()?;
        let p1=context.param(1).as_int() as i32;
        let p2=context.param(2).as_int() as i32;
        let a=IVec4::from_array([p0[0],p0[1],p1,p2]);
        Ok(Value::custom_unmanaged(a))
    }).custom_ref::<IVec2>().int().int().end();

    //ivec4(i,i,iv2)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_int() as i32;
        let p1=context.param(1).as_int() as i32;
        let p2=context.param(2).as_custom().data_clone::<IVec2>()?;
        let a=IVec4::from_array([p0,p1,p2[0],p2[1]]);
        Ok(Value::custom_unmanaged(a))
    }).int().int().custom_ref::<IVec2>().end();

    //ivec4(i,iv2,i)
    lib_scope.method("ivec4",|context|{
        let p0=context.param(0).as_int() as i32;
        let p1=context.param(1).as_custom().data_clone::<IVec2>()?;
        let p2=context.param(2).as_int() as i32;
        let a=IVec4::from_array([p0,p1[0],p1[1],p2]);
        Ok(Value::custom_unmanaged(a))
    }).int().custom_ref::<IVec2>().int().end();

    //string(vec2)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::string(format!("Vec2({}, {})",v[0],v[1])))
    }).custom_ref::<Vec2>().end();

    //string(vec3)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::string(format!("Vec3({}, {}, {})",v[0],v[1],v[2])))
    }).custom_ref::<Vec3>().end();

    //string(vec4)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::string(format!("Vec4({}, {}, {}, {})",v[0],v[1],v[2],v[3])))
    }).custom_ref::<Vec4>().end();

    //string(ivec2)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        Ok(Value::string(format!("IVec2({}, {})",v[0],v[1])))
    }).custom_ref::<IVec2>().end();

    //string(ivec3)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        Ok(Value::string(format!("IVec3({}, {}, {})",v[0],v[1],v[2])))
    }).custom_ref::<IVec3>().end();

    //string(ivec4)
    lib_scope.method("string",|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        Ok(Value::string(format!("IVec4({}, {}, {}, {})",v[0],v[1],v[2],v[3])))
    }).custom_ref::<IVec4>().end();

    //get_field(vec2,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..2 => Value::float(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<Vec2>().int().end();

    //get_field(vec3,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..3 => Value::float(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<Vec3>().int().end();

    //get_field(vec4,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..4 => Value::float(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<Vec4>().int().end();

    //get_field(vec2,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec2>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))

    }).custom_ref::<Vec2>().str().end();

    //get_field(vec3,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec3>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))
    }).custom_ref::<Vec3>().str().end();

    //get_field(vec4,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<Vec4>()?;
        let fields = context.param(1).as_string();
        get_field_util_float(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))
    }).custom_ref::<Vec4>().str().end();

    //get_field(ivec2,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..2 => Value::int(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<IVec2>().int().end();

    //get_field(ivec3,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..3 => Value::int(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<IVec3>().int().end();

    //get_field(ivec4,ind)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        let ind = context.param(1).as_int();
        Ok(match ind {0..4 => Value::int(v[ind as usize]), _ => Value::Nil,})
    }).custom_ref::<IVec4>().int().end();

    //get_field(ivec2,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec2>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))
    }).custom_ref::<IVec2>().str().end();

    //get_field(ivec3,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec3>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))
    }).custom_ref::<IVec3>().str().end();

    //get_field(ivec4,str)
    lib_scope.field(|context|{
        let v=context.param(0).as_custom().data_clone::<IVec4>()?;
        let fields = context.param(1).as_string();
        get_field_util_int(fields.as_str(), &v.to_array()).map_err(|e|context.error(e))
    }).custom_ref::<IVec4>().str().end();

    //set_field(vec2,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()].map(|q|q as f32);

        this.as_custom().with_data_mut_ext(|data:&mut Vec2|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(), &mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec2>().str().float().end();

    //set_field(vec3,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()].map(|q|q as f32);

        this.as_custom().with_data_mut_ext(|data:&mut Vec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec3>().str().float().end();

    //set_field(vec4,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_float()].map(|q|q as f32);

        this.as_custom().with_data_mut_ext(|data:&mut Vec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec4>().str().float().end();

    //set_field(vec2,str,vec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec2|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec2>().str().custom_ref::<Vec2>().end();

    //set_field(vec3,str,vec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec3>().str().custom_ref::<Vec2>().end();

    //set_field(vec3,str,vec3)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec3>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec4>().str().custom_ref::<Vec2>().end();

    //set_field(vec4,str,vec3)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec3>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec4>().str().custom_ref::<Vec3>().end();

    //set_field(vec4,str,vec4)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<Vec4>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut Vec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<Vec4>().str().custom_ref::<Vec4>().end();

    //set_field(ivec2,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()].map(|q|q as i32);

        this.as_custom().with_data_mut_ext(|data:&mut IVec2|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec2>().str().int().end();

    //set_field(ivec3,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()].map(|q|q as i32);

        this.as_custom().with_data_mut_ext(|data:&mut IVec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec3>().str().int().end();

    //set_field(ivec4,str,f)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = [context.param(2).as_int()].map(|q|q as i32);

        this.as_custom().with_data_mut_ext(|data:&mut IVec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec4>().str().int().end();

    //set_field(ivec2,str,ivec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec2|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec2>().str().custom_ref::<IVec2>().end();

    //set_field(ivec3,str,ivec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    })
        .custom_mut_ref::<IVec3>().str().custom_ref::<IVec2>().end();

    //set_field(ivec3,str,ivec3)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec3>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec3|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec3>().str().custom_ref::<IVec3>().end();

    //set_field(ivec4,str,ivec2)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec2>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec4>().str().custom_ref::<IVec2>().end();

    //set_field(ivec4,str,ivec3)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec3>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec4>().str().custom_ref::<IVec3>().end();

    //set_field(ivec4,str,ivec4)
    lib_scope.field(|context|{
        let this=context.param(0);
        let fields = context.param(1).as_string();
        let to = context.param(2).as_custom().data_clone::<IVec4>()?;
        let to=to.to_array();

        this.as_custom().with_data_mut_ext(|data:&mut IVec4|{
            let mut a=data.to_array();
            set_field_util(fields.as_str(),&mut a,&to).map_err(|e|context.error(e))?;
            *data=a.into();
            Ok(Value::Void)
        })
    }).custom_mut_ref::<IVec4>().str().custom_ref::<IVec4>().end();

    //neg(vec2)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<Vec2>().end();

    //neg(vec3)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<Vec3>().end();

    //neg(vec4)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<Vec4>().end();

    //neg(ivec2)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<IVec2>().end();

    //neg(ivec3)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<IVec3>().end();

    //neg(ivec4)
    lib_scope.method("-",|context|{
        let v = context.param(0).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(-v))
    }).custom_ref::<IVec4>().end();

    //add(vec2,vec2)
    lib_scope.method("+",|context|{
        let mut a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //add(vec3,vec3)
    lib_scope.method("+",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //add(vec4,vec4)
    lib_scope.method("+",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //add(ivec2,ivec2)
    lib_scope.method("+",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //add(ivec3,ivec3)
    lib_scope.method("+",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //add(ivec4,ivec4)
    lib_scope.method("+",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a+b))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //sub(vec2,vec2)
    lib_scope.method("-",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //sub(vec3,vec3)
    lib_scope.method("-",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //sub(vec4,vec4)
    lib_scope.method("-",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //sub(ivec2,ivec2)
    lib_scope.method("-",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //sub(ivec3,ivec3)
    lib_scope.method("-",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //sub(ivec4,ivec4)
    lib_scope.method("-",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a-b))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //mul(vec2,f)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec2>().float().end();

    //mul(f,vec2)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_float() as f32;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).float().custom_ref::<Vec2>().end();

    //mul(vec3,f)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec3>().float().end();

    //mul(f,vec3)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_float() as f32;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).float().custom_ref::<Vec3>().end();

    //mul(vec4,f)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec4>().float().end();

    //mul(f,vec4)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_float() as f32;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).float().custom_ref::<Vec4>().end();

    //mul(ivec2,i)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_float() as i32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec2>().int().end();

    //mul(f,ivec2)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_float() as i32;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).int().custom_ref::<IVec2>().end();

    //mul(ivec3,i)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec3>().int().end();

    //mul(i,ivec3)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_int() as i32;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).int().custom_ref::<IVec3>().end();

    //mul(ivec4,i)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec4>().int().end();

    //mul(i,ivec4)
    lib_scope.method("*",|context|{
        let a=context.param(0).as_int() as i32;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).int().custom_ref::<IVec4>().end();

    //mul(vec2,vec2)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //mul(vec3,vec3)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //mul(vec4,vec4)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //mul(ivec2,ivec2)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //mul(ivec3,ivec3)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //mul(ivec4,ivec4)
    lib_scope.method("*",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a*b))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //div(vec2,f)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec2>().float().end();

    //div(vec3,f)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec3>().float().end();

    //div(vec4,f)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec4>().float().end();

    //div(ivec2,i)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec2>().int().end();

    //div(ivec3,i)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec3>().int().end();

    //div(ivec4,i)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec4>().int().end();

    //div(vec2,vec2)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //div(vec3,vec3)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //div(vec4,vec4)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //div(ivec2,ivec2)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //div(ivec3,ivec3)
    lib_scope.method("/",|context|{
        let mut a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //div(ivec4,ivec4)
    lib_scope.method("/",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a/b))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //eq(vec2,vec2)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //eq(vec3,vec3)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //eq(vec4,vec4)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //eq(ivec2,ivec2)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //eq(ivec3,ivec3)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //eq(ivec4,ivec4)
    lib_scope.method("=",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::Bool(a==b))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //dot(vec2,vec2)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::float(a.dot(b)))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //dot(vec3,vec3)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::float(a.dot(b)))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //dot(vec4,vec4)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::float(a.dot(b)))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //dot(ivec2,ivec2)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::int(a.dot(b)))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //dot(ivec3,ivec3)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::int(a.dot(b)))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //dot(ivec4,ivec4)
    lib_scope.method("dot",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::int(a.dot(b)))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //cross(vec3,vec3)
    lib_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a.cross(b)))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //cross(ivec3,ivec3)
    lib_scope.method("cross",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a.cross(b)))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //len(vec2)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        Ok(Value::float(a.length()))
    }).custom_ref::<Vec2>().end();

    //len(vec3)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        Ok(Value::float(a.length()))
    }).custom_ref::<Vec3>().end();

    //len(vec4)
    lib_scope.method("len",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        Ok(Value::float(a.length()))
    }).custom_ref::<Vec4>().end();

    //norm(vec2)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let v=a.try_normalize().ok_or_else(||context.error("len is zero"))?;
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec2>().end();

    //norm(vec3)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let v=a.try_normalize().ok_or_else(||context.error("len is zero"))?;
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec3>().end();

    //norm(vec4)
    lib_scope.method("norm",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let v=a.try_normalize().ok_or_else(||context.error("len is zero"))?;
        Ok(Value::custom_unmanaged(v))
    }).custom_ref::<Vec4>().end();

    //min(vec2,f)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.min(Vec2::new(b, b))))
    }).custom_ref::<Vec2>().float().end();

    //min(vec3,f)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.min(Vec3::new(b, b, b))))
    }).custom_ref::<Vec3>().float().end();

    //min(vec4,f)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.min(Vec4::new(b, b,b,b))))
    }).custom_ref::<Vec4>().float().end();

    //min(ivec2,i)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.min(IVec2::new(b,b))))
    }).custom_ref::<IVec2>().int().end();

    //min(ivec3,i)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.min(IVec3::new(b,b,b))))
    }).custom_ref::<IVec3>().int().end();

    //min(ivec4,i)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.min(IVec4::new(b,b,b,b))))
    }).custom_ref::<IVec4>().int().end();

    //min(vec2,vec2)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //min(vec3,vec3)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //min(vec4,vec4)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //min(ivec2,ivec2)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //min(ivec3,ivec3)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //min(ivec4,ivec4)
    lib_scope.method("min",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a.min(b)))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //max(vec2,f)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.max(Vec2::new(b,b))))
    }).custom_ref::<Vec2>().float().end();

    //max(vec3,f)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.max(Vec3::new(b,b,b))))
    }).custom_ref::<Vec3>().float().end();

    //max(vec4,f)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float() as f32;
        Ok(Value::custom_unmanaged(a.max(Vec4::new(b,b,b,b))))
    }).custom_ref::<Vec4>().float().end();

    //max(ivec2,i)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.max(IVec2::new(b,b))))
    }).custom_ref::<IVec2>().int().end();

    //max(ivec3,i)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.max(IVec3::new(b,b,b))))
    }).custom_ref::<IVec3>().int().end();

    //max(ivec4,i)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int() as i32;
        Ok(Value::custom_unmanaged(a.max(IVec4::new(b,b,b,b))))
    }).custom_ref::<IVec4>().int().end();

    //max(vec2,vec2)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b = context.param(1).as_custom().data_clone::<Vec2>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //max(vec3,vec3)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b = context.param(1).as_custom().data_clone::<Vec3>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //max(vec4,vec4)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b = context.param(1).as_custom().data_clone::<Vec4>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //max(ivec2,ivec2)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b = context.param(1).as_custom().data_clone::<IVec2>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //max(ivec3,ivec3)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b = context.param(1).as_custom().data_clone::<IVec3>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //max(ivec4,ivec4)
    lib_scope.method("max",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b = context.param(1).as_custom().data_clone::<IVec4>()?;
        Ok(Value::custom_unmanaged(a.max(b)))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //clamp(vec2,f,f)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_float() as f32;
        let c=context.param(2).as_float() as f32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(Vec2::new(b,b), Vec2::new(c,c))))
    }).custom_ref::<Vec2>().float().float().end();

    //clamp(vec3,f,f)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_float() as f32;
        let c=context.param(2).as_float() as f32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(Vec3::new(b,b,b), Vec3::new(c,c,c))))
    }).custom_ref::<Vec3>().float().float().end();

    //clamp(vec4,f,f)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_float() as f32;
        let c=context.param(2).as_float() as f32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(Vec4::new(b,b,b,b), Vec4::new(c,c,c,c))))
    }).custom_ref::<Vec4>().float().float().end();

    //clamp(ivec2,i,i)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_int() as i32;
        let c=context.param(2).as_int() as i32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(IVec2::new(b,b), IVec2::new(c,c))))
    }).custom_ref::<IVec2>().int().int().end();

    //clamp(ivec3,i,i)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_int() as i32;
        let c=context.param(2).as_int() as i32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(IVec3::new(b,b,b), IVec3::new(c,c,c))))
    }).custom_ref::<IVec3>().int().int().end();

    //clamp(ivec4,i,i)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_int() as i32;
        let c=context.param(2).as_int() as i32;
        if b > c { return Err(context.error("min>max".to_string()));}
        Ok(Value::custom_unmanaged(a.clamp(IVec4::new(b,b,b,b), IVec4::new(c,c,c,c))))
    }).custom_ref::<IVec4>().int().int().end();

    //clamp(vec2,vec2,vec2)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec2>()?;
        let b=context.param(1).as_custom().data_clone::<Vec2>()?;
        let c=context.param(2).as_custom().data_clone::<Vec2>()?;
        for i in 0..2 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<Vec2>().custom_ref::<Vec2>().custom_ref::<Vec2>().end();

    //clamp(vec3,vec3,vec3)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec3>()?;
        let b=context.param(1).as_custom().data_clone::<Vec3>()?;
        let c=context.param(2).as_custom().data_clone::<Vec3>()?;
        for i in 0..3 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<Vec3>().custom_ref::<Vec3>().custom_ref::<Vec3>().end();

    //clamp(vec4,vec4,vec4)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<Vec4>()?;
        let b=context.param(1).as_custom().data_clone::<Vec4>()?;
        let c=context.param(2).as_custom().data_clone::<Vec4>()?;
        for i in 0..4 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<Vec4>().custom_ref::<Vec4>().custom_ref::<Vec4>().end();

    //clamp(ivec2,ivec2,ivec2)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec2>()?;
        let b=context.param(1).as_custom().data_clone::<IVec2>()?;
        let c=context.param(2).as_custom().data_clone::<IVec2>()?;
        for i in 0..2 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<IVec2>().custom_ref::<IVec2>().custom_ref::<IVec2>().end();

    //clamp(ivec3,ivec3,ivec3)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec3>()?;
        let b=context.param(1).as_custom().data_clone::<IVec3>()?;
        let c=context.param(2).as_custom().data_clone::<IVec3>()?;
        for i in 0..3 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<IVec3>().custom_ref::<IVec3>().custom_ref::<IVec3>().end();

    //clamp(ivec4,ivec4,ivec4)
    lib_scope.method("clamp",|context|{
        let a = context.param(0).as_custom().data_clone::<IVec4>()?;
        let b=context.param(1).as_custom().data_clone::<IVec4>()?;
        let c=context.param(2).as_custom().data_clone::<IVec4>()?;
        for i in 0..4 {if b[i]>c[i] {return Err(context.error("min>max".to_string()));}}
        Ok(Value::custom_unmanaged(a.clamp(b,c)))
    }).custom_ref::<IVec4>().custom_ref::<IVec4>().custom_ref::<IVec4>().end();

    //
    lib_scope.method("copy", |context|{
        let v:Vec2=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Vec2>().end();

    lib_scope.method("copy", |context|{
        let v:Vec3=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<Vec3>().end();

    lib_scope.method("copy", |context|{
        let v:Vec4=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    })
        .custom_ref::<Vec4>()
        .end();

    lib_scope.method("copy", |context|{
        let v:IVec2=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<IVec2>().end();

    lib_scope.method("copy", |context|{
        let v:IVec3=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<IVec3>().end();

    lib_scope.method("copy", |context|{
        let v:IVec4=context.param(0).as_custom().data_clone()?;
        Ok(Value::custom_unmanaged_mut(v))
    }).custom_ref::<IVec4>().end();


    lib_scope.method("type",|_|{
        Ok(Value::string("Vec2"))
    }).custom_ref::<Vec2>().end();
    lib_scope.method("type",|_|{
        Ok(Value::string("Vec3"))
    }).custom_ref::<Vec3>().end();
    lib_scope.method("type",|_|{
        Ok(Value::string("Vec4"))
    }).custom_ref::<Vec4>().end();
    lib_scope.method("type",|_|{
        Ok(Value::string("IVec2"))
    }).custom_ref::<IVec2>().end();
    lib_scope.method("type",|_|{
        Ok(Value::string("IVec3"))
    }).custom_ref::<IVec3>().end();
    lib_scope.method("type",|_|{
        Ok(Value::string("IVec4"))
    }).custom_ref::<IVec4>().end();
}
