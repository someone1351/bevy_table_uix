use std::{ collections::HashMap, ops::Range, sync::Arc};

use bevy::{ecs::{entity::Entity, world::World},  };
use script_lang::{StringT, Value};




#[derive(Clone)]
pub struct AttribFunc(pub Arc<dyn Fn(Entity,&mut World)+Send+Sync>);
impl std::fmt::Debug for AttribFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("ElementAttribFunc").finish()
    }
}

pub struct StuffEnv {
    pub by_ind : Vec<usize>, //[local_node_ind]=node_element_ind
    pub by_name : HashMap<StringT,Vec<usize>> //[node_name]=node_element_inds
}

pub struct StuffResultEnv {
    pub by_ind : Vec<Value>, //[local_node_ind]=node
    pub by_name : HashMap<StringT,Vec<Value>> //[node_name]=nodes
}
 //
 pub struct Stuff {
    // pub root_entity:Entity,
    pub all_stubs: HashMap<usize, Range<usize>> , //[root/stub_element_ind]=(nodes_start,nodes_end)
    pub all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>,//(element_ind,parent_element_ind,attribs_start_end,names_range)
    pub all_attribs:Vec<AttribFunc>, //[]=func
    pub all_names : Vec<StringT>,
    pub all_envs : Vec<StuffEnv>,
}

pub struct StuffResult {
    pub nodes : HashMap<usize,Value>, //[node_element_ind]=entity_val

    pub envs : Vec<Value>, //[env_ind]=env
    // pub ind_envs : Vec<Value>, //[local_node_ind]=node
    // pub name_envs : Vec<HashMap<StringT,Vec<Value>>> //[node_name]=nodes
}
