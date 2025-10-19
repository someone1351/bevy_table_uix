
use std::collections::{HashMap, HashSet};

use std::ops::Range;


use crate::loading::{calc_attribs, calc_envs2, calc_node_creates};

use super::super::script_vals::*;


use super::vals::*;

#[derive(Clone)]
struct Work {
    element_ind:usize,
    parent_element_ind:Option<usize>,
    stub_element_ind:Option<usize>,
}

pub fn gen_stubs(elements:&Vec<Element>) -> Stuff {
    let mut all_stubs: HashMap<usize, Range<usize>> = HashMap::new(); //[root/stub_element_ind]=(nodes_start,nodes_end)
    // let mut all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>=Vec::new(); //(element_ind,parent_ind,attribs_start,attribs_end)
    let mut all_nodes=Vec::new();
    let mut all_init_attribs: Vec<AttribFuncType>=Vec::new(); //[]=func
    let mut all_names: Vec<script_lang::StringT>=Vec::new();
    let mut all_names_map = HashSet::<script_lang::StringT>::new();

    let mut all_state_attribs:HashMap<usize,HashMap<Option<UiAffectState>,(AttribFuncType,Option<i32>)>> = HashMap::new(); //[element_ind][state]=(func,priority)


    let tmp_envs=calc_envs2(elements,);
    let all_envs = tmp_envs.iter().map(|(&stub_element_ind,stub_envs)|{
        (stub_element_ind,stub_envs.iter().map(|(&element_ind,(by_ind,by_name))|(element_ind,StuffEnv{
            by_ind: by_ind.clone(),
            by_name: by_name.iter().map(|(&name,v)|(name.into(), v.clone())).collect(),
        })).collect())
    }).collect();

    let tmp_creates=calc_node_creates(elements); //[root/stub][node]=parent
    let tmp_attribs=calc_attribs(elements); //[element_ind]=attribs


            //env
        // let env_ind=match &cur_element.element_type {
        //     ElementType::Node {..}|ElementType::Apply {..}|ElementType::TemplateDecl {..}|ElementType::Stub {..} if cur_element.has_script => {

        //         let (by_ind,by_name)=&cald_envs[&cur_work.stub_element_ind.unwrap_or(0)][&cur_work.element_ind];
        //         let by_ind=by_ind.clone();
        //         let by_name=by_name.iter().map(|(&k,v)|(StringT::new(k),v.clone())).collect();

        //         all_envs.push(StuffEnv{ by_ind, by_name, });
        //         Some(all_envs.len()-1)
        //     } _=>{
        //         None
        //     }
        // };


    //
    //stubs[stub_element_ind]=(nodes_start,nodes_end)
    //nodes[ind]=(node_element_ind,parent_ind,attribs_start,attribs_end)
    //attribs[ind]=attrib_func

    for (&stub_element_ind,node_parents) in tmp_creates.iter() {
        let nodes_start=all_nodes.len();


        // println!("{node_parents:?}");
        //
        for &(node_element_ind,parent_element_ind) in node_parents.iter() {
            //
            let funcs:Option<&(Vec<AttribFuncType>,HashMap<Option<UiAffectState>,(AttribFuncType,Option<i32>)>)>=tmp_attribs.get(&node_element_ind);
            let attribs_start=all_init_attribs.len();

            if let Some((init_funcs,state_funcs))=funcs { //necessary?
                if !init_funcs.is_empty() { //necessary?
                    all_init_attribs.extend(init_funcs.iter().map(|x|x.clone()));
                }

                if !all_state_attribs.is_empty() {
                    all_state_attribs.entry(node_element_ind).or_default()
                        .extend(state_funcs.iter().map(|(x,(y,z))|(x.clone(),(y.clone(),z.clone()))));
                }
            }

            let attribs_end=all_init_attribs.len();

            let node_element=elements.get(node_element_ind).unwrap();
            let ElementType::Node { names, ..}=&node_element.element_type else {panic!("");};
            let names_start=all_names.len();


            for &n in names.iter() {
                let mut x=script_lang::StringT::new(n);

                if let Some(y)=all_names_map.get(&script_lang::StringT::new(n)).cloned() {
                    x=y;
                } else {
                    all_names_map.insert(x.clone());
                }

                all_names.push(x);
            }

            let names_end=all_names.len();

            //
            all_nodes.push(StuffNode {
                element_ind: node_element_ind,
                parent_element_ind: parent_element_ind,
                attribs: attribs_start..attribs_end,
                names: names_start..names_end,
            });
        }

        //
        let nodes_end=all_nodes.len();
        all_stubs.insert(stub_element_ind, nodes_start..nodes_end);

    }

    Stuff{  all_stubs, all_nodes, all_attribs: all_init_attribs, all_names, all_envs, all_state_attribs }
}
