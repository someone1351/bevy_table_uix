
#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use bevy::ecs::{component::Component, entity::Entity, world::World};

// use bevy::ecs::prelude::*;
use super::values::*;

pub fn create_affect_attrib<C,V,S>(func:fn(&mut C,V) ,default_val:V,state_vals:S,) -> UixAffectAttrib
where
    C : Component<Mutability = bevy::ecs::component::Mutable>+Default,
    V : Clone + 'static+Send+Sync,
    S : IntoIterator<Item=(UixAffectState,V)>,
{
    let state_vals:Vec<(UixAffectState,V)>=state_vals.into_iter().collect();
    let states:HashMap<UixAffectState,usize>=state_vals.iter().enumerate().map(|(i,(k,_v))|(*k,i+1)).collect();
    // let vals:Vec<V>=[default_val].into_iter().chain(state_vals.iter().map(|(_k,v)|v.clone())).collect();

    let mut out_funcs = Vec::new();

    for v in [default_val.clone()].into_iter().chain(state_vals.iter().map(|(_k,v)|v.clone())) {
        let func2:AttribFuncType=Arc::new(move|entity:Entity,world:&mut World,|{
            let mut e=world.entity_mut(entity);
            let mut c=e.entry::<C>().or_default();
            let mut c=c.get_mut();

            func(&mut c,v.clone());
        });
        out_funcs.push(func2.clone());
    }

    // let out_funcs: Vec<AttribFuncType>=[default_val.clone()].into_iter().chain(state_vals.iter().map(|(_k,v)|v.clone())).map(|v|{
    //     Arc::new(move|entity:Entity,world:&mut World,|{
    //         let mut e=world.entity_mut(entity);
    //         let mut c=e.entry::<C>().or_default();
    //         let mut c=c.get_mut();

    //         func(&mut c,v.clone());
    //     })
    // }).collect();

    UixAffectAttrib { funcs:out_funcs, states }
}
