use std::{ collections::HashMap, ops::Range};

// use bevy::{ecs::{entity::Entity, world::World},  };
use script_lang::{StringT, Value};

use crate::affect::{AttribFuncType, UixAffectState};


#[derive(Clone)]
pub struct AttribFunc(pub AttribFuncType);
impl std::fmt::Debug for AttribFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ElementAttribFunc").finish()
    }
}

#[derive(Clone)]
pub struct StuffEnv {
    pub by_ind : Vec<usize>, //[ind]=node_element_ind
    pub by_name : HashMap<StringT,Vec<usize>> //[node_name]=node_element_inds
}

pub struct StuffNode {
    pub element_ind:usize,
    pub parent_element_ind:usize,
    pub attribs:Range<usize>,
    pub names:Range<usize>,
    // pub env:Option<usize>,
}
 //
 pub struct Stuff {
    // pub root_entity:Entity,
    pub all_stubs: HashMap<usize, Range<usize>> , //[root/stub_element_ind]=(nodes_start,nodes_end)
    // pub all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>,//(element_ind,parent_element_ind,attribs_start_end,names_range)
    pub all_nodes:Vec<StuffNode>,
    pub all_attribs:Vec<AttribFuncType>, //[]=func
    pub all_names : Vec<StringT>,
    pub all_envs : HashMap<usize, HashMap<usize, StuffEnv>>, //[root/stub_element_ind][element_ind]=env

    // pub all_state_attribs:HashMap<usize,HashMap<Option<UiAffectState>,Vec<(AttribFuncType,Option<i32>)>>>, //[element_ind][state][attrib_ind]=(func,priority)
    // pub all_state_attribs:HashMap<usize,Vec<HashMap<Option<UiAffectState>,(AttribFuncType,i32)>>>, //[element_ind][attrib_ind][state]=(func,priority)
    pub all_state_attribs:HashMap<usize,Vec<(AttribFuncType,HashMap<UixAffectState,(AttribFuncType,i32)>)>>, //[element_ind][attrib_ind]=(default_func,[state]=(func,priority))
    /*

    * could replace all_attribs with all_state_attribs
    ** store all_state_attribs like all_attribs currently is
    ** replace state hashmap with vec?

    */

}

pub struct StuffResultEnv {
    pub by_ind : Vec<Value>, //[local_node_ind]=node
    pub by_name : HashMap<StringT,Vec<Value>> //[node_name]=nodes
}

pub struct StuffResult {
    pub nodes : HashMap<usize,Value>, //[node_element_ind]=entity_val
    // pub envs : HashMap<usize,Value>, //[node_element_ind]=entity_val

    pub envs : HashMap<usize,Value>, //[element_ind]=env
    // pub ind_envs : Vec<Value>, //[local_node_ind]=node
    // pub name_envs : Vec<HashMap<StringT,Vec<Value>>> //[node_name]=nodes
}
