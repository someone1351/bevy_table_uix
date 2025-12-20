
use std::collections::{HashMap, HashSet};

use bevy::ecs::prelude::*;
use super::values::*;

#[derive(Component,Default)]
pub struct UixAffectComputed {
    pub states : HashMap<UixAffectState,HashSet<DeviceType>>, //[state][device]
    pub cur_attrib_inds:HashMap<usize,usize>, //[attrib_ind]=cur_ind
}

#[derive(Component,Default)]
// #[require(UiLayoutComputed)]
#[require(UixAffectComputed)]
pub struct UixAffect(pub Vec<UixAffectAttrib>); //[attrib_ind]=
// pub struct UixAffect {
//     // pub attribs : HashMap<Option<UiAffectState>,Vec<(AttribFuncType,Option<i32>)>>, //[state][attrib_ind]=(func,priority)
//     // pub attribs2 : Vec<HashMap<Option<UiAffectState>,(AttribFuncType,Option<i32>)>>, //attrib_ind][state]=(func,priority)
//     // pub attribs : Vec<HashMap<Option<UiAffectState>,(AttribFuncType,i32)>>, //[attrib_ind][state]=(func,priority)
//     pub attribs : Vec<(AttribFuncType,HashMap<UiAffectState,(AttribFuncType,i32)>)>, //[attrib_ind](default_func,[state]=(func,priority))
//     // pub states : HashMap<Option<UiAffectState>,>,
//     // pub states : HashSet<UiAffectState>,
//     pub states : HashMap<UiAffectState,HashSet<DeviceType>>, //[state][device]

//     // pub remove_states : BTreeSet<UiAffectState>,
// }
