
use std::collections::{HashMap, HashSet};
use std::collections::BTreeMap;

use std::ops::Range;

use script_lang::StringT;

use super::super::script_vals::*;


use super::vals::*;

#[derive(Clone)]
struct Work {
    element_ind:usize,
    parent:Option<usize>,
    stub:Option<usize>,
}

pub fn gen_stubs(elements:&Vec<Element>) -> Stuff {
    let mut all_stubs: HashMap<usize, Range<usize>> = HashMap::new(); //[root/stub_element_ind]=(nodes_start,nodes_end)
    let mut all_nodes: Vec<(usize,usize,Range<usize>,Range<usize>)>=Vec::new(); //(element_ind,parent_ind,attribs_start,attribs_end)
    let mut all_attribs: Vec<AttribFunc>=Vec::new(); //[]=func
    let mut all_names: Vec<script_lang::StringT>=Vec::new();
    let mut all_names_map = HashSet::<script_lang::StringT>::new();
    let mut all_envs = Vec::new();
    //


    let mut work_stk=vec![Work{ element_ind: 0, parent:None,stub:None,}];
    let mut creates:BTreeMap<usize,BTreeMap<usize,usize>>= BTreeMap::new(); //[root/stub][node]=parent
    let mut attribs:HashMap<usize,Vec<AttribFunc>> = HashMap::new(); //[element_ind]=attribs
    let mut element_ind_inds: HashMap<usize,usize>=HashMap::new(); //[element_ind]=ind;

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        //env
        match &cur_element.element_type { ElementType::Node {..}|ElementType::Apply {..}|ElementType::TemplateDecl {..}|ElementType::Stub {..}  => {
            if cur_element.has_script {
                let mut cur_env_nameds:HashMap<StringT,Vec<usize>>=HashMap::new();
                let mut cur_env_inds: Vec<usize> = Vec::new();

                for &child_element_ind in cur_element.children.iter() {
                    let child_element=elements.get(child_element_ind).unwrap();
                    let ElementType::Node { names, ..}=&child_element.element_type else {continue;};

                    cur_env_inds.push(child_element_ind);

                    for &name in names {
                        cur_env_nameds.entry(name.into()).or_default().push(child_element_ind); //todo use string map/cache for name
                    }
                }

                //envs
                all_envs.push(StuffEnv{ by_ind: cur_env_inds, by_name: cur_env_nameds });
            }
        } _=>{}}


        //
        match &cur_element.element_type {
            ElementType::Node{..}=> {
                if let Some(parent)=cur_work.parent {
                    let stub=cur_work.stub.unwrap_or(0);
                    creates.entry(stub).or_default().insert(cur_work.element_ind,parent);
                }

                //
                let ind=element_ind_inds.len();
                element_ind_inds.insert(cur_work.element_ind,ind);
            }
            ElementType::Attrib{func, calcd,..}=> {
                if calcd.used {
                    attribs.entry(cur_work.parent.unwrap()).or_default().push(func.clone());
                }
            }
            _=>{}
        }

        if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
            let stub=if let ElementType::Stub{..}=&cur_element.element_type {
                Some(cur_work.element_ind)
            } else {
                cur_work.stub
            };

            let parent=if let ElementType::Node{..}=&cur_element.element_type {
                if Some(cur_work.element_ind)==stub { //if parent is stub, then parent = 0
                    Some(0)
                } else {
                    Some(cur_work.element_ind)
                }
            } else {
                cur_work.parent
            };

            work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                element_ind: child, parent, stub,
            }));
        }
    }

    //
    //stubs[stub_element_ind]=(nodes_start,nodes_end)
    //nodes[ind]=(node_element_ind,parent_ind,attribs_start,attribs_end)
    //attribs[ind]=attrib_func

    for (&stub_element_ind,node_parents) in creates.iter() {
        let nodes_start=all_nodes.len();
        // println!("{node_parents:?}");
        //
        for (&node_element_ind,&parent_element_ind) in node_parents.iter() {
            //
            let funcs=attribs.get(&node_element_ind);
            let attribs_start=all_attribs.len();

            if let Some(funcs)=funcs {
                all_attribs.extend(funcs.iter().map(|x|x.clone()));
            }

            let attribs_end=all_attribs.len();

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
            all_nodes.push((node_element_ind,parent_element_ind,attribs_start..attribs_end,names_start..names_end));
        }

        //
        let nodes_end=all_nodes.len();
        all_stubs.insert(stub_element_ind, nodes_start..nodes_end);

    }

    Stuff{  all_stubs, all_nodes, all_attribs, all_names, all_envs }
}
