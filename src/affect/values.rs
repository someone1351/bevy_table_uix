use std::{collections::HashMap, sync::Arc};

use bevy::ecs::prelude::*;

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone,PartialOrd, Ord)]
pub enum UixAffectState {
    // None,
    Select,
    Hover,
    Focus,
    Press(i32),
    Drag,
}

pub type AttribFuncType = Arc<dyn Fn(Entity,&mut World)+Send+Sync>;

// #[derive(Clone)]
// pub struct AttribFunc(pub AttribFuncType);
// impl std::fmt::Debug for AttribFunc {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         f.debug_tuple("ElementAttribFunc").finish()
//     }
// }


#[derive(PartialEq,Eq,Hash)]
pub enum DeviceType{None,Cursor(i32),Focus(i32),}

// type UixAffectAttribFunc = Arc<dyn Fn(&mut World,Entity,usize) + Sync+Send>;

pub struct UixAffectAttrib {
    // func : UixAffectAttribFunc,
    pub funcs:Vec<Arc<dyn Fn(Entity,&mut World,) + Sync+Send>>,
    pub states:HashMap<UixAffectState,usize>,//[state]=val_ind
}
