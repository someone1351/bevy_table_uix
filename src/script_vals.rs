use std::{ collections::HashMap, ops::Range, sync::Arc};

use bevy::{ecs::{entity::Entity, world::World},  };
use script_lang::Value;




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
    pub all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>,//(element_ind,parent_element_ind,attribs_start_end,names_range)
    pub all_attribs:Vec<AttribFunc>, //[]=func
    pub all_names : Vec<script_lang::StringT>,
}

pub struct StuffResult(pub HashMap<usize,Value>);
