

use super::vals::*;

#[derive(Clone)]
struct Work {
    element_ind:usize,
    exit:bool,
    parent:Option<usize>,
    in_template_or_apply_decl:bool,
}

pub fn calc_env_params(elements:&mut Vec<Element>) {
    // let mut work_stk=vec![Work { element_ind:0, exit:false, parent:None,in_template_or_apply_decl:false, }];

    // while let Some(cur_work)=work_stk.pop() {
    //     let cur_element=elements.get(cur_work.element_ind).unwrap();

    //     //on enter
    //     if !cur_work.exit {
    //         work_stk.push(Work{exit:true, ..cur_work.clone()});

    //         let in_template_or_apply_decl=if let ElementType::TemplateDecl{..}|ElementType::Apply{..}=
    //             &cur_element.element_type {true} else {cur_work.in_template_or_apply_decl};

    //         work_stk.extend(cur_element.children.iter().rev().map(|&element_ind|Work {
    //             element_ind,
    //             exit:false,
    //             parent:Some(cur_work.element_ind),
    //             in_template_or_apply_decl,
    //         }));
    //     }

    //     //not root
    //     let Some(parent_element_ind)=cur_work.parent else { continue; };

    //     //
    //     if !cur_work.exit { //on enter
    //         match &cur_element.element_type {
    //             ElementType::Node { .. }  => {
    //                 let parent_element=elements.get_mut(parent_element_ind).unwrap();
    //                 parent_element.calcd_node_params.insert(cur_work.element_ind);
    //             }
    //             ElementType::Apply { .. }  => {
    //             }
    //             ElementType::TemplateUse{ template_decl_element_ind, .. } if cur_work.in_template_or_apply_decl  => {
    //                 let decl_element=elements.get(*template_decl_element_ind).unwrap();
    //                 let params=decl_element.calcd_node_params.clone();

    //                 let cur_element=elements.get_mut(cur_work.element_ind).unwrap();
    //                 cur_element.calcd_node_params.extend(params);
    //             }
    //             _ =>  {}
    //         }
    //     } else { //on exit
    //         match &cur_element.element_type {
    //             ElementType::Stub { .. }|ElementType::Apply { .. }|ElementType::ApplyUse{..}|ElementType::TemplateDecl { .. } => { }
    //             _ => {
    //                 //add cur element's params to parent
    //                 let cur_nodes_params=cur_element.calcd_node_params.clone();
    //                 let parent_element=elements.get_mut(parent_element_ind).unwrap();
    //                 parent_element.calcd_node_params.extend(cur_nodes_params);
    //             }
    //         }
    //     }
    // }
}

