use bevy::{color::Color, ecs::{component::Component, entity::Entity, hierarchy::ChildOf, world::World}};
use bevy_table_ui::{UiRoot, UiVal};
use script_lang::{FloatT, IntT, LibScope, MachineError, Value};

use super::components::*;

pub fn get_component2<T:Component>(world:&World,entity:Entity) -> Option<&T> {
    world.entity(entity).get()
}

pub fn set_component<T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(world:& mut World,entity:Entity,func:impl Fn(&mut T)) {
    let mut entity_mut=world.entity_mut(entity);
    let mut c_entry=entity_mut.entry::<T>().or_default();
    let mut c=c_entry.get_mut();
    func(&mut c);
}

pub fn script_to_ui_val(v:Value) -> Result<UiVal,MachineError> {
    (!v.is_nil()).then(||v.as_custom().data_clone()).unwrap_or(Ok(UiVal::None))
}


pub fn self_entity_from_world(world : &mut World,entity:Entity) -> Value {
    world.entity_mut(entity).entry::<UixSelf>().or_insert_with(||UixSelf::new(entity)).get().entity.to_weak().unwrap()
}



pub fn col_to_script_value(col:Color) -> Value {
    use bevy::prelude::ColorToComponents;
    Value::custom_unmanaged_mut(col.to_srgba().to_f32_array().map(|x|x as FloatT))
}
pub fn script_value_to_col(val:Value) -> Result<Color,MachineError> {
    let v=val.as_custom();

    if let Some(v)=v.get_data_clone::<[FloatT;4]>()? {
        let v=v.map(|x|x.clamp(0.0,1.0) as f32);
        Ok(Color::srgba(v[0],v[1],v[2],v[3]))
    } else if let Some(v)=v.get_data_clone::<[IntT;4]>()? {
        let v=v.map(|x|x.clamp(0,255) as u8);
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


pub fn entity_get_field(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,&World)->Value) {
    lib_scope.field_named(field,move|context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core();
        Ok(f(entity,world))
    }).custom_ref::<Entity>().end();
}

// pub fn entity_get_field2(field:&str,lib_scope:&mut LibScope<World>,f:fn(EntityRef)->Value) {
//     lib_scope.field_named(field,move|context|{
//         let entity:Entity = context.param(0).as_custom().data_clone()?;
//         let world=context.core();
//         let entity_ref=world.entity(entity);
//         Ok(f(entity_ref))
//     }).custom_ref::<Entity>().end();
// }
pub fn entity_get_field3<T:Component+Default+Clone>(field:&str,lib_scope:&mut LibScope<World>,f:fn(&T)->Value) {
    lib_scope.field_named(field,move|context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core();
        let e=world.entity(entity);
        let c=e.get::<T>().cloned().unwrap_or_default();
        Ok(f(&c))
    }).custom_ref::<Entity>().end();
}

pub fn entity_get_field_mut(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,&mut World)->Value) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let world=context.core_mut();
        Ok(f(entity,world))
    }).custom_ref::<Entity>().end();
}

pub fn entity_set_field_mut(field:&str,lib_scope:&mut LibScope<World>,f:fn(Entity,Value,&mut World,)->Result<(),MachineError>) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let to_val=context.param(1);
        let world=context.core_mut();
        f(entity,to_val,world)?;
        Ok(Value::Void)
    }).custom_ref::<Entity>().any().end();
}

pub fn entity_set_field_mut3<T:Component<Mutability = bevy::ecs::component::Mutable>+Default>(field:&str,lib_scope:&mut LibScope<World>,f:fn(&mut T,Value)->Result<(),MachineError>) {
    lib_scope.field_named(field,move|mut context|{
        let entity:Entity = context.param(0).as_custom().data_clone()?;
        let to_val=context.param(1);
        let world=context.core_mut();
        let mut e=world.entity_mut(entity);
        let mut c= e.entry::<T>().or_default();
        let mut c=c.get_mut(); //.get_mut();


        f(&mut c,to_val)?;
        Ok(Value::Void)
    }).custom_ref::<Entity>().any().end();
}
// fn entity_set_field_mut2<T:'static>(field:&str,lib_scope:&mut LibScope<World>,
//     fs:fn(Entity,T,&mut World,),
//     fv:fn(Value,)->Result<T,MachineError>,
// ) {
//     lib_scope.field_named(field,move|mut context|{
//         let entity:Entity = context.param(0).as_custom().data_clone()?;
//         let to_val=context.param(1);
//         let world=context.core_mut();
//         let v=fv(to_val)?;
//         fs(entity,v,world);
//         Ok(Value::Void)
//     }).custom_ref::<Entity>().any().end();
// }

pub fn get_ancestors(world:&World,entity:Entity) -> Vec<Entity> {
    let mut output=Vec::new();
    let mut x=world.entity(entity).get::<ChildOf>().map(|c|c.parent());

    while let Some(x2)=x {
        if world.entity(entity).get::<UiRoot>().is_some() {
            break;
        }

        output.push(x2);
        x=world.entity(x2).get::<ChildOf>().map(|c|c.parent());
    }

    output
}