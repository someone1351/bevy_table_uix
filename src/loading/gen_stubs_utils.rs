
use std::collections::{BTreeMap, HashMap};



use crate::{affect::{AttribFuncType, UixAffectState}, loading::get_default_attribs };

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


pub fn calc_attribs<'a>(elements:& Vec<Element<'a>>) -> HashMap<usize,Vec<(AttribFuncType,HashMap<UixAffectState,(AttribFuncType,i32)>)>>
// HashMap<usize,(
//     Vec<AttribFuncType>,
//     // HashMap<Option<UiAffectState>,Vec<(AttribFuncType,Option<i32>)>>,
//     Vec<HashMap<Option<UiAffectState>,(AttribFuncType,i32)>>,
// )>
{
    let default_attribs=get_default_attribs();

    let mut work_stk=vec![Work{ element_ind: 0, parent_element_ind:None,stub_element_ind:None,}];


    let mut state_attribs:HashMap<usize,
        HashMap<&str,HashMap<Option<UixAffectState>,(AttribFuncType,i32)>>
    > = HashMap::new(); //[element_ind][name][state]=(func,priority)

    // let mut tmp_state_attribs:HashMap<usize,
    //     HashMap<&str,(AttribFuncType,HashMap<UiAffectState,(AttribFuncType,i32)>)>
    // > = HashMap::new(); //[element_ind][attrib_name]=(func,[state]=(func,priority))

    while let Some(cur_work)=work_stk.pop() {
        let cur_element=elements.get(cur_work.element_ind).unwrap();


        //
        match &cur_element.element_type {
            ElementType::Attrib{name, func, calcd,on_state,on_priority,..} if calcd.used => {
                let on_priority=on_priority.unwrap_or(0);
                // tmp_attribs.entry(cur_work.parent_element_ind.unwrap()).or_default().push(func.clone());

                state_attribs
                    .entry(cur_work.parent_element_ind.unwrap()).or_default()
                    .entry(name).or_default()
                    .entry(on_state.clone()).or_insert((func.0.clone(),on_priority))
                    ;
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

    //HashMap<usize,Vec<(AttribFuncType,HashMap<UiAffectState,(AttribFuncType,i32)>)>>
    //
    // let mut tmp_attribs:HashMap<usize,(
    //     Vec<AttribFuncType>,
    //     Vec<HashMap<Option<UiAffectState>,(AttribFuncType,i32)>>,
    // )> = HashMap::new(); //[element_ind]([attrib_ind]=attribs), [state]=(func,priority))

    let mut out_attribs:HashMap<usize,Vec<(AttribFuncType,HashMap<UixAffectState,(AttribFuncType,i32)>)>> = HashMap::new(); //[element_ind][attrib_ind]=(func,state_funcs) //state_funcs[state]=(func,priority)

    for (element_ind,attribs) in state_attribs {
        // let mut tmp_state_attribs:Vec<HashMap<UiAffectState,(AttribFuncType,i32)>> = HashMap::new();
        // let mut tmp_init_attribs:Vec<AttribFuncType> = Vec::new();

        // let cur_out_attribs=out_attribs.entry(element_ind).or_default();
        let mut cur_out_attribs = Vec::new();

        for (name,states) in attribs {
            let default_func=states.get(&None).cloned().map(|x|x.0).unwrap_or_else(||default_attribs.get(name).unwrap().clone());

            let mut out_state_attribs:HashMap<UixAffectState,(AttribFuncType,i32)> = HashMap::new();

            for (state,v) in states.iter() {
                if let Some(state)=state {
                    out_state_attribs.insert(state.clone(), v.clone());

                }
            }

            cur_out_attribs.push((default_func,out_state_attribs));
        }

        out_attribs.insert(element_ind, cur_out_attribs);

    }
    out_attribs

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
