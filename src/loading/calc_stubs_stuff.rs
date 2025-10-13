
use std::collections::{BTreeMap, HashMap};


use crate::script_vals::AttribFunc;

use super::vals::*;

struct Work {
    element_ind:usize,
    parent_element_ind:Option<usize>,
    stub_element_ind:Option<usize>,
}

pub fn calc_envs2<'a>(elements:& Vec<Element<'a>>, ) -> HashMap<usize, HashMap<usize, (Vec<usize>,HashMap<&'a str,Vec<usize>>)>> {//[stub][env]=(by_ind,by_name)
    //todo, skip envs that are never used, ie non stub that don't have env_script?
    let mut output: HashMap<usize, HashMap<usize, (Vec<usize>,HashMap<&'a str,Vec<usize>>)>> = HashMap::new();

    let mut work_stk=vec![Work{ element_ind: 0, parent_element_ind:None,stub_element_ind: None }];

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

        //push children
        {
            let stub_element_ind=match &cur_element.element_type {
                ElementType::Apply {..}|ElementType::TemplateDecl {..} => { continue; }, //don't bother with nodes in decls, only actual nodes
                ElementType::Stub { .. } => Some(cur_work.element_ind),
                _ => cur_work.stub_element_ind,
            };

            let parent_element_ind=Some(cur_work.element_ind);
            work_stk.extend(cur_element.children.iter().map(|&element_ind|Work {element_ind,parent_element_ind,stub_element_ind,}).rev());
        }

        // //
        // if only_script && !cur_element.has_env_script {
        //     continue;
        // }

        //
        let Some(parent_element_ind)=cur_work.parent_element_ind else {continue;};
        let ElementType::Node { names, .. } =&cur_element.element_type else {continue;};

        //
        let stub_output=output.entry(cur_work.stub_element_ind.unwrap_or(0)).or_default(); //[env]
        let env_output=stub_output.entry(parent_element_ind).or_default();
        let (by_inds,by_names)=env_output;

        //
        by_inds.push(cur_work.element_ind);

        //
        for &name in names.iter() {
            let by_name=by_names.entry(name).or_default();
            by_name.push(cur_work.element_ind);
        }
    }

    output
}


pub fn calc_attribs<'a>(elements:& Vec<Element<'a>>) -> HashMap<usize,Vec<AttribFunc>> {
    let mut work_stk=vec![Work{ element_ind: 0, parent_element_ind:None,stub_element_ind:None,}];
    let mut tmp_attribs:HashMap<usize,Vec<AttribFunc>> = HashMap::new(); //[element_ind]=attribs


    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

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
        match &cur_element.element_type {
            ElementType::Attrib{func, calcd,..} if calcd.used => {
                tmp_attribs.entry(cur_work.parent_element_ind.unwrap()).or_default().push(func.clone());
            }
            _=>{}
        }

        if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
            let stub=if let ElementType::Stub{..}=&cur_element.element_type {
                Some(cur_work.element_ind)
            } else {
                cur_work.stub_element_ind
            };

            let parent=if let ElementType::Node{..}=&cur_element.element_type {
                if Some(cur_work.element_ind)==stub { //if parent is stub, then parent = 0
                    Some(0)
                } else {
                    Some(cur_work.element_ind)
                }
            } else {
                cur_work.parent_element_ind
            };

            work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                element_ind: child, parent_element_ind: parent, stub_element_ind: stub,
            }));
        }
    }
    tmp_attribs
}


pub fn calc_node_creates<'a>(elements:& Vec<Element<'a>>) -> BTreeMap<usize,Vec<(usize,usize)>> {
    let mut work_stk=vec![Work{ element_ind: 0, parent_element_ind:None,stub_element_ind:None,}];

    let mut tmp_creates:BTreeMap<usize,Vec<(usize,usize)>>= BTreeMap::new(); //[root/stub element_ind]=(node_element_ind,parent_element_ind)


    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();

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
        match &cur_element.element_type {
            ElementType::Node{..}=> {
                if let Some(parent)=cur_work.parent_element_ind {
                    let stub=cur_work.stub_element_ind.unwrap_or(0);
                    tmp_creates.entry(stub).or_default().push((cur_work.element_ind,parent));
                }

            }
            _=>{}
        }

        if let ElementType::Node{..}|ElementType::Stub{..}|ElementType::ApplyUse{..}|ElementType::TemplateUse{..}=&cur_element.element_type {
            let stub=if let ElementType::Stub{..}=&cur_element.element_type {
                Some(cur_work.element_ind)
            } else {
                cur_work.stub_element_ind
            };

            let parent=if let ElementType::Node{..}=&cur_element.element_type {
                if Some(cur_work.element_ind)==stub { //if parent is stub, then parent = 0
                    Some(0)
                } else {
                    Some(cur_work.element_ind)
                }
            } else {
                cur_work.parent_element_ind
            };

            work_stk.extend(cur_element.children.iter().rev().map(|&child|Work {
                element_ind: child, parent_element_ind: parent, stub_element_ind: stub,
            }));
        }
    }

    tmp_creates
}
